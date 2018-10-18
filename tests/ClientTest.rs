#![allow(non_snake_case)]
#![feature(nll)]


extern crate mio;

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::net::TcpStream;

use std::io::prelude::*;

use std::io::stdout;

static HTTP: &str = "POST / HTTP/1.1
Host: www.baidu.com:80
Connection: keep-alive
Content-Length: 60
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/68.0.3440.106 Safari/537.36
Cache-Control: no-cache
Origin: chrome-extension://fhbjgbiflinjbdggehcddcbncdddomop
Postman-Token: 215dcd65-4df3-c424-62d4-9c8bbb56770e
Content-Type: application/x-www-form-urlencoded
Accept: */*
Accept-Encoding: gzip, deflate, br
Accept-Language: zh-CN,zh;q=0.9
";

#[test]
fn clientConnect() {
	let ref addr = "52.78.196.238:8888".parse().unwrap();
	let mut stream = TcpStream::connect(addr).unwrap();
	let nWrite = stream.write(HTTP.as_bytes()).unwrap();
	stream.flush().unwrap();

	let poll = Poll::new().unwrap();

	let stdout = stdout();

	poll.register(&stream, Token(1usize), Ready::readable(), PollOpt::edge()).unwrap();
	let mut events = Events::with_capacity(1024);
	loop {
		poll.poll(&mut events, None).unwrap();
		for event in events.iter() {
			match event.token() {
				Token(1) => {
					let mut buf = [0u8; 1024];
					loop {
						match stream.read(&mut buf) {
							Ok(nRead) => {
								let mut stdoutLock = stdout.lock();
								stdoutLock.write(&buf[..nRead]).unwrap();
								stdoutLock.flush();
							}
							Err(e) => break
						}
					}
				}
				_ => unreachable!(),
			}
		}
	}
}

#[test]
fn simpleTcpClient() {
	use std::io::prelude::*;
	use std::net::TcpStream;

	let mut stream = TcpStream::connect("52.78.196.238:8888").unwrap();

	let h = HTTP.replace("\n", "\r\n");
	let http = h.as_str();
	let mut buf = [0u8; 2048];
	let _ = stream.write(http.as_bytes()); // ignore the Result
	let _ = stream.read(&mut buf[..]); // ignore this too
	println!("{}", String::from_utf8_lossy(&buf));
	// the stream is closed here
}