use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    // let key = arguments.next().unwrap();
    let key = arguments.next().expect("Missing Key");
    // let value = arguments.next().unwrap();
    let value = arguments.next().expect("Missing Value");
    println!("The key is '{}', and the value is '{}'", key, value);

    let contents = format!("{}\t{}\n",key,value);

    std::fs::write("kv.db", contents).unwrap();
    let database = Database::new().expect("Database Creation Failed");
    // match write_result {
    //     Ok(()) => {
            
    //     }
    //     Err(e) => {

    //     }
    // }



}

struct Database {
    map: HashMap<String, String>,
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
        Ok(Database { map: map })
    }
}

// File structure:
// mykey\tmyvalue\nmykey\tmyvalue\n
