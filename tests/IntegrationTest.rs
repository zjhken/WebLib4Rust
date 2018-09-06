#![allow(non_snake_case)]
#![feature(nll)]

extern crate WebLib4Rust;

use WebLib4Rust::server::HtmlServer;
use WebLib4Rust::router;

#[test]
fn itWork(){
	let router = router::Router::new();
	let server = HtmlServer::new()
			.listenAddress()
			.port()
			.router(router);



	WebLib4Rust::server::run();
}