#![allow(non_snake_case)]
#![feature(nll)]

pub struct Router<'a> {
	routeList: Vec<Route<'a>>,
}

pub fn createRouter<'a>() -> Router<'a> {
	return Router {
		routeList: Vec::new(),
	};
}

use std::borrow::Cow;
use core::borrow::BorrowMut;

impl<'a> Router<'a> {
	pub fn anyMethod(&'a mut self, ctxPath: Cow<'a, str>) -> &'a mut Route<'a> {
		let mut r = Route {
			path: ctxPath,
			interceptors: Vec::new(),
		};
		self.routeList.push(r);
		return self.routeList.last_mut().unwrap();
	}
}

pub struct Route<'a> {
	path: Cow<'a, str>,
	interceptors: Vec<fn(RouteContext) -> RouteContext>,
}

impl<'a> Route<'a> {
	pub fn interceptor(&mut self, middleHandler: fn(RouteContext) -> RouteContext) -> () {
		self.interceptors.push(middleHandler);
	}
}

pub struct RouteContext {}

pub struct RespCtx {}
