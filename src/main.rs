fn main() {
    use std::io;

    const MAX_MISSES: u32 = 6;

    fn main() {
        println!("Welcome to Hangman!");

        println!("Enter the secret word: ");

        let mut secret_word = String::new();
        io::stdin().read_line(&mut secret_word)
            .expect("Failed to read line");

        let secret_word = secret_word.trim();

        let mut missed_letters = Vec::new();
        let mut correct_letters = Vec::new();

        // Initialize the correct letters with underscores to represent the unknown letters
        for _ in 0..secret_word.len() {
            correct_letters.push('_');
        }

        println!("The word is: {:?}", correct_letters);

        while missed_letters.len() < MAX_MISSES as usize {
            println!("Missed letters: {:?}", missed_letters);
            println!("Misses remaining: {}", MAX_MISSES - missed_letters.len() as u32);

            println!("Please enter your guess: ");

            let mut guess = String::new();
            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

            let guess: char = match guess.trim().parse() {
                Ok(c) => c,
                Err(_) => {
                    println!("Please enter a valid letter");
                    continue;
                }
            };

            if secret_word.contains(guess) {
                println!("Correct!");
                for (i, c) in secret_word.chars().enumerate() {
                    if c == guess {
                        correct_letters[i] = c;
                    }
                }
            } else {
                println!("Incorrect!");
                missed_letters.push(guess);
            }

            println!("Current state: {:?}", correct_letters);

            if !correct_letters.contains(&'_') {
                println!("You win! The word was {}", secret_word);
                break;
            }
        }

        if missed_letters.len() == MAX_MISSES as usize {
            println!("You lose! The word was {}", secret_word);
        }
    }
}
