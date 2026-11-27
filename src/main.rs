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

/* use std::thread;

//----------------------------------
//closures
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("the user with preference {:?} gets {:?}",
        user_pref1, giveaway1);

    let user_preference2 = None;
    let giveaway2 = store.giveaway(user_preference2);
    println!("user with preference {:?} gets {:?}",
        user_preference2, giveaway2);


        let list = vec![1,2,3,4,5];
        println!(" this is {list:?}");

        let only_borrows = || println!("{list:?}");
        println!("before calling {list:?}");

        only_borrows();

    let mut list = vec![1,2,3,4,];
    println!("this is {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("borrows here {list:?}");

    let list = vec![1,2,3];
    println!("nefore deifinig closure {list:?}");

    thread::spawn(move || println!("" from thread {list:?}))
    .join()
    .unwrap();

//still leaning closures in detail from youtube

impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where 
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}











}
 */
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width:3, height: 5},
        Rectangle {width: 7, height: 12},
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}

//still just working on a basic calculator