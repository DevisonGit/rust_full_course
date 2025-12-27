// compound data types
// arrays, tuples, slices and strings


fn main(){
    // arrays
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number array: {:?}", numbers);
    // let mix = [1,2, "apple", true];
    // println!("Mix array {:?}", mix);
    let fruits: [&str;3] = ["apple", "banana", "orange"];
    println!("Fruits {:?}", fruits);
    println!("1st element {}", fruits[0]);
    println!("2st element {}", fruits[1]);
    println!("3st element {}", fruits[2]);

    // tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple {:?}", human);
    let mix_tuple = ("Kratos", 23, true, [1,2,3,4]);
    println!("My mix tuple {:?}", mix_tuple);

    // slices
    let my_mix_slices:&[i32] = &[1,2,3,4,5];
    println!("My mix slices {:?}", my_mix_slices);
    let book_slices: & [&String] = & [&"HP".to_string(), &"IT".to_string()];
    println!("My book slices {:?}",  book_slices);
    let animals_slices :&[&str] = &["Lion", "Crocodille"];
    println!("My animals slices {:?}", animals_slices);

    // string
    let mut stone_cold: String = String::from("hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone cold says {}", stone_cold);

    // B- &str string slice
    let string : String = String::from("Hello, World");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}