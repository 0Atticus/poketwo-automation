mod autocorrect;
use autocorrect::read_file;
use std::process::Command;
use std::env;



fn main() {
    let temp = Command::new("xclip")
    .args(["-selection", "c", "-o"])
    .output()
    .expect("nope");
    

    let clue = String::from_utf8_lossy(&temp.stdout);
    let mut best_word = "nope".to_owned();
    let mut possibilities: Vec::<String> = Vec::new();
    let alpha = "qwertyuiopasdfghjklzxcvbnm";
    
    for i in alpha.chars() {
        let new_clue = clue.replace('_', &i.to_string());
        possibilities.push(read_file(&new_clue).0);
        best_word = read_file(&new_clue).0;
    }

    println!("{:?}", possibilities);
    let mut best: (&str, i32) = ("", 0);
    for i in 0..(possibilities.len() as i32) {
        let count = possibilities.iter().filter(|&n| *n == possibilities[i as usize]).count();
        if possibilities[i as usize] != best.0 && count as i32 > best.1 && possibilities[i as usize] != "s" {
            best = (&possibilities[i as usize], count as i32);
        }
    }
    let notify = Command::new("zenity").args(["--notification", &format!("--text='{}'", best.0)]).spawn().expect("nope");
    let copy = Command::new("python3").args(["copy.py", &format!("{}", best.0)]).spawn().expect("nope");
}