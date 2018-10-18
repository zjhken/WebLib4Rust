#![allow(non_snake_case)]
#![feature(nll)]

extern crate WebLib4Rust;

use WebLib4Rust::router;
use WebLib4Rust::server;

use std::borrow::Cow;
use WebLib4Rust::router::RouteContext;

#[test]
fn itWork() {

	let mut router = router::createRouter();
	router
		.anyMethod(Cow::Borrowed("api"))
		.interceptor(validateUser);
	router
		.anyMethod(Cow::Borrowed("*"))
		.interceptor(validateUser);
	router
		.anyMethod(Cow::Borrowed("login"))
		.interceptor(validateUser);

	let server = server::createHtmlServer()
		.listenAddress()
		.port()
		.router(router);

	server::run();
}

fn validateUser(cxt: RouteContext) -> RouteContext {
	return RouteContext{

	}
}

fn login(cxt: RouteContext) -> RouteContext {
	return RouteContext{

	}
}
