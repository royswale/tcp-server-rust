use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::str::from_utf8;
use std::thread::spawn;

fn main() {
    // listen at the address 127.0.0.1:7878 for incoming TCP streams
    //
    // creating a TcpListener by binding it to a socket address,
    // it listens for incoming TCP connections.
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("binding to port 7878 failed");

    // The incoming method on TcpListener returns an iterator that gives us a sequence of streams
    // (more specifically, streams of type TcpStream).
    //
    // iterating over the Incoming iterator returned by incoming
    for stream in listener.incoming() {
        match stream {
            // A single stream represents an open connection between the client and the server.
            Ok(s) => {
                // When it gets an incoming stream, it will print "Connection established!".
                // println!("Connection established!");
                println!("New connection: {}", s.peer_addr().unwrap());

                // handle_connection(s)
                spawn(|| {
                   handle_connection(s);
                });
            },

            Err(e) => {
                // connection failed
                println!("Error: {}", e);
            }
        }

        // The socket will be closed when the value is dropped.
    }
}

fn handle_connection(mut stream: TcpStream) {
    // stream.write(b"Welcome. Please input your name. When you are ready to disconnect, type quit.").unwrap();
    // stream.flush().unwrap();

    let mut reader = &BufReader::new(stream.try_clone().unwrap());
    let mut line = String::new();

    // declare a buffer on the stack to hold the data that is read in.
    // We’ve made the buffer 1024 bytes in size, which is big enough to hold the data of a basic request and sufficient for our purposes
    // let mut buffer = [0; 1024];
    let mut vec: Vec<u8> = vec![];

    // // read bytes from the TcpStream and put them in the buffer
    // let size = steam.read(&mut buffer).unwrap();
    //
    // // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    //
    // // echo everything
    // let echo = "hello";
    // // steam.write(&buffer[0..size]).unwrap();
    // steam.write(echo.as_bytes()).unwrap();
    // steam.flush().unwrap();


    // while match stream.read(&mut buffer) {
    // match stream.read(&mut buffer) {
    // match stream.read_to_end(&mut vec) {
    //     Ok(size) => {
    //         println!("read {} bytes", size);
    //         // println!("{}", from_utf8(&buffer[0..size]).unwrap());
    //         println!("{}", String::from_utf8(vec).unwrap());
    //         // stream.write(&buffer[0..size]).unwrap();
    //         // true
    //     },
    //     Err(_) => {
    //         println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
    //         stream.shutdown(Shutdown::Both).unwrap();
    //         // false
    //     }
    // }
    // }
    // } {}


    while
    // match reader.read_line(&mut line) {
    match reader.lines().last().unwrap() {
        Ok(size) => {
            // println!("read {} bytes: {}", size, line);
            // println!("{} bytes, string: {}", size, line);
            // println!("{}", line);
            // println!("{}", line.as_str().trim());
            println!("{}", size);
            // stream.write(line.as_ref()).unwrap();
            // stream.flush().unwrap();
            // let s = ((&mut reader).fill_buf().unwrap()).len();
            // &mut reader.consume(s);
            // let l = &mut reader.lines().last().unwrap().unwrap();
            // println!("last line: {}", l);
            // String::from_utf8()
            // String::from_utf8_lossy()
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}

// let mut line = String::with_capacity(512);
    // loop {
    //     let result = stream.read_to_string(&mut line);
    //     match result {
    //         Ok(n) => println!("Received {} bytes: {}", n, line),
    //         // _ => {}
    //         Err(e) => {
    //             println!("{}",e)
    //         }
    //     }
    //     line.clear();
    // }
}

// client usage:
//  telnet 127.0.0.1 7878
