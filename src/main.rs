use std::{path::PathBuf, io::{BufReader, BufRead, self, Write}, fs::File};

use rand::Rng;

fn readline(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim_end().to_string()
}

fn read_file(path:PathBuf) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let content: Vec<String> = BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    content
}

fn random(options:Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    options[rng.gen_range(0..options.len())].clone()
}

fn main() {
    let templates = read_file(PathBuf::from("templates.txt"));
    let characters = read_file(PathBuf::from("characters.txt"));
    let settings = read_file(PathBuf::from("settings.txt"));
    let genres = read_file(PathBuf::from("genres.txt"));
    let goals = read_file(PathBuf::from("goals.txt"));

    loop {
        let template = random(templates.clone());
        let fill_in_template: Vec<String> = template.split(' ').map(String::from).collect();
        let mut final_template: Vec<String> = Vec::new();

        for word in fill_in_template {
            match word.as_str() {
                "[character]" => final_template.push(random(characters.clone())),
                "[setting]" => final_template.push(random(settings.clone())),
                "[genre]" => final_template.push(random(genres.clone())),
                "[goal]" => final_template.push(random(goals.clone())),
                _ => final_template.push(word)
            }
        }

        println!("{}", final_template.join(" "));
        readline("Enter to continue");
    }
}
