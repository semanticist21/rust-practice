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

#[derive(Debug)]
struct QuickSorter {
    _arr: Box<[u32]>,
}

impl QuickSorter {
    pub fn _new(arr: Box<[u32]>) -> Self {
        QuickSorter { _arr: arr }
    }

    pub fn _quick_sort(&mut self, left_index: usize, right_index: usize) {
        if left_index >= right_index {
            return;
        }

        let new_pivot_index = self._sort_partition(left_index, right_index);

        if new_pivot_index == 0 {
            return;
        }

        self._quick_sort(left_index, new_pivot_index - 1);
        self._quick_sort(new_pivot_index + 1, right_index);
    }

    fn _sort_partition(&mut self, mut left_index: usize, mut right_index: usize) -> usize {
        let arr = &mut self._arr;

        let pivot_index = right_index;
        let pivot = arr[pivot_index];

        right_index -= 1;
        loop {
            while arr[left_index] < pivot {
                left_index += 1;
            }

            while arr[right_index] > pivot {
                if right_index >= 1 {
                    right_index -= 1;
                } else {
                    break;
                }
            }

            if left_index < right_index {
                arr.swap(left_index, right_index);
                left_index += 1;
                continue;
            }

            break;
        }

        // last step
        arr.swap(left_index, pivot_index);

        // left index is where pivot data is located.
        left_index
    }

    pub fn _quick_select(
        &mut self,
        nth_small_index: usize,
        left_index: usize,
        right_index: usize,
    ) -> u32 {
        if left_index >= right_index {
            return self._arr[left_index];
        }

        let new_pivot_index = self._sort_partition(left_index, right_index);

        if nth_small_index > new_pivot_index {
            self._sort_partition(left_index, new_pivot_index - 1);
        } else if nth_small_index < new_pivot_index {
            self._sort_partition(new_pivot_index + 1, right_index);
        } else {
            return self._arr[nth_small_index];
        }

        0
    }
}

#[test]
pub fn test_quick_sort() {
    let arr = Box::new([5, 1, 2, 5, 6, 5]);
    let size = arr.len();
    let mut quick = QuickSorter::_new(arr);

    quick._quick_sort(0, size - 1);

    println!("{:?}", quick);
}

pub fn is_duplicate_using_sort(arr: &mut [u32]) -> bool {
    arr.sort_unstable();
    let mut result = false;

    for (idx, _) in arr.iter().enumerate() {
        if idx == arr.len() - 1 {
            break;
        }

        if arr[idx] == arr[idx + 1] {
            result = true;
        }
    }

    result
}

#[test]
fn test_is_duplicate() {
    let mut arr: [u32; 9] = [5, 4, 1, 2, 3, 5, 1, 2, 3];
    let flag = is_duplicate_using_sort(&mut arr);
    println!("{}", flag);

    let mut arr = [6, 5, 4, 3, 2, 1];
    let flag = is_duplicate_using_sort(&mut arr);
    println!("{}", flag);
}
