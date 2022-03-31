use std::io::prelude::*;
use std::io;
use std::fs::File;


#[allow(unused)]
pub fn read_file(word: &str) -> (String, i32) {
  let mut candidates: Vec::<(String, i32)> = vec![];
  let mut word_score = 0;

  let file = File::open("src/names.txt").unwrap();

  let reader = io::BufReader::new(file);

  let word: String = word.chars().map(
    |x| match x {
      '.' => "".to_owned(),
      ' ' => "".to_owned(),
      _ => x.to_string()
    }).collect();

  for line in reader.lines() {

    let line = line.unwrap();

    if line.chars().count() == word.chars().count() {
      
      for (b, i) in word.chars().enumerate() {
        if i == line.chars().nth(b).unwrap() {
          word_score += 1
        }
      }

      candidates.push((line.clone(), word_score));

    } else {
      word_score = 0;
    }

  }



  let mut best_word = ("s".to_string(), 0);

  for i in candidates {
    if i.0 != best_word.0 && i.1 > best_word.1  {
      
      best_word = i;
    }
  }


  // println!("best: {:?}", best_word);
  return best_word;
}
