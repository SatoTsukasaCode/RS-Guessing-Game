use std::io;
use rand::seq::SliceRandom;

fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let number = list.choose(&mut rand::thread_rng());
    let mut guess = String::new();

    //debugging & testing start
    println!("{:?}", number.expect("can't convert").to_string());
    //debugging & testing end

    println!("guess a number: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("can't read line");

   if guess == number.expect("can't convert").to_string() {
        println!("Guess is correct! it was {guess}");
   } else {
        println!("guess was not right, it was {:?}", number.expect("can't convert").to_string());
   }

}
