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

    let mut str_dic = HashMap::<&str, bool>::new();

    for item in large_arr {
        str_dic.insert(item, true);
    }

    for item in small_arr {
        if let Some(_) = str_dic.get(item) {
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
    let arr = ["a", "b", "c", "d", "f", "e", "g", "h", "i"];
    let arr2 = ["d", "f", "e"];

    let result = is_subset(&arr, &arr2);
    println!("{}", result);

    let arr = ["a", "b", "c", "d", "f", "e", "g", "h", "i"];
    let arr2 = ["d", "f", "z"];

    let result = is_subset(&arr, &arr2);

    println!("{}", result);
}

#[test]
fn test_check_subset_hash() {
    let arr = ["a", "b", "c", "d", "f", "e", "g", "h", "i"];
    let arr2 = ["d", "f", "e"];

    let result = is_subset_arr_hash(&arr, &arr2);
    println!("{}", result);

    let arr = ["a", "b", "c", "d", "f", "e", "g", "h", "i"];
    let arr2 = ["d", "f", "z"];

    let result = is_subset_arr_hash(&arr, &arr2);

    println!("{}", result);
}
