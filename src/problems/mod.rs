use std::default;

use crate::structure::other::greedy::is_anagram;

mod roman;
mod water_container;
mod longest;
pub mod sum;
mod diff;

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

pub fn roman_to_int(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();
    let mut result = 0;
    let mut is_pass = false;

    for i in 0..chars.len() {
        if is_pass {
            is_pass = false;
            continue;
        }

        let value = chars[i];
        let mut num = 0;

        match value {
            'I' => {
                if i == chars.len() - 1 {
                    num = 1;
                } else {
                    let next_value = chars[i + 1];
                    if next_value == 'V' {
                        is_pass = true;
                        num = 4;
                    } else if next_value == 'X' {
                        is_pass = true;
                        num = 9;
                    } else {
                        num = 1;
                    }
                }
            }
            'V' => num = 5,
            'X' => {
                if i == chars.len() - 1 {
                    num = 10;
                } else {
                    let next_value = chars[i + 1];
                    if next_value == 'L' {
                        num = 40;
                        is_pass = true;
                    } else if next_value == 'C' {
                        num = 90;
                        is_pass = true;
                    } else {
                        num = 10;
                    }
                }
            }
            'L' => num = 50,
            'C' => {
                if i == chars.len() - 1 {
                    num = 100;
                } else {
                    let next_value = chars[i + 1];
                    if next_value == 'D' {
                        num = 400;
                        is_pass = true;
                    } else if next_value == 'M' {
                        num = 900;
                        is_pass = true;
                    } else {
                        num = 100;
                    }
                }
            }
            'D' => num = 500,
            'M' => num = 1000,
            _ => (),
        };

        result += num;
    }
    result
}

pub fn roman_to_int_other(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();
    let mut nums = vec![];
    let mut result = 0;

    nums.push(get_num(*chars.first().unwrap()));

    for i in 1..chars.len() {
        let num = get_num(chars[i]);

        let last_number = nums.last_mut().unwrap();
        if num > *last_number {
            *last_number = num - *last_number;
        } else {
            nums.push(num)
        }
    }

    for num in nums {
        result += num;
    }

    result
}

#[inline]
fn get_num(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}
