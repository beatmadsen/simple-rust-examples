use super::Topic;
use super::FirstUpper;

pub struct Elementary;

struct StructWithState {
    a: u32,
    b: String
}

impl StructWithState {

    fn my_method(&self, my_string: &str) -> i32 {

        println!("a was {}, parameter was {}", self.a, my_string);

        43
    }
}


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

    let mut my_struct = StructWithState { a: 42, b: "lala".to_string() };

    my_struct.a = 89;

    my_struct.my_method("haha");

    println!("Todo");
        // TODO
}
