use std::io;


fn main() {

    println!("GUESS THE NUMBER BAKA!");
    println!("INPUT IT U DUMBASS");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("you guessed {}",guess);





}