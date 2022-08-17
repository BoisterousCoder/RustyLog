use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use crate::LogState;


pub fn is_store_existing(filename:&str) -> bool{
 	return fs::metadata(filename).is_ok();
}
pub fn write_store(state:&LogState, password:&str) -> Result<(), Box<dyn Error>>{
 	let mut file = File::create(state.get_filename())?;
 	let key = new_magic_crypt!(password, 256);
 	let data = key.encrypt_str_to_base64(serde_json::to_string(state)?);

 	file.write_all(data.as_bytes())?;
 	file.sync_all()?;
 	Ok(())
}
pub fn read_store(filename:&str, password:&str) -> Result<LogState, Box<dyn Error>>{
 	let mut file = File::open(filename)?;
 	let mut data = String::new();
 	file.read_to_string(&mut data)?;
 	let key = new_magic_crypt!(password, 256);
 	let decrypted_data = key.decrypt_base64_to_string(&data)?;

 	return Ok(serde_json::from_str(&decrypted_data)?);
}
