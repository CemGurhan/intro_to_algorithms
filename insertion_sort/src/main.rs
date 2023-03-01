fn main() {
    let mut array = [4,1,5,3,7,9];
    insertion_sort(&mut array)
}

// insertion sort sorts an array in ascending order
fn insertion_sort(array: &mut [i32]) {
    for j in 1..array.len() {
        let mut key = array[j];
        let mut i = j - 1;

        while i > 0 && array[i] > key {
            array[i + 1] = array[i];
            i = i - 1;
        }
        array[i] = key;
    }
    println!("Sorted array: {:?}", array);
}
