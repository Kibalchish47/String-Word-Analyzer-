use regex::Regex;
use std::io;

fn main() {
    let mut input:String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input. PANIC! AT THE DISCO");
    
    let filter = Regex::new(r"fuck").unwrap();
        
    filter.find_iter(text)
}