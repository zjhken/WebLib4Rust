#![allow(non_snake_case)]
#![feature(nll)]

extern crate postgres;

use postgres::{Connection, TlsMode};
use std::vec::Vec;

#[test]
fn pg(){
	let conn = Connection::connect("postgresql://postgres:zjhken@localhost:5432/test", TlsMode::None)
			.unwrap();
	for row in &conn.query("select * from users", &[]).unwrap(){
		// let name = row.get::<_,String>(0);
		// let password= row.get::<_,String>(1);
		let name: String = row.get(0);
		let password: String= row.get(1);
		println!("row 1:{}", name);
		println!("row 2:{}", password);
	}

	let b = Vec::<u32>::new();
	let c: Vec<u32> = Vec::new();
	let d = 5_0000u32;
}

#[test]
fn testConcurrent(){
	use std::thread;
	use std::sync::Arc;

	let var = Arc::new(5);
	let shareVar = var.clone();

	let newThread = thread::spawn(move || {
		println!("share value in new thread: {}, addres: {:p}", shareVar, &*shareVar);
	});

	newThread.join().unwrap();
	println!("share value in main thread: {}, address: {:p}", var, &*var);
}