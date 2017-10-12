extern crate encoding;
extern crate walkdir;

use std::{env, process};

fn showhelp() {
	println!("Usage: codepage_converter PATH FROM_CODE(gbk) TO_CODE(shift_jis)");
	println!("Check https://encoding.spec.whatwg.org/#concept-encoding-get for all valid encoding");
	process::exit(1);
}

fn main() {
	let args: Vec<_> = env::args().collect();
	let mut from_code = "gbk";
	let mut to_code = "shift-jis";
	match args.len() {
		2 => {},
		4 => { from_code = &args[2]; to_code = &args[3] },
		_ => { showhelp() }
	}
	
	let from_encoding = encoding::label::encoding_from_whatwg_label(from_code).unwrap();
	let to_encoding = encoding::label::encoding_from_whatwg_label(to_code).unwrap();
	let files = walkdir::WalkDir::new(&args[1]).into_iter().filter_map(|e| e.ok());
	let mut fbuf: Vec<String> = Vec::new();
	let mut cbuf: Vec<String> = Vec::new();
	for f in files {
		let fname = f.path().to_str().unwrap();
		let hex = from_encoding.encode(fname, encoding::EncoderTrap::Ignore).unwrap();
		let cname = to_encoding.decode(&hex, encoding::DecoderTrap::Ignore).unwrap();
		println!("{}\n{}", fname, cname);
		fbuf.push(fname.to_string());
		cbuf.push(cname.to_string());
	}
	let count = fbuf.len();
	if count < 1 {
		println!("Couldn't find any files from given path");
		showhelp();
	}

	println!("\n{} files/directories in all, do the conversion?(Yes/No/Manually)", count);
	let mut pflag = false;
	let mut key = String::new();
	std::io::stdin().read_line(&mut key).unwrap();
	match &*key {
		"m\n" => {},
		"y\n" => {
			pflag = true;
		},
		_ => { process::exit(0) }
	}

	let sep: &[_] = &['\\', '/'];
	for i in 0..count-1 {
		let fname = &fbuf[count-1-i];
		let cname = &cbuf[count-1-i];
		let findex = fname.rfind(sep).unwrap();
		let cindex = cname.rfind(sep).unwrap();
		let mut tname = String::new();
		tname.push_str(&fname[..findex+1]);
		tname.push_str(&cname[cindex+1..]);
		if pflag {
			std::fs::rename(fname, tname).unwrap();
		} else {
			println!("mv {} {}", fname, tname);
		}
	}
}
