use std::cmp::Ordering;
use crdts::glist::GList;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

const FILE_EXT:&str = ".rstore";

#[derive(Serialize, Deserialize, Clone)]
pub struct DB {
    pub msgs:GList<MessageData>,
	pub group:String,
	pub name:String
}
impl DB {
    pub fn get_fileName(&self) -> String {
	 	return format!("{}@{}.{}", self.name, self.group, FILE_EXT);
	}
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
        match DateTime::parse_from_str(&self.signedTimeStamp, "%Y %b %d %H:%M:%S%.3f %z") {
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
