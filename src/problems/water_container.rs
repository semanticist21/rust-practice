pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut max_size = 0;

    while left < right {
        let mut smaller = height[left];
        if height[right] < smaller {
            smaller = height[right];
        }

        let size = (right - left) as i32 * smaller;
        if max_size < size {
            max_size = size;
        }

        if height[right] < height[left] {
            right = right - 1;
        } else {
            left = left + 1;
        }
    }

    max_size
}
#[test]
fn test_max_area() {
    let question = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];

    let answer = max_area(question);
    println!("{}", answer);
}

pub fn max_area_not5(height: Vec<i32>) -> i32 {
    // todo
    // two pointer technique
    let mut max = 0;
    let s = height.len();

    let (mut left, mut right) = (0, s - 1);
    while left < right {
        let h = std::cmp::min(height[left], height[right]);
        max = std::cmp::max(max, h * (right - left) as i32);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max
}

pub fn max_area_not4(height: Vec<i32>) -> i32 {
    let mut max_dic = std::collections::HashMap::<i32, usize>::new();
    let mut min_dic = std::collections::HashMap::<i32, usize>::new();

    for (idx, item) in height.iter().enumerate() {
        max_dic.insert(*item, idx);

        if min_dic.contains_key(item) {
            if idx < min_dic[item] {
                min_dic.insert(*item, idx);
            }
        } else {
            min_dic.insert(*item, idx);
        }
    }

    let mut keys = max_dic.keys().map(|x| *x).collect::<Vec<i32>>();
    keys.sort_by(|a, b| b.cmp(a));

    let mut max_size = 0;

    for i in 0..keys.len() {
        let target_keys = keys.iter().take(i + 1).collect::<Vec<&i32>>();

        let mut max = usize::MIN;
        let mut min = usize::MAX;

        for key in target_keys {
            if min_dic[key] < min {
                min = min_dic[key];
            }

            if max_dic[key] > max {
                max = max_dic[key];
            }
        }

        let size = (max - min) as i32 * keys[i];
        if size > max_size {
            if size == 56 {
                println!("{}", min);
                println!("{}", max);
                println!("{:?}", i);
            }
            max_size = size;
        }
    }

    max_size
}

// too slow
pub fn max_area_not3(height: Vec<i32>) -> i32 {
    let mut dic = std::collections::HashMap::<i32, Vec<usize>>::new();

    for (idx, item) in height.iter().enumerate() {
        if dic.contains_key(&item) {
            let value = dic.get_mut(&item).unwrap();
            value.push(idx);
        } else {
            dic.insert(*item, vec![idx]);
        }
    }

    let mut keys = dic.keys().map(|x| *x).collect::<Vec<i32>>();
    keys.sort_by(|a, b| b.cmp(a));
    let mut max_size = 0;

    for i in 0..keys.len() {
        let keys_i = keys.iter().take(i + 1).collect::<Vec<&i32>>();

        let mut vec = vec![];

        for key in keys_i {
            let mut item = dic.get_mut(key).unwrap().clone();
            vec.append(&mut item);
        }

        let size = (*vec.iter().max().unwrap() - *vec.iter().min().unwrap()) as i32 * keys[i];

        if size > max_size {
            max_size = size;
        }
    }

    max_size
}

// too slow
pub fn max_area_not2(height: Vec<i32>) -> i32 {
    let length = height.len();

    let mut container_length = length;
    let mut max_size = 0;

    while container_length != 0 {
        for i in 0..length {
            let left = height[i];
            let right = height.get(i + container_length - 1);

            if !right.is_some() {
                continue;
            }

            let mut smaller = *right.unwrap();
            if left < smaller {
                smaller = left;
            }

            let size = (container_length - 1) as i32 * smaller;

            if size > max_size {
                max_size = size;
            }
        }

        container_length = container_length - 1;
    }

    max_size
}

// too slow
pub fn max_area_not(height: Vec<i32>) -> i32 {
    let length = height.len();
    let mut max_size: i32 = 0;
    let mut max_height = 0;

    for i in 0..length {
        for j in i..length {
            let first = height[i];
            if first < max_height {
                continue;
            } else {
                max_height = first;
            }

            let second = height[j];

            let mut smaller = first;
            if second < first {
                smaller = second;
            }

            let diff = ((i as i32 - j as i32) as i32).abs();
            let size = diff * smaller;

            if size >= max_size {
                max_size = size;
            }
        }
    }

    max_size
}
