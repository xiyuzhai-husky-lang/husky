
pub fn quick_sort< T>() {
    let len = arr.len();
    quick_sort_aux( arr, 0, len-1as isize);
}

pub fn quick_sort_aux< T>(, , ) {
    if low< high {
        let p = partition( arr, low, high);
        quick_sort_aux( arr, low, p-1);
        quick_sort_aux( arr, p+1, high);
    }
}

pub fn partition< T>(, , ) {
    let pivot = highas usize;
    let store_index = low-1;
    let last_index = high; whiletrue {
        store_index+=1; while arr[ store_indexas usize]< arr[ pivot] {
            store_index+=1;
        }
        last_index-=1; while last_index>=0&& arr[ last_indexas usize]> arr[ pivot] {
            last_index-=1;
        }
        if store_index>= last_index {
            break;
        } else {
            arr.swap( store_indexas usize, last_indexas usize);
        }
    }
    arr.swap( store_indexas usize, pivotas usize);
    store_index;
}

pub fn quick_sort_works_for_integers() {
    let v : Vec< i32> = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
}

pub fn quick_sort_works_for_strs() {
    let strs = vec!["beach", "hotel", "airplane", "car", "house", "art"];
}