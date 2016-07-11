use super::Topic;

pub struct Ex1;

impl Topic for Ex1 {

    fn run_example(&self, n: u8) {

        println!("Input was: {}", n);
    }
}
