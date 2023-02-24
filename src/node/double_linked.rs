use std::{borrow::Borrow, cell::RefCell, io::empty, ops::Deref, rc::Rc, clone};

#[derive(Clone)]
enum Node<T>
where
    T: Clone + PartialEq,
{
    Empty,
    Node {
        element: T,
        left: Rc<RefCell<Node<T>>>,
        right: Rc<RefCell<Node<T>>>,
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
                    left: l_left,
                    right: l_right,
                },
                Self::Node {
                    element: r_element,
                    left: r_left,
                    right: r_right,
                },
            ) => l_element == r_element && l_left == r_left && l_right == r_right,
            (Self::Empty, Self::Empty) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

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
    pub fn new(&self) -> Self {
        let empty_node = self.create_new_empty_node();

        DoubleList {
            first: Rc::clone(&empty_node) ,
            last: Rc::clone(&empty_node),
            current: empty_node,
        }
    }

    pub fn insert(&mut self, element: T)
    where
        T: PartialEq,
    {
        // in case of first insertion
        if self.first == self.create_new_empty_node() {
            let empty_node = self.create_new_empty_node();

            let first_node = Node::Node {
                element,
                left: Rc::clone(&empty_node),
                right: Rc::clone(&empty_node),
            };

            let first_node = self.create_node(first_node);

            self.first = Rc::clone(&first_node);
            self.last = Rc::clone(&first_node);
            self.current = Rc::clone(&first_node);

            return;
        }

        let new_node = Node::Node {
            element,
            left: Rc::clone(&self.current),
            right: self.create_new_empty_node(),
        };

        let ref_cell_item = RefCell::new(new_node);
        let new_node = self.create_node(new_node);

        // fix current Node to include new_node info.
        let borrowed_cur = self.current.borrow_mut();
        if let Node::Node { element, left, right } = *borrowed_cur{
            *right = ref_cell_item;
        }

        // insert last element smart pointer
        self.last = Rc::clone(&new_node);
        self.current = new_node;
    }

    fn create_new_empty_node(&self) -> Rc<RefCell<Node<T>>>{
        Rc::new(RefCell::new(Node::Empty))
    }

    fn create_node(&self, node :Node<T>) -> Rc<RefCell<Node<T>>>{
        Rc::new(RefCell::new(node))
    }
}
