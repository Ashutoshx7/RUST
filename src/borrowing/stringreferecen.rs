fn main(){
    let my_string:String=String:: from("hello")
    borrow_variable(some:&mystring);
    let s2 :&String =&my_string;
    let s2 :&String =&my_string;
    let s2 :&String =&my_string;
    let s2 :&String =&my_string;
    print!("{}",my_string);

}
fn borrow_variable(some:String){
    print!("{}",some)
}