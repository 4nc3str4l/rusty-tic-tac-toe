use std::rc::Rc;


pub struct Node<T> {
    value: T,
    next: Rc<Option<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self{value: val, next: Rc::new(None)}
    }
}

pub struct LinkedList<T> {
    head: Rc<Option<Node<T>>>,
    pub length: usize,
}

impl<T: std::fmt::Display> LinkedList<T> {
    
    pub fn new() -> Self {
        Self {
            head: Rc::new(None),
            length: 0,
        }
    }

    pub fn insert(&mut self, data: T) {
        let mut new_node = Node::new(data);
        match self.head.as_ref() {
            Some(_) => {
                new_node.next = self.head.clone();
                self.head = Rc::new(Some(new_node));

            },
            None => {
                self.head = Rc::new(Some(new_node))
            },
        };
        self.length += 1;
    }

    pub fn pop(&mut self, idx: Option<usize>) -> Option<T> {
        let idxp = idx.unwrap_or(0);
        if self.length == 0 || idxp > self.length {
            return None;
        }

        let mut curr = Rc::clone(&self.head);
        let mut last = None;
        let mut i = 0;
        
        // Move the head to the next and just return the last head value
        if i == 0 {
            self.head = (curr.clone().as_ref().unwrap().next).clone();
            self.length -= 1;
            return Some(curr.unwrap().value);
        }

        while i < idxp {
            last = Some(curr);
            curr = curr.unwrap().next;
            i+=1;
        }

        // Finish the delete algorithm now that we have
        // the pointer located at the right spot of the list

        let val: T = curr.unwrap().value;
        Some(val)
    }

    pub fn print(&mut self) {
        let mut out = String::new();
        let mut curr = &self.head;
        let mut first = true;
        while curr.is_some(){
            if first {
                out.push_str(format!("{}", curr.as_ref().unwrap().value).as_str());  
                first = false;
            }else{
                out.push_str(format!(" -> {}", curr.as_ref().unwrap().value).as_str());
            }
            curr = &curr.as_ref().unwrap().next;
        }
        println!("{}", out);
    }
}
