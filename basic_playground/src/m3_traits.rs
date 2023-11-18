#[cfg(test)]
mod test {

    // Define a trait that describes an attacker and allows us to choose a fighting style
    trait Attacker {
        fn choose_style(&self) -> String;
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Character {
        Warrior,
        Archer,
        Wizard,
    }

    // Pick what type of style we are using based on the type of character
    impl Attacker for Character {
        fn choose_style(&self) -> String {
            // This is like a switch statement
            match self {
                Character::Warrior => "wing chun".to_string(),
                Character::Archer => "kung fu".to_string(),
                Character::Wizard => "thai chi".to_string(),
            }
        }
    }

    #[test]
    fn tests_traits() {
        let my_character: Character = Character::Archer;
        let chosen_fighting_style: String = my_character.choose_style();
        dbg!(chosen_fighting_style);
    }
}
