use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Nhập vào đê !");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_number}"); 
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("ddmm");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("thap hon"),
            Ordering::Greater => println!("lon hon"),
            Ordering::Equal => {
                println!("ya");
                break;
            }
        }
        println!("doan la {guess}");
        println!("ket qua la {} va {}",3+566,"asdkh");
    }        
}
