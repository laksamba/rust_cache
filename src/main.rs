use std::println;

use rust_cache::Cache;


fn main() {
    let mut cache = Cache::new();
    cache.set("user:1".to_string(), "Alice".to_string());

    if let Some(val) = cache.get("user:1"){
        println!("GET user:1 ->{}",val);
    }

    if let Some(val) = cache.del("user:1"){
        println!("DEL user:1 ->remove {}",val);
    }

    if cache.get("user:1").is_none(){
        println!("GET user:1 -> key not found");
    }
}