use std::io;
use rand::prelude::*;

fn main() {
    let guessing_fruits_list:[&str;6]=["banana","apple","grapes","guava","lichi","mango"];
    let mut rng = thread_rng();
    let index =rng.gen_range(0..guessing_fruits_list.len());
    let secret_fruit= guessing_fruits_list[index];
    // println!("secret_fruit :{}",secret_fruit);
    println!("Welcome to fruit guess game");
    println!("Please enter your guessed fruit");
    
    loop{
        let mut guessed_fruit= String::new();
        match io::stdin().read_line(&mut guessed_fruit){
        Ok(_)=>{
            let fruit_selected = guessed_fruit.trim().to_lowercase();
            // println!("{}",fruit_selected);
            if !guessing_fruits_list.contains(&fruit_selected.as_str()){
                println!("Your entered fruit does'nt found");
                continue;
            }
             else if
                guess_checker(&fruit_selected,secret_fruit){
                println!("You are winner!");
                break;
            }
            else {
                println!("Retry")
            }
        }
        Err(error)=>{println!("Error :{}",error)

        }
    }
}
}

fn guess_checker(fruit_selected:&str,secret_fruit:&str)->bool{
    return fruit_selected==secret_fruit;
}