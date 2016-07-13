pub mod topic;

use self::topic::Topic;
use self::topic::elementary::Elementary;
use self::topic::lists_strings::ListStrings;
use std::collections::HashMap;

pub fn parse_input(s: &str) -> Result<(&str, u8), String> {
    let mut iter = s.split(",");
    match (iter.next(), iter.next()) {
        (Some(topic), Some(n)) => Ok((topic, n)),
        _ => Err(format!("Input '{}' didn't match required pattern", s))
    }.and_then(parse_tuple)
}

fn parse_tuple<'a>(tuple: (&'a str, &str)) -> Result<(&'a str, u8), String> {
    let (topic, n) = tuple;
    n.trim()
        .parse()
        .map(|num| { (topic, num) })
        .or_else(|e| { Err(format!("Failed to parse number from '{}': {}", n, e)) })
}

pub fn populate_map() -> HashMap<String, Box<Topic>> {
    let mut map: HashMap<String, Box<Topic>> = HashMap::new();

    map.insert(Elementary.describe(), Box::new(Elementary));
    map.insert(ListStrings.describe(), Box::new(ListStrings));

    map
}


#[test]
fn should_parse_valid_input() {

    let (topic, num) = parse_input("Dance, 42").unwrap();

    assert_eq!(num, 42);
    assert_eq!(topic, "Dance");
}

#[test]
fn should_fail_to_parse_non_number(){

    let result = parse_input("Comom, Lomom");

    assert!(result.is_err());
}


#[test]
fn should_fail_to_parse_missing_comma(){

    let result = parse_input("Toldom Acjdul");

    assert!(result.is_err());
}
