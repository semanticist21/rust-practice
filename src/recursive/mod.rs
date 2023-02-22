use std::collections::{HashMap, HashSet};

// can get upstairs at maximum 3
// getting the number of possible routes
pub fn get_stairs(num_of_stairs: u32) -> u32 {
    return match num_of_stairs {
        3 => 4,
        2 => 2,
        1 => 1,
        _ => {
            get_stairs(num_of_stairs - 1)
                + get_stairs(num_of_stairs - 2)
                + get_stairs(num_of_stairs - 3)
        }
    };
}

#[test]
fn test_stairs() {
    let num_of_stairs = 16;

    println!("{:?}", get_stairs(num_of_stairs));
}

pub fn get_anagram(word: String) -> Vec<String> {
    if word.len() == 1 {
        return vec![word];
    }

    //remove duplicate chars
    let word = word
        .chars()
        .into_iter()
        .collect::<HashSet<char>>()
        .into_iter()
        .collect::<String>();

    let last_char = word.chars().last().unwrap();
    let other_chars = word
        .chars()
        .into_iter()
        .filter(|c| *c != last_char)
        .collect::<String>();

    let mut result = vec![];

    let anagrams = get_anagram(other_chars.clone());

    for anagram in anagrams {
        let anagram_chars = anagram.chars().collect::<Vec<char>>();

        for i in 0..=anagram_chars.len() {
            let mut new_word = anagram_chars.clone();
            new_word.insert(i, last_char);
            result.push(new_word.into_iter().collect::<String>());
        }
    }

    result
}

#[test]
fn test_get_anagram() {
    let test_str = "abcdefghijj".to_string();

    let result = get_anagram(test_str);
    println!("{:?}", result.len());
}

pub fn get_cnt(mut words: Vec<&str>) -> usize {
    if words.len() == 0 {
        return 0;
    }

    let first_word_cnt = words.pop().unwrap().len();

    return first_word_cnt + get_cnt(words);
}

#[test]
fn test_get_cnt() {
    let result = get_cnt(vec!["asdas", "asdasd", "fff"]);

    println!("{:?}", result);
}

pub fn get_only_even_nums_vec(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![];
    }

    if nums.len() == 1 {
        if nums.last().unwrap() % 2 == 0 {
            return vec![nums.pop().unwrap()];
        } else {
            return vec![];
        }
    }

    let num = nums.pop().unwrap();

    if num % 2 != 0 {
        return get_only_even_nums_vec(nums);
    } else {
        let mut item = get_only_even_nums_vec(nums);
        item.push(num);
        return item;
    }
}

pub fn get_only_even_nums(mut nums: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if nums.len() == 0 {
        return vec![vec![]];
    }

    let items = nums.pop().unwrap();
    let mut new_vec = vec![];
    for item in items {
        if item % 2 == 0 {
            new_vec.push(item);
        }
    }

    let mut result = vec![];

    if nums.len() != 0 {
        result.extend(get_only_even_nums(nums));
    }
    result.push(new_vec);

    result
}

#[test]
fn test_even_nums() {
    let nums = vec![
        vec![1, 2, 3, 4, 5, 6],
        vec![1, 6, 2, 4, 1, 2, 3, 1, 3],
        vec![5, 8, 9, 0, 1, 2, 3],
    ];

    let result = get_only_even_nums(nums);
    println!("{:?}", result);

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let result = get_only_even_nums_vec(nums);
    println!("{:?}", result);
}

pub fn get_triangular_number(idx: u32) -> u32 {
    if idx == 1 {
        return 1;
    }

    idx + get_triangular_number(idx - 1)
}

#[test]
fn test_triangular_number() {
    let result = get_triangular_number(7);
    println!("{}", result);
}

pub fn get_first_x_idx(word: String, cnt: u32) -> u32 {
    if word.len() == 0 {
        return 0;
    }

    let first_word = word.chars().next().unwrap();
    let other = word
        .chars()
        .filter(|x| *x != first_word)
        .collect::<String>();

    if first_word != 'x' {
        get_first_x_idx(other, cnt + 1);
    }

    cnt
}

#[test]
fn test_get_first_x() {
    let str = "asdasdasda";

    let result = get_first_x_idx(str.to_string(), 0);
    println!("{:?}", result);
}

pub fn get_max(mut nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    if nums.len() == 1 {
        return nums.pop().unwrap();
    }

    let last_num = nums.pop().unwrap();
    let max = get_max(nums);

    if last_num > max {
        return last_num;
    } else {
        return max;
    }
}

pub fn get_fibonnachi(idx: u32, mem: &mut HashMap<u32, i64>) -> i64 {
    match idx {
        1 => 0,
        2 => 1,
        3 => 1,
        _ => {
            let find_result = mem.get(&idx);
            if let Some(result) = find_result {
                return *result;
            } else {
                let result = get_fibonnachi(idx - 1, mem) + get_fibonnachi(idx - 2, mem);
                mem.insert(idx, result);
                result
            }
        }
    }
}

pub fn get_fibonnachi_hash(idx: u32, mem: &mut HashMap<u32, i64>) -> i64 {
    if mem.len() < 3 {
        mem.insert(0, 0);
        mem.insert(1, 1);
        mem.insert(2, 1);
        mem.insert(3, 2);
    }

    if let None = mem.get(&idx) {
        let value = get_fibonnachi(idx - 1, mem) + get_fibonnachi(idx - 2, mem);
        mem.insert(idx, value);
    }

    *mem.get(&idx).unwrap()
}

#[test]
fn test_fibbonachi() {
    let result = get_fibonnachi(66, &mut HashMap::new());
    println!("{:?}", result);

    let result = get_fibonnachi_hash(66, &mut HashMap::new());
    println!("{:?}", result);
}
