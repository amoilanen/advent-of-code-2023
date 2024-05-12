#[cfg(test)]
use crate::days::day12;
#[cfg(test)]
use crate::days::day12::{ ConditionRecord };

#[test]
fn test_parse_input() {
    let input: &str = "#.#.### 1,1,3
    .#...#....###. 1,1,3
    .#.###.#.###### 1,3,1,6
    ####.#...#... 4,1,1
    #....######..#####. 1,6,5
    .###.##....# 3,2,1";

    let parsed = day12::parse(input).unwrap();
    assert_eq!(parsed,
        vec![
            ConditionRecord::new("#.#.###".chars().collect(), vec![1, 1, 3]),
            ConditionRecord::new(".#...#....###.".chars().collect(), vec![1, 1, 3]),
            ConditionRecord::new(".#.###.#.######".chars().collect(), vec![1, 3, 1, 6]),
            ConditionRecord::new("####.#...#...".chars().collect(), vec![4, 1, 1]),
            ConditionRecord::new("#....######..#####.".chars().collect(), vec![1, 6, 5]),
            ConditionRecord::new(".###.##....#".chars().collect(), vec![3, 2, 1])
        ]
    )
}