use std::collections::HashMap;

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
