use crate::day5::get_conversions;
#[cfg(test)]
use crate::days::day5;
#[cfg(test)]
use crate::days::day5::Almanac;
#[cfg(test)]
use crate::days::day5::Map;
#[cfg(test)]
use crate::days::day5::RangeRule;

#[test]
fn test_parse() {
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
    let parsed = day5::parse(input);
    assert_eq!(parsed, Almanac::new(
        vec![79, 14, 55, 13],
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
    let almanac = day5::parse(day5::INPUT);
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
fn test_solution_part1() {
    let parsed = day5::parse(day5::INPUT);
    assert_eq!(day5::solution_part_1(&parsed), 35)
}