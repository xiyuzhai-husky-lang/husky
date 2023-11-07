
pub fn quick_sort<T>(mut arr: Slice< HirTypeSymbolTodo >) {
    let len = v0.len();
    quick_sort_aux(v0, 0, v1 - 1 as isize)
}

pub fn quick_sort_aux<T>(mut arr: Slice< HirTypeSymbolTodo >, low: isize, high: isize) {
    if v1 < v2 {
        let p = partition(v0, v1, v2);
        quick_sort_aux(v0, v1, v3 - 1);
        quick_sort_aux(v0, v3 + 1, v2)
    }
}

pub fn partition<T>(mut arr: Slice< HirTypeSymbolTodo >, low: isize, high: isize) -> isize {
    let pivot = v2 as usize;
    let mut store_index = v1 - 1;
    let mut last_index = v2;
    while true {
        v4 += 1;
        while v0[v4 as usize] < v0[v3] {
            v4 += 1
        }
        v5 -= 1;
        while v5 >= 0 && v0[v5 as usize] > v0[v3] {
            v5 -= 1
        }
        if v4 >= v5 {
            break;
        } else {
            v0.swap(v4 as usize, v5 as usize)
        }
    }
    v0.swap(v4 as usize, v3 as usize);
    v4
}

pub fn quick_sort_works_for_integers() {
    let mut v: Vec<i32> = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
}

pub fn quick_sort_works_for_strs() {
    let mut strs = vec!["beach", "hotel", "airplane", "car", "house", "art"];
}