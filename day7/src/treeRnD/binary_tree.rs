//which means that it allows you to mutate the contents of value even if you only have an immutable reference (&T) to it.

// A tree data structure in computer science represents
// a hierarchical tree structure with a set of connected nodes.

// Therefore, need to Handle:
// Shared ownership (Rc) - necessary to optimally provide access to these children node to other code that use this tree data structure.
// Interior mutability (RefCell) - necessary to allow modifications to the underlying node itself.


// RefCell<T> (and Cell<T>) is a type that allows for interior mutability,
// which means that it allows you to mutate the contents of value even if you only have an immutable reference (&T) to it.
// By wrapping the left and right nodes in RefCell<T>, you don't have to keep track of mutable and immutable parts of the struct.
// This is especially useful if you're working with a recursive data structure.

// Rc<T> (stands for reference counting) "provides shared ownership of a value of type T, allocated in the heap".
// It allows multiple parts of your code to share ownership of a value and no one has exclusive ownership.

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct TreeNode {
    val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

// We are doing a traversing the binary tree using depth first search algorithm and summing its values
pub fn tree_sum(root: TreeNodeRef) -> i32 {
    let mut sum = 0i32;
    // We'll use a `vec` as a
    // stack LIFO data structure.
    // Start by adding the root node
    // to the stack.
    let mut stack = vec![root];

    while !stack.is_empty() {
        // current points to top most
        // item in the stack
        let current: Rc<RefCell<TreeNode>> = stack.pop().unwrap();
        sum += current.borrow().val;

        // if there is a right node,
        // then push it on top of the stack
        if let Some(right) = &current.borrow().right {
            stack.push(right.to_owned());
        };
        // if there is a left node,
        // then push it on top of the stack
        if let Some(left) = &current.borrow().left {
            stack.push(left.to_owned());
        };
    }
    sum
}

pub fn tree_sum_recursive(root: Option<&TreeNodeRef>) -> i32 {
    // if `root` has `Some`thing
    // return `root.val` + left_node_val + right_node_val
    if let Some(root) = root {
        root.borrow().val
            // recursively call left path
            + tree_sum_recursive(root.borrow().left.as_ref())
            // recursively call right path
            + tree_sum_recursive(root.borrow().right.as_ref())
    } else {
        // root is None (i.e. empty or null)
        // so return `0`
        0
    }
}
