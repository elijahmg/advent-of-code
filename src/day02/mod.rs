use regex::Regex;
use crate::utils::get_input;


pub fn part_one() {
  // condition 12 red, 13 green, and 14 blue?
  let content = get_input("./src/day02/part_1.txt");

  let sum: i32 = content
      .into_iter()
      .filter(|line| {
        let rest_reg = Regex::new(r"([0-9]+) (blue|red|green)").unwrap();

        let is_enough = rest_reg
            .captures_iter(&line)
            .map(|c| c.extract())
            .all(|(full, [count, color])| {
              match color {
                "red" => count.parse::<u8>().unwrap() <= 12,
                "green" => count.parse::<u8>().unwrap() <= 13,
                "blue" => count.parse::<u8>().unwrap() <= 14,
                _ => true
              }
            });

        is_enough
      })
      .map(|line| {
        let reg_id = Regex::new(r"Game ([0-9]+)").unwrap();
        let (_, [id]) = reg_id.captures(&line).unwrap().extract();

        id.parse::<i32>().unwrap()
      })
      .sum();

  println!("Sum {:?}", sum);
}

fn get_max(regex: Regex, line: &str) -> i32 {
  regex.captures_iter(&line)
      .map(|c| c.extract())
      .map(|(full, [count])| {
        count.parse::<i32>().unwrap()
      })
      .max()
      .unwrap()
}

pub fn part_two() {
  let content = get_input("./src/day02/part_1.txt");

  let sum: i32 = content
      .into_iter()
      .map(|line| {
        let red = Regex::new(r"([0-9]+) red").unwrap();
        let blue = Regex::new(r"([0-9]+) blue").unwrap();
        let green = Regex::new(r"([0-9]+) green").unwrap();

        let red_max = get_max(red, &line);
        let green_max = get_max(green, &line);
        let blue_max = get_max(blue, &line);

        red_max * green_max * blue_max
      })
      .sum();

  println!("{}", sum)
}