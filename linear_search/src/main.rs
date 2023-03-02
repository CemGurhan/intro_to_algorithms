fn main() {
    let mut array: [i32; 5] = [3,55,-10,2,4];
    linear_search(&mut array, -10);
    linear_search(&mut array, 231);
}

fn linear_search(array: &mut [i32], search_value: i32)  {
    for i in 0..array.len() {
        if array[i] == search_value {
            println!("linear search found search value {} at index: {}", search_value, i);
            return
        }
    }

    println!("search value {} not present in array", search_value);
}
