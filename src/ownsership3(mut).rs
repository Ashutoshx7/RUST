fn main(){
    let mut s1:String=String ::from("hello");
    s2= take_ownership(some_string:s1);
    print!("{}",s2);
}

fn take_ownsership(some_string:String){
    print!("{}",some_string);
    return some_string;
}