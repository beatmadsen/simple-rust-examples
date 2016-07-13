use super::Topic;
use super::FirstUpper;

pub struct ListStrings;


impl FirstUpper for String {

    fn first_upper(&self) -> String {

        let (first, last) = self.split_at(1);
        first.to_uppercase() + last
    }
}

impl Topic for ListStrings {

    fn run_example(&self, n: u8) {

        println!("Topic: {}, example {}", self.describe().first_upper(), n);
        match n {
            1 => { println!("Hello, world!"); },
            _ => { println!("Example not known or not implemented!"); }
        };
    }

    fn describe(&self) -> String { "list-strings".to_string() }

}

//
// fn first_upper(name: String) -> String {
//
//     let (first, last) = name.split_at(1);
//     first.to_uppercase() + last
// }
//
// #[test]
// fn should_raise_first_char_to_upper() {
//
//     let result = first_upper("yo".to_string());
//     assert_eq!(result, "Yo");
// }
