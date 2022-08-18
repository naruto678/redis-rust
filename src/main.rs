use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
mod client;
mod error;
mod store;
use client::ClientInput;
use error::AppError;
use store::Store;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut val = 1;
    Store::init();
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                thread::spawn(|| handle_connection(stream));
                println!("Connection number {}", val);
                val += 1;
            }
            _ => println!("Something else"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut client = ClientInput::new();
    loop {
        let result = handle_connection_helper(&mut stream, &mut client);
        if result.is_err() {
            let error = result.unwrap_err();
            match error {
                AppError::IncompleteInput(_) => continue,
                AppError::ConnectionClosed(_) => break,
            }
        }
    }
}

fn handle_connection_helper(
    stream: &mut TcpStream,
    client_input: &mut ClientInput,
) -> Result<(), AppError> {
    let mut buffer: [u8; 1024] = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                println!("closing the connection");
                return Err(AppError::ConnectionClosed(String::from(
                    "connection closed",
                )));
            }
            client_input.parse_input(&buffer[0..1024]);
            client_input.respond(stream);
            client_input.reset();
        }
        Err(_) => {
            println!("Error closed");
        }
    }

    Ok(())
}
