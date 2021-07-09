mod linked_list;

use crate::linked_list::LinkedList;

fn main() {
    let mut ll = LinkedList::<i64>::new();
    ll.insert(1);
    ll.insert(2);
    ll.insert(3);
    ll.insert(4);
    ll.insert(5);
    ll.insert(6);
    ll.insert(7);
    ll.insert(8);
    ll.print();
    println!("Linked List Length {}", ll.length);
    
}


// TODO: Implement Delete
// TODO: Implement Contains
// TODO: Implement Reverse
// TODO: Implement Clear
