## `quick_sort` decl

```rust
Some(
    [
        "Ord",
        "[:]",
        "T",
        "[:]T",
    ],
)
```

## `quick_sort` defn

```rust
Some(
    [
        "arr",
        "arr.len()",
        "let len = arr.len()",
        "quick_sort_aux",
        "arr",
        "0",
        "len",
        "1",
        "len - 1",
        "(len - 1)",
        "isize",
        "(len - 1) as isize",
        "quick_sort_aux(arr, 0, (len - 1) as isize)",
        "quick_sort_aux(arr, 0, (len - 1) as isize)",
        "let len = arr.len()\n    quick_sort_aux(arr, 0, (len - 1) as isize)",
    ],
)
```

## `quick_sort_aux` decl

```rust
Some(
    [
        "Ord",
        "[:]",
        "T",
        "[:]T",
        "isize",
        "isize",
    ],
)
```

## `quick_sort_aux` defn

```rust
Some(
    [
        "low",
        "high",
        "low < high",
        "low < high",
        "partition",
        "arr",
        "low",
        "high",
        "partition(arr, low, high)",
        "let p = partition(arr, low, high)",
        "quick_sort_aux",
        "arr",
        "low",
        "p",
        "1",
        "p - 1",
        "quick_sort_aux(arr, low, p - 1)",
        "quick_sort_aux(arr, low, p - 1)",
        "quick_sort_aux",
        "arr",
        "p",
        "1",
        "p + 1",
        "high",
        "quick_sort_aux(arr, p + 1, high)",
        "quick_sort_aux(arr, p + 1, high)",
        "if low < high:\n        let p = partition(arr, low, high)\n        quick_sort_aux(arr, low, p - 1)\n        quick_sort_aux(arr, p + 1, high)",
        "if low < high:\n        let p = partition(arr, low, high)\n        quick_sort_aux(arr, low, p - 1)\n        quick_sort_aux(arr, p + 1, high)",
    ],
)
```

## `partition` decl

```rust
Some(
    [
        "Ord",
        "[:]",
        "T",
        "[:]T",
        "isize",
        "isize",
        "isize",
    ],
)
```

## `partition` defn

```rust
Some(
    [
        "high",
        "usize",
        "high as usize",
        "let pivot = high as usize",
        "low",
        "1",
        "low - 1",
        "let mut store_index = low - 1",
        "high",
        "let mut last_index = high",
        "true",
        "true",
        "store_index",
        "1",
        "store_index += 1",
        "store_index += 1",
        "arr",
        "store_index",
        "usize",
        "store_index as usize",
        "arr[store_index as usize]",
        "arr",
        "pivot",
        "arr[pivot]",
        "arr[store_index as usize] < arr[pivot]",
        "arr[store_index as usize] < arr[pivot]",
        "store_index",
        "1",
        "store_index += 1",
        "store_index += 1",
        "while arr[store_index as usize] < arr[pivot]:\n            store_index += 1",
        "last_index",
        "1",
        "last_index -= 1",
        "last_index -= 1",
        "last_index",
        "0",
        "last_index >= 0",
        "arr",
        "last_index",
        "usize",
        "last_index as usize",
        "arr[last_index as usize]",
        "arr",
        "pivot",
        "arr[pivot]",
        "arr[last_index as usize] > arr[pivot]",
        "last_index >= 0 && arr[last_index as usize] > arr[pivot]",
        "last_index >= 0 && arr[last_index as usize] > arr[pivot]",
        "last_index",
        "1",
        "last_index -= 1",
        "last_index -= 1",
        "while last_index >= 0 && arr[last_index as usize] > arr[pivot]:\n            last_index -= 1",
        "store_index",
        "last_index",
        "store_index >= last_index",
        "store_index >= last_index",
        "break",
        "arr",
        "store_index",
        "usize",
        "store_index as usize",
        "last_index",
        "usize",
        "last_index as usize",
        "arr.swap(store_index as usize, last_index as usize)",
        "arr.swap(store_index as usize, last_index as usize)",
        "if store_index >= last_index:\n            break\n        else:\n            arr.swap(store_index as usize, last_index as usize)",
        "while true:\n        store_index += 1\n        while arr[store_index as usize] < arr[pivot]:\n            store_index += 1\n        last_index -= 1\n        while last_index >= 0 && arr[last_index as usize] > arr[pivot]:\n            last_index -= 1\n        if store_index >= last_index:\n            break\n        else:\n            arr.swap(store_index as usize, last_index as usize)",
        "arr",
        "store_index",
        "usize",
        "store_index as usize",
        "pivot",
        "usize",
        "pivot as usize",
        "arr.swap(store_index as usize, pivot as usize)",
        "arr.swap(store_index as usize, pivot as usize)",
        "store_index",
        "store_index",
        "let pivot = high as usize\n    let mut store_index = low - 1\n    let mut last_index = high\n\n    while true:\n        store_index += 1\n        while arr[store_index as usize] < arr[pivot]:\n            store_index += 1\n        last_index -= 1\n        while last_index >= 0 && arr[last_index as usize] > arr[pivot]:\n            last_index -= 1\n        if store_index >= last_index:\n            break\n        else:\n            arr.swap(store_index as usize, last_index as usize)\n    arr.swap(store_index as usize, pivot as usize)\n    store_index",
    ],
)
```

## `quick_sort_works_for_integers` decl

```rust
Some(
    [],
)
```

## `quick_sort_works_for_integers` defn

```rust
Some(
    [
        "4",
        "65",
        "2",
        "31",
        "-31",
        "0",
        "99",
        "2",
        "83",
        "782",
        "1",
        "[4, 65, 2, -31, 0, 99, 2, 83, 782, 1]",
        "let mut v: []i32 = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1]",
        "let mut v: []i32 = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1]",
    ],
)
```

## ``quick_sort_works_for_integers`::#test` decl

```rust
Some(
    [],
)
```

## ``quick_sort_works_for_integers`::#test` defn

```rust
None
```

## `quick_sort_works_for_strs` decl

```rust
Some(
    [],
)
```

## `quick_sort_works_for_strs` defn

```rust
Some(
    [
        "\"beach\"",
        "\"hotel\"",
        "\"airplane\"",
        "\"car\"",
        "\"house\"",
        "\"art\"",
        "[\"beach\", \"hotel\", \"airplane\", \"car\", \"house\", \"art\"]",
        "let mut strs = [\"beach\", \"hotel\", \"airplane\", \"car\", \"house\", \"art\"]",
        "let mut strs = [\"beach\", \"hotel\", \"airplane\", \"car\", \"house\", \"art\"]",
    ],
)
```

## ``quick_sort_works_for_strs`::#test` decl

```rust
Some(
    [],
)
```

## ``quick_sort_works_for_strs`::#test` defn

```rust
None
```
