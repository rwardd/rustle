extern crate rand;

use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use rand::seq::IteratorRandom; 

struct Game {
    //lol
}

//implementation of helper methods
impl Game {
    /* Helper function to get randomly generated word */
    fn get_word() -> String {
        let path = "./util/answers.txt";
        let f = File::open(path).unwrap_or_else(|e| panic!("(;_;) file not found {}: {}", path, e));
        let f = BufReader::new(f);
        let lines = f.lines().map(|l| l.expect("Couldn't read line"));
        let answer_string = lines.choose(&mut rand::thread_rng()).expect("File had no lines");
        answer_string

    }
    /* Helper function to get user input */
    pub fn get_input() -> String {
        let mut guess = String::new();
        println!("Enter your guess: ");
        io::stdin().read_line(&mut guess).expect("failed to read line");
        guess.pop();
        guess
    }
    /* mainloop */
    pub fn mainloop() {
        let mut ansArray;
        let mut num_guess;
        loop {
            ansArray = [0, 0, 0, 0, 0];
            num_guess = 0;
            let word = Game::get_word();
            let mut guess = String::from("");
            while guess != word {
                //rust didnt like this statment ||'d with above line
                if num_guess > 5 {
                    break; //gross
                }
                ansArray = [0, 0, 0, 0, 0];
                print!("{}. ", num_guess);
                guess = Game::get_input();
                if guess.len() != 5 {
                    println!("Incorrect length, try again!");
                    continue;
                }
                let mut letter_count = 0;
                for (i, c) in guess.chars().enumerate() {
                    let guess_count = guess.matches(c).count();
                    let ans_count = word.matches(c).count();
                    if word.contains(c) {
                        if (word.matches(c).count() > 1) && (letter_count <= guess.matches(c).count()){
                            ansArray[i] = 1;
                            letter_count += 1;
                            println!("Letter Count: {}", letter_count);
                        }
                        if ans_count == guess_count {
                            ansArray[i] = 1;
                        }
                    }
                    if word.chars().nth(i).unwrap() == c {
                        ansArray[i] = 2;
                    }
                }
                //print iconic wordle emojis
                for j in ansArray {
                    if j == 0 {
                        print!("⬛️");
                    }
                    if j == 1 {
                        print!("🟨");
                    }
                    if j == 2 {
                        print!("🟩");
                    }
                }
                println!("");
                num_guess += 1;
            }
            //guess checking 
            if guess == word {
                println!("Correct!");
            } else {
                println!("Out of guesses, game over! The word was {}", word);
            }
        }
    }
}


//main fn
fn main() -> io::Result<()> {
    Game::mainloop();

    Ok(())
}
