extern crate simple_examples;
use simple_examples::topic::Topic;
use simple_examples::*;
use std::io;

fn main() {

    let map = populate_map();

    loop {
        println!("Choose an example ('topic, number')");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        if choice == "q" || choice == "Q" {
            break;
        }

        let (topic, example) = match parse_input(&choice) {
            Ok(tuple) => tuple,
            Err(e) => {
                println!("Error when parsing: {}", e);
                continue;
            }
        };

        let topic = match map.get(topic) {
            Some(t) => t,
            None => {
                println!("Didn't find matching topic: {}", topic);
                continue;
            }
        };

        topic.run_example(example);

    }

    println!("Goodbye!")




}
