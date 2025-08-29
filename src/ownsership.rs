fn main(){
    let my_string :String=String:: from("hello");
    let my_string_3= String:take_ownership(some_string:my_string);
    pritn!{"{}",my_string_3};
}


fn take_ownership(some_string:String)->String{
    print!{"{}",some_string};
    return some_string;

}