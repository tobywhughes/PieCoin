extern crate bincode;

use bincode::rustc_serialize::{encode_into, decode_from};
use std::io::prelude::*;
use std::fs::File;

pub fn parse_trans_script(script : String) {
	let magic = &script[0..4];
	let tcom = &script[4..8];
	let psize = &script[8..12];
	let pchecksum = &script[12..16];
	let ssize = (script.len() - 20) as u32;
	let checksum = 0; //until implementation
	let header_flag : bool = check_header(magic, tcom, psize, pchecksum, ssize, checksum);
	if header_flag {
		println!("######################");
	}
}

fn check_header(magic : &str, transcommand : &str, psize : &str, pchecksum : &str, scriptsize : u32, checksum : u32) -> bool{
	let mut valid : bool = true;
	if magic != "FF00" {
		valid = false;
	}
	if transcommand != "FF10" {
		valid = false;
	}
	if u32::from_str_radix(psize, 16).unwrap() != scriptsize {
		valid = false;
	}
	if u32::from_str_radix(pchecksum, 16).unwrap() != checksum {
		valid = false;
	}
	
	valid
}

//Testing
pub fn create_example_file() {
	let hex_string : String = "FF00FF1000000000FFFF".to_string();
	let mut file = File::create("extrans.bin").unwrap();
	encode_into(&hex_string, &mut file, bincode::SizeLimit::Infinite).unwrap();
}



//Magic: FF00
//Transaction Command: FF10
//Stop : FFFF
