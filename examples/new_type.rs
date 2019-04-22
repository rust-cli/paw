use std::env::Args;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;

//! To run this example (e.g. with port 8080): 
//! ```
//! cargo run --example new_type -- --port 8080 
//! ```
//! (note that the port should be separated by a space, and port=8080 doesn't work with this code)

#[derive(Debug)]
pub struct ArgWrapper(pub Args);

impl paw::TryParse for ArgWrapper {
    type Error = std::io::Error;
    fn try_parse() -> Result<Self, Self::Error> {
        Ok(Self(std::env::args()))
    }
}

#[derive(Debug)]
pub struct ArgNotFoundError {
    arg: String,
}

impl std::fmt::Display for ArgNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "the argument {} is not passed", self.arg)
    }
}

impl Error for ArgNotFoundError {}

#[paw::main]
fn main(args: ArgWrapper) -> Result<(), Box<dyn Error>> {
    // Rust reads the arguments to the process as an env::Args which is
    // an iterator over stirngs. It contains the arguments as strings if
    // they are separated by space. Ensure you give the argument as --port 8080
    // and not --port=8080 in this case.

    let mut iter = args.0.skip_while(|e| e != "--port");
    iter.next();
    let port = iter
        .next()
        .ok_or_else(|| ArgNotFoundError { arg: "port".into() })?;

    dbg!(&port);
    let listener = TcpListener::bind(("127.0.0.1", port.parse()?))?;
    println!("listening on {}", listener.local_addr()?);
    for stream in listener.incoming() {
        stream?.write(b"hello world!")?;
    }
    Ok(())
}