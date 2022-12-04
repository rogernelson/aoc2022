use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn get_contained_ranges() -> std::io::Result<(u32, u32)> {
  let file = File::open("./inputs/day04-1.txt")?;
  let reader = io::BufReader::new(file);

  let mut contained = 0;
  let mut overlap = 0;

  for line in reader.lines() {
    let ranges: Vec<Vec<i32>> = line?
      .split(',')
      .collect::<Vec<&str>>()
      .into_iter()
      .map(|r| {
        r.split('-')
          .collect::<Vec<&str>>()
          .into_iter()
          .map(|a| a.parse::<i32>().unwrap())
          .collect()
      })
      .collect();

    let minmax = vec![
      cmp::min(ranges[0][0], ranges[1][0]),
      cmp::max(ranges[0][1], ranges[1][1]),
    ];

    if minmax == ranges[0] || minmax == ranges[1] {
      contained += 1;
    };
    if minmax[1] - minmax[0] + 1 < ranges[0][1] - ranges[0][0] + ranges[1][1] - ranges[1][0] + 2 {
      overlap += 1;
    }
  }
  Ok((contained, overlap))
}

fn main() -> std::io::Result<()> {
  println!("Priorities {:?} ", get_contained_ranges()?);
  Ok(())
}
