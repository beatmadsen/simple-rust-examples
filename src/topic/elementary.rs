use super::Topic;

pub struct Elementary;

impl Topic for Elementary {

    fn run_example(&self, n: u8) {

        println!("Topic: Elementary, example {}", n);
        match n {
            1 => { println!("Hello, world!"); },
            2 => { example_2(); },
            _ => { println!("Example not known or not implemented!"); }
        };
    }

    fn describe(&self) -> String { "elementary".to_string() }
}



fn example_2() {
    println!("Todo");
        // TODO
}
