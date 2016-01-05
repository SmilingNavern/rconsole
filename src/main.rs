use std::fs::File;
use std::string::String;
use std::io::{Read, Write, Error, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};


fn read_password() -> Result<(String),Error> {
    let mut f = try!(File::open("/etc/rconsole.pw"));
    let mut s = String::with_capacity(128);

    try!(f.read_to_string(&mut s));
    Ok((s))
}

fn handle_client(mut stream: TcpStream, password: String) {
    let mut rstream = BufReader::new(stream.try_clone().unwrap());
    let _ = stream.write(b"Password: ");
    let mut buffer = String::new();
    let _ = rstream.read_line(&mut buffer).unwrap();
    let s1 = password.trim();
    let s2 = buffer.trim();

    print!("{} and {}", s1, s2);

    if s1 == s2 {
        let _ = stream.write(b"Hello, world\n");
    } else {
        let _ = stream.write(b"Wrong pass\n");
    }
    let _ = stream.write(b"TEST\n");
    
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    let password = read_password().unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, password.clone());
            }
            Err(e) => { 
                print!("Connection failed: {}", e);
            }
        }
    }

    drop(listener);
   
}
