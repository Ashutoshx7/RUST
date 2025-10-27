use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("guess the number");
    println!("please iuput your guess");

    let secret_number:i32=rand::thread_rng().gen_range(1..=100);
    
    loop{
        println!("genrated secreet num {}",secret_number);

    let  mut guess:String=String::new();
    io::stdin().read_line( &mut guess)
    .expect( "failed to read user input");
    
    println!("you guessed {}",guess);

    let guess_number:u32=guess.trim().parse().expect("you need to enter a number  guess you enter aa string");

    match guess_number.cmp(&(secret_number as u32)){
        Ordering::Equal=>println!("you are equal"),
        Ordering::Greater=> println!("you are great"),
        Ordering::Less=>println!("you less")
    }

}

}
