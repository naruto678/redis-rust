#[allow(unused)]
use crate::error::AppError;
use crate::store::Store;
use std::{io::Write, net::TcpStream};

pub struct ClientInput {
    input: Vec<u8>,
}

impl ClientInput {
    pub fn new() -> Self {
        Self { input: Vec::new() }
    }
    pub fn respond(&self, stream: &TcpStream) {
        let parsed_input = String::from_utf8_lossy(&self.input);
        let starting_char = &parsed_input[0..1];
        match starting_char {
            "*" => self.parse_array(&parsed_input[1..], stream),
            "+" => self.parse_simple_string(&parsed_input[1..], stream),
            "-" => self.parse_error(&parsed_input[1..], stream),
            ":" => self.parse_integer(&parsed_input[1..], stream),
            "$" => self.parse_bulk_string(&parsed_input[1..], stream),
            _ => println!("Did not match anything .dude . do you even resp"),
        }
    }

    fn parse_array(&self, parsed_input: &str, stream: &TcpStream) {
        let mut command_vec: Vec<&str> = parsed_input
            .split("\r\n")
            .filter(|val| !val.starts_with("$"))
            .collect();

        command_vec.remove(command_vec.len() - 1);
        println!("This is the command vec {:?}", command_vec);
        let command = *command_vec.get(1).unwrap();
        match command {
            "ping" => self.write_to_stream(stream, String::from("+PONG\r\n")),
            "echo" => {
                let response_string: String = command_vec.split_off(2).concat();
                println!("Got the response string as {}", response_string);

                self.write_to_stream(
                    stream,
                    format!("${}\r\n{}\r\n", response_string.len(), response_string),
                )
            }
            "set" => {
                let key_val = command_vec.split_off(2);
                println!("this is the key-val {:?}", key_val);
                let key = key_val.get(0).unwrap();
                let val = key_val.get(1).unwrap();

                if key_val.len() >= 4 {
                    let px_comm = key_val.get(2).unwrap();
                    let expiry_time: u64 = key_val
                        .get(3)
                        .unwrap()
                        .trim()
                        .parse()
                        .expect("Failed to parse the expiry time");
                    println!("key- {}, val-{}, expiry-time- {}", key, val, expiry_time);
                    self.write_to_stream(stream, "+OK\r\n".to_string());
                    Store::set_with_expiry(key, val, expiry_time);
                    return;
                }

                println!("key- {}, val - {}", key, val);
                Store::set(key, val);
                self.write_to_stream(stream, "+OK\r\n".to_string());
            }
            "get" => {
                let key_val = command_vec.split_off(2);
                println!("this is the val {:?} ", key_val);
                let parsed_val = key_val.get(0).unwrap();
                match Store::get(parsed_val) {
                    Some(val) => {
                        self.write_to_stream(stream, format!("${}\r\n{}\r\n", val.len(), val))
                    }
                    _ => {
                        println!("Got nothing from the store");
                        self.write_to_stream(stream, "$-1\r\n".to_string());
                    }
                }
            }
            _ => println!("Got something else "),
        }
    }

    fn parse_simple_string(&self, parsed_input: &str, mut stream: &TcpStream) {}

    fn parse_error(&self, parsed_input: &str, mut stream: &TcpStream) {}

    fn parse_integer(&self, parsed_input: &str, mut stream: &TcpStream) {}

    fn parse_bulk_string(&self, parsed_input: &str, mut stream: &TcpStream) {}

    pub fn reset(&mut self) {
        self.input = Vec::new();
    }
    fn write_to_stream(&self, mut stream: &TcpStream, val: String) {
        match stream.write_all(val.as_bytes()) {
            Ok(_) => println!("Successfully wrote {} to stream", val),
            Err(_) => println!("Error while writing to stream"),
        }
    }

    pub fn parse_input(&mut self, buffer: &[u8]) -> Result<(), AppError> {
        self.input.extend_from_slice(buffer);
        Ok(())
    }
}
