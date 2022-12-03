use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn to_priority(letter: char) -> u32 {
  letter as u32 - (if letter.is_ascii_lowercase() { 96 } else { 38 })
}

fn intersection<T: Eq + Hash>(a: HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
  a.into_iter().filter(|e| b.contains(e)).collect()
}

fn get_priorities() -> std::io::Result<(u32, u32)> {
  let file = File::open("./inputs/day03-1.txt")?;
  let reader = io::BufReader::new(file);

  let mut priorities1: u32 = 0;
  let mut priorities2: u32 = 0;
  let mut sets = Vec::<HashSet<char>>::new();

  for line in reader.lines() {
    let l = line?;
    sets.push(HashSet::from_iter(l.chars()));

    let mut items = HashSet::new();
    items.extend(l[..l.chars().count() / 2].chars());

    for item in l[l.chars().count() / 2..].chars() {
      if items.contains(&item) {
        priorities1 += to_priority(item);
        break;
      }
    }
    if sets.len() == 3 {
      let inter = intersection(intersection(sets.remove(0), &sets[0]), &sets[1]);
      priorities2 += to_priority(*inter.iter().next().unwrap());
      sets.clear();
    }
  }
  Ok((priorities1, priorities2))
}

fn main() -> std::io::Result<()> {
  println!("Priorities {:?} ", get_priorities()?);
  Ok(())
}
