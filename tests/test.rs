use std::fs;
use AdventOfCode21_Day1::count_number_of_increases;

#[test]
fn part_1_count_number_of_increases_in_data_series() {
    // Arrange
    let values: Vec<u32> =
        fs::read_to_string("input.txt").unwrap()
            .lines()
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .map(|s| u32::from_str_radix(&s,10).unwrap())
            .collect();
    
    // Act
    let actual = count_number_of_increases(values);
    
    // Assert
    assert_eq!(1688, actual);
    println!("Answer 1: {}", actual)
}   