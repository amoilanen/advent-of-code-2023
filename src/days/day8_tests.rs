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