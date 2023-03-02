use std::{collections::HashMap, hash::Hash};

pub fn is_subset(arr: &[&str], arr2: &[&str]) -> bool {
    let ordered = get_ordered_arrs(&arr, &arr2);
    let large_arr = ordered[0];
    let small_arr = ordered[1];

    let mut count = 0;

    for large_item in large_arr {
        for small_item in small_arr {
            if small_item.eq(large_item) {
                count += 1;
            }
        }
    }

    if small_arr.len() == count {
        return true;
    }

    false
}

pub fn is_subset_arr_hash(arr: &[&str], arr2: &[&str]) -> bool {
    let ordered = get_ordered_arrs(&arr, &arr2);
    let large_arr = ordered[0];
    let small_arr = ordered[1];

    let mut map = HashMap::<&str, bool>::new();

    for item in large_arr {
        map.insert(item, true);
    }

    for item in small_arr {
        if let Some(_) = map.get(item) {
            continue;
        } else {
            return false;
        }
    }

    true
}

fn get_ordered_arrs<'a>(arr: &'a [&str], arr2: &'a [&str]) -> [&'a [&'a str]; 2] {
    if arr.len() > arr2.len() {
        return [arr, arr2];
    } else {
        return [arr2, arr];
    }
}

#[test]
fn test_check_subset() {
    let arr = _get_large_arr();
    let arr2 = _get_small_arr();

    let result = is_subset(&arr, &arr2);
    println!("{}", result);

    let arr = _get_large_arr();
    let arr2 = _get_small_arr_no_subset();

    let result = is_subset(&arr, &arr2);

    println!("{}", result);
}

#[test]
fn test_check_subset_hash() {
    let arr = _get_large_arr();
    let arr2 = _get_small_arr();

    let result = is_subset_arr_hash(&arr, &arr2);
    println!("{}", result);

    let arr = _get_large_arr();
    let arr2 = _get_small_arr_no_subset();

    let result = is_subset_arr_hash(&arr, &arr2);

    println!("{}", result);
}

pub fn get_intersection_hash<'a>(arr: &[&'a str], arr2: &[&'a str]) -> Box<Vec<&'a str>> {
    let mut result = Box::new(vec![]);
    let mut map = HashMap::<&str, bool>::new();

    for item in arr {
        map.insert(item, true);
    }

    for item in arr2 {
        if let Some(_) = map.get(item) {
            result.push(*item);
        }
    }

    result
}

#[test]
fn test_check_intersection_hash() {
    let arr = _get_large_arr();
    let arr2 = _get_small_arr();

    let result = get_intersection_hash(&arr, &arr2);
    println!("{:?}", result);

    let arr = _get_large_arr();
    let arr2 = _get_small_arr_no_subset();

    let result = get_intersection_hash(&arr, &arr2);
    println!("{:?}", result);
}

fn _get_large_arr() -> [&'static str; 9] {
    ["a", "b", "c", "d", "f", "e", "g", "h", "i"]
}

fn _get_small_arr() -> [&'static str; 3] {
    ["d", "f", "e"]
}

fn _get_small_arr_no_subset() -> [&'static str; 3] {
    ["d", "f", "z"]
}

pub fn get_unique_first_char(words: &str) -> char {
    let mut map = HashMap::<char, u8>::new();

    for char in words.chars() {
        if char == ' ' {
            continue;
        }
        if let Some(_) = map.get(&char) {
            let item = map.get_mut(&char).unwrap();
            *item = *item + 1;
        } else {
            map.insert(char, 1);
        }
    }

    for (_, (char, cnt)) in map.iter().enumerate() {
        if *cnt == 1 {
            return *char;
        }
    }

    return ' ';
}

#[test]
fn test_unique() {
    let str = "minimum";
    let result = get_unique_first_char(str);

    println!("{:?}", result);
}

pub fn test_get_ten_sum(nums: Vec<i32>) -> bool {
    let mut dic = HashMap::<i32, bool>::new();

    for num in nums {
        if dic.contains_key(&(10 - num)) {
            return true;
        }

        dic.insert(num, true);
    }

    false
}

#[test]
fn test_ten_sum() {
    let arr = vec![2, 8];
    let result = test_get_ten_sum(arr);
    assert_eq!(result, true);

    let arr = vec![2, 7];
    let result = test_get_ten_sum(arr);
    assert_eq!(result, false);
}

pub fn make_equity(arr1: &mut [i32], arr2: &mut [i32]) {
    let mut sum = 0;
    let mut sum2 = 0;

    let mut arr1_dic = HashMap::new();
    let mut arr2_dic = HashMap::new();

    arr1.iter().enumerate().for_each(|(i, v)| {
        sum += v;
        arr1_dic.insert(*v, i);
    });

    arr2.iter().enumerate().for_each(|(i, v)| {
        sum2 += v;
        arr2_dic.insert(*v, i);
    });

    if (sum + sum2) % 2 != 0 {
        return;
    }

    let median = (sum + sum2) / 2;
    if sum == median {
        return;
    }

    let shift_amount = sum - median;

    for i in 0..arr1.len() {
        let v = arr1[i];
        let find_item = v - shift_amount;

        println!("**");
        println!("{}", v);
        println!("{}", shift_amount);
        println!("{}", find_item);
        println!("**");

        if arr2_dic.contains_key(&find_item) {
            let arr2_idx = arr2_dic.get(&find_item).unwrap();

            arr1[i] = find_item;
            arr2[*arr2_idx] = v;

            return;
        }
    }
}

#[test]
fn test_equity() {
    let mut arr1 = [5, 3, 3, 7];
    let mut arr2 = [4, 1, 1, 6];

    make_equity(&mut arr1, &mut arr2);
}
