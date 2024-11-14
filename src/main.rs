use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("correct: {correct}");
    println!("Hey, guess a number 1 - 10:");

    // error handling for invalid guess
    loop {
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
        match guess.cmp(&correct) {
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Less => println!("You guessed too low"),
            Ordering::Equal => {
                println!("You guessed the correct number");
                break;
            }
        };
    }

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
