pub mod topic;
use std::num::ParseIntError;


fn parse_input(s: &str) -> Result<&str, &str> {
    let mut iter = s.split(", ");

    match (iter.next(), iter.next()) {
        (Some(topic), Some(n)) => {
            Ok("") // TODO: parse n
        },
        _ => Err("Bad input")
    }
}
