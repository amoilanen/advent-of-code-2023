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