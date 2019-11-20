
fn append(from: Vec<i32>, mut from_idx: usize, to:  &mut Vec<i32>) {
    while from_idx <= from.len() - 1 {
        to.push(from[from_idx]);
        from_idx += 1;
    }
}


fn merge (mut left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut new: Vec<i32> = Vec::new();
    let mut left_idx = 0;
    let mut right_idx = 0;
    left.push(5);
    while new.len() < left.len() + right.len()  {
        while left[left_idx] <= right[right_idx]  {
            new.push(left[left_idx]);
            left_idx += 1;
            if left_idx > left.len() - 1 {
                append(right, right_idx, & mut new);
                return new
            }
        }
        while right[right_idx] <= left[left_idx] {
            new.push(right[right_idx]);
            right_idx += 1;
            if right_idx > right.len() - 1 {
                append(left, left_idx, & mut new);
                return new
            }
        }
    }
    new
}

pub fn merge_sort(list: Vec<i32>, _unused_length: usize) -> Vec<i32> {
   return merge_sort_recur(list);
}

fn merge_sort_recur(list: Vec<i32>) -> Vec<i32> {
    if list.len() == 1 {
        return list;
    }
    let mid = (list.len()-1)/2 + 1;
    // Print me here....
    let left = merge_sort_recur(list.clone()[0..mid].to_vec()); //left 
    let right = merge_sort_recur(list.clone()[mid..list.len()].to_vec()); //right 
    
    merge(left, right)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_merge_lst() {
        let left = vec![1, 3];
        let right = vec![2];
        // Private functions can be tested too!
        assert_eq!(merge(left, right), vec![1, 2, 3]);
    }
    
    #[test]
    fn test_merge_sort() {
        let list = vec![1, 3, 2, 5, 8, 7, 0, 4, 6, 9];
        assert_eq!(merge_sort(list, 5), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}


// println!("left: {:?}", list.clone()[0..mid].to_vec());
// println!("right: {:?}", list.clone()[mid..list.len()].to_vec());