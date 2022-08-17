use crdts::glist::{GList};
use crdts::CmRDT;
use std::error::Error;
use std::cmp::Ordering;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

mod save;

const FILE_EXT:&str = ".rstore";
const DATE_FMT:&str = "%Y %b %d %H:%M:%S%.3f %z";

fn fmt_file_name(name:&str, group:&str) -> String{
    format!("{}@{}.{}", name, group, FILE_EXT)
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
        if save::is_store_existing(&filename){
            match save::read_store(&filename, password) {
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
        save::write_store(self, password)?;
        Ok(())
	}

	pub fn add_message(&mut self, msg:MessageData){
        self.msgs.apply(self.msgs.insert_after(self.msgs.last(), msg))
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
    pub signed_time_stamp:String,
    pub content:Option<String>
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
