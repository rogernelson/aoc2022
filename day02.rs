use std::fs::File;
use std::io::{self, BufRead};

fn calculate_score(elf: char, me: char) -> u32 {
  (match me as i32 - elf as i32 - 23 {
    -1 | 2 => 0, // loss
    0 => 3,      // draw
    1 | -2 => 6, // win
    _ => panic!("Unknown scenario me: {}, elf: {}", me, elf),
  }) + me as u32
    - 87
}

fn main() -> std::io::Result<()> {
  let file = File::open("./inputs/day02-1.txt")?;
  let reader = io::BufReader::new(file);

  let mut score1: u32 = 0;
  let mut score2: u32 = 0;

  for line in reader.lines() {
    let l = line?;
    let elf = l.chars().nth(0).unwrap();
    let me1 = l.chars().nth(2).unwrap();
    let me2 = match (me1, elf) {
      ('X', 'A') => 'Z',
      ('X', 'B' | 'C') => char::from_u32(elf as u32 + 22).unwrap(),
      ('Y', 'A' | 'B' | 'C') => char::from_u32(elf as u32 + 23).unwrap(),
      ('Z', 'C') => 'X',
      ('Z', 'A' | 'B') => char::from_u32(elf as u32 + 24).unwrap(),
      (_, _) => panic!("Unknown scenario {}", l),
    };

    score1 += calculate_score(elf, me1);
    score2 += calculate_score(elf, me2);
  }
  println!("My score part1 {}", score1);
  println!("My score part2 {}", score2);

  Ok(())
}
