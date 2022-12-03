// link list in rust
// evert thing is privated by default in rust
#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node { data, next: None }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    // add node
    pub fn append(&mut self, data: i32) {
        let new_node = Box::new(Node::new(data));
        let mut current_node = &mut self.head;
        while let Some(node) = current_node {
            // Some is a enum is used to check if value is present or not
            current_node = &mut node.next;
        }
        *current_node = Some(new_node); //  * dereference operator
    }
    // show node
    pub fn show_node(&self) {
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            println!("{}", node.data);
            current_node = &node.next;
        }
    }
    // serach
    pub fn search(&self, data: i32) -> bool {
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            if node.data == data {
                return true;
            }
            current_node = &node.next;
        }
        return false;
    }
    // get length
    pub fn length(&self) -> i32 {
        let mut current_node = &self.head;
        let mut count = 0;
        while let Some(node) = current_node {
            count += 1;
            current_node = &node.next;
        }
        return count;
    }
    // delete node
    pub fn delete(&mut self, data: i32) {
       let mut current = &mut self.head;
         while let Some(node) = current {
              if node.data == data {
                print!("Founded");
                // *current = node.next.take();
                break;
              }
              current = &mut node.next;
         }

    }
}
