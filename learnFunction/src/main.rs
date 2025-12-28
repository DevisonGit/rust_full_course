fn main() {
    hello_world();
    tell_height(165);
    human("devison", 37, 165.5);
    // expressions can be in code blocks
    let _x: i32 = {
        let  price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("Rusult is: {}", _x);
    let y = add(69, 69);
    println!("Value of y is {}", y);
    println!("Value from function 'add' is {}", add(69, 69));

    let weight = 70.0;
    let height = 1.82;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi)
}


fn hello_world(){
    println!("Hello world from the call function");
}

// you can insert input values
fn tell_height(height: u32){
    println!("My height is {} cm", height);
}

// you can insert more than one value
fn human(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}

// expressions and statements
// expressions: Anything that returns a value.
// statement: Anything that does not return a value. 

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn calculate_bmi(weight: f64, height: f64) -> f64{
    weight / (height * height)
}