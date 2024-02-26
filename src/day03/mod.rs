use std::vec::Vec;
use crate::utils::get_input;
use struct_iterable::Iterable;

pub fn part_one() {
  let content = get_input("./src/day03/part_1.txt");


  let mut matrix: Vec<Vec<char>> = content
      .iter()
      .map(|line| line.chars().collect())
      .collect();

  let placeholders: Vec<char> = String::from_iter(vec!["."; matrix[0].len()]).chars().collect();

  matrix.push(placeholders.clone());
  matrix.insert(0, placeholders.clone());

  let mut sum = 0;

  for (idx_line, line) in matrix.iter().enumerate() {
    let mut number: Vec<&char> = vec![];
    let mut neighbours: Vec<char> = vec![];

    let mut last_digit_idx = 0;

    for (idx_char, char) in line.iter().enumerate() {
      if char.is_digit(10) {
        last_digit_idx = idx_char;

        number.push(char);

        let neighbour_idx = get_idx_to_check(idx_line as i32, idx_char as i32, line.len() as i32);


        for (neig_line_idx, neig_char_idx) in neighbour_idx {
          let el = matrix[neig_line_idx][neig_char_idx];

          if !el.is_numeric() && el != '.' {
            neighbours.push(el)
          }
        }

        if idx_char == line.len() - 1 {
          if neighbours.len() > 0 {
            let as_str = String::from_iter(number.clone()).parse::<i32>().unwrap();
            sum = sum + as_str;
          }
        }
      } else {
        if idx_char > 0 && idx_char - 1 == last_digit_idx {
          if neighbours.len() > 0 {
            let as_str = String::from_iter(number.clone()).parse::<i32>().unwrap();
            sum = sum + as_str;
          }

          number = vec![];
          neighbours = vec![];
          last_digit_idx = 0;
        }
      }
    }
  }

  println!("SUM {}", sum);
}

fn get_idx_to_check(line_idx: i32, el_idx: i32, line_len: i32) -> Vec<(usize, usize)> {
  let top_left = (line_idx - 1, el_idx - 1);
  let top = (line_idx - 1, el_idx);
  let top_right = (line_idx - 1, el_idx + 1);
  let left = (line_idx, el_idx - 1);
  let bottom_left = (line_idx + 1, el_idx - 1);
  let bottom = (line_idx + 1, el_idx);
  let bottom_right = (line_idx + 1, el_idx + 1);
  let right = (line_idx, el_idx + 1);

  vec![top_left, top, top_right, right, bottom_right, bottom, bottom_left, left]
      .into_iter()
      .filter(|(_, el_dx)| el_dx > &-1 && el_dx < &line_len)
      .map(|(line, el)| (line as isize as usize, el as isize as usize))
      .collect()
}

#[derive(Debug, Iterable)]
struct Numbers {
  // 0
  top_left: Option<(char, usize, usize)>,
  // 1
  top: Option<(char, usize, usize)>,
  // 2
  top_right: Option<(char, usize, usize)>,
  // 3
  right: Option<(char, usize, usize)>,
  //4
  bottom_right: Option<(char, usize, usize)>,
  // 5
  bottom: Option<(char, usize, usize)>,
  //5
  bottom_left: Option<(char, usize, usize)>,
  // 7
  left: Option<(char, usize, usize)>, // 7
}

impl Numbers {
  fn add_number(&mut self, neig_idx: usize, chr: (char, usize, usize)) {
    match neig_idx {
      0 => self.top_left = Some(chr),
      1 => self.top = Some(chr),
      2 => self.top_right = Some(chr),
      3 => self.right = Some(chr),
      4 => self.bottom_right = Some(chr),
      5 => self.bottom = Some(chr),
      6 => self.bottom_left = Some(chr),
      7 => self.left = Some(chr),
      _ => panic!()
    }
  }

  fn is_valid_gear(&self) -> bool {
    if self.top_left.is_some() && (self.top_right.is_some() || self.bottom_left.is_some() || self.bottom_right.is_some()) {
      return true;
    }

    if self.top_right.is_some() && (self.top_left.is_some() || self.bottom_left.is_some() || self.bottom_right.is_some()) {
      return true;
    }

    if self.bottom_left.is_some() && (self.top_right.is_some() || self.top_right.is_some() || self.bottom_right.is_some()) {
      return true;
    }

    if self.bottom_right.is_some() && (self.top_right.is_some() || self.top_left.is_some() || self.bottom_left.is_some()) {
      return true;
    }

    self.left.is_some() && self.right.is_some()
  }

  fn get_valid_points(&self) -> Vec<(char, usize, usize)> {
    vec![self.right, self.left, self.top_left, self.top_right, self.bottom_left, self.bottom_right]
        .iter()
        .filter(|val| val.is_some())
        .map(|va| va.unwrap())
        .collect()
  }
}


