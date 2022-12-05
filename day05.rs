use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn parse_stacks() -> std::io::Result<Vec<Vec<char>>> {
  let file = File::open("./inputs/day05-1.txt")?;
  let reader = io::BufReader::new(file);

  let mut stacks = Vec::<Vec<char>>::new();
  for line in reader.lines() {
    let l = line?;
    for pos in (1..l.len()).step_by(4) {
      let cargo = l.chars().nth(pos).unwrap();
      if cargo != ' ' {
        let stack_num = (pos - 1) / 4;
        if stacks.len() <= stack_num {
          for _ in stacks.len()..=stack_num {
            stacks.push(Vec::<char>::new());
          }
        }
        stacks[stack_num].insert(0, cargo);
      }
    }
  }
  Ok(stacks)
}

fn main() -> std::io::Result<()> {
  let file = File::open("./inputs/day05-2.txt")?;
  let reader = io::BufReader::new(file);

  let mut stacks_part1 = parse_stacks()?;
  let mut stacks_part2 = stacks_part1.clone();

  let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  for line in reader.lines() {
    for cap in re.captures_iter(&line?) {
      let moves = &cap[1].parse::<usize>().unwrap();
      let from = &cap[2].parse::<usize>().unwrap();
      let to = &cap[3].parse::<usize>().unwrap();

      for _ in 0..*moves {
        match stacks_part1[*from - 1].pop() {
          Some(cargo) => stacks_part1[*to - 1].push(cargo),
          None => panic!("Moving from empty stack"),
        }
      }
      let start_offset = stacks_part2[*from - 1].len() - moves;
      let to_move = stacks_part2[*from - 1].split_off(start_offset);
      stacks_part2[*to - 1].extend(to_move);
    }
  }
  for mut stack in stacks_part1 {
    print!("{}", stack.pop().unwrap_or(' '));
  }
  println!();

  for mut stack in stacks_part2 {
    print!("{}", stack.pop().unwrap_or(' '));
  }
  println!();

  Ok(())
}
