// https://en.wikipedia.org/wiki/Quicksort#Hoare_partition_scheme
// https://itnext.io/a-sort-of-quick-guide-to-quicksort-and-hoares-partitioning-scheme-in-javascript-7792112c6d1
// In place = An algorithm in which the data that is being operated on does not leave its data structure.
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};


static mut COMPARISONS: i32 = 0;

fn main() {
    let mut arr : Vec<i32> = read_file_to_sort();
    println!("{}", arr.len());
    //let mut arr : Vec<i32> =vec![ 3, 8, 2, 5, 1, 4, 7, 6 ];
    let left = 0;
    let right = arr.len() - 1;
    quicksort(&mut arr, left as i32, right as i32);
    unsafe { println!("{}", COMPARISONS); }
}

fn quicksort(arr : &mut Vec<i32>, left : i32, right : i32) {

    if left >= right { 
        return;
    }

    // Pick pivot element
    let pivot = arr[left as usize];

    // Update comparisons
    unsafe { COMPARISONS += right - left; }

    // Partition
    let mut i : i32 = left + 1;
    for j in (left + 1)..(right + 1) {
        if arr[j as usize] < pivot {
         arr.swap(i as usize, j as usize);
         i += 1;
        }
    }
    arr.swap(left as usize, (i - 1) as usize);

    // Recursive Calls
    quicksort(arr, left, i-2);
    quicksort(arr, i, right);

    // answers tried: 176404400 | 58893 | 176463294 | 1910516808 | 162085

}

fn read_file_to_sort() -> Vec<i32> {
    let file = File::open("QuickSort.txt").expect("no such file");
    let buf : Vec<i32> = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    return buf;
}