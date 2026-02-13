
// Diego Zuniga

fn check_guess(guess: i32, secret: i32) -> i32 {    // function check guess
    if guess == secret {                        // if the guess matches the secret number, returns 0
        0
    }
    else if guess < secret {                    // if the guess is smaller, returns -1
        -1
    }
    else{                                       // if the guess is bigger, returns 1
        1
    }
}

fn main() {

    let secret: i32 = 7;                        // variable that stores the secret number
    let mut guess_count: i32 = 0;               // count variable that stores the amount of times there was a guess
    let guesses: [i32; 5] = [30, 42, 5, 12, 7]; // array that simulates the guesses input
    let mut index: usize = 0;                   // index to traverse on the array
    
    loop {                                      // loop that checks the conditions 
        // simulate user input
        let guess: i32 = guesses[index];
        guess_count += 1;

        let result = check_guess(guess, secret);    // calls the function check_guess

        if result == 0 {                            // depending on the result the function returns, a message is printed
            println!("{} is correct!", guess);
            break;
        } else if result == 1 {
            println!("{} is too high.", guess);
        } else {
            println!("{} is too low.", guess);
        }

        index += 1;                                 // index is updated
    }

    println!("It took {} guesses.", guess_count);   // amount of guesses is printed out
}
