#![allow(non_snake_case)]
#![feature(nll)]

extern crate postgres;

use postgres::{Connection, TlsMode};

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

}