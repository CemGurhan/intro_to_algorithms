fn main() {
    let mut array1 = [4,1,5,3,7,9];
    let mut array2 = [2,3,4,1,0,-3,10];
    insertion_sort(&mut array1);
    insertion_sort(&mut array2);
    non_increasing_insertion_sort(&mut array1);
    non_increasing_insertion_sort(&mut array2);
}

// insertion sort sorts an array in ascending order
fn insertion_sort(array: &mut [i32]) {
    for i in 1..array.len() {
        for j in 0..i {
            if array[j] > array[i] {
                array.swap(j, i);
            }
        }
    }
    println!("sorted array: {:?}", array);
}

fn non_increasing_insertion_sort(array: &mut [i32]) {
    for i in 1..array.len() {
        for j in 0..i {
            if array[j] < array[i] {
                array.swap(i, j);
            }
        }
    }
    println!("non-increasing sorted array: {:?}", array);
}


