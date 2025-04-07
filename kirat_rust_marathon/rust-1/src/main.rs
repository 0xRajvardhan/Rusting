mod utils;

struct User {
    first_name: String,
    last_name: String,
    age: u32,
}

fn main() {
    println!("{}", utils::is_even(20));
    println!("{}", utils::fibonacci(4));
    let name = String::from("rajvardhAN");
    let len = utils::get_str_len(name);
    println!("the length of the string is {}", len);

    let user = User {
        first_name: String::from("Rajverdhan"),
        last_name: String::from("Singh"),
        age: 82,
    };
    println!("{}, {}, {}", user.first_name, user.last_name, user.age)
}
