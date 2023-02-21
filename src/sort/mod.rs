pub fn bubble_sort(arr: &mut [u8]) -> &[u8] {
    let mut sorted = false;
    let loop_len = arr.len() - 1;

    while !sorted {
        sorted = true;

        for i in 0..loop_len {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
    }

    arr
}

pub fn selection_sort(arr: &mut [u8]) -> &[u8] {
    let loop_len = arr.len();

    for i in 0..loop_len {
        for j in i + 1..loop_len {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }

    arr
}

pub fn insertion_sort(arr: &mut [u8]) -> &[u8] {
    let loop_size = arr.len();

    for i in 1..loop_size {
        let mut position = i;

        while position != 0 {
            let temp = arr[position];
            let compare_val = arr[position - 1];

            if temp < compare_val {
                arr.swap(position, position - 1);
                position -= 1;
            } else {
                break;
            }
        }
    }

    arr
}

#[test]
fn test_bubble() {
    let mut test_arr: [u8; 12] = _get_test_arr();

    bubble_sort(&mut test_arr);
    println!("{:?}", test_arr);
}

#[test]
fn test_sort() {
    let mut test_arr: [u8; 12] = _get_test_arr();
    selection_sort(&mut test_arr);

    println!("{:?}", test_arr);
}

#[test]
fn test_insertion() {
    let mut test_arr: [u8; 12] = _get_test_arr();
    insertion_sort(&mut test_arr);

    println!("{:?}", test_arr);
}

fn _get_test_arr() -> [u8; 12] {
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 13, 4]
}
