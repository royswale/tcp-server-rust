use std::io::{ Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread::spawn;

fn main() {
    // listen at the address 127.0.0.1:7878 for incoming TCP streams
    //
    // creating a TcpListener by binding it to a socket address,
    // it listens for incoming TCP connections.
    let listener = TcpListener::bind("127.0.0.1:7879").expect("binding to port 7878 failed");

    // The incoming method on TcpListener returns an iterator that gives us a sequence of streams
    // (more specifically, streams of type TcpStream).
    //
    // iterating over the Incoming iterator returned by incoming
    for stream in listener.incoming() {
        match stream {
            // A single stream represents an open connection between the client and the server.
            Ok(stream) => {
                // When it gets an incoming stream, it will print "Connection established!".
                // println!("Connection established!");
                println!("New connection: {}", stream.peer_addr().unwrap());

                // handle_connection(stream)

                // spawn a new thread to handle this connection
                // use move, in order to give ownership of values to a thread.
                spawn(move || {
                    handle_connection(stream);
                });
            }

            Err(e) => {
                // connection failed
                println!("Error: {}", e);
            }
        }

        // The socket will be closed when the value is dropped.
    }
}

fn handle_connection(mut stream: TcpStream) {
    // tell client type quit to exit.
    stream
        .write(b"Welcome. When you are ready to disconnect, type quit.\n")
        .unwrap();
    stream.flush().unwrap();

    // declare a buffer on the stack to hold the data that is read in.
    // We’ve made the buffer 1024 bytes in size, which is big enough to hold the data of a basic request and sufficient for our purposes
    let mut buffer = [0; 1024];

    // read bytes from the TcpStream and put them in the buffer
    while match stream.read(&mut buffer) {
        Ok(size) => {
            // Caution: always read starting with [13, 0] on Windows Cmder telnet
            // println!("{:?}", &buffer[..size]);

            // println!("read {} bytes", size);

            match from_utf8(&buffer[..size]) {
                Ok(s) => {
                    print!("{}", s);

                    if s.len() >= 4 && s[..4] == "quit".to_string() {
                        println!("Bye bye.");
                        stream.write(b"Bye bye.\n").unwrap();
                        stream.flush().unwrap();

                        false
                    } else {
                        // echo everything
                        // stream.write(&buffer[..size]).unwrap();
                        stream.write(s.as_bytes()).unwrap();
                        stream.flush().unwrap();
                        true
                    }
                }
                Err(_) => false,
                // Err(e) => {
                //     println!("{}", e);
                //     false
                // }
            }
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

// client usage:
//  telnet 127.0.0.1 7878
