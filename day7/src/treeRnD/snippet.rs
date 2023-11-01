#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    array,
    cell::RefCell,
    fmt::Display,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex, RwLock, Weak},
};

/// <https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#adding-a-reference-from-a-child-to-its-parent>
/// Thinking about the relationships another way, a parent node should own its children: if a parent
/// node is dropped, its child nodes should be dropped as well. However, a child should not own its
/// parent: if we drop a child node, the parent should still exist. This is a case for weak
/// references!
#[test]
fn test_weak_refs() {
    type NodeRef<T> = Arc<Node<T>>;
    type Parent<T> = RefCell<Weak<Node<T>>>; // not `RefCell<<Rc<Node>>>` which would cause memory leak.
    type Children<T> = RefCell<Vec<NodeRef<T>>>;

    #[derive(Debug)]
    struct Node<T> {
        value: T,
        parent: Parent<T>,
        children: Children<T>,
    }

    // TODO: start add Tree w/ root & methods.
    struct Tree<T> {
        root: NodeRef<T>,
    }

    impl<T> Tree<T> {
        fn new(root: NodeRef<T>) -> Tree<T> {
            Tree { root }
        }
    }
    // TODO: end add Tree w/ root & methods.

    /// `child_node.parent` is set to weak reference to `parent_node`.
    fn set_parent<T>(child: &NodeRef<T>, parent: &NodeRef<T>) {
        *child.parent.borrow_mut() = Arc::downgrade(&parent);
    }

    fn add_child<T>(child: &NodeRef<T>, parent: &NodeRef<T>) {
        parent.children.borrow_mut().push(child.clone());
    }

    fn create_node<T>(value: T) -> NodeRef<T> {
        let node = Node {
            value,
            parent: RefCell::new(Weak::new()),  // Basically None.
            children: RefCell::new(Vec::new()), // Basically [].
        };
        let node_ref = Arc::new(node);
        node_ref
    }

    let child_node: NodeRef<i32> = create_node(3);

    {
        let parent_node: NodeRef<i32> = create_node(5);
        add_child(&child_node, &parent_node);
        set_parent(&child_node, &parent_node);

        assert_eq!(Arc::strong_count(&child_node), 2); // `child_node` has 2 strong references.
        assert_eq!(Arc::weak_count(&child_node), 0);

        assert_eq!(Arc::strong_count(&parent_node), 1); // `parent_node` has 1 strong reference.
        assert_eq!(Arc::weak_count(&parent_node), 1); // `parent_node` also has 1 weak reference.

        assert!(child_node.parent.borrow().upgrade().is_some());
        assert_eq!(child_node.parent.borrow().upgrade().unwrap().value, 5);
    } // `parent_node` is dropped here.

    // `child_node`'s parent is now `None`.
    assert!(child_node.parent.borrow().upgrade().is_none());
    assert_eq!(child_node.value, 3);

    assert_eq!(Arc::strong_count(&child_node), 1); // `child_node` has 1 strong references.
    assert_eq!(Arc::weak_count(&child_node), 0); // `child_node` still has no weak references.
}
