struct User{
    name:String,
    last_name:String,
    age:i32}
fn main(){
 let user=User{
    name: String::from("karnx"),
    last_name:String::from("Singh"),
    age:19,
    

 };
 println!("{}", user.name)
}