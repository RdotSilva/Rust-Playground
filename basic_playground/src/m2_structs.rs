#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Implement some functionality for a type
// https://doc.rust-lang.org/std/keyword.impl.html
impl User {
    fn increment_signin_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
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

        let mut user_2: User = User {
            username: String::from("someusername2"),
            email: String::from("someone2@example2.com"),
            active: false,
            sign_in_count: 7,
        };

        user_2.increment_signin_count();

        user_2.change_email("anotheremail@email.com");

        dbg!(user_2);
    }
}
