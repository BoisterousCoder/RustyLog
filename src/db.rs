use crdts::glist::{GList};
use crdts::CmRDT;
use std::iter::Iterator;
use std::error::Error;
use std::cmp::Ordering;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::save;

const FILE_EXT:&str = ".rstore";
const DATE_FMT:&str = "%Y %b %d %H:%M:%S%.3f %z";

fn fmtFileName(name:&str, group:&str) -> String{
    format!("{}@{}.{}", name, group, FILE_EXT)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DB {
    pub msgs:GList<MessageData>,
	pub group:String,
	pub name:String,
	pub idKeys: KeyPair
}
impl DB {
    pub fn new(name:&str, group:&str, password:&str) -> DB{
        let fileName = fmtFileName(name, group);
        if save::isStoreExisting(&fileName){
            match save::readStore(&fileName, password, FILE_EXT) {
                Ok(db) => return db,
                Err(e) => println!("Failed to read {}. Prog is creating a new db. The error is as follows:\n{}", fileName, e)
            };
        }
        DB{
            msgs: GList::<MessageData>::new(),
            group:group.to_string(),
            name:name.to_string(),
            idKeys: KeyPair{
                public:"".to_string(),//TODO: Change this once RSA is added
                private:"".to_string(),//TODO: Change this once RSA is added
            }
        }
    }
    pub fn getFileName(&self) -> String {
	 	fmtFileName(&self.name, &self.group)
	}
	pub fn save(&self, password:&str) -> Result<(), Box<dyn Error>>{
        save::writeStore(self, password)?;
        Ok(())
	}

	pub fn addMessage(&mut self, msg:MessageData){
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
    pub encryptedContent:String,
    pub signature:String,
    pub signedTimeStamp:String,
    pub content:Option<String>
}

impl MessageData {
    fn getTimeStamp(&self) -> DateTime<FixedOffset>{
        //TODO: Remove Server Signature here
        match DateTime::parse_from_str(&self.signedTimeStamp, DATE_FMT) {
            Ok(x) => return x,
            Err(e) => {
                panic!("Unable to parse date {}. \n The err reads as follows:\n {}", self.signedTimeStamp, e);
            },
        }
    }
}

impl PartialEq for MessageData {
    fn eq(&self, other: &Self) -> bool {
        self.getTimeStamp() == other.getTimeStamp() && self.from == other.from
    }
}

impl Eq for MessageData {}

impl PartialOrd for MessageData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let timeOrder = self.getTimeStamp().cmp(&other.getTimeStamp());
        return if Ordering::Equal == timeOrder {
            Some(self.from.cmp(&other.from))
        }else{ Some(timeOrder) }
    }
}

impl Ord for MessageData {
    fn cmp(&self, other: &Self) -> Ordering {
        let timeOrder = self.getTimeStamp().cmp(&other.getTimeStamp());
        return if Ordering::Equal == timeOrder {
            self.from.cmp(&other.from)
        }else{ timeOrder }
    }
}
