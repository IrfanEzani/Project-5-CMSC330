use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::panic;

pub trait HeapQueue<T: PartialOrd> {
    fn enqueue(&mut self, element: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

struct HeapNode<T> {
    priority_level: i32,
    content: T,
}

/** 
    Implement comparison traits for HeapNode to enable comparison based on priority_level 
**/
impl<T> PartialOrd for HeapNode<T> {
    fn partial_cmp(&self, other: &HeapNode<T>) -> Option<Ordering> {
        self.priority_level.partial_cmp(&other.priority_level)
    }
}

impl<T> PartialEq for HeapNode<T> {
    fn eq(&self, other: &HeapNode<T>) -> bool {
        self.priority_level == other.priority_level
    }
}

impl<T: PartialOrd> HeapQueue<T> for Vec<T> {
    /** Adds an element to the heap queue and rearranges it to maintain the min heap property. **/
    fn enqueue(&mut self, element: T) -> () {
        self.push(element);
        let mut index = self.len() - 1;
        while index > 0 && self[index] < self[(index - 1) / 2] {
            self.swap(index, (index - 1) / 2);
            index = (index - 1) / 2;
        }
    }

    /**
        Removes and returns the top element from the heap queue, ensuring min heap property is maintained.
        Returns None if the queue is empty, otherwise returns the top element.
    **/
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let top_element = Some(self.swap_remove(0));
        let mut index = 0;
        while 2 * index + 1 < self.len() {
            let mut child_index = 2 * index + 1;
            if child_index + 1 < self.len() && self[child_index + 1] < self[child_index] {
                child_index += 1;
            }
            if self[index] <= self[child_index] {
                break;
            }
            self.swap(index, child_index);
            index = child_index;
        }
        top_element
    }

    /**
        Returns the top element of the heap queue without removing it.
        Returns None if the queue is empty, otherwise returns the top element.
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
    Calculates the orthogonal distance between two points represented as (x, y) coordinates.
**/
pub fn calculate_orthogonal_distance(point1: (i32,i32), point2: (i32,i32)) -> i32 {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
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
    let mut g1_seen = HashSet::new();
    let mut g2_seen = HashSet::new();
    let mut matches = HashMap::new();


    for (g1_name, g1_loc) in group1 {
        for (g2_name, g2_loc) in group2 {
            let dist = distance(*g1_loc, *g2_loc);
            heap.enqueue(Node{ priority: dist, data: (g1_name, g2_name) });
        }
    }
    
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
        None => panic!("Elem not found")
    }
}





