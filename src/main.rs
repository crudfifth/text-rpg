use std::io::{self, Write};

#[derive(PartialEq)]
enum Language {
  Japanese,
  English,
}

fn main() {
  let language = select_language();
  println!("{}", get_message("welcome", &language));

  loop {
    print!(">");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let command = input.trim();

    if command == "exit" {
      println!("{}", get_message("goodbye", &language));
      break;
    } else if command == "hello" {
      println!("{}", get_message("hello", &language));
    } else {
      println!("{}", get_message("unknown_command", &language));
    }
  }
}

fn select_language() -> Language {
  loop {
    println!("Select Language / 言語を選択してください");
    println!("1: English");
    println!("2: Japanese");
    println!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    match choice {
      "1" => return Language::English,
      "2" => return Language::Japanese,
      _ => println!("Invalid choice / 無効な選択です。"),
    }
  }
}

fn get_message<'a>(key: &str, language: &Language) -> &'a str {
  match (key, language) {
    ("welcome", Language::English) => "Welcome to Text RPG!\nType 'exit' to quit.",
    ("welcome", Language::Japanese) => {
      "テキストRPGへようこそ！\n終了するには'exit'と入力してください"
    }
    ("goodbye", Language::English) => "Goodbye!",
    ("goodbye", Language::Japanese) => "お疲れ様でした！",
    ("hello", Language::English) => "Hello. adventurer!",
    ("hello", Language::Japanese) => "こんにちは、冒険者よ！",
    ("unknown_command", Language::English) => "Unknown command. Please try again.",
    ("unknown_command", Language::Japanese) => "未知のコマンドです。もう一度試してください。",
    _ => "Unknown key",
  }
}
