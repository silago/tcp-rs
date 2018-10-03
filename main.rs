use std::io::prelude::*;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::Result;
use std::str;
use std::sync::mpsc;
use std::collections::HashMap;

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

//fn client_handle(stream: TcpStream) {
//    let msg = String::from("Test");
//    stream.write(msg.as_bytes());
//}

// type Chan<T> = (Sender<T>, Receiver<T>);

struct Chan {
    tx: Sender<String>,
    rx: Receiver<String>
}

impl Chan {
    fn fromTuple(  input: ( Sender<String>, Receiver<String>) ) -> Chan {
        let (tx, rx) = input;
        return Chan{tx: tx, rx:rx};
    }
}

fn main() {
    let addr = "192.168.1.153:9009";
    let listener = TcpListener::bind(addr).unwrap();
    //println( 
    println!("listening:: {0} ",addr);
    let mut connections: HashMap<i32, Chan> = HashMap::new();


    loop {
        match listener.accept() {
            Err(_) => println!("Listener error"),
            Ok((mut s , addr)) => {
                let mut stream:TcpStream = s;
                println!("Client connected");
                let _ = stream.write(String::from("connection_successfull").as_bytes());
                let chan:Chan = Chan::fromTuple( 
                        mpsc::channel()
                );
                let mut msg:String = String::from("");
                loop {
                    //let mut msg = String::new();
                    //let mut buffer = [0; 10];
                    let mut buffer = vec![0; 10];
                    stream.read(&mut buffer).unwrap();   // ignore here too
                    let mut part = &String::from_utf8(buffer).unwrap().to_owned();
                    msg.push_str(part);

                    let x = msg.find("<CLOSE>");//.unwrap_or(-1);
                    let mut terminate=false;
                    match(x) {
                        Some(val) => {
                            terminate = true;
                        }
                        None => {}
                    }


                    let x = msg.find("<EOF>");//.unwrap_or(-1);
                    match(x) {
                        Some(val) => {
                            let result:String= msg.chars().skip(0).take(val).collect();
                            println!("msg: {0}", result); 
                            if (val!=msg.len()) {
                                msg = msg.chars().skip(val+5).take(msg.len()).collect();
                            } else {
                                msg= String::from("");
                            }
                        }
                        None => {
                        }
                    }
                    if (terminate) {
                        break;
                    }
                }
            }
        }
    }
}

