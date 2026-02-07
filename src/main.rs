// in rust struct is custom data type group multiple values into a single group using key value pair.

// there are mainly 3 types of struct
// 1: Fields strcuct
// 2: Tuple struct
// 3: Unit-Like Struct

// 1. Field Struct
#[derive(Debug)] // this is to print full value using debug mode
struct User {
    name: String, // unknown sized values. so stored in heap so that it will be a owned type value
    age: u32, // known sized value so it stored in stack. and implement Copy trait, so that it will not be an owned type value.
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

struct Color(u16, u16, u16);
struct Point(u16, u16, u16);

fn main() {
    let user_1: User = User {
        active: true,
        age: 26,
        email: String::from("shaon@gmail.com"),
        name: String::from("shaon"),
        sign_in_count: 0,
        username: String::from("shaon07"),
    };

    println!("The name of the user is {}", user_1.name);
    println!("The username of the user is {}", user_1.username);
    println!("The email of the user is {}", user_1.email);
    println!("The age of the user is {}", user_1.age);
    println!("The status of the user is {}", user_1.active);
    println!("The sign in count of the user is {}", user_1.sign_in_count);

    let user_2: User = build_user(
        String::from("shaon ali"),
        String::from("shaonali@gmail.com"),
        String::from("shaonali07"),
        26
    );

    println!("User information of user_2 is {:#?}", user_2);

    // by default its also immuteable. so if we need to mutable, we can create a muteable value.
    let mut user_3: User = User {
        active: true,
        age: 26,
        email: String::from("shaon1@gmail.com"),
        name: String::from("shaon1"),
        sign_in_count: 0,
        username: String::from("shaon007"),
    };

    // now we can update it value
    user_3.sign_in_count = 10;

    println!("The informations of user 3 is {:#?}", user_3);

    // if we need to create a updated user based on a previous user data we can now do
    let user_4 = User {
        // write updated values
        sign_in_count: 15,
        active: false,
        username: String::from("shaon_new_008"),
        ..user_1
    };

    println!("The Update user_1 value is now {:#?}", user_4);

    // println!("user_1 owned value: {}", user_1.name); user_1 name is String type. so that when we copied it inside user_4
    // so that user_1 heap value is moved into user_4. so we cant use it anymore. but we can user the stack value like age,sign_in_count or active
    println!(
        "User_1 old sign_in_count is {} and new sign_in_count is {}",
        user_1.sign_in_count,
        user_4.sign_in_count
    );

    // 2. now create Tuple Struct
    let color: Color = Color(255, 0, 0);
    let point: Point = Point(123, 180, 100);

    print_color(color);
    print_pointer(point);
}

fn build_user(name: String, email: String, username: String, age: u32) -> User {
    let user: User = User {
        name, // short hand syntax, if the key and value name are same
        email,
        username: username,
        active: true,
        sign_in_count: 0,
        age,
    };

    return user;
}

fn print_pointer(points: Point) {
    println!("The cursor position is: X={}, Y={} and Z={}", points.0, points.1, points.2);
}

fn print_color(colors: Color) {
    println!("The RGB Code of the color is: R={}, G={} and B={}", colors.0, colors.1, colors.2);
}
