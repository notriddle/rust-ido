#[macro_use] extern crate ido;
use std::{env, fs, io};
use std::io::{Read, Write};

pub fn main() {
    let mut args = env::args();
    let _ = args.next(); // discard executable name.
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    for ref file_name in args {
        let result = ido!{
            let file =<< fs::OpenOptions::new().read(true).open(file_name);
            let mut reader = io::BufReader::new(file);
            let mut buf = Vec::new();
            let _ =<< reader.read_to_end(&mut buf);
            stdout.write_all(&buf[..])
        };
        if let Err(e) = result {
            // If we fail to fail, we crash.
            write!(stderr, "{}: {}\n", file_name, e).unwrap();
        }
    }
}
