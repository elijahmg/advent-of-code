use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct Map {
  destination: i64,
  source: i64,
  range: i64,
}

fn collect_input(input: &&str) -> Vec<Map> {
  input.split("\n")
      .skip(1)
      .map(|line| {
        let [destination, source, range] = line
            .split_whitespace()
            .map(|char| char.parse::<i64>()
                .unwrap())
            .collect::<Vec<i64>>()[..] else { todo!() };

        Map {
          destination,
          source,
          range,
        }
      })
      .collect()
}

// source
fn get_destination(initial_source: &i64, all: &Vec<Vec<Map>>, start_idx: usize) -> i64 {
  if all.len() == start_idx {
    return *initial_source;
  }

  let maps = &all[start_idx];

  for map in maps {
    let Map { destination, source, range } = map;

    if initial_source >= source && initial_source < &(source + range) {
      let res = initial_source + (destination - source);

      return get_destination(&res, all, start_idx + 1);
    }
  }

  get_destination(initial_source, all, start_idx + 1)
}


pub fn part_one() {
  let file = fs::read_to_string("./src/day05/input.txt")
      .expect("Should have been able to read the file");

  let parts: Vec<&str> = file.split("\n\n").collect();

  let seeds: Vec<i64> = parts[0].split(": ")
      .skip(1)
      .collect::<Vec<&str>>()[0]
      .split_whitespace()
      .map(|char| char.parse::<i64>().unwrap())
      .collect();

  let all: Vec<Vec<Map>> = parts
      .iter()
      .skip(1)
      .map(collect_input)
      .collect();

  let min_location = seeds
      .iter()
      .map(|seed| get_destination(seed, &all, 0))
      .min()
      .unwrap();

  println!("Dest {:?}", min_location)
}


type Seed = (i64, i64);

pub fn part_two() {
  let file = fs::read_to_string("./src/day05/input.txt")
      .expect("Should have been able to read the file");

  let parts: Vec<&str> = file.split("\n\n").collect();

  let seeds: Vec<i64> = parts[0].split(": ")
      .skip(1)
      .collect::<Vec<&str>>()[0]
      .split_whitespace()
      .map(|char| char.parse::<i64>().unwrap())
      .collect();

  let all_seeds: Vec<Seed> = seeds
      .chunks(2)
      .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1 ))
      .sorted()
      .collect();

  println!("{:?}", all_seeds)
}



/// learning solution
use std::collections::BTreeSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct AlmanacMapEntry {
  dest_start: u64,
  source_start: u64,
  length: u64,
}

impl AlmanacMapEntry {
  fn convert(&self, source: u64) -> Option<u64> {
    if self.source_start <= source && source < (self.source_start + self.length) {
      Some(source - self.source_start + self.dest_start)
    } else {
      None
    }
  }
}

#[derive(Debug, PartialEq)]
struct AlmanacMap(Vec<AlmanacMapEntry>);

#[derive(Debug, PartialEq)]
struct ValueRange {
  start: u64,
  length: u64,
}

impl AlmanacMap {
  fn convert(&self, source: u64) -> u64 {
    self
        .0
        .iter()
        .map(|entry| entry.convert(source))
        .find_map(|e| e)
        .unwrap_or_else(|| source)
  }

  fn convert_range(&self, range: &ValueRange) -> Vec<ValueRange> {
    let mut slices = BTreeSet::new();
    let range_end = range.start + range.length;

    for entry in &self.0 {
      let source_end = entry.source_start + entry.length;

      if range_end < entry.source_start || range.start > source_end {
        continue;
      }

      if entry.source_start > range.start {
        slices.insert(entry.source_start);
      }

      if source_end < range_end {
        slices.insert(source_end);
      }
    }
    slices.insert(range_end);

    let mut output = Vec::new();
    let mut current = range.start;

    for position in slices {
      output.push(ValueRange {
        start: self.convert(current),
        length: position - current,
      });
      current = position;
    }

    output
  }
}

#[derive(Debug, PartialEq)]
struct Almanac {
  seeds: Vec<u64>,
  maps: Vec<AlmanacMap>,
}

impl Almanac {
  fn seed_to_location(&self, seed: u64) -> u64 {
    self.maps.iter().fold(seed, |value, map| map.convert(value))
  }

  fn seed_ranges(&self) -> impl Iterator<Item = ValueRange> + '_ {
    (0..self.seeds.len()).step_by(2).map(|ix| ValueRange {
      start: self.seeds[ix],
      length: self.seeds[ix + 1],
    })
  }
}

#[derive(Debug, PartialEq)]
struct ParseAlmanacError;

impl FromStr for Almanac {
  type Err = ParseAlmanacError;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let mut seeds = Vec::new();
    let mut maps = Vec::new();

    for (ix, section) in text.split("\n\n").enumerate() {
      if ix == 0 {
        if let Some(seeds_str) = section.strip_prefix("seeds: ") {
          for seed in seeds_str.split_whitespace().map(u64::from_str) {
            let seed = seed.map_err(|_| ParseAlmanacError)?;
            seeds.push(seed);
          }
        } else {
          return Err(ParseAlmanacError);
        }
      } else {
        let map: AlmanacMap = section.parse()?;
        maps.push(map);
      }
    }

    Ok(Self { seeds, maps })
  }
}

impl FromStr for AlmanacMap {
  type Err = ParseAlmanacError;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let mut entries = Vec::new();

    for (ix, line) in text.lines().enumerate() {
      if ix == 0 {
        continue;
      }

      let entry: AlmanacMapEntry = line.parse()?;
      entries.push(entry);
    }

    Ok(Self(entries))
  }
}

impl FromStr for AlmanacMapEntry {
  type Err = ParseAlmanacError;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let mut dest_start: Result<u64, Self::Err> = Err(ParseAlmanacError);
    let mut source_start: Result<u64, Self::Err> = Err(ParseAlmanacError);
    let mut length: Result<u64, Self::Err> = Err(ParseAlmanacError);

    for (ix, value) in text
        .split_whitespace()
        .map(|value| u64::from_str(value).map_err(|_| ParseAlmanacError))
        .enumerate()
    {
      match ix {
        0 => dest_start = value,
        1 => source_start = value,
        2 => length = value,
        _ => return Err(ParseAlmanacError),
      }
    }

    let dest_start = dest_start?;
    let source_start = source_start?;
    let length = length?;

    Ok(AlmanacMapEntry {
      dest_start,
      source_start,
      length,
    })
  }
}



pub fn part_two_learn() -> Option<u64> {
  let file = fs::read_to_string("./src/day05/input.txt")
      .expect("Should have been able to read the file");

  if let Ok(almanac) = file.parse::<Almanac>() {
    let mut current: Vec<ValueRange> = almanac.seed_ranges().collect();

    let mut future = Vec::new();

    for map in almanac.maps {
      println!("MAP======\n {:?}", map);

      for range in current {
        // println!("SEED======\n {:?}", range);
        // println!("converted range=======\n {:?}", map.convert_range(&range));

        future.extend(map.convert_range(&range));
      }

      // println!("extended seed====\n{:?}", future);
      current = future;
      future = Vec::new();
    }

    current.iter().map(|range| range.start).min()
  } else {
    None
  }
}