pub fn part_two() {
  let content = get_input("./src/day03/part_2.txt");

  let mut matrix: Vec<Vec<char>> = content
      .iter()
      .map(|line| line.chars().collect())
      .collect();

  let placeholders: Vec<char> = String::from_iter(vec!["."; matrix[0].len()]).chars().collect();

  matrix.push(placeholders.clone());
  matrix.insert(0, placeholders.clone());

  let mut sum: i32 = 0;

  for (idx_line, line) in matrix.iter().enumerate() {
    for (idx_char, char) in line.iter().enumerate() {
      if char == &'*' {
        let mut nums = String::new();

        let mut prev_idx: usize = 0;

        let mut stop_collection = false;

        for sub_line_idx in idx_line - 1..=idx_line + 1 {
          stop_collection = false;

          let mut line_num = String::new();

          for sub_char_idx in idx_char - 3..=idx_char + 3 {
            let el = matrix[sub_line_idx][sub_char_idx];

            if sub_char_idx > 0 && sub_char_idx - 1 == idx_char && el == '.' {
              println!("el {}, nums {}, {}", sub_char_idx, idx_char, el);
              // line_num.clear();
              stop_collection = true;
            }

            if sub_char_idx + 1 == idx_char && el == '.' {
              line_num.clear();
              // stop_collection = true;
            }

            if el.is_numeric() && !stop_collection {
              line_num.push(el)
            }
          }
          nums.push('.');
          nums = nums + line_num.as_str();
        }

        println!("EBANA {}", nums);

        // let n: i32 = nums.split(".").filter_map(|c| c.parse::<i32>().ok()).product();
        //
        // sum = sum + n;
      }
    }
  }
  println!("NHA {}", sum)
}


//spizdil ;(
use grid::Grid;
use itertools::Itertools;
use regex::{Regex};

#[derive(Clone, PartialEq)]
struct Point {
  x: usize,
  y: usize,
}

#[derive(Clone)]
struct Part {
  location: Point,
  part_type: char,
}

fn create_grid(input: &str) -> Grid<char> {
  // Create a grid and fill it with our input
  let mut grid: Grid<char> = Grid::new(0, 0);
  for l in input.lines() {
    grid.push_row(l.chars().collect_vec());
  }
  grid
}

fn parse_part(location: (usize, usize), part_type: &char) -> Option<Part> {
  match part_type {
    // Filter out any dots or numbers, keeping all the parts
    '0'..='9' | '.' => None,
    _ => Some(Part {
      part_type: *part_type,
      location: Point {
        x: location.0,
        y: location.1,
      },
    }),
  }
}

fn find_adjacent_points(point: &Point) -> Vec<Point> {
  // For the part, calculate all it's adjacent coordinates
  let mut adjacent_points: Vec<Point> = vec![];

  // Loop around the point generating a vec
  // There are no parts on the edge of the schematic so we do not worry about over/underflowing
  for x in (point.x - 1)..=point.x + 1 {
    for y in (point.y - 1)..=(point.y + 1) {
      adjacent_points.push(Point { x, y });
    }
  }

  adjacent_points
}

fn discover_numbers(part: &Part, grid: &Grid<char>) -> Vec<u32> {
  // For the part, calculate all it's adjacent coordinates
  let adjacent_points = find_adjacent_points(&part.location);

  // Regex matcher for numbers
  let re = Regex::new(r"\d+").unwrap();
  let mut matches: Vec<u32> = vec![];

  for x in (part.location.x - 1)..=part.location.x + 1 {
    // build string from the grid
    let row = grid.iter_row(x).collect::<String>();

    // Iterate through the matches and attach them to the part
    for m in re.find_iter(&row) {
      let match_range = m.start()..m.end();

      for y in match_range {
        if adjacent_points.contains(&Point { x, y }) {
          //Parse the match and push the result into the part
          matches.push(m.as_str().parse::<u32>().unwrap());
          // Move onto the next regex match if a gear is touching
          break;
        }
      }
    }
  }

  println!("FUNC exit, {:?}", matches);

  matches
}

fn get_parts_list(grid: Grid<char>) -> Vec<(Part, Vec<u32>)> {
  grid.indexed_iter()
      .filter_map(|(location, part_type)| parse_part(location, part_type))
      .map(|part| {
        // Discover the matches for the part
        let matches = discover_numbers(&part, &grid);
        (part, matches)
      })
      .collect_vec()
}

fn part_a(parts: Vec<(Part, Vec<u32>)>) -> u32 {
  parts
      .iter()
      .map(|(_, matches)| matches.iter().sum::<u32>())
      .sum::<u32>()
}

fn part_b(parts: Vec<(Part, Vec<u32>)>) -> u32 {
  parts
      .iter()
      .filter_map(|(part, matches)| match part.part_type {
        // Find the gears
        '*' => {
          // A gear must have two numbers
          if matches.len() == 2 {
            Some(matches.iter().product::<u32>())
          } else {
            None
          }
        }
        _ => None,
      }) //
      .sum::<u32>()
}

pub fn solve() {
  // Build out the grid and match all the numbers to the parts
  let grid = create_grid(include_str!("part_2.txt"));
  let parts: Vec<(Part, Vec<u32>)> = get_parts_list(grid);

  let answer = part_b(parts.clone());

  println!("{}", answer);
}