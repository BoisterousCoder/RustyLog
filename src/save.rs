// use magic_crypt::MagicCryptTrait;
// use magic_crypt::new_magic_crypt;
// use std::fs;
// use std::fs::File;
// use std::io::prelude::*;
// use serde::{Deserialize, Serialize};
// use serde_json::Result;
// use crate::utils::*;

// #[derive(Serialize, Deserialize)]
// pub struct DBData {
//     pub msgs:Vec<MessageData>
// }

// #[derive(Serialize, Deserialize)]
// pub struct MessageData {
//     pub from:String,
//     pub tag:String,
//     pub content:String,
//     pub signature:String,
//     pub timeStamp:String
// }

// pub fn attemptFetchIdData(connData:ConnectionData) -> Option<DBData>{
//  	if isStoreExisting(connData.get_fileName()) {
//  		let dataStr = read(connData);
//  		Some(serde_json::from_str(dataStr.as_str()).unwrap())
//  	}else {
//  		None
//  	}
// }

// fn isStoreExisting(file:String) -> bool{
//  	return fs::metadata(file).is_ok();
// }
// fn write(connData:ConnectionData, text:String){
//  	let mut file = File::create(connData.get_fileName()).unwrap();
//  	let key = new_magic_crypt!(connData.password, 256);
//  	let data = key.encrypt_str_to_base64(text);

//  	file.write_all(data.as_bytes());
//  	file.sync_all();
// }
// fn read(connData:ConnectionData) -> String{
//  	let mut file = File::open(connData.get_fileName()).unwrap();
//  	let mut data = String::new();
//  	file.read_to_string(&mut data);
//  	let key = new_magic_crypt!(connData.password, 256);

//  	return key.decrypt_base64_to_string(&data).unwrap();
// }
