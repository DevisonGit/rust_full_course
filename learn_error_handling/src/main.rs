// enum Option<T>{
//     Some(T), 
//     None,
// }

// enum Result<T, E>{
//     Ok(T),
//     Err(E),
// }

fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64,String>{
    if denominator == 0.0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}


fn main() {
    let result = divide_option(10.0, 0.00);
    match result{
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero!"),
    }

    match divide_result(10.0, 2.0){
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
    

}
