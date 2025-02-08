use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

// we comment this out so that the 'secret number' is actually secret
//    println!("The secret number is: {secret_number}"); 
    loop {
	println!("Please input your guess.");

	let mut guess = String::new();
	
	io::stdin()
	    .read_line(&mut guess)
            .expect("Failed to read line");
	
// this is somehow 'shadowing' the previous string variable named 'guess'
// with a new (identically named) variable that's the result of trimming
// the string-guess to get rid of newlines, and parse to turn the string into
// a different type (u32, I guess?). 'parse' also apparently returns a Result
// type which is why we use 'expect' in case the value parsed cannot
// be converted to the specified type 'u32'

//        let guess: u32 = guess.trim().parse().expect("Please type a number!");
// the line above would crash if a non-number (or something that couldn't be parsed to a
// u32 was entered.  With the code below, unparseable input is just ignored

// parse returns a 'Result' type, an enum w/ two values Ok and Err; Ok means it could parse
// the input to a number and it returns that number ('in' somehow) the Ok value.
// that then matches the pattern 'Ok(num)' and that num is returned inline.
// Failing to parse returns 'Err' and the _ is a wildcard like pattern that will match
// anything that might be inside Err

   let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	
        println!("You guessed: {}", guess);
	
// this is pattern matching, cmp compares two values and
// returns one of the variants of the 'Ordering' type brought in
// by 'use std::cmp::Ordering;' above

	 match guess.cmp(&secret_number) {
   	 Ordering::Less => println!("Too smal!"),
	 Ordering::Greater => println!("Too big!"),
	 Ordering::Equal => {
	     println!("You win!");
	     break;  // when the guess is correct, also break the loop above
	  }

       }	 
    }
}		 
