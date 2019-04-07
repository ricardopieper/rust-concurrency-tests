use std::collections::VecDeque;
use std::mem;
use std::sync::{Arc, Condvar, Mutex};


pub struct LinkedSet<T> where T: Eq,
{
    root: Option<Box<LinkedSetNode<T>>>,
}

//O(n) lookups
struct LinkedSetNode<T> where T: Eq,
{
    item: T,
    next: Option<Box<LinkedSetNode<T>>>,
}

impl<T> LinkedSet<T> where T: Eq,
{
    pub fn new() -> LinkedSet<T> {
        LinkedSet { root: None }
    }
    
    pub fn add(&mut self, item: T) {
        let root = &mut self.root;
        if let Some(node) = root {
            if node.item == item {
                return;
            }
        }
        match root {
            Some(node) => {
                let mut current = node;
                while let Some(ref mut next_node) = current.next {
                    if next_node.item == item {
                        return;
                    }
                    current = next_node;
                }
                current.next = Some(Box::new(LinkedSetNode {
                    item: item,
                    next: None,
                }));
            }
            None => {
                let new_node = LinkedSetNode {
                    item: item,
                    next: None,
                };
                //substitute the None in root by Some(item)
                mem::replace(&mut self.root, Some(Box::new(new_node)));
            }
        }
    }
    
    pub fn len(&self) -> i32 {
        let root = &self.root;
        match root {
            Some(node) => {
                let mut count = 1;
                let mut current = node;
                while let Some(ref next_node) = current.next {
                    count = count + 1;
                    current = next_node;
                }
                return count;
            }
            None => 0,
        }
    }
    
    pub fn remove(&mut self, item: T) -> Option<T> {

        match self.root {
            Some(ref mut root_node) => {                
                //corner_case: root should be removed
                if root_node.item == item {
                    let next_item = mem::replace(&mut root_node.next, None); //get ownership of the next item
                    let removed = mem::replace(&mut self.root, next_item); //move root out of the set
                    return Some(removed.unwrap().item); //and give it to the caller
                }
                //find a prev node whose next node is the item
                //starting with the root, analyze 1st -> 2nd, then 2nd -> 3rd, etc
                let mut prev = &mut self.root;

                //If the node is not null (guaranteed to be true in the first iteration)
                while let Some(ref mut prev_node) = prev {
                    //temporarily replace current with something else to obtain ownership
                    //since there is a possibility that it be dropped
                    let current = mem::replace(&mut prev_node.next, None);

                    //If there exists a next node and it is the item,
                    if let Some(current_node) = current {
                        if current_node.item == item {

                            //found the prev, cur is target
                            //link prev to cur.next
                            prev_node.next = current_node.next;
                            return Some(current_node.item);
                        } else {
                            //restore the value
                            mem::replace(&mut prev_node.next, Some(current_node));
                        }
                    } else {
                        //restore the value
                        mem::replace(&mut prev_node.next, current);
                    }

                    prev = &mut prev_node.next;
                }
                None
            }
            None => return None,
        }
    }
}
