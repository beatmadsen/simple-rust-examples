

pub trait Topic {

    fn run_example(&self, n: u8);

    fn describe(&self) -> String;
}

pub mod elementary;
