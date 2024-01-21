mod color;

use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

// run using: cargo run -- 192.168.0.38:1337

fn main() -> std::io::Result<()> {
    let mut args = env::args_os();
    let progname = match args.next() {
        None => return Ok(()),
        Some(x) => x,
    };
    let addr = match args.next() {
        None => {
            println!("usage: <{}>", progname.to_str().unwrap());
            return Ok(());
        }
        Some(x) => x,
    };

    let mut stream = TcpStream::connect(addr.to_str().unwrap())?;

    for i in 0..1920 {
        for j in 0..1080 {
            let h = (i as u64 + j as u64) % 255;
            let cmd = format!(
                "PX {} {} {}\n",
                i,
                j,
                color::HSV {
                    h: h as u8,
                    s: 255,
                    v: 120
                }
                .hex()
            );
            stream.write(cmd.as_bytes())?;
        }
    }
    Ok(())
}
