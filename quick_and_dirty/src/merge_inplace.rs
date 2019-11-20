fn merge (list: Vec<i32>, start_idx: usize, start_second_idx: usize, stop_idx: usize) -> Vec<i32> {
    let mut new: Vec<i32> = Vec::new();
    let mut current = start_idx;
    let mut second = start_second_idx;
    while current <= stop_idx {

        if current >= start_second_idx {
            //push all elements from start_second on.
            new.push(list[start_second_idx].clone());
            second += 1;
        }
        if second >= stop_idx {
            //push all elements from current on.
            new.push(list[current].clone());
            current += 1;
        }
        if list[current] >= list[start_second_idx]{
            //swap(list, current, start_second_idx);
            new.push(list[start_second_idx].clone());
            second += 1;
        }
        else {
            //swap(list, start_second_idx, current);
            new.push(list[current].clone());
            current += 1;
        }
    }
    new
}

pub fn merge_sort(list: Vec<i32>, list_length: usize) -> Vec<i32> {
   return merge_sort_recur(list, 0, list_length-1);
}

fn merge_sort_recur(list: Vec<i32>, start_idx: usize, stop_idx: usize) -> Vec<i32> {
    if stop_idx - start_idx <= 0 {
        return list;
    }
    let mid = start_idx + (stop_idx - start_idx)/2; // 0 + (3 - 0)/2 = 1 
    merge_sort_recur(list.clone(), start_idx, mid); //left 0 , 1
    merge_sort_recur(list.clone(), mid + 1, stop_idx); //right 2, 3
    merge(list, start_idx, mid + 1,  stop_idx)
}