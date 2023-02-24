// Define the Binary Tree data structure
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    Node {
        element: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

// Insert a new element into the Binary Tree
fn insert<T: Ord>(tree: &mut BinaryTree<T>, element: T) {
    match tree {
        BinaryTree::Empty => {
            *tree = BinaryTree::Node {
                element,
                left: Box::new(BinaryTree::Empty),
                right: Box::new(BinaryTree::Empty),
            };
        }
        BinaryTree::Node { element: e, left, right } => {
            if element <= *e {
                insert(left, element);
            } else {
                insert(right, element);
            }
        }
    }
}

// Search for an element in the Binary Tree
fn search<T: Ord>(tree: &BinaryTree<T>, element: T) -> bool {
    match tree {
        BinaryTree::Empty => false,
        BinaryTree::Node { element: e, left, right } => {
            if element == *e {
                true
            } else if element < *e {
                search(left, element)
            } else {
                search(right, element)
            }
        }
    }
}

fn main() {
    // Create an empty Binary Tree
    let mut tree = BinaryTree::Empty;

    // Insert some elements into the Binary Tree
    insert(&mut tree, 5);
    insert(&mut tree, 3);
    insert(&mut tree, 7);
    insert(&mut tree, 1);

    // Search for an element in the Binary Tree
    let element_to_search = 7;
    if search(&tree, element_to_search) {
        println!("{} is in the Binary Tree", element_to_search);
    } else {
        println!("{} is not in the Binary Tree", element_to_search);
    }

    // Print the Binary Tree for debugging purposes
    println!("Binary Tree: {:?}", tree);
}