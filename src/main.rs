// https://en.wikipedia.org/wiki/Quicksort#Hoare_partition_scheme
// https://itnext.io/a-sort-of-quick-guide-to-quicksort-and-hoares-partitioning-scheme-in-javascript-7792112c6d1
// In place = An algorithm in which the data that is being operated on does not leave its data structure.

fn main() {
    let mut arr : Vec<i32> = vec![1,3,5,2,7,8,4];
    let left = 0;
    let right = arr.len() - 1;
    let result = quicksort(&mut arr, left, right as i32);
    println!("{:?}", result);
}

fn quicksort(arr : &mut Vec<i32>, left : i32, right : i32) -> &Vec<i32> {
    if left >= right { return arr };

    let pivot = arr[((left + right)/2) as usize];
    let index = partition(arr, left, right, pivot);

    quicksort(arr, left, index - 1);
    quicksort(arr, index, right);

    return arr;

}

fn partition(arr : &mut Vec<i32>, mut left : i32, mut right : i32, pivot : i32) -> i32  {
    while left <= right {
        while arr[left as usize] < pivot && left <= right {
            left = left + 1;
        }
        while arr[right as usize] > pivot {
            right = right - 1;
        }

        if left <= right {
            arr.swap(left as usize, right as usize);
            left = left + 1;
            right = right - 1;
        }
    }
    return left;
}