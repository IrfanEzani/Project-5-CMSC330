/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    if n < 0 {
        return -1;
    }
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    let mut count = 0;
    for &num in ls {
        if num >= s && num <= e {
            count += 1;
        }
    }
    count
}


/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    target.iter().all(|t| set.contains(t))
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    if ls.is_empty() {
        return None;
    }

    let sum:f64 = ls.iter().sum();
    let mean = sum / (ls.len() as f64);
    Some(mean)
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    ls.iter().fold(0, |acc, x| acc * 2 + x)
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut factors = vec![];
    let mut num = n;

    for factor in 2..=n {
        while num % factor == 0 {
            factors.push(factor);
            num /= factor;
        }
        if num == 1 {
            break;
        }
    }

    factors
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    let mut result = vec![];

    for i in 1..=lst.len() {
        result.push(lst[i % lst.len()]);
    }

    result
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {

    let s_chars: Vec<char> = s.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let s_len = s_chars.len();
    let target_len = target_chars.len();
    if target_len == 0 {
        return true;
    }
    if s_len < target_len {
        return false;
    }
    for i in 0..s_len {
        if i + target_len > s_len {
            break;
        }
        let mut found = true;
        for j in 0..target_len {
            if s_chars[i + j] != target_chars[j] {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }
    false
}   



/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
    if s.is_empty() {
        return None;
    }
    let mut start = 0;
    let mut end = 1;

    let mut max = 1;
    let mut curr = 1;
    
    let mut curr_char = s.chars().next().unwrap();


    for (i, c) in s.chars().enumerate().skip(1) {
        if c == curr_char {
            curr += 1;
        } else {
            if curr > max {
                max = curr;
                start = i - curr;
                end = i;
            }
            curr = 1;
            curr_char = c;
        }
    }
    if curr > max {
        start = s.len() - curr;
        end = s.len();
    }
    Some(&s[start..end])
}

