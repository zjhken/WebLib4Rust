extern crate WebLib4Rust;
extern crate core;

use WebLib4Rust::router;
use std::borrow::Cow;
use WebLib4Rust::server;
use WebLib4Rust::router::RouteContext;
use core::borrow::BorrowMut;
use core::borrow::Borrow;

fn main() {
	let mut routerR = router::createRouter();

	let router = routerR.borrow_mut();

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
		.router(routerR);

	server::run();
}

fn validateUser(cxt: RouteContext) -> RouteContext {
	return RouteContext{

	}
}
