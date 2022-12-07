use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn is_done(map: &HashMap<char, usize>, num_chars: usize) -> bool {
  map.len() == num_chars && map.values().sum::<usize>() == num_chars
}

fn parse_codes(num_chars: usize) -> std::io::Result<usize> {
  let file = File::open("./inputs/day06-1.txt")?;
  let reader = io::BufReader::new(file);

  let mut total: usize = 0;

  for line in reader.lines() {
    let l = line?;
    let mut map = HashMap::new();

    for (i, c) in l.chars().enumerate() {
      if is_done(&map, num_chars) {
        total += i;
        break;
      } else if i >= num_chars {
        let key = l.chars().nth(i - num_chars).unwrap();
        match map.get_mut(&key) {
          Some(value) => {
            *value -= 1;
            if *value == 0 {
              map.remove(&key);
            }
          }
          None => panic!("Missing value"),
        }
      }
      match map.get_mut(&c) {
        Some(num_times) => *num_times += 1,
        None => {
          map.insert(c, 1);
          ()
        }
      };
    }
  }
  Ok(total)
}

fn main() -> std::io::Result<()> {
  println!("Characters processed {}", parse_codes(4)?);
  println!("Characters processed {}", parse_codes(14)?);
  Ok(())
}
