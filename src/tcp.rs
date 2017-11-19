extern crate serde;
extern crate bincode;

use bincode::{serialize, deserialize, deserialize_from, serialize_into, Infinite};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};
use std::thread;
use std::fs::File;
use std::str::FromStr;

//Opens up port 24843 on localhost for a tcp connection
pub fn tcp_listen() {
	let listener = TcpListener::bind("127.0.0.1:24843").unwrap();

	//Whenever a new tcp connection begins, a new thread is created to handle it
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn( move || {
					handle_client(stream)
				});
			}

			Err(e) => {}
		}
	}

	drop(listener);
}

//Connects to an open node
//Currently requires IP to be given.
//Eventually will grab different known nodes from the iplog
pub fn tcp_stream(ip : String) {
	let port : String= ":24843".to_string();
	let str = ip + &port;
	let mut stream = TcpStream::connect(&*str).unwrap();
	//Test string to send
	let f = File::open("blockchain.bin").unwrap();
  let mut byte_buffer = Vec::new();
  for byte in f.bytes(){
    byte_buffer.push(byte.unwrap());
  }
  let buffer = &byte_buffer[..];
	let _ = stream.write(buffer);
}

//Handles incoming connections to tcp listener
fn handle_client(mut stream : TcpStream){
	let mut byte_buffer : Vec<u8> = Vec::new();
  stream.read_to_end(&mut byte_buffer);
  let mut f = File::create("bchaintemp.bin").unwrap();
  let buffer = &byte_buffer[..];
  f.write_all(buffer);
  
	//Logs ip
	let ip = stream.peer_addr().unwrap().ip();
	println!("{}", ip);
	ip_log(ip.to_string());
}

//Logs new IP addresses. If the address already exists, ignore it.
fn ip_log(ip : String){
	let mut file_exists = false;
	//If the file opens, it exists. If there is an error, it doesn't.
	match File::open("iplog.bin"){
		Ok(attr) => {file_exists = true;},
		Err(_) => {}
	};

	if file_exists {
		//Opens file and saves contents to string vector
		let mut file = File::open("iplog.bin").unwrap();
		let mut ipvec : Vec<String> = deserialize_from(&mut file, Infinite).unwrap();
		//Checks if current IP is in list
		let mut ip_exists : bool = false;
		for i in ipvec.clone() {
			if ip == i {
				ip_exists = true;
				break;
			}
		}	
		//If IP doesn't exist, push it to the list. If it does, do nothing.
		if !ip_exists {
			ipvec.push(ip);
		}
		//Save list back to file
		let mut c_file = File::create("iplog.bin").unwrap();
		let encode = serialize(&ipvec, Infinite).unwrap();
		c_file.write(&encode);
	}
	else{
		//Creates an iplog and saves IP as first element
		let mut file = File::create("iplog.bin").unwrap();
	        let ipvec = vec!(ip);
		let encode = serialize(&ipvec, Infinite).unwrap();
		file.write(&encode);	
	}
}

