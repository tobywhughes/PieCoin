extern crate time;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::io::Write;
use std::io::stdout;

static VERSION : u32 = 1;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
pub struct BlockContents {
	pub v : u32,
	pub p_hash : String,
	pub m_tree : String,
	pub t_sec : String,
	pub b_size : u8,
	pub n : u32,
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
pub struct BlockChainNode {
	pub p_hash : String,
	pub current_block : BlockContents,
	//Full Block Chain Node includes transaction
	//However, transaction system not yet implemented
}

impl BlockChainNode {
	pub fn new(h : String, c_block : BlockContents) -> BlockChainNode {
		BlockChainNode {p_hash : h, current_block : c_block}
	}
}

pub fn init_hash(last_hash : String) -> (String, BlockContents){
	println!("Getting Hash");
	let ver : u32 = VERSION;
	let prev_hash : String = last_hash;
	let merkle_tree : String = init_merkle_coinbase();
	println!("Merkle Tree Result: {}", merkle_tree);
	let mut time_sec : String = time::strftime("%s",&time::now()).unwrap();
	let bit_size : u8 = 32;
	let mut nonce : u32 = 0;
	let mut sha_string : String = "".to_string();
	let mut done : bool = false;
	println!("NONCE:");
	while !done {
		if nonce % 100 == 0 {
			print!("@");
			stdout().flush();
		}
		let comb_string : String = format!("{}{}{}{}{}{}", ver, prev_hash, merkle_tree, time_sec, bit_size, nonce);
		if nonce == 0xFFFFFFFF{
			nonce = 0;
			time_sec = time::strftime("%s", &time::now()).unwrap();	
		}
		else {
			nonce += 1;
		}
		let mut sha = Sha256::new();
		sha.input_str(&comb_string);
		let mut sha_2 = Sha256::new();
		sha_2.input_str(&sha.result_str());
		sha_string = sha_2.result_str();
		done = check_start_string(bit_size, &sha_2.result_str());
	}
	if nonce != 0 {
		nonce -= 1;
	}
	else {
		nonce = 0xFFFFFFFF;
	}
	let return_struct = BlockContents {v : ver, p_hash : prev_hash, m_tree : merkle_tree, t_sec : time_sec, b_size: bit_size, n : nonce};
	(sha_string, return_struct)
}

fn check_start_string(bit_size : u8, s : &str) -> bool {
	//For implementation ease, bit-size always divisible by 8
	//TODO Improve so that stilly easy with other bit size. Need to figure out better string indexing in rust.
	let bytes : u8 = bit_size / 8;
	let mut zero_match : bool = false;
	let mut counter : u8 = 0;
	for c in s.chars(){
		if c  == '0' {
			zero_match = true;
		}
		else {
			zero_match = false;
			break;
		}
		if counter == (bytes - 1) {
			break;
		}
		else {
			counter += 1;
		}
	}
	zero_match
}

pub fn check_hash(hash_string : String, b : BlockContents) -> bool {
	let check_string = format!("{}{}{}{}{}{}", b.v, b.p_hash, b.m_tree, b.t_sec, b.b_size, b.n);
	let mut sha_1 = Sha256::new();
	let mut sha_2 = Sha256::new();
	sha_1.input_str(&check_string);
	sha_2.input_str(&sha_1.result_str());
	let mut hash_match : bool = false;
	if hash_string == sha_2.result_str(){
		hash_match = true;
	}
	hash_match
}

pub fn init_merkle_coinbase() -> String{
	let ver : u32 = VERSION;
	let extra_nonce : u32 = 0;
	let in_size : u32 = 1;
	let in_script : String = String::from("00");
	let val : u64 = 314;
	let out_size : u32 = 1;
	let out_script : String = String::from("00");
	let lock_counter : u32 = 0;
	let comb_string = format!("{}{}{}{}{}{}{}{}", ver, extra_nonce, in_size, in_script, val, out_size, out_script, lock_counter);
	let mut sha = Sha256::new();
	sha.input_str(&comb_string);
	let mut sha_2 = Sha256::new();
	sha_2.input_str(&sha.result_str());
	sha_2.result_str()
}


