extern crate bincode;

use bincode::rustc_serialize::{encode_into, decode_from};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};
use std::thread;
use std::fs::File;
use std::str::FromStr;

pub fn tcp_listen() {
	let listener = TcpListener::bind("127.0.0.1:24843").unwrap();
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

pub fn tcp_stream(ip : String) {
	let port : String= ":24843".to_string();
	let str = ip + &port;
	let mut stream = TcpStream::connect(&*str).unwrap();
	let _ = stream.write(b"foo");
}

fn handle_client(mut stream : TcpStream){
	let str = &mut String::new();
	stream.read_to_string(str);
	println!("{}", str);
	let ip = stream.peer_addr().unwrap().ip();
	println!("{}", ip);
	ip_log(ip.to_string());
}

fn ip_log(ip : String){
	let mut file_exists = false;
	match File::open("iplog.bin"){
		Ok(attr) => {file_exists = true;},
		Err(_) => {}
	};
	if file_exists {
		let mut file = File::open("iplog.bin").unwrap();
		let mut ipvec : Vec<String> = decode_from(&mut file, bincode::SizeLimit::Infinite).unwrap();
		let mut ip_exists : bool = false;
		for i in ipvec.clone() {
			if ip == i {
				ip_exists = true;
				break;
			}
		}	
		if !ip_exists {
			ipvec.push(ip);
		}
		let mut c_file = File::create("iplog.bin").unwrap();
		encode_into(&ipvec, &mut c_file, bincode::SizeLimit::Infinite).unwrap();
	}
	else{
		let mut file = File::create("iplog.bin").unwrap();
	        let ipvec = vec!(ip);
		encode_into(&ipvec, &mut file, bincode::SizeLimit::Infinite).unwrap();	
	}
}

