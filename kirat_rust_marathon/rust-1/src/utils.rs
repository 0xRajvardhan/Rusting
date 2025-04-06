//i in i32 stands for signed which means the input can be +ve or -ve as well, 32 stands for 32 bits

pub fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

pub fn fibonacci(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }
    for _i in 0..num - 1 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}
