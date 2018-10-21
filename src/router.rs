#![allow(non_snake_case)]
#![feature(nll)]

use std::borrow::Cow;
use core::borrow::BorrowMut;
use std::collections::HashMap;



type Handler = fn(Context);

pub struct Router<'a> {
	pub routes: Vec<Route<'a>>,
}

pub fn new<'a>() -> Router<'a> {
	return Router {
		routes: Vec::new(),
	};
}



impl<'a> Router<'a> {
	pub fn post(&mut self, path:Cow<'a, str>) -> &mut Route<'a> {
		let route = Route{
			path,
			method: HttpMethod::POST,
			handlers: Vec::new(),
		};
		self.routes.push(route);
		let r = self.routes.last_mut().unwrap();
		return r;
	}
	pub fn get(&mut self, path:Cow<'a, str>) -> &mut Route<'a> {
		let route = Route{
			path,
			method: HttpMethod::GET,
			handlers: Vec::new(),
		};
		self.routes.push(route);
		let r = self.routes.last_mut().unwrap();
		return r;
	}
	pub fn route(&mut self, path:Cow<'a, str>) -> &mut Route<'a> {
		let route = Route{
			path,
			method: HttpMethod::ANY,
			handlers: Vec::new(),
		};
		self.routes.push(route);
		let r = self.routes.last_mut().unwrap();
		return r;
	}
}

pub struct Context {}

pub struct RespCtx {}

pub enum HttpMethod {
	GET,
	POST,
	ANY,
}

pub struct Route<'a>{
	path: Cow<'a, str>,
	method: HttpMethod,
	handlers: Vec<Handler>
}

impl<'a> Route<'a>{
	pub fn method(&mut self, method: HttpMethod) -> &mut Route<'a>{
		self.method = method;
		return self;
	}
	pub fn handler(&mut self, handler: Handler) -> &mut Route<'a>{
		self.handlers.push(handler);
		return self;
	}
}
