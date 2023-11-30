// MACRO CAPTURES

/* expr
    matches to a valid rust expression
    "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
*/

/* stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>
*/

/* path
    matches to a rust path
    std::collections::HashMap
*/

// Repetition SPECIFIER

// * - match zero or more repetitions
// + - match one or more repetitions
// ? - Match zero or one repetitions

#[cfg(test)]
mod tests {

    macro_rules! mad_skills {
        ($x: expr) => {
            format!("You sent an expression: {}", $x)
        };
    }

    #[test]
    fn tests_declarative_macro() {
        let mut _x: Vec<i32> = vec![];
        dbg!(_x);

        let formatted: String = format!("Hello with vec {:?}", _x);

        let some_var: String = mad_skills!(1 + 2);

        dbg!(some_var);

        dbg!(formatted);
    }
}
