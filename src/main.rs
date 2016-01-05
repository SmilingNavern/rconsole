use std::fs::File;
use std::string::String;
use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream};

fn utf8_to_string(vector: Vec<u8>) -> String {
  String::from_utf8(vector).unwrap()
}

fn read_password(s: &mut String) -> Result<(),Error> {
    let mut f = try!(File::open("/etc/rconsole.pw"));

    try!(f.read_to_string(s));
    Ok(())
}

fn handle_client(mut stream: TcpStream, password: String) {
    let _ = stream.write(b"Password: ");
    let mut buffer : Vec<u8> = vec![0; 128];
    let _ = stream.read(&mut buffer[..]).unwrap();
    let bla = password.clone().into_bytes();
//    let bla = match String::from_utf8(buffer) {
//        Ok(v) => v,
//        Err(e) => panic!("Invalid UTF-8: {}", e),
//    };
    let s1 = utf8_to_string(bla);
    let s2 = utf8_to_string(buffer);
    let s3= s1.trim();
    let s4= s2.trim();
    print!("{} and {}", s3, s4);

    if s3 == s4 {
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
