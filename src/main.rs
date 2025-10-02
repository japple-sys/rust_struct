// use std::mem;
use std::io;
use std::cmp::Ordering;

use rand::Rng;

// fn analyze_slice(slice: &[i32]) {
//     println!("First element of the slice: {}", slice[0]);
//     println!("Second element of the slice: {}", slice[1]);
// }

fn main() {
    // let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // let ys: [i32; 500] = [0; 500];
    // println!("First element of the array: {}", xs[0]);
    // println!("Second element of the arry: {}", xs[1]);

    // println!("Number of elements in array: {}", xs.len());

    // println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // println!("Brrow the whole array as a slice.");
    // analyze_slice(&xs);

    // println!("Borrow a section of the array as a slice.");
    // analyze_slice(&ys[1..4]);

    // let empty_array: [u32; 0] = [];
    // assert_eq!(&empty_array, &[]);
    // assert_eq!(&empty_array, &[][..]);

    // for i in 0..xs.len() + 1 {
    //     match xs.get(i) {
    //         Some(xval) => println!("{}: {}", i, xval),
    //         None => println!("Slow down! {} is too far!", i),
    //     }
    // }

    println!("Guess the number!");

    // let secret_number = rand::thread_rng().get_range(1..=100);
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
