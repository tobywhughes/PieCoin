use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn tcp_listen() {
	let listener = TcpListener::bind("127.0.0.1:80").unwrap();

	drop(listener);
}

fn handle_client(stream : TcpStream){

}
