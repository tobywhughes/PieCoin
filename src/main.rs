extern crate crypto;
extern crate rustc_serialize;
extern crate bincode;

use bincode::rustc_serialize::{encode, decode, encode_into, decode_from};
use std::fs::File;

mod blockchain;
mod tcp;

fn main() {
  	//Gets a hash and the corresponding block header
	let (gen_hash, gen_block) = blockchain::init_hash(String::from_utf8(vec![0;32]).unwrap());
	//Checks if hash is valid. If the block is hashed twice with Sha256, the resulting string should match the hash
	println!("Valid Hash: {}", blockchain::check_hash(gen_hash.to_string(), gen_block.clone()));
	//Creates two more blocks for the blockchain
	let (hash1, block1) = blockchain::init_hash(gen_hash.to_string());
	let (hash2, block2) = blockchain::init_hash(hash1.to_string());
	let mut bchain = vec![blockchain::BlockChainNode::new(String::from_utf8(vec![0;32]).unwrap(), gen_block)];
	bchain.push(blockchain::BlockChainNode::new(gen_hash, block1));
	bchain.push(blockchain::BlockChainNode::new(hash1, block2));
	let mut counter = 0;
	for i in bchain.clone(){
		println!("\n\nBlock#{}", counter);
		print_bchainnode(i);
		counter += 1;
	}

	{
		let mut file = File::create("blockchain.bin").unwrap();
		encode_into(&bchain, &mut file, bincode::SizeLimit::Infinite).unwrap();		
	}

	let mut file = File::open("blockchain.bin").unwrap();	
	let decoded : Vec<blockchain::BlockChainNode> = decode_from(&mut file, bincode::SizeLimit::Infinite).unwrap();

	println!("FILE DECODE CHECK");
	counter = 0;
	for i in decoded{
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
