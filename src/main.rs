use std::{eprint, print, println};
use std::io::{self,Write};
use rust_cache::{Cache, Command, parse};


fn main() {
    let mut cache = Cache::new();


    println!("Rust Cache");
    println!("Commands: SET <key> <value> , GET <key>, DEL <key>,EXIT");

   loop {
       print!(">");
       if let Err(error) = io::stdout().flush() {
           eprint!("Output error:{error}");
           break;
       }

       let mut input = String::new();

       match io::stdin().read_line(&mut input) {
           Ok(0) => break,
           Ok(_) => {}
           Err(error)=>{
            eprint!("Input error:{error}");
            break;
           }
       }

       let command = match parse(&input){
        Ok(command)=> command,
        Err(error)=>{
            println!("ERR{error:?}");
            continue;
        }
       };

       match command{
        Command::Set { key, value } =>{
            cache.set(key, value);
            println!("ok");
        }

        Command::Get { key }=> match cache.get(&key){
            Some(value)=> println!("{}",value),
            None=> println!("(nil)")
        }

        Command::Del { key }=>{
            let count = if cache.del(&key).is_some() { 1 } else { 0 };
                println!("{count}");
        }

        Command::Exit=>{
            println!("bye");
            break;
        }
       }
   }
}