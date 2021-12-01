use std::fs;
use advent_of_code21_day1::*;

fn get_data(filename: &str) -> Vec<u32> {
  fs::read_to_string(filename).unwrap()
    .lines()
    .map(|s| s.trim().to_owned())
    .filter(|s| !s.is_empty())
    .map(|s| u32::from_str_radix(&s,10).unwrap())
    .collect()
}

#[test]
fn part_1_count_number_of_increases_in_data_series() {
  // Arrange
  let values: Vec<u32> = get_data("input.txt");
  
  // Act
  let actual = count_number_of_increases(values);

  // Assert
  assert_eq!(1688, actual);
  println!("Answer 1: {}", actual)
}

#[test]
fn part_2_count_number_of_increases_in_sliding_window() {
  // Arrange
  let values: Vec<u32> = get_data("input.txt");

  // Act
  let actual = count_number_of_increases_in_sliding_window(values);

  // Assert
  assert_eq!(Ok(1728), actual);
  println!("Answer 2: {}", actual.unwrap())
}   