use std::fs::File;
use std::string::String;
use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream};


fn read_password(s: &mut String) -> Result<(),Error> {
    let mut f = try!(File::open("/etc/rconsole.pw"));

    try!(f.read_to_string(s));
    Ok(())
}

fn handle_client(mut stream: TcpStream, password: String) {
    let _ = stream.write(b"Password: ");
    let mut buffer : Vec<u8> = vec![0; 128];
    let len = stream.read(&mut buffer[..]).unwrap();
    let bla = match String::from_utf8(buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8: {}", e),
    };

    print!("{}", bla);

    if bla == password {
        let _ = stream.write(b"Hello, world\n");
    }
    print!("Test");
    
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
