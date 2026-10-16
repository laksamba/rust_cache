
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{eprintln, format, println};

use crate::command::{parse, Command};
use crate::store::Cache;

pub fn run(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;

    println!("Rust Cache listening on {address}");

    let mut cache = Cache::new();

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                let peer = stream.peer_addr().ok();
                println!("Client connected: {peer:?}");

                if let Err(error) = handle_client(stream, &mut cache) {
                    eprintln!("Client error: {error}");
                }

                println!("Client disconnected: {peer:?}");
            }
            Err(error) => {
                eprintln!("Connection failed: {error}");
            }
        }
    }

    Ok(())
}

fn handle_client(
    stream: TcpStream,
    cache: &mut Cache,
) -> std::io::Result<()> {
    let reader_stream = stream.try_clone()?;
    let mut reader = BufReader::new(reader_stream);
    let mut writer = stream;

    loop {
        let mut input = String::new();

        // Read one newline-terminated command.
        let bytes_read = reader.read_line(&mut input)?;

        // Zero bytes means the client disconnected.
        if bytes_read == 0 {
            break;
        }

        let response = match parse(&input) {
            Ok(Command::Set { key, value }) => {
                cache.set(key, value);
                "OK\n".to_string()
            }

            Ok(Command::Get { key }) => {
                match cache.get(&key) {
                    Some(value) => format!("{value}\n"),
                    None => "(nil)\n".to_string(),
                }
            }

            Ok(Command::Del { key }) => {
                 let deleted = if cache.del(&key).is_some() { 1 } else { 0 };
                format!("{deleted}\n")
            }

            Ok(Command::Exit) => {
                writer.write_all(b"BYE\n")?;
                break;
            }

            Err(error) => format!("ERR {error:?}\n"),
        };

        writer.write_all(response.as_bytes())?;
        writer.flush()?;
    }

    Ok(())
}
