use crate::day5::get_conversions;
#[cfg(test)]
use crate::days::day5;
#[cfg(test)]
use crate::days::day5::Almanac;
#[cfg(test)]
use crate::days::day5::Map;
#[cfg(test)]
use crate::days::day5::RangeRule;
#[cfg(test)]
use crate::days::day5::Range;
#[cfg(test)]
use std::collections::HashSet;

#[test]
fn test_parse_for_part_1() {
    let input: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69";
    let parsed = day5::parse_for_part_1(input);
    assert_eq!(parsed, Almanac::new(
        Some(vec![79, 14, 55, 13]),
        None,
        vec![
            Map::new(vec![
                RangeRule::new(50, 98, 2),
                RangeRule::new(52, 50, 48)
            ]),
            Map::new(vec![
                RangeRule::new(0, 15, 37),
                RangeRule::new(37, 52, 2),
                RangeRule::new(39, 0, 15)
            ]),
            Map::new(vec![
                RangeRule::new(49, 53, 8),
                RangeRule::new(0, 11, 42),
                RangeRule::new(42, 0, 7),
                RangeRule::new(57, 7, 4)
            ]),
            Map::new(vec![
                RangeRule::new(88, 18, 7),
                RangeRule::new(18, 25, 70)
            ]),
            Map::new(vec![
                RangeRule::new(45, 77, 23),
                RangeRule::new(81, 45, 19),
                RangeRule::new(68, 64, 13)
            ]),
            Map::new(vec![
                RangeRule::new(0, 69, 1),
                RangeRule::new(1, 0, 69)
            ])
        ]
    ))
}

#[test]
fn test_get_conversions() {
    let almanac = day5::parse_for_part_1(day5::INPUT);
    assert_eq!(
        get_conversions(79, &almanac.maps),
        vec![79, 81, 81, 81, 74, 78, 78, 82]
    );
    assert_eq!(
        get_conversions(14, &almanac.maps),
        vec![14, 14, 53, 49, 42, 42, 43, 43]
    );
    assert_eq!(
        get_conversions(55, &almanac.maps),
        vec![55, 57, 57, 53, 46, 82, 82, 86]
    );
    assert_eq!(
        get_conversions(13, &almanac.maps),
        vec![13, 13, 52, 41, 34, 34, 35, 35]
    )
}

#[test]
fn range_rule_convert_range() {
    let rule = RangeRule::new(20, 10, 5); // 10 11 12 13 14 -> 20 21 22 23 24
    let range_left_of_rule = Range::new(4, 8);
    assert_eq!(rule.convert_range(&range_left_of_rule), (Vec::new(), vec![Range::new(4, 8)]));
    let range_right_of_rule = Range::new(16, 18);
    assert_eq!(rule.convert_range(&range_right_of_rule), (Vec::new(), vec![Range::new(16, 18)]));
    let range_intersecting_with_rule_from_left = Range::new(6, 12);
    assert_eq!(rule.convert_range(&range_intersecting_with_rule_from_left), (vec![Range::new(20, 22)], vec![Range::new(6, 9)]));
    let range_intersecting_with_rule_from_right = Range::new(13, 18);
    assert_eq!(rule.convert_range(&range_intersecting_with_rule_from_right), (vec![Range::new(23, 24)], vec![Range::new(15, 18)]));
    let range_subsuming_rule = Range::new(8, 18);
    assert_eq!(rule.convert_range(&range_subsuming_rule), (vec![Range::new(20, 24)], vec![Range::new(8, 9), Range::new(15, 18)]));
    let range_subsumed_by_rule = Range::new(11, 13);
    assert_eq!(rule.convert_range(&range_subsumed_by_rule), (vec![Range::new(21, 23)], Vec::new()));
}

#[test]
fn map_convert_range() {
    let map = Map::new(vec![
        RangeRule::new(20, 10, 2),
        RangeRule::new(30, 12, 2),
        RangeRule::new(40, 14, 2)
    ]);
    let range = Range::new(8, 20);
    let expected = [
        Range::new(8, 9),
        Range::new(20, 21),
        Range::new(30, 31),
        Range::new(40, 41),
        Range::new(16, 20)
    ];
    let result: HashSet<Range> = HashSet::from_iter(map.convert_ranges(&vec![range]));
    assert_eq![result, HashSet::from(expected)];
}

#[test]
fn test_parse_for_part_2() {
    let input: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69";
    let parsed = day5::parse_for_part_2(input);
    assert_eq!(parsed, Almanac::new(
        None,
        Some(vec![Range::new(79, 92), Range::new(55, 67)]),
        vec![
            Map::new(vec![
                RangeRule::new(50, 98, 2),
                RangeRule::new(52, 50, 48)
            ]),
            Map::new(vec![
                RangeRule::new(0, 15, 37),
                RangeRule::new(37, 52, 2),
                RangeRule::new(39, 0, 15)
            ]),
            Map::new(vec![
                RangeRule::new(49, 53, 8),
                RangeRule::new(0, 11, 42),
                RangeRule::new(42, 0, 7),
                RangeRule::new(57, 7, 4)
            ]),
            Map::new(vec![
                RangeRule::new(88, 18, 7),
                RangeRule::new(18, 25, 70)
            ]),
            Map::new(vec![
                RangeRule::new(45, 77, 23),
                RangeRule::new(81, 45, 19),
                RangeRule::new(68, 64, 13)
            ]),
            Map::new(vec![
                RangeRule::new(0, 69, 1),
                RangeRule::new(1, 0, 69)
            ])
        ]
    ))
}

#[test]
fn test_solution_part1() {
    let parsed = day5::parse_for_part_1(day5::INPUT);
    assert_eq!(day5::solution_part_1(&parsed), 35)
}

#[test]
fn test_solution_part2() {
    let parsed = day5::parse_for_part_2(day5::INPUT);
    assert_eq!(day5::solution_part_2(&parsed), 46)
}