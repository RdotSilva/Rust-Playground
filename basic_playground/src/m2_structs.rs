#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        // Create a new user
        let user_1: User = User {
            username: String::from("Someone"),
            email: String::from("someone@example.com"),
            active: true,
            sign_in_count: 1,
        };

        dbg!(user_1);
    }
}
