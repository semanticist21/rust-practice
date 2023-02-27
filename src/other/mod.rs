pub fn get_largest_multiplied_in_arr(arr: &mut [i32]) -> i32 {
    if arr.len() < 3 {
        return 0;
    }

    if arr.len() == 3 {
        let mut result = 1;
        for item in arr {
            result *= *item;
        }
        return result;
    }

    arr.sort_unstable();
    let last_idx = arr.len() - 1;

    let result = arr[last_idx - 2] * arr[last_idx - 1] * arr[last_idx];

    result
}

#[test]
fn test_largest_mutiplied() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = get_largest_multiplied_in_arr(&mut arr);

    println!("{}", result);
}

pub fn get_empty_num(mut arr: Box<[u32]>) -> Vec<u32> {
    arr.sort_unstable();

    let mut marker: u32 = 0;
    let mut result = Vec::with_capacity(arr.len() / 3);

    for (idx, item) in arr.iter().enumerate() {
        if idx == 0 {
            marker = item.clone();
        } else {
            // if number is same with previous one.
            if *item == marker {
                marker += 1;
                continue;
            }

            marker += 1;

            if *item == marker {
                continue;
            }

            let mut item_copy = item.clone();

            while item_copy != marker {
                item_copy -= 1;
                result.push(item_copy);
            }

            marker = *item;
        }
    }

    result
}

#[test]
fn test_empty_num() {
    let box_item: Box<[u32]> = Box::new([1, 2, 3, 11, 5, 6, 7, 8, 8, 10]);

    let result = get_empty_num(box_item);
    println!("{:?}", result);
}

fn _find_largest_big_n(arr: Vec<i32>) -> i32 {
    if arr.len() == 0 {
        return -1;
    }

    let mut largest = 0;

    for (idx, item) in arr.iter().enumerate() {
        if idx == 0 {
            largest = *item;
        }

        if *item > largest {
            largest = *item;
        }
    }

    largest
}

fn _find_largest_big_n2(mut arr: Box<[i32]>) -> i32 {
    if arr.len() == 0 {
        return -1;
    }

    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }

    *arr.last().unwrap()
}

const _NUM: usize = 1000000;
const _LEN: usize = _NUM - 1;
const _TARGET_STRING: &str = "asdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjas
daksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasd
jasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjla
sdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlas
djasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj]
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjas
daksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasd
jasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjla
sdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlas
djasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj]
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjas
daksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasd
jasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjla
sdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlas
djasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj]
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjas
daksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasd
jasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjla
sdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlas
djasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj]
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjas
daksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasd
jasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjla
sdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlas
djasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj]
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjas
daksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasd
jasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjla
sdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlas
djasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaks
djlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj]
asdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdjasdaksdjlasdj";

#[test]
fn test_arr_speed() {
    const STR: String = String::new();
    let mut arr: [String; _NUM] = [STR; _NUM];

    for i in 0.._LEN {
        arr[i] = _TARGET_STRING.to_string();
    }

    for i in 0.._LEN {
        arr[i] = String::new();
    }
}

#[test]
fn test_vec_speed_with_capacity() {
    let mut vec = Vec::with_capacity(_NUM);

    for _ in 0..=_LEN {
        vec.push(String::from(_TARGET_STRING));
    }

    for _ in 0..=_LEN {
        vec.remove(vec.len() - 1);
    }
}

#[test]
fn test_vec_speed_() {
    let mut vec = vec![];

    for _ in 0..=_LEN {
        vec.push(String::from(_TARGET_STRING));
    }

    for _ in 0..=_LEN {
        vec.remove(vec.len() - 1);
    }
}

#[test]
fn test_hash_map_speed() {
    use std::collections::HashMap;
    let mut hash_map = HashMap::new();

    for i in 0..=_LEN {
        hash_map.insert(i, String::from(_TARGET_STRING));
    }

    for i in 0..=_LEN {
        hash_map.remove(&i);
    }
}

pub fn find_duplicate(arr: &[i32]) -> i32{
    let mut tortoise = arr[0];
    let mut hare = arr[1];

    loop{
        tortoise = arr[tortoise as usize];
        hare = arr[arr[hare as usize] as usize];

        if tortoise == hare{
            break;
        }
    }

    let mut ptr_1 = arr[0];
    let mut ptr_2 = tortoise;

    while ptr_1 != ptr_2{
        ptr_1 = arr[ptr_1 as usize];
        ptr_2 = arr[ptr_2 as usize];
    }

    ptr_1
}

#[test]
fn test_duplciate(){
    let arr = [1,2,3,4,5,5];

    let result = find_duplicate(&arr);
    println!("{}", result);
}
