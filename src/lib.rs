#![allow(non_snake_case)]
#![feature(nll)]

#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate mio;
extern crate core;

use router::Router;
use log::Level;
use mio::Token;
use mio::Poll;
use mio::Ready;
use mio::PollOpt;
use mio::tcp::TcpListener;
use mio::tcp::TcpStream;
use mio::Events;
use std::net::SocketAddr;

pub fn newHttpServer<'a>() -> HttpServer<'a>{
	return HttpServer{
		port: "",
		router: Router{ routes: Vec::new() },
	};
}

pub mod router;
pub mod asyncio;

pub struct HttpServer<'a>{
	port: &'a str,
	router: Router<'a>,
}

impl <'a>HttpServer<'a>{
	pub fn listenPort(mut self, port: &'a str) -> HttpServer{
		self.port = port;
		return self;
	}
	pub fn setRouter(mut self, router: Router<'a>) -> HttpServer<'a>{
		self.router = router;
		return self;
	}

	pub fn run(mut self){
		simple_logger::init_with_level(Level::Info).unwrap();

		// Setup some tokens to allow us to identify which event is for which socket.
		const SERVER: Token = Token(0);
		const CLIENT: Token = Token(1);

		let addr = format!("127.0.0.1:{}", &self.port).as_str().parse().unwrap();

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

}


