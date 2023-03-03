use core::borrow;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
// End -> shows popularity
pub enum TrieNode {
    Empty,
    End(usize),
    Data(HashMap<char, Rc<RefCell<TrieNode>>>),
}

// struct TrieNode {
//     data: HashMap<char, Option<Rc<RefCell<TrieNode>>>>,
// }

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode::Data(HashMap::<char, Rc<RefCell<TrieNode>>>::new())
    }

    pub fn end() -> TrieNode {
        TrieNode::End(0)
    }

    pub fn get(&self) -> Option<&HashMap<char, Rc<RefCell<TrieNode>>>> {
        if let TrieNode::End(_) = self {
            return None;
        }

        if let TrieNode::Data(dic) = self {
            return Some(dic);
        }

        None
    }
}

#[derive(Debug)]
pub struct Trie {
    root: Rc<RefCell<TrieNode>>,
}

impl Trie {
    const END_MARK: char = '*';

    pub fn new() -> Self {
        let node = TrieNode::new();

        Trie {
            root: Rc::new(RefCell::new(node)),
        }
    }

    pub fn search(&self, word: &str) -> Option<Rc<RefCell<TrieNode>>> {
        let mut cur_node = Rc::clone(&self.root);

        for c in word.chars() {
            let borrowed = cur_node.borrow();
            let cur_dic = borrowed.get();

            let next_node = cur_dic.unwrap().get(&c);

            if let None = next_node {
                return None;
            }

            let next_node = Rc::clone(next_node.unwrap());
            drop(borrowed);

            cur_node = next_node;
        }

        // check whether it has an end mark.
        let borrowed = cur_node.borrow();

        if let TrieNode::Data(data) = &*borrowed {
            let opt_end = data.get(&Trie::END_MARK);

            if let None = opt_end {
                return None;
            } else if let Some(node) = opt_end {
                return Some(Rc::clone(&node));
            }
        }

        None
    }

    pub fn search_also_prefix(&self, word: &str) -> Option<Rc<RefCell<TrieNode>>> {
        let mut cur_node = Rc::clone(&self.root);

        for c in word.chars() {
            let borrowed = cur_node.borrow();
            let cur_dic = borrowed.get();

            let next_node = cur_dic.unwrap().get(&c);

            if let None = next_node {
                return None;
            }

            let next_node = Rc::clone(next_node.unwrap());
            drop(borrowed);

            cur_node = next_node;
        }

        return Some(Rc::clone(&cur_node));
    }

    pub fn insert(&mut self, word: &str) -> bool {
        // deleted because of performance
        // if word.contains(' ') {
        //     return false;
        // }

        let mut cur_node = Rc::clone(&self.root);

        for (idx, c) in word.chars().enumerate() {
            let mut cur_mut_borrowed = cur_node.borrow_mut();

            if let TrieNode::Data(ref mut data) = *cur_mut_borrowed {
                let next_node = data.get(&c);

                // move forward
                if let Some(next_node) = next_node {
                    let next_node = Rc::clone(next_node);
                    drop(cur_mut_borrowed);

                    cur_node = next_node;
                }
                // if none, make dic
                else if let None = next_node {
                    let new_node = Rc::new(RefCell::new(TrieNode::new()));

                    if let TrieNode::Data(ref mut data) = *cur_mut_borrowed {
                        data.insert(c, Rc::clone(&new_node));
                    }

                    drop(cur_mut_borrowed);
                    cur_node = new_node;
                }

                // mark end if last element
                if idx == word.len() - 1 {
                    let cur_borrowed = cur_node.borrow();

                    if let TrieNode::Data(data) = &*cur_borrowed {
                        let check_mark_dic = data.get(&Trie::END_MARK);

                        if let None = check_mark_dic {
                            drop(cur_borrowed);
                            let mut cur_mut_borrowed = cur_node.borrow_mut();

                            if let TrieNode::Data(ref mut data) = *cur_mut_borrowed {
                                data.insert(Trie::END_MARK, Rc::new(RefCell::new(TrieNode::new())));
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        true
    }

    pub fn list_all(&self, node: Rc<RefCell<TrieNode>>, mut word: &str, vec: &mut Vec<String>) {
        // initial searching
        if let TrieNode::Empty = &*node.borrow() {
            if let TrieNode::Data(data) = &*self.root.borrow() {
                for (key, value) in data {
                    self.list_all(Rc::clone(value), key.to_string().as_str(), vec);
                }
            }
        } else if let TrieNode::Data(data) = &*node.borrow() {
            for (key, value) in data {
                if key == &Trie::END_MARK {
                    vec.push(word.to_string());
                } else {
                    let new_word = format!("{}{}", word, key);
                    self.list_all(Rc::clone(value), new_word.as_str(), vec);
                }
            }
        }
    }

    pub fn auto_complete(&self, prefix: &str) -> Vec<String> {
        let node = self.search_also_prefix(prefix);

        if let None = node {
            return vec![];
        }

        let mut vec = vec![];

        self.list_all(node.unwrap(), prefix, &mut vec);

        vec
    }
}

#[test]
fn test_trie() {
    let word = "super";
    let mut trie = Trie::new();

    trie.insert(word);

    let result = trie.search(word);
    println!("{:?}", result);
    let result = trie.search("yollo");
    println!("{:?}", result);
}

#[test]
fn test_listing() {
    let word = "super";
    let word2 = "superstar";
    let word3 = "helloooo";

    let mut trie = Trie::new();

    trie.insert(word);
    trie.insert(word2);
    trie.insert(word3);

    let mut vec = vec![];
    trie.list_all(Rc::new(RefCell::new(TrieNode::Empty)), "", &mut vec);

    println!("{:?}", vec);
}

#[test]
fn teat_auto_complete() {
    let word = "super";
    let word2 = "superstar";
    let word3 = "helloooo";

    let mut trie = Trie::new();

    trie.insert(word);
    trie.insert(word2);
    trie.insert(word3);

    let result = trie.auto_complete("super");
    println!("{:?}", result);
}
