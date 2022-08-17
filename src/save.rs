use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use crate::db::DB;


pub fn isStoreExisting(filename:&str) -> bool{
 	return fs::metadata(filename).is_ok();
}
pub fn writeStore(db:&DB, password:&str) -> Result<(), Box<dyn Error>>{
 	let mut file = File::create(db.getFileName())?;
 	let key = new_magic_crypt!(password, 256);
 	let data = key.encrypt_str_to_base64(serde_json::to_string(db)?);

 	file.write_all(data.as_bytes())?;
 	file.sync_all();
 	Ok(())
}
pub fn readStore(filename:&str, password:&str, fileExt:&str) -> Result<DB, Box<dyn Error>>{
 	let mut file = File::open(filename)?;
 	let mut data = String::new();
 	file.read_to_string(&mut data);
 	let key = new_magic_crypt!(password, 256);
 	let decryptedData = key.decrypt_base64_to_string(&data)?;

 	return Ok(serde_json::from_str(&decryptedData)?);
}
