use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use regex::Regex;
use crate::utils::get_input;


pub fn part_one() {
  let content = get_input("./src/day04/input.txt");

  let res: i32 = content
      .iter()
      .map(|line| {
        let lotto: Vec<HashSet<i32>> = line.split(": ").collect::<Vec<&str>>()[1]
            .split(" | ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|numbers| {
              let re = Regex::new(r"\d+").unwrap();

              let iter_nums = re.find_iter(numbers)
                  .map(|n| n.as_str().parse::<i32>().unwrap());

              HashSet::from_iter(iter_nums)
            })
            .collect();

        let winning_numbers = &lotto[0];
        let your_numbers = &lotto[1];

        let intersection: Vec<&i32> = winning_numbers.intersection(your_numbers).collect();

        if intersection.len() == 0 {
          return 0;
        }

        2_i32.pow((intersection.len() - 1) as u32)
      })
      .sum();

  println!("Res {}", res);
}


pub fn part_two() {
  let content = get_input("./src/day04/input.txt");

  let mut map: HashMap<i32, i32> = HashMap::new();

  content
      .iter()
      .for_each(|line| {

        // println!("LINE {}", line);
        let re = Regex::new(r"Card\s+(\d+):").unwrap();
        let (_, [card_idx]) = re.captures(line).unwrap().extract();

        let lotto: Vec<HashSet<i32>> = line.split(": ").collect::<Vec<&str>>()[1]
            .split(" | ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|numbers| {
              let re = Regex::new(r"\d+").unwrap();

              let iter_nums = re.find_iter(numbers)
                  .map(|n| n.as_str().parse::<i32>().unwrap());

              HashSet::from_iter(iter_nums)
            })
            .collect();

        let winning_numbers = &lotto[0];
        let your_numbers = &lotto[1];

        let intersection: Vec<&i32> = winning_numbers.intersection(your_numbers).collect();
        let intersection_len = intersection.len() as i32;
        let card_idx_as_num = card_idx.parse::<i32>().unwrap();

        if map.get(&card_idx_as_num).is_some() {
          let old_value = map.get(&card_idx_as_num).unwrap();
          map.insert(card_idx_as_num, old_value + 1);
        } else {
          map.insert(card_idx_as_num, 1);
        }

        let amount = map.get(&card_idx_as_num).unwrap();

        for _ in 0..*amount {
          for idx in card_idx_as_num + 1..=card_idx_as_num + intersection_len {

            if map.get(&idx).is_some() {
              let old_value = map.get(&idx).unwrap();
              map.insert(idx, old_value + 1);
            } else {
              map.insert(idx, 1);
            }
          }
        }
      });

  let res: i32 = map.iter().map(|(card_idx, count)| count).sum();

  println!("TRES {:?}", res);
}