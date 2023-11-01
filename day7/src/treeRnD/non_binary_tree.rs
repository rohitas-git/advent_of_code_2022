// https://developerlife.com/2022/02/24/rust-non-binary-tree/

///  Impl Details
/// - a reference to a parent that it does not own, but has a weak reference to
/// - hold references to children that it owns

// Therefore, need to Handle:
// Shared ownership (Rc) - necessary to optimally provide access to these children node to other code that use this tree data structure.
// Interior mutability (RefCell) - necessary to allow modifications to the underlying node itself.

// THREAD SAFETY #
// While this is a good start, we haven’t dealt with thread safety.
// Rust makes it very easy to handle this paralellism, we simply do the following:
// - Replace Rc with Arc.
// - Replace RefCell with RwLock (note that we could have also used Mutex but we are using RwLock for better performance in use cases where more nodes will be accessed in the tree, rather than new nodes added to the tree).

/// This struct holds underlying data. It shouldn't be created directly, instead use:
/// [`Node`](struct@Node).
///
/// ```text
/// NodeData
///  | | |
///  | | +- value: T ---------------------------------------+
///  | |                                                    |
///  | |                                        Simple onwership of value
///  | |
///  | +-- parent: RwLock<WeakNodeNodeRef<T>> --------+
///  |                                            |
///  |                 This describes a non-ownership relationship.
///  |                 When a node is dropped, its parent will not be dropped.
///  |
///  +---- children: RwLock<Vec<Child<T>>> ---+
///                                           |
///                 This describes an ownership relationship.
///                 When a node is dropped its children will be dropped as well.
/// ```
use std::{
    array,
    cell::RefCell,
    fmt::Display,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex, RwLock, Weak},
};

#[derive(Debug)]
pub struct NodeData<T>
where
    T: Display,
{
    value: T,
    parent: Parent<T>,
    children: Children<T>,
}

type NodeDataRef<T> = Arc<NodeData<T>>;
type WeakNodeNodeRef<T> = Weak<NodeData<T>>;

/// Parent relationship is one of non-ownership.
/// This is not a `RwLock<NodeDataRef<T>>` which would cause memory leak.
type Parent<T> = RwLock<WeakNodeNodeRef<T>>;

/// Children relationship is one of ownership.
type Children<T> = RwLock<Vec<Child<T>>>;
type Child<T> = NodeDataRef<T>;

// When a node is dropped, its children will be dropped as well (since it owns them).
// We represent this relationship w/ a strong reference.
//
// However, the parent should not be dropped (since it does not own them).
// We represent this relationship w/ a weak reference.

/// This struct is used to own a [`NodeData`] inside an [`Arc`]. The [`Arc`]
/// can be shared, so that it can have multiple owners. It does not have
/// getter methods for [`NodeData`]'s properties, instead it implements the
/// `Deref` trait to allow it to be used as a [`NodeData`].
///
/// # Shared ownership
///
/// After an instance of this struct is created and it's internal reference is
/// cloned (and given to another) dropping this instance will not drop the cloned
/// internal reference.
///
/// ```text
/// Node { arc_ref: Arc<NodeData> }
///    ▲                 ▲
///    │                 │
///    │      This atomic ref owns the
///    │      `NodeData` & is shared
///    │
///    1. Has methods to manipulate nodes and their children.
///
///    2. When it is dropped, if there are other `Arc`s (shared via
///       `get_copy_of_internal_arc()`) pointing to the same underlying
///       `NodeData`, then the `NodeData` will not be dropped.
///
///    3. This struct is necessary in order for `add_child_and_update_its_parent`
///       to work. Some pointers need to be swapped between 2 nodes for this work
///       (and one of these pointers is a weak one). It is not possible to do this
///       using two `NodeData` objects, without wrapping them in `Arc`s.
/// ```

#[derive(Debug)]
pub struct Node<T: Display> {
    arc_ref: NodeDataRef<T>,
}

impl<T> Deref for Node<T>
where
    T: Display,
{
    type Target = NodeData<T>;

    fn deref(&self) -> &Self::Target {
        &self.arc_ref
    }
}

