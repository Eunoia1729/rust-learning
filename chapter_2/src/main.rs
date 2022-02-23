use std::io;
use rand::Rng;
use std::cmp::Ordering;

const NUM_ATTEMPTS: i32 = 5;

fn get_random_number() -> u32 {
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    return random_number;
}
fn main() {
    
    let random_number = get_random_number();
    println!("Random number: {}", random_number);
    
    for _attempt in 0..NUM_ATTEMPTS {
        let mut input_num = String::new();
        io::stdin().read_line(&mut input_num).expect("Error while reading input num");
        let input_num : u32 = match input_num.trim().parse() {
            Ok(num) => num,
            Err(error_msg) =>  {
                println!("Error: {}",error_msg);
                continue
            },
        };
        println!("Guess: {}", input_num);

    
        match input_num.cmp(&random_number) {
            Ordering::Less => println!("Your guess is lower"),
            Ordering::Greater => println!("Your guess is higher"),
            Ordering::Equal => {
                println!("Correct Answer !!");
                break;
            },
        }
    }
    
    
}
