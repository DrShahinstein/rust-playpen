struct TreeNode {
    data: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn create_node(data: i32) -> Box<TreeNode> {
    Box::new(TreeNode {
        data,
        left: None,
        right: None,
    })
}

fn insert(root: &mut Option<Box<TreeNode>>, data: i32) {
    match root {
        Some(node) => {
            if data < node.data {
                insert(&mut node.left, data);
            } else {
                insert(&mut node.right, data);
            }
        }
        None => *root = Some(create_node(data)),
    }
}

fn inorder_traversal(root: &Option<Box<TreeNode>>) {
    if let Some(node) = root {
        inorder_traversal(&node.left);
        println!("{}", node.data);
        inorder_traversal(&node.right);
    }
}

pub fn run() {
    let mut root = None;

    let elements = vec![5, 3, 7, 2, 4, 6, 8];
    for &element in &elements {
        insert(&mut root, element);
    }

    println!("In-order traversal:");
    inorder_traversal(&root);
}

/*
This program demonstrates how Box<T> is an excellent choice for managing memory and
creating recursive data structures like binary trees. By using Box<T>,
we ensure that memory is automatically deallocated when it goes out of scope,
preventing memory leaks and ensuring safety.
*/
