mod utils;
fn main() {
    println!("{}", utils::is_even(20));
    println!("{}", utils::fibonacci(4));
    let name = String::from("rajvardhAN");
    let len = utils::get_str_len(name);
    println!("the length of the string is {}", len);
}
