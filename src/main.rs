use std::fs::File;
use std::string::String;
use std::io::{Read, Write, Error, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};


fn read_password(s: &mut String) -> Result<(),Error> {
    let mut f = try!(File::open("/etc/rconsole.pw"));

    try!(f.read_to_string(s));
    Ok(())
}

fn handle_client(mut stream: TcpStream, password: String) {
    let mut rstream = BufReader::new(stream.try_clone().unwrap());
    let _ = stream.write(b"Password: ");
    let mut buffer = String::new();
    let _ = rstream.read_line(&mut buffer).unwrap();
    let bla = password.clone();
//    let bla = match String::from_utf8(buffer) {
//        Ok(v) => v,
//        Err(e) => panic!("Invalid UTF-8: {}", e),
//    };
    let s1 = bla.trim();
    let s2 = buffer.trim();
    print!("{} and {}", s1, s2);
    print!("LEN {} and {}\n", s1.len(), s2.len());

    if s1 == s2 {
        let _ = stream.write(b"Hello, world\n");
    } else {
        let _ = stream.write(b"Wrong pass\n");
    }
    let _ = stream.write(b"TEST\n");
    
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let mut password = String::new();

    let _ = read_password(&mut password);

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
