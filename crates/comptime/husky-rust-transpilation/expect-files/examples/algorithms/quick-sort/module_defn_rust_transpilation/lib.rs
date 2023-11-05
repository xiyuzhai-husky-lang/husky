pub!fn!quick_sort<T>{
    let!len=arr.len();
    quick_sort_aux(arr, 0, len-1asisize);
}
pub!fn!quick_sort_aux<T>{    if!low<high{
        let!p=partition(arr, low, high);
        quick_sort_aux(arr, low, p-1);
        quick_sort_aux(arr, p+1, high);
    }
;
}
pub!fn!partition<T>{
    let!pivot=highasusize;
    let!store_index=low-1;
    let!last_index=high;while!true{
        store_index+=1;while!arr[store_indexasusize]<arr[pivot]{
            store_index+=1;
        }
        last_index-=1;while!last_index>=0&&arr[last_indexasusize]>arr[pivot]{
            last_index-=1;
        }
        if!store_index>=last_index{
            break!;
        }
else!{
            arr.swap(store_indexasusize, last_indexasusize);
        }
;
    }
    arr.swap(store_indexasusize, pivotasusize);
    store_index;
}
pub!fn!quick_sort_works_for_integers{
    let!v:Vec<i32>=vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
}
pub!fn!quick_sort_works_for_strs{
    let!strs=vec!["beach", "hotel", "airplane", "car", "house", "art"];
}
