// https://en.wikipedia.org/wiki/Quicksort#Hoare_partition_scheme
// https://itnext.io/a-sort-of-quick-guide-to-quicksort-and-hoares-partitioning-scheme-in-javascript-7792112c6d1
// In place = An algorithm in which the data that is being operated on does not leave its data structure.
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut arr : Vec<i32> = read_file_to_sort().unwrap().into();
    let left = 0;
    let right = arr.len() - 1;
    let result = quicksort(&mut arr, left, right as i32, &mut 0);
    println!("{:?}", result);
}

fn quicksort(arr : &mut Vec<i32>, left : i32, right : i32, compares : &mut i32) -> i32 {

    if left >= right { return *compares };

    let pivot = arr[((left + right)/2) as usize];
    let index = partition(arr, left, right, pivot);
    
    *compares = *compares + (index - 1);
    println!("a: {}", *compares);
    quicksort(arr, left, index - 1, compares);
    *compares = *compares + (right - 1);
    println!("b: {}", *compares);
    quicksort(arr, index, right, compares);

    return *compares;

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

fn read_file_to_sort() -> Result<Vec<i32>, io::Error> {
    let mut f = File::open("QuickSort.txt")?;
    let mut buffer = Vec::new();

    match f.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer.into_iter().map(|value| value as i32).collect()),
        Err(e) => Err(e),
    }
}