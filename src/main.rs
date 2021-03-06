extern crate encoding_rs;
extern crate walkdir;

use std::{env::args, fs::{OpenOptions, rename}, io::{BufReader, BufWriter, prelude::*, stdin}, path::Path, process::exit};

fn showhelp() {
  println!("Usage: codepage_converter PATH|FILE FROM_CODE(gbk) TO_CODE(shift_jis)");
  println!("Check https://encoding.spec.whatwg.org/#concept-encoding-get for all valid encoding");
  exit(1);
}

fn main() {
  let args: Vec<_> = args().collect();
  let mut from_code = "gbk";
  let mut to_code = "shift-jis";
  match args.len() {
    2 => {},
    4 => { from_code = &args[2]; to_code = &args[3] },
    _ => { showhelp() }
  }
  
  let from_encoding = encoding_rs::Encoding::for_label(from_code.as_bytes()).unwrap();
  let to_encoding = encoding_rs::Encoding::for_label(to_code.as_bytes()).unwrap();
  let path = Path::new(&args[1]);
  if path.is_dir() {
    let files = walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok());
    let mut fbuf: Vec<String> = Vec::new();
    let mut cbuf: Vec<String> = Vec::new();
    for f in files {
      let fname = f.path().to_str().unwrap();
      if fname != "." {
        let hex = from_encoding.encode(fname).0;
        let cname = to_encoding.decode(&hex).0;
        println!("{}\n{}", fname, cname);
        fbuf.push(fname.to_string());
        cbuf.push(cname.to_string());
      }
    }

    let count = fbuf.len();
    println!("\n{} files/directories in all, do the conversion?(Yes/No/Manually)", count);
    let mut pflag = false;
    let mut key = String::new();
    stdin().read_line(&mut key).unwrap();
    match key.chars().next().unwrap() {
      'm'|'M' => {},
      'y'|'Y' => {
        pflag = true;
      },
      _ => { exit(0) }
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
        rename(fname, tname).unwrap();
      } else {
        println!("mv \"{}\" \"{}\"", fname, tname);
      }
    }
    if pflag {
      rename(&fbuf[0], &cbuf[0]).unwrap();
    } else {
      println!("mv \"{}\" \"{}\"", &fbuf[0], &cbuf[0]);
    }
  } else {
    let file = match OpenOptions::new().read(true).open(path) {
      Ok(f) => f,
      Err(_) => {
        println!("Couldn't find any files from given path\n");
        return showhelp()
      }
    };
    let mut reader = BufReader::new(file);
    let mut raw:Vec<u8> = Vec::new();
    reader.read_to_end(&mut raw).unwrap();
    let hex = from_encoding.decode(&raw).0;
    let traw = to_encoding.encode(&hex).0;
    let tfile = OpenOptions::new().write(true).truncate(true).open(path).unwrap();
    let mut writer = BufWriter::new(tfile);
    writer.write_all(&traw).unwrap();
  }
}
