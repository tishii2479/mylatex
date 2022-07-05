use std::{env, fs};

struct Config {
    file_name: String,
}

fn parse_arguments(args: Vec<String>) -> Config {
    let file_name = if args.len() <= 1 {
        String::from("main.tex")
    } else {
        args[1].clone()
    };

    Config { 
        file_name 
    }
}

fn replace_punctuation(content: String) -> String {
    let new_content = content.replace("。", "．").replace("、", "，");
    println!("Finished to replace text");
    new_content
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = parse_arguments(args);

    println!("Modify file at: {}", config.file_name.as_str());

    let content = fs::read_to_string(config.file_name.as_str())
        .expect("Something went wrong reading the file");

    let new_content = replace_punctuation(content);

    fs::write(config.file_name.as_str(), new_content)
        .expect("Something went wrong writing the file");
}
