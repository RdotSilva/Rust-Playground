pub fn add_five(num: u32) -> u32 {
    num + 5
}

// pub allows us to import the function in other areas
pub fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

#[cfg(test)]
mod test {
    // Inherit all modules that are imported
    use super::*;

    #[test]
    fn add_five_test() {
        let x: u32 = 100;
        let y: u32 = add_five(x);

        println!("x and y are from test: {} {}", x, y);
        assert_eq!(y, 105);
    }
}
