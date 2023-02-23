use std::{borrow::Borrow, cell::RefCell, clone, ops::Deref, rc::Rc};

trait Linkable {
    type Data;
    type DataPt;

    fn get_current(&self) -> &Option<Self::DataPt>;
    fn get_next(&self) -> &Option<Self::DataPt>;
    fn add(&mut self, data: Self::Data);
    fn read(&self, index: usize) -> Option<Self::DataPt>;
}

#[derive(Clone)]
struct Node<T>
where
    T: Clone,
{
    _data: T,
    next_node: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Clone,
{
    pub fn new(data: T) -> Self {
        Node {
            _data: data,
            next_node: None,
        }
    }

    pub fn set_next(&mut self, value: Option<Rc<RefCell<Node<T>>>>) {
        if let Some(item) = value {
            self.next_node = Some(item);
            return;
        }

        self.next_node = None;
    }
}

struct LinkedList<T>
where
    T: Clone,
{
    first: Option<Rc<RefCell<Node<T>>>>,
    current: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> LinkedList<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        LinkedList {
            first: None,
            current: None,
            len: 0,
        }
    }
}

impl<T> Linkable for LinkedList<T>
where
    T: Clone,
{
    type Data = T;
    type DataPt = Rc<RefCell<Node<T>>>;

    fn get_current(&self) -> &Option<Self::DataPt> {
        &self.current
    }

    fn get_next(&self) -> &Option<Self::DataPt> {
        if let None = self.current {
            return &None;
        }

        let item = self.current.clone().unwrap();

        // &item
        &None
    }
    //todo
    //fix to if let some() pattern
    fn add(&mut self, data: T) {
        // fisrt adding
        if let None = self.first {
            let node = Node::new(data);
            let smart_pt = Rc::new(RefCell::new(node));
            self.first = Some(Rc::clone(&smart_pt));
            self.current = Some(smart_pt);
            self.len += 1;
            return;
        }

        let node = RefCell::new(Node::new(data));
        let smart_pt = Rc::new(node);

        let current_node = self.current.clone().unwrap();
        let mut borrowed = current_node.borrow_mut();
        borrowed.next_node = Some(Rc::clone(&smart_pt));
        self.len += 1;
        // current should be added.

        self.current = Some(smart_pt);
    }
    
    //todo
    //fix to if let some() pattern
    fn read(&self, mut index: usize) -> Option<Rc<RefCell<Node<T>>>> {
        if let None = self.current {
            return None;
        }

        if index == 0 {
            return self.current.clone();
        }

        let mut current_item = self.current.clone().unwrap();
        while index != 0 {
            let next_item = current_item.deref().borrow().next_node.clone();

            if let None = next_item {
                return None;
            }

            current_item = next_item.unwrap();
            index -= 1;

            if index == 0 {
                return Some(current_item);
            }
        }

        None
    }
}

#[test]
fn test_() {
    let mut a = 3;
    let mut b = Rc::new(&mut a);
    let e = Rc::get_mut(&mut b).unwrap();
    **e = 4;

    println!("{}", a);

    let mut x = Rc::new(3);
    *Rc::get_mut(&mut x).unwrap() = 4;
    assert_eq!(*x, 4);
}
