extern crate num;

use num::bigint::*;
use num::Zero;
use num::Integer;
use std::io::Read;
use std::io::prelude::*;
use std::io::Error;
use std::net::TcpStream;


fn main() {
	work().unwrap();
}

fn work() -> Result<(), Error> {
	let mut stream = TcpStream::connect("195.154.53.62:1337").unwrap();
	let mut buf = [0; 1500];

	loop {
		let amt = stream.read(&mut buf).unwrap();
		if amt != 0 {
			let lines: Vec<&str> = std::str::from_utf8(&buf[..amt]).unwrap().split("\n").collect();
			let line: Vec<&str> = lines[lines.len()-2].split(" ").collect();

			if line[0] == "Tell" {
				println!("");
				println!("Finished.  Flag is: {}", line[6]);
				break;
			}
			else if line[0] == "Freeze!" {
				println!("");
				println!("Made some mistake.");
				break;
			}


			// either "freeze" or...
			// basically do an if-eif, print message, quit
			// OR print dot

			let fin: BigInt;
			let first = line[0].parse::<BigInt>().unwrap();
			let second = line[2].parse::<BigInt>().unwrap();

			match &*line[1] {
				"+" => fin = first.checked_add(&second).unwrap(),
				"-" => fin = first.checked_sub(&second).unwrap(),
				"*" => fin = first.checked_mul(&second).unwrap(),
				"/" => fin = first.checked_div(&second).unwrap(),
				"%" => fin = first.div_rem(&second).1,
				_ => fin = Zero::zero(),
			};
			try!(writeln!(stream, "{}", fin.to_str_radix(10)));
			print!(".");
		}
		
	}

	Ok(())
}

	
