// currently implented only Add, Search
use std::{
    cell::{BorrowMutError, RefCell},
    fmt::Debug,
    rc::Rc,
};

#[derive(Debug, Clone)]
enum Node<T>
where
    T: Clone + PartialEq,
{
    Empty,
    Node {
        element: T,
        head: Rc<RefCell<Node<T>>>,
        tail: Rc<RefCell<Node<T>>>,
    },
}

impl<T> PartialEq for Node<T>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Node {
                    element: l_element,
                    head: l_left,
                    tail: l_right,
                },
                Self::Node {
                    element: r_element,
                    head: r_left,
                    tail: r_right,
                },
            ) => l_element == r_element && l_left == r_left && l_right == r_right,
            (Self::Empty, Self::Empty) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug)]
struct DoubleList<T>
where
    T: Clone + PartialEq,
{
    first: Rc<RefCell<Node<T>>>,
    last: Rc<RefCell<Node<T>>>,
    current: Rc<RefCell<Node<T>>>,
}

impl<T> DoubleList<T>
where
    T: Clone + PartialEq,
{
    pub fn new() -> Self {
        let empty_node = Self::_create_new_empty_node();

        DoubleList {
            first: Rc::clone(&empty_node),
            last: Rc::clone(&empty_node),
            current: empty_node,
        }
    }

    pub fn insert(&mut self, element: T) -> Result<(), Box<BorrowMutError>>
    where
        T: PartialEq + Debug,
    {
        // in case of first insertion
        if self.first == DoubleList::_create_new_empty_node() {
            let empty_node = DoubleList::_create_new_empty_node();

            let first_node = Node::Node {
                element,
                head: Rc::clone(&empty_node),
                tail: Rc::clone(&empty_node),
            };

            let first_node = DoubleList::_create_node(first_node);

            self.first = Rc::clone(&first_node);
            self.last = Rc::clone(&first_node);
            self.current = Rc::clone(&first_node);

            return Ok(());
        }

        let new_node = Node::Node {
            element,
            head: Rc::clone(&self.current),
            tail: DoubleList::_create_new_empty_node(),
        };

        let new_node = DoubleList::_create_node(new_node);

        // fix current Node to include new_node info.
        if let Err(e) = self.current.try_borrow_mut() {
            return Err(Box::new(e));
        }
        let mut borrowed_cur = self.current.borrow_mut();
        let mut replace_node = Node::Empty;
        if let Node::Node {
            element,
            head,
            tail: _,
        } = &*borrowed_cur
        {
            let new_replace_node = Node::Node {
                element: element.clone(),
                head: Rc::clone(head),
                tail: Rc::clone(&new_node),
            };
            replace_node = new_replace_node;
        }

        *borrowed_cur = replace_node;

        drop(borrowed_cur);

        // insert last element smart pointer
        self.last = Rc::clone(&new_node);
        self.current = new_node;

        Ok(())
    }

    fn _create_new_empty_node() -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node::Empty))
    }

    fn _create_node(node: Node<T>) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(node))
    }

    pub fn _get_last(&self) -> Option<T> {
        let borrowed = self.last.borrow();

        if let Node::Empty = &*borrowed {
            return None;
        }

        if let Node::Node {
            element,
            head: _,
            tail: _,
        } = &*borrowed
        {
            return Some(element.clone());
        }

        return None;
    }

    pub fn _get_current(&self) -> Option<T> {
        let borrowed = self.current.borrow();
        if let Node::Node {
            element,
            head: _,
            tail: _,
        } = &*borrowed
        {
            return Some(element.clone());
        } else {
            return None;
        }
    }

    pub fn _search(&mut self, target: T) -> bool {
        // current -> to first
        self.current = Rc::clone(&self.first);

        if let None = self._get_current() {
            return false;
        }

        let cur_value = self._get_current().unwrap();

        if target.eq(&cur_value) {
            return true;
        }

        while !target.eq(&self._get_current().unwrap()) {
            let forward_result = self._next_forward();

            // when there is no more contents
            if let Err(()) = forward_result {
                return false;
            }
        }

        true
    }

    fn _next_forward(&mut self) -> Result<(), ()> {
        let borrowed = self.current.borrow();
        let mut next_node = DoubleList::_create_new_empty_node();

        if let Node::Node {
            element: _,
            head: _,
            tail,
        } = &*borrowed
        {
            if let Node::Empty = *tail.borrow() {
                return Err(());
            }

            if let Node::Node {
                element: _,
                head: _,
                tail: _,
            } = *tail.borrow()
            {
                next_node = Rc::clone(tail);
            }
        }

        drop(borrowed);
        self.current = next_node;

        Ok(())
    }
}

#[test]
fn test_list() {
    let mut list = DoubleList::<String>::new();

    _ = list.insert("test".to_string());
    _ = list.insert("test222".to_string());
    _ = list.insert("test333".to_string());

    let cur = list._get_current().unwrap();
    println!("{:?}", cur);

    let flag = list._search("test222".to_string());
    println!("{}", flag);
}
