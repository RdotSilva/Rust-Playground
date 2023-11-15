#[derive(Debug)]
#[allow(dead_code)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver,
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

fn create_car_colour_blue() -> CarColour {
    let my_car_colour: CarColour = CarColour::Blue;
    my_car_colour
}

// Generics example
fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok((num_check))
    } else {
        GivenResult::Err(("Not under 5!".to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour: CarColour = create_car_colour_blue();
        dbg!(car_colour);

        let under_five_res = check_under_five(2);
        dbg!(under_five_res);

        let under_five_res = check_under_five(7);
        dbg!(under_five_res);
    }
}
