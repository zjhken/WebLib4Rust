#![allow(non_snake_case)]
#![feature(nll)]

use mio::*;
use mio::tcp::{TcpListener, TcpStream};
//use std::io::{Read,Write};

use simple_logger;
use std::net::SocketAddr;

use log::Level;
use router::Router;

pub struct HtmlServer{
	SERVER_ID:Token,
	localAddr: SocketAddr,

}

pub fn createHtmlServer() -> HtmlServer{
	return HtmlServer{
		SERVER_ID: Token(8usize),
		localAddr: "52.78.196.238:8888".parse().unwrap(),
	}
}

impl HtmlServer{
	pub fn new() -> HtmlServer {
		return HtmlServer{
			SERVER_ID: Token(1),
			localAddr: "1.0.0.1:9090".parse().unwrap(),
		}
	}

	pub fn listenAddress(self) -> HtmlServer{
		return self;
	}
	pub fn port(self) -> HtmlServer{
		return self;
	}
	pub fn router(self, router: Router) -> HtmlServer{
		return self;
	}
}



pub fn run() {


}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}

	#[test]
	fn haha(){
		use asyncio;
		asyncio::run();
	}
}