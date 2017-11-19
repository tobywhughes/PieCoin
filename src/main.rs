#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate crypto;
extern crate bincode;

use bincode::{serialize, deserialize, deserialize_from, serialize_into, Infinite};
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::env;
use std::thread;

mod blockchain;
mod tcp;
mod transactions;

fn main() {
	let mut file_exists : bool = true;
	let mut mine_blocks : bool = false;
	let mut print_bchain : bool = true;
	let mut tcp_test : bool = false;
	let mut create_trans_test : bool = false;

	//Iterates through command line arguments
	for i in env::args(){
		if i == "--create" {
			file_exists = false;
		}
		else if i == "--mine" {
			mine_blocks = true;
		}
		else if i == "--silent" {
			print_bchain = false;
		}

		else if i == "--tcptest"{
			tcp_test = true;
		}
		else if i == "--transtest" {
			create_trans_test = true;
		}
	}

	//Keep commented until serialization fixed

	// if create_trans_test {
	// 	transactions::create_example_file();
	// 	let mut file = File::open("extrans.bin").unwrap();
	// 	let mut trans_string : String = decode_from(&mut file, bincode::SizeLimit::Infinite).unwrap();
	// 	transactions::parse_trans_script(trans_string);
		
	// }

	//Commented until tcp is figured out
	thread::spawn(|| {
		tcp::tcp_listen();
	});

	//If no genesis block exits, create it
	if !file_exists {create_genesis_block();}

	//Opens file and reads into blockchain
	let mut file = File::open("blockchain.bin").unwrap();
	let mut bchain : Vec<blockchain::BlockChainNode> = deserialize_from(&mut file, Infinite).unwrap();

	//Creates command line reader and input string
	let stdin = io::stdin();
	let mut input = String::new();

	//Mines blocks until told to stop
	while mine_blocks{
		//Grabs top element
		let ref top_elem = bchain.clone()[bchain.len() - 1];
		//Takes hash of block and uses that to feed the mining function
		let mined_block = mine_block(blockchain::do_hash(top_elem.clone().current_block));
		bchain.push(mined_block);
		//Input to leave mining
		println!("\nPrint 'x' to stop mining:");
		let mut input = String::new();
		io::stdin().read_line(&mut input);
		print!("{}", input);
		let teststring = String::from("x");
		if "x" == input.trim() {
			println!("test");
			//Saves new blocks to the blockchain
			let mut file = File::create("blockchain.bin").unwrap();
			let mut encode: Vec<u8> = serialize(&bchain, Infinite).unwrap();
			file.write(&encode);
			break;
		}
	}

	//Prints out entire blockchain
	let mut counter = 0;
	if print_bchain {
		for i in bchain{
			println!("\n\nBlock#{}", counter);
			print_bchainnode(i);
			counter += 1;
		}
	}

	let ip_addr = &mut String::new();
	if tcp_test {
		println!("\nInput IP: ");
		stdin.read_line(ip_addr);
	}

	//Holds the program open
	if tcp_test {
		tcp::tcp_stream(ip_addr.to_string());
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

//Generates first block in blockchain, the genesis block
fn create_genesis_block() {
  //Gets a hash and the corresponding block header, throws away hash as it is
  //not used
  let ( _ , gen_block) = blockchain::init_hash(String::from_utf8(vec![0;32]).unwrap());
  let bchain = vec!(blockchain::BlockChainNode::new(String::from_utf8(vec![0;32]).unwrap(), gen_block));
  {
    let mut file = File::create("blockchain.bin").unwrap();
	let mut encode: Vec<u8> = serialize(&bchain, Infinite).unwrap();
	file.write(&encode);
    //deserialize_from(&bchain, &mut Infinite).unwrap();
  }
}
