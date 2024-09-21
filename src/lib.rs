use rand::rngs::ThreadRng;
use rand::Rng;
use std::io::{self, Write};

fn read_user_input() -> String {
  // read user input here
  let mut user_input = String::new();
  io::stdin()
    .read_line(&mut user_input)
    .expect("Error reading user input");
  user_input.trim().to_string()
}

fn game_round(rng: &mut ThreadRng) {
  println!("\n=== Guess the number! ===");

  // ## Generate a secret number between 1 and 100
  // let random: u8 = (rand::random::<u8>() % 100) + 1;
  let random: u8 = rng.gen_range(1..=100);

  // println!("Random generated number is: {random}");

  loop {
    // ## User input a guess
    print!("Your guess: ");
    io::stdout().flush().unwrap();
    let guess: u8 = read_user_input()
      .parse()
      .expect("Your guess must be a number between 1 and 100");

    if guess == random {
      break;
    } else if guess < random {
      println!("Your guess is too low.");
    } else if guess > random {
      println!("Your guess is too high.");
    }
  }

  println!("Bravo! You guessed it!");
}

pub fn play_game() {
  let mut rng = rand::thread_rng();
  'game: loop {
    game_round(&mut rng);

    loop {
      print!("\nPlay again [y/n]? ");
      io::stdout().flush().unwrap();
      let again = read_user_input();

      match again.as_str() {
        "n" | "N" => break 'game,
        "y" | "Y" => break,
        _ => continue,
      }
    }
  }
}
