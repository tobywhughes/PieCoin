extern crate serde;

extern crate bincode;

use bincode::{serialize, deserialize, deserialize_from, serialize_into, Infinite};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::io::prelude::*;
use std::fs::File;

//Parses the script into sections that can be run by individual functions
pub fn parse_trans_script(script : String) {
	//Grabs the items in the 16 byte header. The header consists of four 4 byte items
	//A magic number to identify the PieCoin protocol
	let magic = &script[0..4];
	//The command to run. In this case we are looking for a transaction command FF10
	let tcom = &script[4..8];
	//Size of the payload (minus header and stop) itself, in bytes
	let psize = &script[8..12];
	//Checksum of payload. First four bytes of SHA256(SHA256(payload))
	let pchecksum = &script[12..16];
	//Evaluates these values at run time to check them against the header
	let ssize = (script.len() - 20) as u32;
	let checksum = get_checksum((&script[16..script.len() - 4]).to_string());
	let header_flag : bool = check_header(magic, tcom, psize, pchecksum, ssize, &checksum);
	//Continues if header is valid
	if header_flag {
		println!("######################");
	}
}

//Checks magic number, command, size, and checksum that make up the header
fn check_header(magic : &str, transcommand : &str, psize : &str, pchecksum : &str, scriptsize : u32, checksum : &str) -> bool{
	let mut valid : bool = true;

	//If anything doesn't match, set the valid flag to false
	if magic != "FF00" {
		valid = false;
	}
	if transcommand != "FF10" {
		valid = false;
	}
	if u32::from_str_radix(psize, 16).unwrap() != scriptsize {
		valid = false;
	}
	if pchecksum != checksum {
		valid = false;
	}
	
	valid
}

//Creates a simple file for testing
pub fn create_example_file() {
	let script : String = "FFFF".to_string();
	let script_header : String = generate_header(script.clone());
	let hex_string : String = script_header + &script;
	let mut file = File::create("extrans.bin").unwrap();
	let encode = serialize(&hex_string, Infinite).unwrap();
	file.write(&encode);
}

//Generates a checksum from provided script. Checksum is first 4 bytes of Sha256(Sha256(Payload))
fn get_checksum(payload : String) ->  String {
	let mut sha_1 = Sha256::new();
	let mut sha_2 = Sha256::new();
	sha_1.input_str(&payload);
	sha_2.input_str(&sha_1.result_str());
	(&sha_2.result_str()[0..4]).to_string()
}

//Generates a transaction header for given script
fn generate_header(script : String) -> String {
	let mut sbuf = "FF00".to_string(); //Starts buffer with magic number
	sbuf = sbuf + "FF10";
	sbuf = sbuf + &(format!("{:04X}", (script.len()-4)));
	sbuf = sbuf + &(get_checksum(script[0..script.len()-4].to_string()));
	sbuf
}



//Magic: FF00
//Transaction Command: FF10
//Stop : FFFF
