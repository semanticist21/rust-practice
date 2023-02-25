use std::{clone, fmt::Debug};

#[derive(Debug, Clone)]
struct PriorityQueue<T>
where
    T: Clone + Debug,
{
    data: Box<Vec<T>>,
    len: usize,
}

impl<T> PriorityQueue<T>
where
    T: Clone + Debug + PartialEq + PartialOrd,
{
    pub fn new() -> Self {
        Self {
            data: Box::new(vec![]),
            len: 0,
        }
    }

    pub fn insert(&mut self, data: T) {
        self.data.push(data);
        self.len += 1;

        let mut cur_index_marker = self.len() - 1;

        if cur_index_marker == 0 {
            return;
        }

        let mut parent_index = PriorityQueue::<T>::get_parent_node_index(cur_index_marker);

        while cur_index_marker != 0 && self.data[cur_index_marker] > self.data[parent_index] {
            self.data.swap(cur_index_marker, parent_index);
            cur_index_marker = parent_index;

            if cur_index_marker == 0 {
                break;
            }

            parent_index = PriorityQueue::<T>::get_parent_node_index(cur_index_marker);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() < 1 {
            return None;
        }

        if self.len() == 1 {
            return self.data.pop();
        }

        let first_item = self.data.swap_remove(0);
        let mut cur_index_marker = 0;

        let mut has_greater = self.has_greater_child(cur_index_marker);

        while has_greater {
            let bigger_idx = self.get_bigger_child_from_parent_index(cur_index_marker);
            self.data.swap(cur_index_marker, bigger_idx);

            cur_index_marker = bigger_idx;
            has_greater = self.has_greater_child(cur_index_marker);
        }

        Some(first_item)
    }

    pub fn has_greater_child(&self, index: usize) -> bool {
        let mut flag_left = false;
        let mut flag_right = false;

        let child_left_index = PriorityQueue::<T>::get_left_child_index(index);
        let child_right_index = PriorityQueue::<T>::get_right_child_index(index);
        let parent_node_data = &self.data[index];

        let left_child_opt = self.data.get(child_left_index);
        let right_child_opt = self.data.get(child_right_index);

        if let Some(left_item) = left_child_opt {
            if parent_node_data < left_item {
                flag_left = true;
            }
        }

        if let Some(right_item) = right_child_opt {
            if parent_node_data < right_item {
                flag_right = true;
            }
        }

        if flag_left || flag_right {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_bigger_child_from_parent_index(&self, index: usize) -> usize {
        let child_left_index = PriorityQueue::<T>::get_left_child_index(index);
        let child_right_index = PriorityQueue::<T>::get_right_child_index(index);

        let left_child_opt = self.data.get(child_left_index);
        let right_child_opt = self.data.get(child_right_index);

        if let None = right_child_opt {
            return child_left_index;
        }

        if let Some(left_item) = left_child_opt {
            if let Some(right_item) = right_child_opt {
                if left_item > right_item {
                    return child_left_index;
                } else {
                    return child_right_index;
                }
            }
        }

        // in the case of same ==
        child_left_index
    }

    pub fn len(&self) -> usize {
        self.len
    }

    const fn get_parent_node_index(index: usize) -> usize {
        (index - 1) / 2
    }

    const fn get_left_child_index(index: usize) -> usize {
        2 * index + 1
    }

    const fn get_right_child_index(index: usize) -> usize {
        2 * index + 2
    }
}

#[test]
fn test_insert() {
    let mut queue = PriorityQueue::<i32>::new();

    queue.insert(3);
    queue.insert(4);
    queue.insert(5);
    queue.insert(6);

    println!("{:?}", queue);
    println!("{:?}", queue.pop());
    println!("{:?}", queue.pop());
}
