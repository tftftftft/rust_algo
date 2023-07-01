fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quicksort(&mut arr[0..pivot]);
    quicksort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    // Select the last element as the pivot
    let pivot_index = arr.len() - 1;
    
    // Initialize the partition index
    let mut i = 0;
    
    // Iterate over the elements from the beginning until the second-to-last element
    for j in 0..pivot_index {
        // If the current element is less than or equal to the pivot
        if arr[j] <= arr[pivot_index] {
            // Swap the current element with the element at the partition index
            arr.swap(i, j);
            
            // Move the partition index forward
            i += 1;
        }
    }
    
    // Swap the pivot element with the element at the partition index
    arr.swap(i, pivot_index);
    
    // Print the partition index (for visualization, can be removed)
    println!("{i}");
    
    // Return the partition index
    i
}

fn main() {
    let mut numbers = vec![4, 2, 8, 6, 1, 9, 5, 7, 3];
    println!("Before sorting: {:?}", numbers);
    quicksort(&mut numbers);
    println!("After sorting: {:?}", numbers);
}
