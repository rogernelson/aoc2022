use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
  println!("Max calories {}", get_max_calories(1)?);
  println!("Top Three calories {}", get_max_calories(3)?);
  Ok(())
}

fn get_max_calories(num_elves: usize) -> std::io::Result<u32> {
  let mut heap = BinaryHeap::new();
  let file = File::open("./inputs/day01-1.txt")?;
  let reader = io::BufReader::new(file);

  let mut sum: u32 = 0;
  for line in reader.lines() {
    match line?.parse::<u32>() {
      Ok(calories) => sum += calories,
      Err(_) => {
        heap.push(Reverse(sum));
        if heap.len() > num_elves {
          heap.pop();
        }
        sum = 0;
      }
    }
  }

  let mut max_sum: u32 = 0;
  while heap.len() > 0 {
    if let Some(Reverse(c)) = heap.pop() {
      max_sum += c;
    }
  }
  Ok(max_sum)
}
