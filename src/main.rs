#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::os::raw::c_void;
use std::thread;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let listener = TcpListener::bind("[::]:4711").unwrap();
    println!("Server listening on port 4711");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| buffer_to_msg(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn buffer_to_msg(mut stream: TcpStream) {
    let mut buffer = [0; 72];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            unsafe {
                let msg = j_message_new(JMessageType_J_MESSAGE_NONE, size as gsize);
                let g_stream = g_memory_input_stream_new_from_data(
                    buffer.as_ptr() as *const c_void,
                    buffer.len() as gssize,
                    None,
                );
                j_message_read(msg, g_stream);
                msg_to_action(msg);
            }

            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            false
        }
    } {}
}

unsafe fn msg_to_action(msg: *mut JMessage) { unsafe {
    match j_message_get_type(msg) {
        JMessageType_J_MESSAGE_NONE => {}
        JMessageType_J_MESSAGE_PING => {
            // Attempt to handle the ping
            let client_checksum = j_message_get_string(msg);
            // I'm not loading any configuration.
            // let server_checksum = j_configuration_get_checksum(j_configuration());

            // For now, I just assume that the checksum matches
            println!("client checksum = {:?}", client_checksum);

            let reply = j_message_new_reply(msg);
            j_message_append_string(reply, client_checksum);

            // I'm not loading any back ends currently, so that part gets skipped and the reply sent.

            // I will stop here for now.
            // I neither know how to create the GSocketConnection nor does this appear to be a path worth following.
            let connection: gpointer = std::ptr::null_mut();
            j_message_send(reply, connection);
        }
        _ => {}
    }
}}