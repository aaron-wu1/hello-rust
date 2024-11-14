use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut how_many = String::new();
    println!("How many random numbers do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");

    let num_guesses: u8 = how_many.trim().parse().expect("Error reading input");

    let mut correct = Vec::new();

    for _ in 0..num_guesses {
        correct.push(rand::thread_rng().gen_range(1..=10));
    }

    println!("{correct:?}");

    // let correct = rand::thread_rng().gen_range(1..=10);
    // println!("correct: {correct}");

    let mut guesses_made = 0;
    println!("Hey, guess a number 1 - 10:");
    // add while loop
    while guesses_made < num_guesses {
        let mut guess = String::new();
        // read line into string buf num
        io::stdin().read_line(&mut guess);

        // guess check if valid int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again");
                continue;
            }
        };

        // USING match
        // removed mut added uint32 cmp
        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Less => println!("You guessed too low"),
            Ordering::Equal => {
                println!("You guessed the correct number");
                guesses_made += 1;
                if guesses_made < num_guesses {
                    println!("Let's now try the next number!")
                }
            }
        };
    }
    println!("Thanks for playing! The correct numbers were:");
    for item in &correct {
        println!("{item}")
    }

    // // error handling for invalid guess
    // loop {
    //     let mut guess = String::new();
    //     // read line into string buf num
    //     io::stdin().read_line(&mut guess);

    //     // guess check if valid int
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(e) => {
    //             println!("Error with parse, try again");
    //             continue;
    //         }
    //     };

    //     // USING match
    //     // removed mut added uint32 cmp
    //     match guess.cmp(&correct) {
    //         Ordering::Greater => println!("You guessed too high"),
    //         Ordering::Less => println!("You guessed too low"),
    //         Ordering::Equal => {
    //             println!("You guessed the correct number");
    //             break;
    //         }
    //     };
    // }

    // // add loop
    // loop {
    //     let mut guess = String::new();
    //     // read line into string buf num
    //     io::stdin().read_line(&mut guess);

    //     // possible to create the same name variable in the same scope
    //     // with different types
    //     let guess: u32 = guess.trim().parse().expect("Error with parse.");

    //     // USING match
    //     // removed mut added uint32 cmp
    //     match guess.cmp(&correct) {
    //         Ordering::Greater => println!("You guessed too high"),
    //         Ordering::Less => println!("You guessed too low"),
    //         Ordering::Equal => {
    //             println!("You guessed the correct number");
    //             break;
    //         }
    //     };
    // }

    // let mut message = if correct == guess {
    //     String::from("You guessed the correct number")
    // } else if correct < guess {
    //     String::from("You guessed too high")
    // } else {
    //     String::from("You guessed too low")
    // };
    // println!("{message}");

    // if correct == guess {
    //     println!("You guessed the correct number")
    // } else if correct < guess {
    //     println!("You guessed too high: {}", guess);
    // } else {
    //     println!("You guessed too low: {}", guess);
    // }

    // if correct == guess {
    //     println!("You guessed the correct number")
    // } else {
    //     println!("You guessed the wrong num: {}", guess);
    // }

    // // USER INPUT STRING 2
    // println!("Hey, guess a number:");
    // let mut num = String::new();

    // // read line into string buf num
    // io::stdin().read_line(&mut num);

    // println!("You guessed: {}", num.trim());

    // // USER INPUT STRING
    // println!("Hey, what's your name?");
    // let mut name = String::new();

    // // end program if it's an issue with expect
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Error reading input");

    // // get rid of enter key with .trim
    // println!("Hello {}!", name.trim());

    //  //Dynamic String
    //   let first = String::from("Caleb");
    //   let last = "last";
    //   println!("");
}
