#[cfg(test)]
use crate::days::day8;
#[cfg(test)]
use crate::days::day8::{ Map, Direction, Node };

#[test]
fn test_parse_input() {
    let input: &str = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

    let parsed = day8::parse(input).unwrap();
    assert_eq!(parsed.instructions, vec![
        Direction::R, Direction::L
    ]);
    assert_eq!(parsed.nodes, vec![
        Node::new("AAA", "BBB", "CCC"),
        Node::new("BBB", "DDD", "EEE"),
        Node::new("CCC", "ZZZ", "GGG"),
        Node::new("DDD", "DDD", "DDD"),
        Node::new("EEE", "EEE", "EEE"),
        Node::new("GGG", "GGG", "GGG"),
        Node::new("ZZZ", "ZZZ", "ZZZ")
    ])
}

#[test]
fn test_number_of_steps() {
    let input: &str = "LLR
    
    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    ";

    let parsed = day8::parse(input).unwrap();

    assert_eq!(day8::steps_to_reach("AAA", |label| label == "ZZZ", &parsed), 6)
}

#[test]
fn test_find_gcd() {
    assert_eq!(day8::greatest_commond_divisor(15, 5), 5);
    assert_eq!(day8::greatest_commond_divisor(30, 45), 15);
    assert_eq!(day8::greatest_commond_divisor(13, 17), 1);
    assert_eq!(day8::greatest_commond_divisor(56, 13), 1);
    assert_eq!(day8::greatest_commond_divisor(30, 42), 6);
}

#[test]
fn test_find_lcm() {
    assert_eq!(day8::least_common_multiple(15, 5), 15);
    assert_eq!(day8::least_common_multiple(30, 45), 90);
    assert_eq!(day8::least_common_multiple(13, 17), 221);
    assert_eq!(day8::least_common_multiple(56, 13), 728);
    assert_eq!(day8::least_common_multiple(30, 42), 210);
}


#[test]
fn test_solution_1() {
    let parsed = day8::parse(day8::INPUT).unwrap();
    assert_eq!(day8::solution_part_1(&parsed), 2)
}