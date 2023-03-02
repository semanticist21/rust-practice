use std::{collections::HashMap, hash::Hash};

fn get_largest_sum(arr: &[i32]) -> i32 {
    let mut largest = 0;

    for item in arr {
        let mut largest_copy = largest.clone();
        largest_copy += item;

        if largest_copy < 0 {
            largest_copy = 0;
        }

        if largest_copy > largest {
            largest = largest_copy;
        }
    }

    largest
}

#[test]
fn test_largest() {
    let arr: [i32; 10] = [1, -2, 3, -4, 5, -6, 7, -8, 9, 10];

    println!("{:?}", get_largest_sum(&arr));
}

type Price = f64;

pub fn is_in_trend(trends: Vec<Price>) -> bool {
    let mut lowest_price = *trends.first().unwrap();
    let mut middle_price = f64::MAX;

    for trend in trends {
        if trend <= lowest_price {
            lowest_price = trend;
            continue;
        }

        if trend <= middle_price {
            middle_price = trend;
            continue;
        }

        return true;
    }

    false
}

#[test]
fn test_trend() {
    let trends: Vec<Price> = vec![22., 25., 21., 18., 19.6, 17., 16., 20.5];
    let flag = is_in_trend(trends);
    println!("{:?}", flag);

    let trends: Vec<Price> = vec![22., 25., 21., 18., 19.6, 17., 16., 11.5];
    let flag = is_in_trend(trends);
    println!("{:?}", flag);
}

pub fn is_anagram_deprecated(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    let mut dic = HashMap::<char, u8>::new();

    for c in str1.chars() {
        let a = dic[&c];
        if dic.contains_key(&c) {
            let count = dic.get(&c).unwrap();
            dic.insert(c, count + 1);
            continue;
        }

        dic.insert(c, 1);
    }

    for c in str2.chars() {
        if dic.contains_key(&c) {
            let count = dic.get(&c).unwrap();
            if *count == 0 {
                return false;
            }

            dic.insert(c, count - 1);

            continue;
        }

        return false;
    }

    true
}

pub fn is_anagram(str1: &str, str2: &str) -> bool {
    if iter_make_map(str1) == iter_make_map(str2) {
        return true;
    }

    false
}

pub fn iter_make_map(str: &str) -> HashMap<char, u8> {
    let mut dic = HashMap::new();

    for c in str.chars() {
        if dic.contains_key(&c) {
            dic.insert(c, dic[&c] + 1);
            continue;
        }

        dic.insert(c, 1);
    }

    dic
}

#[test]
fn test_anagram() {
    let str1 = "rattels";
    let str2 = "startle";

    let flag = is_anagram(str1, str2);
    println!("{:?}", flag);

    let str1 = "abcdefg";
    let str2 = "eeeeeee";

    let flag = is_anagram(str1, str2);
    println!("{:?}", flag);
}

fn find_empty_num_deprecated(arr: &[u32]) -> u32 {
    let mut marker = 0;

    for num in arr {
        if *num == marker + 1 {
            marker = *num;
            continue;
        }

        return *num;
    }

    0
}

fn find_empty_num(arr: &[u32]) -> u32 {
    let mut dic = HashMap::<u32, bool>::new();

    for item in arr {
        dic.insert(*item, true);
    }

    for n in 0..arr.len() {
        let n_u32 = n as u32;
        let find = dic.get(&n_u32);

        if let None = find {
            return n_u32;
        }
    }

    0
}
#[test]
fn test_empty() {
    let arr: [u32; 8] = [0, 1, 2, 3, 4, 6, 7, 8];
    let result = find_empty_num(&arr);
    println!("{:?}", result);
}

fn find_best(prices: Vec<i32>) -> i32 {
    let mut lowest = prices[0];
    let mut profit = 0;

    for price in prices {
        if price <= lowest {
            lowest = price;
            continue;
        }

        if (price - lowest) > profit {
            profit = price - lowest;
        }
    }

    profit
}

#[test]
fn test_best() {
    let vec = vec![10, 7, 5, 8, 11, 2, 6];
    let result = find_best(vec);
    println!("{}", result);
}

pub fn get_highest_mutiply(vec: Vec<i32>) -> i32 {
    let mut largest = 0;
    let mut second_largest = 0;

    let mut lowest = 0;
    let mut second_lowest = 0;

    for item in vec {
        if item > largest {
            largest = item;
            continue;
        }

        if item > second_largest {
            second_largest = item;
            continue;
        }

        if item < lowest {
            lowest = item;
            continue;
        }

        if item < second_lowest {
            second_lowest = item;
            continue;
        }
    }

    let mutiplied = largest * second_largest;
    let minus_multiplied = lowest * second_lowest;

    if mutiplied > minus_multiplied {
        return mutiplied;
    } else {
        return minus_multiplied;
    }
}

#[test]
fn test_highest() {
    let arr = vec![5, -10, -6, 9, 4];
    let result =get_highest_mutiply(arr);
    println!("{}", result);
}
