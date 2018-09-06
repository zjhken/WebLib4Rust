#![allow(non_snake_case)]
#![feature(nll)]

use mio::*;
use mio::tcp::{TcpListener, TcpStream};
use std::io::{Read,Write};

use simple_logger;
use std::net::SocketAddr;

use log::Level;

pub struct HtmlServer{
	SERVER_ID:Token,
	localAddr: SocketAddr,

}

impl HtmlServer{
	pub fn new() -> HtmlServer {
		return HtmlServer{
			SERVER_ID: Token(1),
			localAddr: "1.0.0.1:9090".parse().unwrap(),
		}
	}
}



pub fn run() {

	simple_logger::init_with_level(Level::Info).unwrap();

	// Setup some tokens to allow us to identify which event is for which socket.
	const SERVER: Token = Token(0);
	const CLIENT: Token = Token(1);

	let addr = "127.0.0.1:12345".parse().unwrap();

	// Setup the server socket
	let server = TcpListener::bind(&addr).unwrap();

	// Create a poll instance
	let poll = Poll::new().unwrap();

	// Start listening for incoming connections
	poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

	// Setup the client socket
	let sock = TcpStream::connect(&addr).unwrap();

	// Register the socket
	poll.register(&sock, CLIENT, Ready::readable(), PollOpt::edge()).unwrap();


	// Create storage for events
	let mut events = Events::with_capacity(1024);

	loop {
		poll.poll(&mut events, None).unwrap();

		for event in events.iter() {
			match event.token() {
				SERVER => {
					// Accept and drop the socket immediately, this will close
					// the socket and notify the client of the EOF.
					let (stream,addr) = server.accept().unwrap();
					info!("Listener accept {:?}",addr);
				},
				CLIENT => {
					// The server just shuts down the socket, let's just exit
					// from our event loop.
					info!("client response.");
					return;
				},
				_ => unreachable!(),
			}
		}
	}
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}

	#[test]
	fn haha(){
		use server;
		server::run();
	}
}