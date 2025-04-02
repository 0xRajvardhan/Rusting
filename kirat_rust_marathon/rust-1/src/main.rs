fn main() {
    println!("{}", is_even(20));
}

//i in i32 stands for signed which means the input can be +ve or -ve as well, 32 stands for 32 bits

fn is_even(num:i32)->bool{
    if num%2==0{
        return true;
    }
    return false;
}
