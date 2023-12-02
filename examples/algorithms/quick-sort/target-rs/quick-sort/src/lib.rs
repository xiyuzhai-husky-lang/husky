pub fn quick_sort<T>(ref mut arr: &mut [T]) {
    let len = arr.len();
    quick_sort_aux(arr, 0, len - 1 as isize)
}

pub fn quick_sort_aux<T>(ref mut arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        quick_sort_aux(arr, low, p - 1);
        quick_sort_aux(arr, p + 1, high)
    }
}

pub fn partition<T>(ref mut arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;
    while true {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize)
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}

pub fn quick_sort_works_for_integers() {
    let mut v: Vec<i32> = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
}

pub fn quick_sort_works_for_strs() {
    let mut strs = vec!["beach", "hotel", "airplane", "car", "house", "art"];
}