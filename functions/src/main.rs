fn main() {
    hello_world();
    tell_height(32);
    human_id("Efetobor", 56, 182.0);
}

//Hoisting - can call function anywhere in your code
fn hello_world(){
    println!("Hello, Efetobor! 😉");
}

// you can insert input values
fn tell_height(height: i32){
    println!("My height is {}", height);
}

//you can insert more than one value
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, 
    and my height is {}cm. ", name, age, height);
}

//Expressions and Statements
