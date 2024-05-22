use std::io; //bring the input/output from the standard library

fn main() {
    println!("GUESS THE NUMBER!"); //the "!" means that it's a macro

    println!("Please input your guess:");

    let mut guess = String::new(); //declaring a variable. let declares. mut makes it mutable.
    //String::new() is a function that returns a new instance of a String. 
    //String is a type provided by the standard library that is a growable, UTF-8 encoded bit of text.
    //:: syntax in the ::new line indicates that new is an associated function of the String type.

    io::stdin() //another way of using this without the initial import is std::io::stdin, that returns an instance of std::io::Stdin
        .read_line(&mut guess) 
        .expect("Failed to read line"); 
        //calls the read_line method. & indicates that this argument is a reference.
        //gives you a way to let multiple parts of the code access data without needing to copy it into memory multiple times.
        //references are immutable by default. &mut makes it mutable.
        //could have been written with io::stdin().read_line(&mut guess).expect("Failed to read line");
        //expect is a method of the io::Result type. io::Result is an enum.
        //expect crashes the program and display a message if the Result is an Err value. Result's variants can be Ok or Err.
        //without the .expect, the program compiles but ggives you a warning.

    println!("You guessed: {guess}");

}
