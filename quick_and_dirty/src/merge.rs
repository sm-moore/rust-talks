
fn append<T: PartialOrd + Clone>(from: Vec<T>, mut from_idx: usize, to:  &mut Vec<T>) {
    while from_idx <= from.len() - 1 {
        to.push(from[from_idx].clone());
        from_idx += 1;
    }
}


fn merge<T: PartialOrd + Clone>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut new: Vec<T> = Vec::new();
    let mut left_idx = 0;
    let mut right_idx = 0;

    while new.len() < left.len() + right.len()  {
        while left[left_idx] <= right[right_idx]  {
            new.push(left[left_idx].clone());
            left_idx += 1;
            if left_idx > left.len() - 1 {
                append(right, right_idx, & mut new);
                return new
            }
        }
        while right[right_idx] <= left[left_idx] {
            new.push(right[right_idx].clone()); // Point out explicit clone here vs copy
            right_idx += 1;
            if right_idx > right.len() - 1 {
                append(left, left_idx, & mut new);
                return new
            }
        }
    }
    new
}

pub fn merge_sort<T: PartialOrd + Clone>(list: Vec<T>, _unused_length: usize) -> Vec<T> {
   return merge_sort_recur(list);
}

fn merge_sort_recur<T: PartialOrd + Clone>(list: Vec<T>) -> Vec<T> {
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