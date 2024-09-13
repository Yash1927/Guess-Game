use std::io; 
use rand::prelude::*; 


fn main() {
    loop{
        let guess_list = ["apple","banana","orange","grapes","pineapple"];
        let mut guessed = String::new(); 
        println!("Enter Guess :");
        let _ = io::stdin().read_line(&mut guessed); 
        let guess = guessed.trim().to_lowercase(); 

        //thread_rng generates random numbers
        let secret = thread_rng().gen_range(0..guess_list.len());
        let random = guess_list[secret];
        if guess == random{
            println!("CONGRATULATIONS YOU WON");

        } else if !guess_list.contains(&guess.as_str()){
            println!("Wrong Input");
        } else if guess != random{
            println!("Incorrect guess, the correct answer was {}", random);
        } else{
            println!("Something Went Wrong");
        }
    }

}
