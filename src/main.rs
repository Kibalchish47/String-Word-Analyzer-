use regex::Regex;
use std::io::{self, Write};

fn input() -> String { // helper function just for the input 
    let mut input:String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input. PANIC! AT THE DISCO");
    
    return input.trim().to_string()
}

fn main()
{
    println!("STRING ANALYZER : Does your sentence contain a given word ? ");
    println!("");
   loop
   {
        // the sentence input
        print!("Enter your sentence : ");
        io::stdout().flush().unwrap();
       
        let sentence = input();

        // the word input
        print!("Enter the word you are looking for : ");
        io::stdout().flush().unwrap();
        
        let word = input();

        // the filter filtering the given word
        let filter = Regex::new(&word).unwrap();

        // counting the occurences (ie the amount of times the word appears in the sentence)
        let occurences = filter.find_iter(&sentence).count();

        println!("Your word '{word}' has appeared {occurences} times in the sentence !");

        match &word{
            "fuck"   => println!("Bro ! Not the f-word ! Badmouthing gorilla !"),
            "bitch"  => println!("Bro ! What da female dog doin !"),
            "cunt"   => println!("Bruh that's something you'll never touch in your entire life..."),
            "bread"  => println!("Bread"),
            "sussy"  => println!("I see you use Reddit..."),
            "balls"  => println!("You are now officially G-A-Y !"),
            "penis"  => println!("Penis"),
            "dick"   => println!("Penis"),
            "amogus" => println!("Among us sussy balls."),
            _ => todo!(),
        }
        println!("");
    }
}