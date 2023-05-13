use std::io;
use rand::Rng;
use std::cmp::Ordering;
// this ordering is the enum that is the result of 2 thing being compared

fn main() {
    println!("Guess the Number ");
    // let use to declare and by default the var is unmutable to make it mutable use mut
    let secret: i32 = rand::thread_rng().gen_range(1, 101);

    println!("The secret no. {}",secret);

    //new is function and its an associative function other language called it as static method 
    let mut guess: String = String::new();

    loop {
        println!("Enter your guess : ");

        //use karne se pehle clear karlo kyuki parse vo newline character nahi samjh paega
        guess.clear();
    
    // enter the input in the specified buffer
    // it means it takes the referrence of the string and modify it without taking the ownership of the string
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read");

        

        let guess: i32 = guess.trim().parse().expect("Please Type in a number");

        println!("Your guess {}",guess);

        match guess.cmp(&secret){
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }



}
