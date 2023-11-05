pub fn add_five(num: u32) -> u32 {
    num + 5
}

// pub allows us to import the function in other areas
pub fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}
