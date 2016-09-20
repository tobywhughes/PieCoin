extern crate crypto;
extern crate rustc_serialize;
extern crate bincode;

use bincode::rustc_serialize::{encode, decode, encode_into, decode_from};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::env;

mod blockchain;
mod tcp;

fn main() {
	let mut file_exists : bool = true;
	let mut mine_blocks : bool = false;

	let mut arg_vec : Vec<String> = Vec::new();

	for i in env::args(){
		arg_vec.push(i);
	}

	for i in arg_vec{
		if i == "--create" {
			file_exists = false;
		}
		if i == "--mine" {
			mine_blocks = true;
		}
	}

	if file_exists == false {
  		//Gets a hash and the corresponding block header
		let (gen_hash, gen_block) = blockchain::init_hash(String::from_utf8(vec![0;32]).unwrap());
		let bchain = vec!(blockchain::BlockChainNode::new(String::from_utf8(vec![0;32]).unwrap(), gen_block));
		//Checks if hash is valid. If the block is hashed twice with Sha256, the resulting string should match the hash
		{	
			let mut file = File::create("blockchain.bin").unwrap();
			encode_into(&bchain, &mut file, bincode::SizeLimit::Infinite).unwrap();		
		}

	}

	let mut file = File::open("blockchain.bin").unwrap();	
	let mut bchain : Vec<blockchain::BlockChainNode> = decode_from(&mut file, bincode::SizeLimit::Infinite).unwrap();

	let stdin = io::stdin();
	let input = &mut String::new();

	while mine_blocks{
		let ref top_elem = bchain.clone()[bchain.len() - 1];
		let mined_block = mine_block(blockchain::do_hash(top_elem.clone().current_block));
		bchain.push(mined_block);
		println!("\nPrint 'x' to stop mining:");
		input.clear();
		stdin.read_line(input);
		if input == "x\n" {
			let mut file = File::create("blockchain.bin").unwrap();
			encode_into(&bchain, &mut file, bincode::SizeLimit::Infinite).unwrap();
			break;
		}
	}

	println!("FILE DECODE CHECK");
	let mut counter = 0;
	for i in bchain{
		println!("\n\nBlock#{}", counter);
		print_bchainnode(i);
		counter += 1;
	}
}

fn print_bchainnode(node : blockchain::BlockChainNode){
	println!("###############");
	println!("{}", node.p_hash);
	print_blockcontents(node.current_block);
	println!("###############");
}

fn print_blockcontents(b : blockchain::BlockContents){
	println!("Version Num: {}", b.v);
	println!("Previous Hash: {}", b.p_hash);
	println!("Merkle Root: {}", b.m_tree);
	println!("Time (in seconds): {}", b.t_sec);
	println!("Bitsize: {}", b.b_size);
	println!("Nonce: {}", b.n);
}

fn mine_block(prev_hash : String) -> blockchain::BlockChainNode {
	let (hash, block_header) = blockchain::init_hash(prev_hash.to_string());
	blockchain::BlockChainNode::new(prev_hash, block_header)
}
