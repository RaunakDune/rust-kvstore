use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    // let key = arguments.next().unwrap();
    let key = arguments.next().expect("Missing Key");
    // let value = arguments.next().unwrap();
    let value = arguments.next().expect("Missing Value");
    println!("The key is '{}', and the value is '{}'", key, value);

    // let contents = format!("{}\t{}\n",key,value);

    // std::fs::write("kv.db", contents).unwrap();
    let mut database = Database::new().expect("Database Creation Failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
    // match write_result {
    //     Ok(()) => {
            
    //     }
    //     Err(e) => {

    //     }
    // }



}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the database kv.db
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //          return Err(error);
        //     }
        // };
        // parse
        // populate

        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt Database");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key,value);
    }
    // Makes sure that noone else can touch the database after flush
    // Ensures flush is the last thing we do
    fn flush(&mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)

    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }

    }
}

fn do_flush(database: &Database)  -> std::io::Result<()>{
    let mut contents = String::new();
    for (key, value) in &database.map {
        // let kvpair = format!("{}\t{}\n", key, value);
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
// File structure:
// mykey\tmyvalue\nmykey\tmyvalue\n
