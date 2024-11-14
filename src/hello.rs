use ferris_says::say;
use std::io::{stdout, BufWriter};

#[derive(Debug)]
struct Structure(i32);

fn main() {
    // called "binding" in Rust
    let name = "Brother";
    println!("Hello, {name}!");

    // mutable
    let mut mutable_name = "Brother";
    mutable_name = "Bro";
    println!("Hello, {mutable_name}!");

    // multiple vars
    let first = "Brother";
    let last = "MAN";
    // errors out
    // println!("Hello, {first} {last.to_lowercase()}!");

    // any function calls to data type needs to be put to the outside
    println!("Hello, {first} {}!", last.to_lowercase());

    let data = [1, 2, 3, 4, 5];
    // formating with :? -> debug format
    println!("{data:?}");

    // println!("Hello, world!");
    let stdout = stdout();
    let message = String::from("Hello Fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    // comments
    /*
     *Block
     */

    // printing line
    // every arge filled
    println!("{} {} {} days", 31, 30, 29);
    println!("{1}{0} days", 31, 30);
    // print library types
    println!("{:?} days", 31);
    println!("Now {:?} will print!", Structure(3));
}
