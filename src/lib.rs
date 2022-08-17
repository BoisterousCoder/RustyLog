use crdts::glist::{GList, Op};
use crdts::identifier::Identifier;
use crdts::CmRDT;

use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use std::fs;
use std::fs::File;
use std::error::Error;
use std::cmp::Ordering;
use std::iter::Iterator;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::io::prelude::*;


const SAVE_DIR:&str = ".";
const FILE_EXT:&str = ".rstore";
const DATE_FMT:&str = "%Y %b %d %H:%M:%S%.3f %z";

fn fmt_file_name(name:&str, group:&str) -> String{
    format!("{}/{}@{}.{}", SAVE_DIR, calc_hash(name), calc_hash(group), FILE_EXT)
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
    pub msgs:GList<MessageData>,
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
            group:group.to_string(),
            name:name.to_string(),
            id_keys: KeyPair{
                public:"".to_string(),//TODO: Change this once RSA is added
                private:"".to_string(),//TODO: Change this once RSA is added
            }
        }
    }
    pub fn get_filename(&self) -> String {
	 	fmt_file_name(&self.name, &self.group)
	}
	pub fn save(&self, password:&str) -> Result<(), Box<dyn Error>>{
        let mut file = File::create(self.get_filename())?;
     	let key = new_magic_crypt!(password, 256);
     	let data = key.encrypt_str_to_base64(serde_json::to_string(self)?);

     	file.write_all(data.as_bytes())?;
     	file.sync_all()?;
        Ok(())
	}

	pub fn add_message(&mut self, msg:MessageData) -> Op<MessageData>{
	    //let msgs: Vec<&MessageData> = self.msgs.read().collect();
	    for other_msg_id in self.msgs.iter(){
	        if other_msg_id.value().get_time_stamp() > msg.get_time_stamp(){
	            let op = self.msgs.insert_after(Some(other_msg_id), msg);
	            self.msgs.apply(op.clone());
                return op;
	        }
	    }
	    let op = self.msgs.insert_after(self.msgs.last(), msg);
        self.msgs.apply(op.clone());
        return op;
	}
	pub fn delete_store(&self) -> std::io::Result<()>{
	    fs::remove_file(&self.get_filename())?;
	    Ok(())
	}
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct KeyPair {
    pub public:String,
	pub private:String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageData {
    pub from:String,
    pub tag:String,
    pub encrypted_content:String,
    pub signature:String,
    pub signed_time_stamp:String
}

impl MessageData {
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

#[cfg(test)]
mod tests;
