// primitive data type
// int, float, bool, char

// Integer
// signed (+ and -) unsigned (only +) 
// integer type of different sizes
// i8, i32, i64, i128: Signed integers
// u8, u32, u64, u128: Unsigned integrs

fn main(){
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y); 
// diff bet i32 (32 bits) and i64 (64 bits)
// range:
// i32 : -2147483648..2147483647
// i64 : -9223372036854775808..9223372036854775807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

}
