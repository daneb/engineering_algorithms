fn merge_sort(array_to_sort : &mut [i32]) -> (Vec<i32>, i64) {

    if array_to_sort.len() == 1 { return (array_to_sort.to_vec(), 0) };
    
    let (left, right) = array_to_sort.split_at_mut(array_to_sort.len() / 2);

    let (part1, count1) = merge_sort(left);
    let (part2, count2) = merge_sort(right);

    println!("{:?}", part1);
    println!("{:?}", part2);

    println!("{:?}", count1);
    println!("{:?}", count2);

    let inversion = count1 + count2;

    let (result, count) = merge(&mut part1.to_vec(), &mut part2.to_vec());

    println!("{}", inversion);
    println!("{}", count);

    return (result, inversion + count);

}

fn merge(left : &mut Vec<i32>, right : &mut Vec<i32>)  -> (Vec<i32>, i64)
{
    let capacity : usize = left.len() + right.len();
    let mut v : std::vec::Vec<i32> = Vec::with_capacity(capacity);
    let (mut left_idx ,mut right_idx) = (0, 0);
    let mut inversion_count : i64 = 0;

    while left_idx < left.len() && right_idx < right.len()
    {
        if left[left_idx] <= right[right_idx] {
            v.push(left[left_idx]);
            left_idx += 1;
        } else {
            v.push(right[right_idx]);
            right_idx += 1;
            // The distance of the swap 
            // Merge sort does "split" inversions
            // This is more along the lines of insertion sort.
            inversion_count += (left.len() as i64) - (left_idx as i64);
        }

    } 

    while left_idx < left.len() {
        v.push(left[left_idx]);
        left_idx += 1;
    }

    while right_idx < right.len() {
        v.push(right[right_idx]);
        right_idx += 1;
    };

    return (v, inversion_count);
    
}