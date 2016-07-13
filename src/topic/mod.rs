
pub trait FirstUpper {

    fn first_upper(&self) -> String;
}


pub trait Topic {

    fn run_example(&self, n: u8);

    fn describe(&self) -> String;
}

pub mod elementary;
pub mod lists_strings;
