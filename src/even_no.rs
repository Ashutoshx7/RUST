


fn main(){
    let ans=is_even(10192749127597129579871259712597);
    println!("{}",ans);
}


    fn is_even(num:i128)->bool{
        if num%2==0{
            return true;
        }
        return false;
    }
