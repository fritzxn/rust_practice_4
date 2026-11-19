//-----------------------------------------------------
//how top write tests


/* fn main() {
    println!("Hello");
} */

//If our project is a binary crate that only contains a 
//src/main.rs file and doesn’t have a src/lib.rs file, 
//we can’t create integration tests in the tests directory and 
//bring functions defined in the src/main.rs file into scope with 
//a use statement. Only library crates expose functions that 
//other crates can use; binary crates are meant to be run on 
//their own.

//----------------------------------
//closures

fn main() {
    let x = 2;

    let doubled = |z| z == x;

    let y = 2;

    assert!(doubled(y)); 

}