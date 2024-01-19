use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use rust_socketio::{Payload, SocketBuilder, Socket};
use serde_json::json;
use std::os::unix::process;
use std::process::exit;
use std::ptr::null;
use std::time::Duration;
use std::collections::hash_map::HashMap;
use env_file_reader::read_file;
use std::io::Error;
use std::io::stdin;


fn read_env() -> Result<HashMap<String, String>, Error> {
    let env_variables = read_file("./.env");
    return env_variables;
}



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

fn process_instruction(instruction: &str, socket: Socket) {
    let decoded = Instruction::decode(instruction);
    Instruction::process(decoded, None);
}

fn main() {
    let env = read_env();
    if env.is_err() {exit(1)}

    let env_variables = env.unwrap();

    println!("String::{:?}", env_variables["HOST_URL"]);
    let callback = |payload: Payload, socket: Socket| match payload {
        Payload::String(str) => {
            let message = str[1..str.len() - 1].to_string();
            process_instruction(&message, socket);
            // alert();
        }
        Payload::Binary(bin_data) => println!("{:?}", bin_data),
    };

    let has_host = env_variables.contains_key("HOST_URL");

    if !has_host { 
        println!("HOST_URL is None");
        exit(1)
    }

    let host = env_variables.get("HOST_URL");
    let mut socket = SocketBuilder::new(host.unwrap())
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
