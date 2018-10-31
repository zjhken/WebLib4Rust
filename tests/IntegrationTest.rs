#![allow(non_snake_case)]
#![feature(nll)]

extern crate WebLib4Rust;

use WebLib4Rust::router;
use WebLib4Rust::asyncio;

use std::borrow::Cow;
use WebLib4Rust::router::Context;
use WebLib4Rust::router::HttpMethod;

#[test]
fn itWork() {
	let mut router = router::new();

	router.route(Cow::Borrowed("api")).handler(validateUser);
	router.post(Cow::Borrowed("api/haha")).handler(validateUser);


	let server = WebLib4Rust::newHttpServer()
			.bindAddr("0.0.0.0")
			.listenPort("9999")
			.setRouter(router);

	server.run();
}

fn validateUser(cxt: Context) {
	println!("handling context");
}

