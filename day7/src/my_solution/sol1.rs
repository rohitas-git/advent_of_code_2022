

use std::{
    array,
    cell::RefCell,
    fmt::Display,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex, RwLock, Weak},
};

// Store entry data  
// File{size: N, name: ".."} -> Leaf nodes
// Dir {size: 0, name: "..."} -> Internal nodes: Parent, Children 
// Parent -> Weak ownership 
// Children -> Strong ownership

// Rc -> Shared ownership
// RefCell -> Interior mutability with check at runtime

type NodeRef = Rc<Node>;
// Children -> Strong ownership
type ChildrenRef = RefCell<Vec<NodeRef>>;

type WeakNodeRef = Weak<Node>;
// Parent -> Weak ownership 
type ParentRef = RefCell<WeakNodeRef>;



#[derive(Debug, Clone, Default)]
pub struct Node{
    size: u32,
    name: String,
    parent: Option<ParentRef>,
    children: Option<ChildrenRef>
}

impl Node{
    fn new_dir(name: String) -> Node{
        Node { size: 0, name, parent: None, children: None }
    }


    fn new_file(&self, name: String, size: u32) -> Node{
        Node { size: 0, name, parent: Some(RefCell::new(Weak::)), children: None }
    }

    fn update_child(&self, child: NodeRef)-> bool{
        if let Some(children) = self.children {
            children.borrow_mut().push(child);
            true
        } else{
            false
        }

    }

    fn is_dir(&self) -> bool{
        self.size == 0
    }
}


#[cfg(test)]
mod test_sol1 {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node{}
    }
}



