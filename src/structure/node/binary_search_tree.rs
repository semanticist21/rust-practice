// todo
// Define the Binary Tree data structure
#[derive(Debug)]
enum BinaryTree<T> {
    _Empty,
    _Node {
        element: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

// Insert a new element into the Binary Tree
fn _insert<T: Ord>(tree: &mut BinaryTree<T>, element: T) {
    match tree {
        BinaryTree::_Empty => {
            *tree = BinaryTree::_Node {
                element,
                left: Box::new(BinaryTree::_Empty),
                right: Box::new(BinaryTree::_Empty),
            };
        }
        BinaryTree::_Node {
            element: e,
            left,
            right,
        } => {
            if element <= *e {
                _insert(left, element);
            } else {
                _insert(right, element);
            }
        }
    }
}

// Search for an element in the Binary Tree
fn _search<T: Ord>(tree: &BinaryTree<T>, element: T) -> bool {
    match tree {
        BinaryTree::_Empty => false,
        BinaryTree::_Node {
            element: e,
            left,
            right,
        } => {
            if element == *e {
                true
            } else if element < *e {
                _search(left, element)
            } else {
                _search(right, element)
            }
        }
    }
}

#[test]
fn test_main() {
    // Create an empty Binary Tree
    let mut tree = BinaryTree::_Empty;

    // Insert some elements into the Binary Tree
    _insert(&mut tree, 5);
    _insert(&mut tree, 3);
    _insert(&mut tree, 7);
    _insert(&mut tree, 1);

    // Search for an element in the Binary Tree
    let element_to_search = 7;
    if _search(&tree, element_to_search) {
        println!("{} is in the Binary Tree", element_to_search);
    } else {
        println!("{} is not in the Binary Tree", element_to_search);
    }

    // Print the Binary Tree for debugging purposes
    println!("Binary Tree: {:?}", tree);
}
