pub mod topic;
use std::num::ParseIntError;


pub fn parse_input(s: &str) -> Result<(&str, u8), &str> {
    let mut iter = s.split(",");

    match (iter.next(), iter.next()) {
        (Some(topic), Some(n)) => {
            let num: u8 = match n.trim().parse() {
                Ok(m) => m,
                Err(_) => { return Err("Bad input"); }
            };
            Ok((topic, num))
        },
        _ => Err("Bad input")
    }
}
