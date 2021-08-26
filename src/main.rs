use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    play_guessing_game();
}

// generates a random number from start (inclusive) to end (inclusive)
fn get_secret_number(start: u32, end: u32) -> u32 {
    rand::thread_rng().gen_range(start..=end)
}

fn play_guessing_game() {
    println!("Guess the number!");
    let secret_number = get_secret_number(1, 100);

    loop {
        println!("type your guess here:");
        let mut player_guess = String::new();
        io::stdin()
            .read_line(&mut player_guess)
            .expect("Failed to read line");

        // go back to start of loop if the player guess is not a number else go on
        let player_guess: u32 = match player_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", player_guess);

        check_player_guess(&player_guess, &secret_number);

        if player_guess == secret_number {
            break;
        }
    }
}

fn check_player_guess(&player_guess: &u32, &secret_number: &u32) {
    match player_guess.cmp(&secret_number) {
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => {
            println!("Correct!");
        }
    }
}
