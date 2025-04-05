fn main() {
    hello_world();
    tell_height(32);
    human_id("Efetobor", 56, 182.0);

    let _x: i32 ={
        let price = 5;
        let qty = 10;
        price * qty 
    };
    println!("Result is {}", _x);

    add(4, 6);
    println!("Value of y is {}", add(4, 6));


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

//functions return values
fn add(a: i32, b: i32) -> i32{
    a+b
}

//Expressions and Statements

//Expression: Anything that returns a value.
// Statement: Anything that does not return a value.

//Examples of Expression : 5, true & false, add(3,4), if condition 
// {value 1} else {value 2}

