fn main() {
    let my_variable: bool = false;
    if my_variable == true {
        println!("{}",true)
    }else{
        println!("{}",false)
    }
    let result = is_number_greater_than(1,29);
    println!("{}", result )
}


fn is_number_greater_than(x:i32, y:i32) -> bool{
    if x > y  {
        return true;
    }else{
        return false;
    }
}