/////////////////////////////////////////////////
///////// Day 2 - Variables and Data Types //////
/////////////////////////////////////////////////

use std::array;


//Data Types
fn main() {

    let repositary_name = "30-Days-Of-Rust";        //string type

    let rating_float = 4.2;                         //float type

    let is_growing_boolean = true;                  //boolean type

    let icon_char = '♥';                            // Unicode character type

    println!("Repository name is: {}", repositary_name);
    println!("Repository rating on 5 is {}", rating_float);
    println!("Repository is growing: {}", is_growing_boolean);
    println!("Repository icon is: {}", icon_char);

    // Call all other functions
    variable_and_mutability();
    mut_ex();
    data_types();
    data_types_();
    string_ex();
    compound_types();
    hands_on_challenge();
}


//Variables and Mutability

fn variable_and_mutability() {
    let x = 5; // Immutable variable
    let mut y = 10; // Mutable variable
    y = 15; // Now this is valid
}

fn mut_ex() {
    let mut mutable_variable = 10;
    println!("Before: {}", mutable_variable);
    mutable_variable += 5;
    println!("After: {}", mutable_variable);
}


//Data Types

fn data_types() {
    let integer = 25;              //integer
    let float = 2.0;               //floating-point
    let boolean = false;          //boolean
    let character = '😂';         //character
    let tuple = (10,9.0);   // tuple
    let array = [1,2,3];      // array
}

fn data_types_() {
    let integer = 34;
    let float = 3.14;
    let is_active = true;
    let char = '😂';
    let name = "Rusty";

    println!(" {} {} {} {} {}", integer, float, is_active, char, name);

}

fn string_ex() {
    // String::from() → Creates an owned, heap-allocated string (mutable if declared 'mut')
    // You can change it later because it's owned by 'my_string'
    let mut my_string = String::from("Hello, Rust!");

    // &str → A string slice, immutable reference to a string literal
    // Stored in read-only memory (static)
    let slice: &str = "Hello, Rust!";

    // You can modify 'my_string' since it's mutable
    my_string.push_str(" Learning Rust!"); 

    println!("Owned String: {}", my_string);
    println!("String Slice: {}", slice);

    my_string.push_str("Love Rust!");
}


fn compound_types() {
    let tuple = (42, 3.14, 'R');
    let array = [1,2,3];
}



/* 🎯 Hands-On Challenge
Create a Rust program that uses variables and demonstrates different data types. Your program should:

Declare and print a variable holding your name.
Create a mutable integer variable representing your current age and update it by adding one.
Use a floating-point variable to store your favorite number and print it.
Include a boolean variable that indicates whether you are learning Rust (set it to true).
Use a character variable to store the first letter of your name. */

fn hands_on_challenge() {
    let name = "Darkstar";
    
    let mut age = 24;
    age += 1;

    let favorite_number = 3.33;

    let is_learning_rust = true;

    let initial = 'P';

    println!("Name: {}", name);
    println!("Updated age: {}", age);
    println!("Favorite Number: {}", favorite_number);
    println!("Learning Rust: {}", is_learning_rust);
    println!("Initial: {}", initial);
}


// 💻 Exercises - Day 2

fn main() {
    let my_age = 25;
    let mut my_height = 160;
    my_height = 162;

    let mut my_name = "Darkstar";
    println!("My name is {}", my_name);

    let is_student = true;
    println!("is_student {}", is_student);

    let birth_year = 2025 - 25;
    println!("Birth year is {}", birth_year);

        let my_integer = 3;
    let my_float = 3.3;
    println!("The interger and float is {},{}", my_integer, my_float);

    let is_learning_rust = true;
    println!("Is learning rust?? {}", is_learning_rust);

    let favorite_letter = 'P';
    println!("My favorite letter is {}", favorite_letter);

    let my_scores = [99,99,91,91,70];
    println!("My marks in boards are {:?}", my_scores);

    let mut hobby = String::from("Coding");
    println!("{}", hobby);

    hobby.push_str(" and cooking");
    println!("I enjoy {}!", hobby);

}
