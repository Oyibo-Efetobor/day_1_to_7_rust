// compound data types

fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);

    //str type
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);

    // ///////////////////////////////////////////
    //Tuples

    let tuple_1 = ("Alice", 30, false);
    println!("Human Tuple: {:?}", tuple_1);

    //mix tuple

    let my_mix_tuple = ("kratos", 23, true, [1,23,4,5]);
    println!("My mix tuple: {:?} ", my_mix_tuple);

    //slices: [1,2,3,4,5]
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    //string
    let animal_slices = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slices: {:?}", animal_slices);

    //string slices
    let book_slices = &[&"IT", &"Harry Potter", &"Generator Rex"];
    println!("Book Slice: {:?} ", book_slices);

    //add to string, Data is by default immutable in Rust
    //example

    let mut stone_cold = String::from("HELL, ");
    stone_cold.push_str("YEAH!");
    println!("Stone cold says: {}", stone_cold);

    //B- &str (string slice)
    let b_string = String::from("Hello, World!");
    let slice= &b_string[0..3];
    println!("Slice Value: {}", slice);
}
