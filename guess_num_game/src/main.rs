use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's Guess A Num!");

    let random_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please Enter A Num:");
        let mut guess_num = String::new();
        io::stdin()
            .read_line(&mut guess_num)
            .expect("Failed to read line");
        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess_num.cmp(&random_num) {
            Ordering::Less => println!("Less!"),
            Ordering::Greater => println!("Big!"),
            Ordering::Equal => {
                println!("Bingo!You win.");
                break;
            }
        }
    }
}
