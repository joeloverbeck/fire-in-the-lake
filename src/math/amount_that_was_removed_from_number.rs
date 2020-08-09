pub fn amount_that_was_removed_from_number(attempt_of_removal: Result<u8, String>) -> u8 {
    let mut amount_removed_mut: u8 = 0;

    if let Ok(amount_removed) = attempt_of_removal {
        amount_removed_mut = amount_removed;
    }

    amount_removed_mut
}
