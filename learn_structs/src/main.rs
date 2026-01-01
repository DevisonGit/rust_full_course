fn main() {

    // struct
    struct Book{
        title: String,
        author: String,
        pages: u32,
        avalible: bool,
    }

    struct User{
        name: String,
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let new_user: User = User{
        active: true,
        name: String::from("Ciri"),
        username: String::from("zirael"),
        email: String::from("zira@gmail.com"),
        sign_in_count: 120,
    };

    println!("User name: {}", new_user.name);

    // return a struct from a function
    fn build_user(email: String, name: String, username: String) -> User{
        User{
            active: true,
            sign_in_count: 1,
            email,
            name, 
            username,
        }
    }

    // create instances from other instances
    let new_user = User{
        email: String::from("gerado@gmail.com"),
        username: String::from("geraldo"),
        name: String::from("Geraldo"),
        ..new_user
    };
    println!("User name: {}", new_user.name);
    println!("User active {}", new_user.active);

    // tuple struct 
    struct Color(i32, i32, i32);
    struct Point(i64, i64, i64);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // Unit-like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;

}
