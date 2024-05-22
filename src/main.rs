use rand::Rng; //bring the random trait from the rand crate
use std::cmp::Ordering;
use std::io; //bring the input/output from the standard library //bring the Ordering type from the standard library

fn main() {
    println!("GUESS THE NUMBER!"); //the "!" means that it's a macro

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        //declaring a variable. let declares. mut makes it mutable.
        //String::new() is a function that returns a new instance of a String.
        //String is a type provided by the standard library that is a growable, UTF-8 encoded bit of text.
        //:: syntax in the ::new line indicates that new is an associated function of the String type.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //another way of using this without the initial import is std::io::stdin, that returns an instance of std::io::Stdin
        //calls the read_line method. & indicates that this argument is a reference.
        //gives you a way to let multiple parts of the code access data without needing to copy it into memory multiple times.
        //references are immutable by default. &mut makes it mutable.
        //could have been written with io::stdin().read_line(&mut guess).expect("Failed to read line");
        //expect is a method of the io::Result type. io::Result is an enum.
        //expect crashes the program and display a message if the Result is an Err value. Result's variants can be Ok or Err.
        //without the .expect, the program compiles but ggives you a warning.

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        //trim eliminates the whitespace at the beginning and end of the string. eliminates \n\r.
        //parse parses a string into a number.

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