impl<T> Node<T>
where
    T: Display,
{
    pub fn new(value: T) -> Node<T> {
        let new_node = NodeData {
            value,
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new()),
        };
        let arc_ref = Arc::new(new_node);
        Node { arc_ref }
    }

    pub fn get_copy_of_internal_arc(self: &Self) -> NodeDataRef<T> {
        Arc::clone(&self.arc_ref)
    }

    pub fn create_and_add_child(self: &Self, value: T) -> NodeDataRef<T> {
        let new_child = Node::new(value);
        self.add_child_and_update_its_parent(&new_child);
        new_child.get_copy_of_internal_arc()
    }

    /// 🔏 Write locks used.
    pub fn add_child_and_update_its_parent(self: &Self, child: &Node<T>) {
        {
            let mut my_children = self.arc_ref.children.write().unwrap();
            my_children.push(child.get_copy_of_internal_arc());
        } // `my_children` guard dropped.

        {
            let mut childs_parent = child.arc_ref.parent.write().unwrap();
            *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
        } // `my_parent` guard dropped.
    }

    pub fn has_parent(self: &Self) -> bool {
        self.get_parent().is_some()
    }

    /// 🔒 Read lock used.
    pub fn get_parent(self: &Self) -> Option<NodeDataRef<T>> {
        let my_parent_weak = self.arc_ref.parent.read().unwrap();
        if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
            Some(my_parent_arc_ref)
        } else {
            None
        }
    }
}

// !!!!! SOPHISTICATED APPROACH USING MEMORY ARENA
// In our naive example, we manage references that are strong (owned, children) and weak (not owned, parent). And we have to wrap the NodeData inside of a Node in order to be able to share it. This is quite cumbersome to use. We will use the idea of a memory arena to simplify this. Here’s the wikipedia definition of a memory arena.

// ! TRAITS

// They are meant for 2 things:
// 1. Extension of existing types -
// 2. Adaptation of existing types -

mod memory_arena {
    trait HasId {
        type Id;
        fn id(&self) -> &Self::Id;
    }

    struct Node {
        id: i32,
        payload: String,
        children: Vec<i32>,
    }

    impl HasId for Node {
        type Id = i32;

        fn id(&self) -> &Self::Id {
            &self.id
        }
    }

    impl HasId for i32 {
        type Id = i32;

        fn id(&self) -> &Self::Id {
            self
        }
    }

    mod test_different_styles_of_passing_args_via_trait_impl {
        use super::*;
        
        /// This accepts a borrowed `Node` object.
        fn fun_0(node: &Node) {
            println!("{}: {}", "fun_0:", node.id());
        }

        /// This accepts a borrowed object that implements `HasId`.
        fn fun_1(node: &dyn HasId<Id = i32>) {
            println!("{}: {}", "fun_1:", node.id());
        }

        /// This takes an object that implements `HasId`.
        fn fun_2_own(node: impl HasId<Id = i32>) {
            println!("{}: {}", "fun_2:", node.id());
        }

        fn fun_2(node: &impl HasId<Id = i32>) {
            println!("{}: {}", "fun_2:", node.id());
        }

        /// This takes an `i32` which also implements `HasId`.
        fn fun_3_own(node: i32) {
            println!("{}: {}", "fun_3:", &node.id());
        }

        fn fun_3(node: &i32) {
            println!("{}: {}", "fun_3:", &node.id());
        }

        /// This takes a `Node` object that's in a `Box` reference.
        fn fun_4(node: Box<dyn HasId<Id = i32>>) {
            println!("{}: {}", "fun_4:", node.id());
        }

        #[test]
        fn test_funs() {
            // Here is what it looks like to use these various forms.
            let my_node = Node {
                id: 1,
                payload: "payload".to_string(),
                children: vec![2, 3, 4],
            };
            let my_i32_id = 1;

            fun_0(&my_node);

            fun_1(&my_node);
            fun_1(&my_i32_id);

            fun_2(&my_node);
            fun_2(&my_i32_id);

            fun_3(&my_i32_id);

            fun_4(Box::new(my_node)); // `my_node` is moved into `fun_4`.
            fun_4(Box::new(my_i32_id)); // `my_i32_id` is moved into `fun_4`.
        }
    }
}
