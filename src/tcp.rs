use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::net::Ipv4Addr;
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
	let port = 24843;
	let mut stream = TcpStream::connect((Ipv4Addr::from_str(&ip).unwrap() , port)).unwrap();
	let _ = stream.write(b"foo");
}

fn handle_client(stream : TcpStream){

}
