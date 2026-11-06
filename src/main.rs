use std::collections::HashMap;


fn median_values(num: &mut Vec<i32>) -> Option<f64> {
    if num.is_empty() {
        return None;
    }

    num.sort();

    let mid = num.len() / 2;

    if num.len() % 2 == 0 {
        let left = num[mid - 1];
        let right = num[mid];
        Some((left + right) as f64 / 2.0)
    } else {
        Some(num[mid] as f64)
    }
}


fn mode(num: &Vec<i32>) -> Option<i32> {
    if num.is_empty() {
        return None;
    }

    let mut map = HashMap::new(); // Needs `mut` to modify
    for &number in num {          // Iterate over references
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    
    let mut max_count = 0;
    let mut mode_val = None;

    for (key, count) in map {
        if count > max_count {
            max_count = count;
            mode_val = Some(key);
        }
    }

    mode_val
}

fn main() {
    let mut list1: Vec<i32> = vec![32, 1, 33, 92];
    let mut list2 = vec![7, 11, 90, 25];

    // 4. Call functions separately to avoid borrow checker conflicts
    let med1 = median_values(&mut list1);
    let mod1 = mode(&list1);

    let med2 = median_values(&mut list2);
    let mod2 = mode(&list2);

    // Use {:?} for Options, because we don't know if they are Some or None
    println!("List 1 -> median: {:?}, mode: {:?}", med1, mod1);
    println!("List 2 -> median: {:?}, mode: {:?}", med2, mod2);
}