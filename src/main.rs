/// write a function tthat finds the fibonacci  of a number its take as inpur
/// 
/// 
fn main(){
    println!("{}",fibonacci_no(4))
}
fn fibonacci_no(num:i32)->i32{
    let mut first =0;
    let mut second=1;
    if  num==0  {
        return first;

    }if  num==1 {
        return 1;
    }


    for _ in 0..(num -2){
        let temp=second;
        second =second +first ;
        first =temp;
    }
    return second;
}