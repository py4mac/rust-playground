use std::{collections, env};
use std::fs;

pub fn run() {
    let mut arguments= env::args().skip(1);

    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    println!("The key is {} and the value is {}", key, value);

    let contents = format!("{}\t{}\t", key, value);

    let write_result = fs::write("kb.db", contents);

    match write_result {
        Ok(()) => {
            println!("OK");
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    let database = Database::new();

}

struct Database {
    map: collections::HashMap<String, String>,
}

impl Database {
    fn new() ->  Result<Database, std::io::Error> {
        // let contents = match fs::read_to_string("kv.db") {
        //     Ok(c) => {
        //         c
        //     }   
        //     Err(error) => {
        //         return Err(error);
        //     }

        // };
        let contents = fs::read_to_string("kv.db")?;
        let mut map = collections::HashMap::new();
        for line in contents.lines() {
            let mut chunks = line.splitn(2, "\t");
            let key = chunks.next().expect("No Key");
            let value = chunks.next().expect("No Value");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {map: map})
    }
}