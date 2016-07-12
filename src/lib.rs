pub mod topic;


pub fn parse_input(s: &str) -> Result<(&str, u8), &str> {
    let mut iter = s.split(",");
    match (iter.next(), iter.next()) {
        (Some(topic), Some(n)) => Ok((topic, n)),
        _ => Err("Bad input")
    }.and_then(|(topic, n)| {
        n.trim()
            .parse::<u8>()
            .map(|num| { (topic, num) })
            .or(Err("Bad input"))
    })
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

    assert_eq!(result, Err("Bad input"));
}


#[test]
fn should_fail_to_parse_missing_comma(){

    let result = parse_input("Toldom Acjdul");

    assert_eq!(result, Err("Bad input"));
}
