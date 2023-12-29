use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::panic;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
    **/
    fn enqueue(&mut self, ele: T) -> () {
        self.push(ele);
        let mut current = self.len() - 1;
        while current > 0 && self[current] < self[(current - 1) / 2] {
            self.swap(current, (current - 1) / 2);
            current = (current - 1) / 2;
        }
    }

    /**
        Function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  
        Return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let res = Some(self.swap_remove(0));
        let mut current = 0;
        while 2 * current + 1 < self.len() {
            let mut child = 2 * current + 1;
            if child + 1 < self.len() && self[child + 1] < self[child] {
                child += 1;
            }
            if self[current] <= self[child] {
                break;
            }
            self.swap(current, child);
            current = child;
        }
        res
    }

    /**
        Function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self[0])  
        }
    }
}


/**
    Function computes the orthogonal distance between two coordinates.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    (x1 - x2).abs() + (y1 - y2).abs()
}

/**
    Function that determines the closest match between elements of two groups.
    Returns a specific element's match and its coordinates.
    Given two hashmaps, `group1` and `group2`, each mapping a name to their current coordinates.
    Can assume that `group1` will always have a specific element (e.g., "Stark"). 
    Return the name and coordinates of the matched element from `group2`
     for the specified element in `group1` in a 3-tuple.
**/
pub fn target_locator<'a>
    (group1: &'a HashMap<&String, (i32,i32)>, 
    group2: &'a HashMap<&String, (i32,i32)>, 
    specific_element: &str) -> (&'a str, i32, i32) {
    
    let mut heap = vec![];
    for (g1_name, g1_loc) in group1 {
        for (g2_name, g2_loc) in group2 {
            let dist = distance(*g1_loc, *g2_loc);
            heap.enqueue(Node{ priority: dist, data: (g1_name, g2_name) });
        }
    }
    
    let mut g1_seen = HashSet::new();
    let mut g2_seen = HashSet::new();

    let mut matches = HashMap::new();

    for node in heap.iter() {
        let names = node.data;
        let member_g1 = *names.0;
        let member_g2 = *names.1;

        if !g1_seen.contains(member_g1) && !g2_seen.contains(member_g2) {
            g1_seen.insert(member_g1);
            g2_seen.insert(member_g2);
            matches.insert(member_g1, member_g2);

            if g1_seen.len() == group1.len() || g2_seen.len() == group2.len() {
                break;
            }
        }
    }

    match matches.get(&String::from(specific_element)) {
        Some(matched_member) => { 
            let (x, y) = group2.get(matched_member).unwrap(); 
            (*matched_member, *x, *y) 
        }
        None => panic!("Specified element not found in group1")
    }
}





