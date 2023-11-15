#[derive(Debug)]
#[allow(dead_code)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver,
}

fn create_car_colour_blue() -> CarColour {
    let my_car_colour: CarColour = CarColour::Blue;
    my_car_colour
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour: CarColour = create_car_colour_blue();
        dbg!(car_colour);
    }
}
