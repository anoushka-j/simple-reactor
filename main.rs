extern crate rand;
use std::io;
use rand::Rng; 

fn main() {
    let emotions = ['😁', '😃', '😐', '😟', '😞', '😠'];
    let phrases = ["Green is a bad color", "Dogs are better pets than cats", "Die Hard is overrated", "The book is always better than the movie", "Bitcoin is not a good investment", "Bonds are death traps", "Value investing is far better than growth investing"];
    let phrases_len = phrases.len();
    println!("A phrase will be shown to you, and you will be asked to react.\n To exit, hit ENTER without entering anything.");


    loop {        
        let phrase_num = rand::thread_rng().gen_range(0, phrases_len-1);
        println!("*******{}*******", phrases[phrase_num]);
        println!("How do you want to react?\n1. elated? \n2. happy?\n3. indifferent\n4. worried?\n5. sad?\n6. angry?\nEnter the number: ");
        let mut emotion = String::new();
        io::stdin().read_line(&mut emotion)
            .expect("Failed to read line");
        let emotion : usize = match emotion.trim().parse() {
            Ok(emotion) => emotion,
            Err(_) => {println!("Plese enter a number from 1-6"); 
            break; 
        }
        }; 
        
        println!("You reacted as {}", emotions[emotion - 1]);

    }
}
