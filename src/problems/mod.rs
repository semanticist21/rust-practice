pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    // reverse number.
    let mut rev = 0;
    let mut x_copy = x;

    while x_copy != 0 {
        rev = rev * 10 + x_copy % 10;
        x_copy /= 10;
    }

    if rev == x {
        true
    } else {
        false
    }
}

#[test]
fn test_palindrom() {
    let value = 123;
    let result = is_palindrome(value);
    println!("{}", result);

    let value = 313;
    let result = is_palindrome(value);
    println!("{}", result);
}

#[test]
fn chars_method() {
    let num = 123;
    let string = num.to_string();

    for digit in string.chars() {
        println!("{}", digit);
    }
}
