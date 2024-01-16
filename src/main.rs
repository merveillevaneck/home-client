use rust_socketio::{Payload, SocketBuilder};
use serde_json::json;
use std::io::*;

enum Instruction {
    Alert,
    Print,
}

impl Instruction {
    fn decode(instruction: &str) -> Instruction {
        match instruction {
            "alert" => Instruction::Alert,
            "print" => Instruction::Print,
            _ => panic!("Unknown instruction"),
        }
    }
    fn process(instruction: Instruction, message: Option<&str>) {
        match instruction {
            Instruction::Alert => alert(),
            Instruction::Print => match message {
                Some(message) => print(message),
                None => (),
            },
        }
    }
}

fn print(message: &str) {
    println!("{}", message);
}

fn alert() {
    println!("Alert!");
}

fn process_instruction(instruction: &str) {
    let code = Instruction::decode(instruction);
    Instruction::process(code, None);
}

fn main() {
    let callback = |payload: Payload, _| match payload {
        Payload::String(str) => process_instruction(&str),
        Payload::Binary(bin_data) => println!("{:?}", bin_data),
    };

    let host = "127.0.0.1:3001";

    let mut socket = SocketBuilder::new(host)
        .on("message", callback)
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");

    loop {
        let mut input = String::new();
        let _input = stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        input = input[0..input.len() - 2].to_string();
        input = format!("{:?}: {}", "machine", input);
        socket
            .emit("message", json!(input))
            .expect("Server unreachable");
    }
}
