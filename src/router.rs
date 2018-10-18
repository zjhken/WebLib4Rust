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

impl<'a> Router<'a> {
	pub fn anyMethod(&mut self, ctxPath: Cow<str>) -> &'a mut Route<'a> {
		let mut r = Route {
			path: Cow::Borrowed("haha"),
			interceptors: Vec::new(),
		};
		self.routeList.push(r);
		return &r;
	}
}

pub struct Route<'a> {
	path: Cow<'a, str>,
	interceptors: Vec<fn(RouteContext) -> RouteContext>,
}

impl<'a> Route<'a> {
	pub fn interceptor(&mut self, middleHandler: fn(RouteContext) -> RouteContext) -> () {
		self.interceptors.push(middleHandler);
		//if let Some(ref mut interceptors) = self.interceptors{
		//	interceptors.push()
		//}
	}
}

pub struct RouteContext {}

pub struct RespCtx {}
