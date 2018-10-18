extern crate WebLib4Rust;

fn main() {
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
