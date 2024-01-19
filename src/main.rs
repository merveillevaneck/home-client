use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use rust_socketio::{Payload, SocketBuilder};
use serde_json::json;
use std::time::Duration;
use std::{io::*, os::unix::process};

enum Instruction {
    Alert,
    Print,
}

impl Instruction {
    fn decode(instruction: &str) -> Instruction {
        match instruction {
            "alert" => Instruction::Alert,
            "print" => Instruction::Print,
            _ => panic!("unknown instruction"),
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

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let wrapped_sink = Sink::try_new(&stream_handle);

    // Add a dummy source of the sake of the example.
    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20);
    let sink = wrapped_sink.unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

fn process_instruction(instruction: &str) {
    let decoded = Instruction::decode(instruction);
    Instruction::process(decoded, None);
}

fn main() {
    let callback = |payload: Payload, _| match payload {
        Payload::String(str) => {
            // let message = str[1..str.len() - 1].to_string();
            // process_instruction(&message);
            alert();
        }
        Payload::Binary(bin_data) => println!("{:?}", bin_data),
    };

    //todo replace this with an env config
    let host = "https://pop-os.tail4f070.ts.net";

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
