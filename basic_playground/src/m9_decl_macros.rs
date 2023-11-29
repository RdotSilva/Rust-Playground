#[cfg(test)]
mod tests {

    #[test]
    fn tests_declarative_macro() {
        let mut _x: Vec<i32> = vec![];
        dbg!(_x);

        let formatted: String = format!("Hello with vec {:?}", _x);

        dbg!(formatted);
    }
}
