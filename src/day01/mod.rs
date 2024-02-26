use std::ops::Add;
use regex::Regex;

use std::vec::Vec;
use itertools::Itertools;
use crate::utils::get_input;

// took it from https://github.com/yaheath/advent2023/blob/main/src/bin/day01.rs
trait FirstLastAdaptor: Iterator {
  fn first_last(self) -> Option<(Self::Item, Self::Item)>;
}

impl<I> FirstLastAdaptor for I
  where I: Iterator, I::Item: Clone {
  fn first_last(mut self) -> Option<(I::Item, I::Item)>
    where I::Item: Clone {
    if let Some(first) = self.next() {
      let mut last = first.clone();
      while let Some(next) = self.next() {
        last = next;
      }
      Some((first, last))
    } else {
      None
    }
  }
}

pub fn part_one() {
  let content = get_input("./src/day01/part_1.txt");

  let mut sum = 0;

  for line in &content {
    let chars: Vec<&str> = line.matches(char::is_numeric).collect();

    let fst = *chars.first().expect("No fst");
    let last = *chars.last().expect("No last");

    let result = String::from(fst).add(last);

    let number = result.parse::<i32>().unwrap();

    sum = sum + number
  }
  println!("{}", sum)
}


// mine is failed :(
pub fn part_two() {
  let content = get_input("./src/day01/part_2.txt");

  let possible_values = [
    ("1", 1), ("2", 2), ("3", 3), ("4", 4),
    ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ("one", 1), ("two", 2), ("three", 3), ("four", 4),
    ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
  ];

  // took it from https://github.com/yaheath/advent2023/blob/main/src/bin/day01.rs
  let summ: u64 = content.iter()
      .map(|s| possible_values.iter()
          .map(|(k, v)| s.match_indices(k).map(move |(idx, _)| (idx, v)))
          .flatten()
          .sorted_unstable_by_key(|t| t.0)
          .map(|(_, v)| v)
          .first_last()
          .map(|(f, l)| f * 10 + l)
          .unwrap()
      )
      .sum();

  let mut sum = 0;

  for line in &content {
    let reg = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();

    let mut new_array: Vec<&str> = vec![];

    for (full, []) in reg.captures_iter(&line).map(|c| c.extract()) {
      match full {
        "one" => new_array.push("1"),
        "two" => new_array.push("2"),
        "three" => new_array.push("3"),
        "four" => new_array.push("4"),
        "five" => new_array.push("5"),
        "six" => new_array.push("6"),
        "seven" => new_array.push("7"),
        "eight" => new_array.push("8"),
        "nine" => new_array.push("9"),
        char => new_array.push(char)
      }
    }

    let fst = *new_array.first().expect("No fst");
    let last = *new_array.last().expect("No last");

    let mut result = String::from(fst).add(last);

    println!("Line {}, new array {:?}, result {}", line, new_array, result);

    let number = result.parse::<i32>().unwrap();

    sum = sum + number;
  }


  println!("Mine Sum {:?}", sum);
  println!("Correct {:?}", summ)
}