#[cfg(test)]
mod test {

    #[test]
    fn tests_match_literals() {
        let number: i32 = 20;

        let res: &str = match number {
            1 => "It was the first!",
            2 | 3 | 5 | 7 | 15 | 20 => "We found it in the sequence!",
            _ => "It was something else!",
        };

        println!("{}", res);
    }

    #[test]
    fn tests_match_option() {
        let some_num: Option<i32> = Some(10);

        let my_int: i32 = if let Some(i) = some_num {
            i
        } else {
            panic!("There was a problem");
        };

        println!("My int: {}", my_int);

        // let res = match some_num {
        //   Some(i) => i,
        //   None => {
        //     panic!("There was a problem");
        //   }
        // };

        // println!("{:?}", some_num);
        // println!("{}", res);
    }

    #[test]
    fn tests_match_result() {
        let some_res: Result<i32, &str> = Ok(50);
        let _some_err: Result<i32, &str> = Err("There was a problem");

        let res = match some_res {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };

        println!("{}", res);

        let my_int: i32 = if let Ok(r) = some_res {
            r
        } else {
            panic!("There was a problem");
        };

        println!("{}", my_int);
    }

    #[test]
    fn tests_match_guard() {
        let pair: (i32, i32) = (2, -2);
        match pair {
            (x, y) if x == y => println!("They match!"),
            (x, y) if x + y == 0 => println!("They neutralize"),
            (_, y) if y == 2 => println!("Y is indeed +2"),
            _ => println!("We are no bothered"),
        };
    }
}
