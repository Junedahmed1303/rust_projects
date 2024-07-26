use rand::{thread_rng,Rng};
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to this guess and win game!");
    let secret_number =thread_rng().gen_range(1..101);
    // println!("Secret number is : {}",secret_number);
    loop {
        println!("Please Enter a number you guessed!");
        let mut guessed_number=String::new();
        io::stdin().read_line(&mut guessed_number).expect("failed to read the line ");
        println!("your guessed number is : {}",guessed_number);
        let guessed_number:u32 =guessed_number.trim().parse().expect("Type an integer");
        match guessed_number.cmp(&secret_number){
            Ordering::Less =>println!("Number you entered is smaller than secret number"),
            Ordering::Greater =>println!("Number you entered is greater than secret number"),
            Ordering::Equal =>{println!("Number you entered is equivalent to secret number and YOU ARE THE WINNER!");
        break;
        },
    }
    }
}
