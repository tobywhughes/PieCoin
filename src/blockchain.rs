extern crate time;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::io::Write;
use std::io::stdout;

static VERSION : u32 = 1;

//Struct to hold all items in a block. If a hash is taken, it will be the hash in the blockchain
#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
pub struct BlockContents {
	pub v : u32,
	pub p_hash : String,
	pub m_tree : String,
	pub t_sec : String,
	pub b_size : u8,
	pub n : u32,
}

//A node on the blockchain. Currently holds the previous hash and the current block.
//Will eventually hold information about transactions, but transaction system yet to be implemented
#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
pub struct BlockChainNode {
	pub p_hash : String,
	pub current_block : BlockContents,
	//Full Block Chain Node includes transaction
	//However, transaction system not yet implemented
}

//Used to create a new node
impl BlockChainNode {
	pub fn new(h : String, c_block : BlockContents) -> BlockChainNode {
		BlockChainNode {p_hash : h, current_block : c_block}
	}
}

//Finds a valid hash. This function returns everything needed to add a node to the blockchain.
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

	//Functions loops to try to find nonce. Nonce is the counter at the end of the service string
	//That is continually changed until a hash is found with bit_size amount of leading zeros
	while !done {
		//Visual representation. Every 100 attempts is displayed as a @
		if nonce % 100 == 0 {
			print!("@");
			stdout().flush();
		}
		//Appends nonce to full service string. Format is:
		//VERSION
		//PREVIOUS HASH
		//MERKLE ROOT
		//TIME IN SECONDS
		//BIT SIZE
		//NONCE (COUNTER)
		let comb_string : String = format!("{}{}{}{}{}{}", ver, prev_hash, merkle_tree, time_sec, bit_size, nonce);
		
		//IF the counter reaches the max value of u32, reset nonce and update the time
		if nonce == 0xFFFFFFFF{
			nonce = 0;
			time_sec = time::strftime("%s", &time::now()).unwrap();	
		}
		//Otherwise, increment the counter as normal
		else {
			nonce += 1;
		}
		
		//Hashes the string with two SHA256 hashes
		let mut sha = Sha256::new();
		sha.input_str(&comb_string);
		let mut sha_2 = Sha256::new();
		sha_2.input_str(&sha.result_str());
		sha_string = sha_2.result_str();
		//Checks if the string has bit_size amount of leading zeros in a binary representation
		done = check_start_string(bit_size, &sha_2.result_str());
	}

	//Since nonce is incremented before check, decrement it so it is represented currectly when returned
	if nonce != 0 {
		nonce -= 1;
	}
	else {
		nonce = 0xFFFFFFFF;
	}
	
	let return_struct = BlockContents {v : ver, p_hash : prev_hash, m_tree : merkle_tree, t_sec : time_sec, b_size: bit_size, n : nonce};
	
	//Returns the hashed string as well as the block
	(sha_string, return_struct)
}

//Checks if hash is a valid hash to mine
fn check_start_string(bit_size : u8, s : &str) -> bool {
	//For implementation ease, bit-size always divisible by 8
	//TODO Improve so that still easy with other bit size. Need to figure out better string indexing in rust.
	let bytes : u8 = bit_size / 8;
	let mut zero_match : bool = false;
	let mut counter : u8 = 0;
	//Checks each digit. If one is off, then the entire hash is invalid
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


//Checks is the provided hash string is the same as when a hash is taken of the contents in the block
pub fn check_hash(hash_string : String, b : BlockContents) -> bool {
	hash_string == do_hash(b)
}

//Initializes the coinbase node of the merkle tree
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

//Takes contents of the block, formats it in a service string, and does the sha256 hash twice
pub fn do_hash(b : BlockContents) -> String{
	let hash_string = format!("{}{}{}{}{}{}", b.v, b.p_hash, b.m_tree, b.t_sec, b.b_size, b.n);
	let mut sha_1 = Sha256::new();
	let mut sha_2 = Sha256::new();
	sha_1.input_str(&hash_string);
	sha_2.input_str(&sha_1.result_str());
	sha_2.result_str()	
}


