///Day 3 - Control Flow

// if Statement
fn main() {
    let number = 10;
    if number > 5 {
        println!("The number is greater than 5");
    }

    // call the other functions
    if_else_elseif();
    else_and_elseif();
    loop_statement();
    while_loop();
    for_loop();
    match_match();
}

fn if_else_elseif() {
    let marks = 85;

    if marks >= 90 {
        println!("Grade: A");
    } else if marks >= 75 {
        println!("Grade: B");
    } else if marks >= 60 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }
}


// else and else if Statements

fn else_and_elseif() {
    let age = 18;
    if age >= 21 {
        println!("You can Drink alcohol");
    } else if age >= 18 {
        println!("You are an adult, but cannot drink alcohol");
    } else {
        println!("You are a minor.");
    }
}



// loop Statement

fn loop_statement() {

    let mut count = 0;

    loop {
        count += 1;
        if count == 10 {
            println!("Breaking the loop at count: {}", count);
            break;
        }
    }

}



// while Loop 

fn while_loop() {

    let mut num = 1;

    while num <= 5 {
        println!("Loop count: {}", num);
        num += 1;
    }
}




// for Loop

fn for_loop() {    

    for num in 1..4 {
        println!("Num: {}", num);
    }
}




// Control Flow with match

fn match_match() {
    let traffic_light = "green";

    match traffic_light {
        "green" => println!("Go"),
        "yellow" => println!("Slow down"),
        "red" => println!("Stop"),
        _  => println!("Invalid color"),        
    }
}



// ////////////  Hands-On Challenge ////////////

fn main() {

    // Uses an if statement to check if the number is even or odd.
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();
 
    if num % 2 == 0 {
        println!("The number is even: {}", num)
    } else {
        println!("The number is odd: {}", num)
    }

    // Use a loop to print numbers from 1 to 5

    for i in 1..=5 {
        println!("The number is: {}", i);
    }


    //Implement a match statement to respond to different days of the week, 
    //e.g., "Monday" => "Start of the week!", "Friday" => "Weekend is coming!", etc.

    let day = "Monday";


    match day {
    "Monday" => println!("Start of the week!"),
    "Tuesday" => println!("2nd day of work"),
    "Wednesday" => println!("3rd day"),
    "Thursday" => println!("4th day"),
    "Friday" => println!("Weekend is coming!"),
    _ => println!("Unknown day"),
    }

}



use std::io;

fn main() {

    //Write a program that checks if a number is even or odd using the if statement
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();
 

    if num % 2 == 0 {
        println!("The number is even: {}", num)
    } else {
        println!("The number is odd: {}", num)
    }

    //Create a while loop that prints numbers from 1 to 10
    let mut i = 1;

    while i <= 10 {
        println!("Number: {}", i);
        i += 1;
    }


    // Use the for loop to iterate over an array of your favorite colors and print each one
    let favorite_colors = ["Pink", "Blue", "Black"];

    for color in favorite_colors {
        println!("Favorite color: {}", color);
    }





    // 2) Operation from user: +, -, *, /
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();
    let problem = op.trim();  // "Addition", "Subtraction", etc.

    // 3) Second number
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).unwrap();
    let num2: i32 = input2.trim().parse().unwrap();
        
    match problem {
        "Addition" => println!("Addition: {}", num + num2),
        "Subtraction" => println!("Subtraction: {}", num - num2),
        "MultiPlication" => println!("Multiplcation: {}", num * num2),
        "Division" => println!("Division: {}", num / num2),
        _ => println!("Unknown operation"),
    }

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let text = input.trim();

        if text == "exit" {
            break;
        }

        println!("You typed:{}", text);
    }

}



// // bash comment after run

// 10
// Addition
// 20
// hello
// rust
// exit






//// ✅ Exercise: Level 2

use rand::Rng;
use std::io;
// add rand = "0.8" 


fn main() {

    //Create a program that calculates the factorial of a given number using a while loop
    let num = 5;
    let mut fact = 1;
    let mut i = 1;


    while i <= num {
        fact = fact * i;
        i += 1;
    }
    
    println!("Factorial: {}", fact);

    //Write a program that simulates a countdown timer using a loop and breaks when the countdown reaches zero
    let mut countdown = 60;

    loop {
        if countdown == 0 {
            break;
        }

        println!("{}", countdown);
        countdown -= 1;
    }


    //Use the for loop to calculate the sum of even numbers from 1 to 50

    let mut sum = 0;
    for i in 0..=50 {
        if i % 2 == 0 {
        sum = sum + i;
    }
    }



    //Write a program that reads a string input and uses the match statement to respond with different outputs based on the input 
    //(e.g., "hello" => "Hi there!", "bye" => "Goodbye!", etc.).

    let greet = "hello";

    match greet {
        "hello" =>println!("Hi there!"),
        "bye" => println!("Goodbye!"),
        _ => println!("Unknown input"),
    }


    //Implement a program that uses if statements inside a for loop to print all the odd numbers from 1 to 20

    for i in 1..=20 {
        if i % 2 == 1 {
            println!("{}", i);
        }
    }


    //Create a small game where the program generates a random number between 1 and 10, and the user has to guess it. 
    //Use a loop to keep asking until the user gets it right

    let secret = rand::thread_rng().gen_range(1..=10);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let guess: i32 = input.trim().parse().unwrap();

        if guess == secret {
            println!("Correct!");
            break;
        } else {
            println!("Try again!");
        }
    }

}