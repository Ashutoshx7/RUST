
fn get_string_length_chars(s:&str)-> usize{
    s.chars().count();

}

fn main(){
   let my_string=String::from("hello world");
   let length= get_string_length_chars(&my_string);

   print!("the lenth  of the string {}",len);
}



