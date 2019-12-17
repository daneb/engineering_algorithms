fn main() {
    let mut a : [i32; 8] = [14, 33, 27, 10, 35, 19, 42, 44];
    let result : Vec<i32> = mergesort(&mut a);
    println!("{:?}", result);
}

fn mergesort(array_to_sort : &mut [i32]) -> Vec<i32>
{
    if array_to_sort.len() == 1 { return array_to_sort.to_vec() };

    let (left, right) = array_to_sort.split_at_mut(array_to_sort.len() / 2); 
     
    let part1 = mergesort(left);
    let part2 = mergesort(right);

    return merge(&mut part1.to_vec(), &mut part2.to_vec());

}

fn merge(left : &mut Vec<i32>, right : &mut Vec<i32>)  -> Vec<i32>
{
    let capacity : usize = left.len() + right.len();
    let mut v : std::vec::Vec<i32> = Vec::with_capacity(capacity);

    while left.len() > 0 && right.len() > 0 {
        if left[0] > right[0] {
            v.push(right[0]);
            right.remove(0);
        }
        else {
            v.push(left[0]);
            left.remove(0);
        }
    }

    while left.len() > 0 {
        v.push(left[0]);
        left.remove(0);
    }

    while right.len() > 0 {
        v.push(right[0]);
        right.remove(0);
    }

    return v;
}