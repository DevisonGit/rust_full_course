// ownership


fn main(){
    // 1 - Each value in Rust has variable that's its owner.
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("The size {} is {}", s1, len);
    
    // 2 - There can be only one owner at a time
    let s2 = s1;
    println!("{}", s2);

}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn print_lost(s: &String) -> usize{
    // 3 - When the owner goes out of scope, the value will be dropped
    println!("{}", &s2);
    s.len()
}