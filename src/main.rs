use std::env;
use std::fs;
use std::io::{self, Write};

fn read_byte(fname: &str) -> io::Result<Vec<u8>> {
	let v = fs::read(fname)?;
	Ok(v)
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("rat: need file name");
	} else {
		let stdout = io::stdout();
		let mut stdout_handle = stdout.lock();
		let mut buf: Vec<u8> = Vec::new();

		for fname in args[1..].iter() {
			println!("rat: {}\n", fname);

			match read_byte(fname) {
				Ok(v) => {
					for b in v {
						buf.push(b)
					}
				}
				Err(_) => println!("rat: failed to read file"),
			}
		}
		stdout_handle
			.write(&buf)
			.expect("rat: failed to write to stdout");
	}
}
