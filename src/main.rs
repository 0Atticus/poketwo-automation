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
    if possibilities.iter().filter(|&best_word| *best_word == best_word.to_owned()).count() >= 10 {
        let notify = Command::new("zenity")
        .args(["--notification", &format!("--text='{}'", best_word)])
        .spawn()
        .expect("nope");
        let fuck_clipboards_in_rust = Command::new("python3")
        .args(["copy.py", &format!("{}", best_word)])
        .spawn()
        .expect("nonon");
    } else {
        let notify = Command::new("zenity")
        .args(["--notification", "--text='unsure'"])
        .spawn()
        .expect("nope");
    }
}