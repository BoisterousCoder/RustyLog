use crdts::glist::{GList, Op};
use crdts::CmRDT;

use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use std::fs;
use std::fs::File;
use std::error::Error;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::io::prelude::*;


const SAVE_DIR:&str = "./saves";
const FILE_EXT:&str = "msglog";
const DATE_FMT:&str = "%Y %b %d %H:%M:%S%.3f %z";

fn fmt_file_name(name:&str, group:&str) -> String{
    let filename = calc_hash(&format!("{}@{}", name, group));
    format!("{}/{}.{}", SAVE_DIR, filename, FILE_EXT)
}

fn calc_hash(text:&str) -> String{
    let mut hasher = DefaultHasher::new();
    text.to_string().hash(&mut hasher);
    format!("{}", hasher.finish())
}

fn read_store(filename:&str, password:&str) -> Result<LogState, Box<dyn Error>>{
 	let mut file = File::open(filename)?;
 	let mut data = String::new();
 	file.read_to_string(&mut data)?;
 	let key = new_magic_crypt!(password, 256);
 	let decrypted_data = key.decrypt_base64_to_string(&data)?;

 	return Ok(serde_json::from_str(&decrypted_data)?);
}

pub fn is_store_existing(filename:&str) -> bool{
 	return fs::metadata(filename).is_ok();
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LogState {
    msgs:GList<MessageData>,
    decrypted_data:Vec<DecryptedData>,
	pub group:String,
	pub name:String,
	pub id_keys: KeyPair
}
impl LogState {
    pub fn new(name:&str, group:&str, password:&str) -> LogState{
        let filename = fmt_file_name(name, group);
        if is_store_existing(&filename){
            match read_store(&filename, password) {
                Ok(state) => return state,
                Err(e) => println!("Failed to read {}. Prog is creating a new state. The error is as follows:\n{}", filename, e)
            };
        }
        LogState{
            msgs: GList::<MessageData>::new(),
            decrypted_data: Vec::new(),
            group:group.to_string(),
            name:name.to_string(),
            id_keys: KeyPair{
                public:"".to_string(),//TODO: Change this once RSA is added
                private:"".to_string(),//TODO: Change this once RSA is added
            }
        }
    }
    pub fn filename(&self) -> String {
	 	fmt_file_name(&self.name, &self.group)
	}
	pub fn save(&self, password:&str) -> Result<(), Box<dyn Error>>{
        let mut file = File::create(self.filename())?;
     	let key = new_magic_crypt!(password, 256);
     	let data = key.encrypt_str_to_base64(serde_json::to_string(self)?);

     	file.write_all(data.as_bytes())?;
     	file.sync_all()?;
        Ok(())
	}
	pub fn messages(&self) -> Vec<&MessageData> {
	    return self.msgs.iter().map(|msg_id| msg_id.value()).collect();
	}
	pub fn decrypt(&mut self, msg:&MessageData, content:&str){
	    self.decrypted_data.push(DecryptedData{
            id:msg.get_id(),
            content:content.to_string()
	    });
	}
	pub fn decrypted(&self, msg:&MessageData) -> Option<String>{
	    let msg_id = msg.get_id();
	    for decrypted_data in &self.decrypted_data {
	        if decrypted_data.id == msg_id {
	            return Some(decrypted_data.content.clone());
	        }
	    }
	    None
	}
	pub fn add_message(&mut self, msg:MessageData) -> Option<String>{
	    for other_msg_id in self.msgs.iter(){
	        if other_msg_id.value() == &msg{
	            return None;
	        }else if other_msg_id.value() > &msg{
	            let op = self.msgs.insert_after(Some(other_msg_id), msg);
	            self.msgs.apply(op.clone());
                return Some(serde_json::to_string(&op).unwrap());
	        }
	    }
	    let op = self.msgs.insert_after(self.msgs.last(), msg);
        self.msgs.apply(op.clone());
        return Some(serde_json::to_string(&op).unwrap());
	}

	pub fn apply_op(&mut self, op_data:&str) -> Result<(), Box<dyn Error>>{
	    let op:Op<MessageData> = serde_json::from_str(op_data)?;
	    self.msgs.apply(op);
	    Ok(())
	}
	pub fn delete_store(&self) -> std::io::Result<()>{
	    fs::remove_file(&self.filename())?;
	    Ok(())
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageData {
    pub from:String,
    pub tag:String,
    pub content:String,
    pub signature:String,
    pub signed_time_stamp:String//This is not actually signed until after I impliment the rsa stuff
}

impl MessageData {
    fn get_id(&self) -> u64{
        let mut hasher = DefaultHasher::new();
        format!("{}{}", self.from, self.signed_time_stamp).hash(&mut hasher);
        return hasher.finish()
    }
    fn get_time_stamp(&self) -> DateTime<FixedOffset>{
        //TODO: Remove Server Signature here
        match DateTime::parse_from_str(&self.signed_time_stamp, DATE_FMT) {
            Ok(x) => return x,
            Err(e) => {
                panic!("Unable to parse date {}. \n The err reads as follows:\n {}", self.signed_time_stamp, e);
            },
        }
    }
}

impl PartialEq for MessageData {
    fn eq(&self, other: &Self) -> bool {
        self.get_time_stamp() == other.get_time_stamp() && self.from == other.from
    }
}

impl Eq for MessageData {}

impl PartialOrd for MessageData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let time_order = self.get_time_stamp().cmp(&other.get_time_stamp());
        return if Ordering::Equal == time_order {
            Some(self.from.cmp(&other.from))
        }else{ Some(time_order) }
    }
}

impl Ord for MessageData {
    fn cmp(&self, other: &Self) -> Ordering {
        let time_order = self.get_time_stamp().cmp(&other.get_time_stamp());
        return if Ordering::Equal == time_order {
            self.from.cmp(&other.from)
        }else{ time_order }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DecryptedData {
    pub id:u64,
    pub content:String
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct KeyPair {
    pub public:String,
	pub private:String
}

#[cfg(test)]
mod tests;
