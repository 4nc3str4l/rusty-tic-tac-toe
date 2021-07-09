
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self{value: val, next: None}
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    pub length: usize,
}

impl<T: std::fmt::Display> LinkedList<T> {
    
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, data: T) {
        let mut new_node = Node::new(data);
        match &self.head {
            Some(_) => {
                new_node.next = self.head.take();
                self.head = Some(Box::new(new_node));

            },
            None => {
                self.head = Some(Box::new(new_node))
            },
        };
        self.length += 1;
    }

    pub fn pop(&mut self, idx: Option<usize>) -> Option<T> {
        let mut idxp = idx.unwrap_or(0);
        if self.length == 0 || idxp > self.length {
            return None;
        }
        let mut curr = &self.head;
        let last= None;
        let i = 0;
        while i < idxp {
            last = Some(curr);
            curr = &curr.as_ref().unwrap().next;
            i+=1;
        }
        // We have last
        // We have next
        // We don't have last or next
        if idxp == 0 {
            match &curr.as_ref().unwrap().next {
                Some(next) => {
                    self.head = Some(next.as_ref());
                },
                None => {
                    self.head = None;
                },
            }
        }

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
