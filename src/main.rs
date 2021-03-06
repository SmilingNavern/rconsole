use std::fs::File;
use std::fs::OpenOptions;
use std::string::String;
use std::io::{Read, Write, Error, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};


fn read_password() -> Result<String,Error> {
    let mut f = try!(File::open("/etc/rconsole.pw"));
    let mut s = String::with_capacity(128);

    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn read_char(stream: &TcpStream) -> String {
    let mut rstream = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::with_capacity(2);
    let _ = rstream.read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn do_sysreq(stream: &mut TcpStream, key: String, sysreq_fh: &mut File) -> Result<(), Error> {
    if !key.chars().all(char::is_alphabetic) {
        let _ = stream.write(b"Key out of range\n");
        return Ok(()); 
    } 

    let _ = stream.write(format!("Send {} to sysreq? (y/n)\n", key).as_bytes());
    let answer = read_char(&stream);
    if answer.to_lowercase() == "y".to_string() {
        sysreq_fh.write(key.as_bytes());
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, password: &String, sysreq_fh: &mut File) {
    let mut rstream = BufReader::new(stream.try_clone().unwrap());
    let _ = stream.write(b"Password: ");
    let mut buffer = String::new();
    let _ = rstream.read_line(&mut buffer).unwrap();
    let s1 = password.trim();
    let s2 = buffer.trim();


    //let _ = stream.write(format!("{}",c).as_bytes());
    if s1 == s2 {
        let _ = stream.write(b"Hello, world\n");
        let _ = stream.write(b"PUT A CHAR: ");       
        let c = read_char(&stream);
        if c.chars().all(char::is_uppercase) {
            do_sysreq(&mut stream, c.to_lowercase(), sysreq_fh);
        }

    } else {
        let _ = stream.write(b"Wrong pass\n");
    }
    
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    let password = read_password().unwrap();
    let mut sysreq_fh = OpenOptions::new()
                            .write(true)
                            .open("/proc/sysrq-trigger")
                            .unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &password, &mut sysreq_fh);
            }
            Err(e) => { 
                print!("Connection failed: {}", e);
            }
        }
    }

    drop(listener);
   
}
