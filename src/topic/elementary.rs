use super::Topic;

pub struct Elementary;

impl Topic for Elementary {

    fn run_example(&self, n: u8) {

        println!("Input was: {}", n);
    }

    fn describe(&self) -> String { "elementary".to_string() }
}
