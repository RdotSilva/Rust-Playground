#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        // Create a new user
        let mut user_1: User = User {
            username: String::from("Someone"),
            email: String::from("someone@example.com"),
            active: true,
            sign_in_count: 1,
        };

        change_username(&mut user_1, "somenewusername");

        dbg!(user_1);
    }
}
