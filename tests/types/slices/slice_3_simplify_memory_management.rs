fn largest_num_caller() {
    // Define an array of 10 u8 numbers
    let num_list: [u8; 10] = [43, 97, 12, 55, 28, 84, 39, 62, 76, 19];

    // Call the largest_num function passing a reference to the array
    let largest = largest_num(&num_list);

    // Print the largest number found
    println!("The largest number is: {}", largest);
}

// Define a function to find the largest number in an array
fn largest_num(nums: &[u8; 10]) -> &u8 {
    // Initialize variables to track the index of the largest number and the current highest number
    let mut start_index: usize = 0;
    let mut current_highest: u8 = 0;

    // Iterate through the array elements using an iterator and enumerate
    for (index, &item) in nums.iter().enumerate() {
        // Compare each element with the current highest number
        if current_highest < item {
            // If the current element is larger, update the current highest number and its index
            current_highest = item;
            start_index = index;
        }
    }

    // Return a reference to the slice containing the largest number (slice of length 1)
    return &nums[start_index..start_index + 1][0];
}
// In our example, we delve into arrays, iterating through elements to find the largest number. By returning a
// reference to the slice containing the largest number, we maintain efficiency without consuming additional resources,
// thanks to Rust's memory management mechanisms.
