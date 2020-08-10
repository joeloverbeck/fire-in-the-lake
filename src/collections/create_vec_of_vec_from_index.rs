pub fn create_vec_of_vec_from_index<T: Copy + std::fmt::Debug>(
    original_vec: &[T],
    from_index_inclusive: usize,
) -> Result<Vec<T>, String> {
    // Sanity check
    if from_index_inclusive >= original_vec.len() {
        panic!(format!("Demanded to create a vector from an original vector from the index {:?}, but that index was larger than the len of the original vector. Original vector: {:?}", from_index_inclusive, original_vec));
    }

    let mut vector: Vec<T> = Vec::new();

    for item in original_vec
        .iter()
        .take(original_vec.len() - 1)
        .skip(from_index_inclusive)
    {
        vector.push(*item);
    }

    Ok(vector)
}
