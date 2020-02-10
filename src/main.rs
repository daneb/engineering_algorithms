// https://en.wikipedia.org/wiki/Quicksort#Hoare_partition_scheme
// https://itnext.io/a-sort-of-quick-guide-to-quicksort-and-hoares-partitioning-scheme-in-javascript-7792112c6d1
// In place = An algorithm in which the data that is being operated on does not leave its data structure.
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    //let mut arr : Vec<i32> = read_file_to_sort().unwrap().into();
    let mut arr : Vec<i32> =vec![ 3, 8, 2, 5, 1, 4, 7, 6 ];
    let left = 0;
    let right = arr.len() - 1;
    quicksort(&mut arr, left, right as i32);
}

fn quicksort(arr : &mut Vec<i32>, left : i32, right : i32) {

    if left < right {

        let pi = partition(arr, left, right);
    
        quicksort(arr, left, pi);
        quicksort(arr, pi + 1, right);
    }

}

// Two boundaries:
// j = what we looked at so far, and what we haven't looked at
// i = from what we've seen, where are those left that are less then the pivot,
//     and those to the right greater then the pivot
// ~ pursuing linear time
fn partition(arr : &mut Vec<i32>, left : i32, right : i32) -> i32 {

   let pivot : i32 = arr[left as usize];
   let mut i : i32 = left + 1;

   for j in (left + 1)..right {
       if arr[j as usize] < pivot {
        arr.swap(j as usize, i as usize);
        i = i + 1;
       }
   }

   arr.swap(left as usize, (i - 1) as usize);

   return i;
}

fn read_file_to_sort() -> Result<Vec<i32>, io::Error> {
    let mut f = File::open("QuickSort.txt")?;
    let mut buffer = Vec::new();

    match f.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer.into_iter().map(|value| value as i32).collect()),
        Err(e) => Err(e),
    }
}