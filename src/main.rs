#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Message of size: {:?}", size);
            /*
            for chunk in buffer[..size].chunks(16) {
                println!("  {:02x?}", chunk);
            }
             */
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            false
        }
    } {}
}

fn main() {
    unsafe {
        let msg = j_message_new(JMessageType_J_MESSAGE_DB_QUERY, 50);
        let msg_type =j_message_get_type(msg);
        println!("{}", msg_type)
    }

    let listener = TcpListener::bind("[::]:4711").unwrap();
    println!("Server listening on port 4711");


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
