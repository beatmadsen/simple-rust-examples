extern crate simple_examples;
use simple_examples::topic::Topic;
use simple_examples::topic::ex1::Ex1;
use simple_examples;

fn main() {
    Ex1.run_example(42);

    println!("Choose an example ('topic, number')");
}
