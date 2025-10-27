use std::io;


fn main(){
    println!("gues the  number");
    println!("please input your guess");
    let mut guess:String=String::new();
    i0::stdin().read_line( &mut guess).expect(msg:"please enter a number");
    println!("you guess {guess}x")
}