#[cfg(test)]
mod test {
    use super::*;

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() >= y.len() {
            x
        } else {
            y
        }
    }

    #[test]
    fn lifetimes_with_equal_scope() {
        let s1 = String::from("abcd");
        let s2 = String::from("xyz");
        let result = longest(s1.as_str(), s2.as_str());
        println!("{}", result);
    }

    #[test]
    fn lifetimes_with_unequal_scope() {
        let s1 = String::from("abcd");
        {
            let s2 = String::from("xyz");
            let result = longest(s1.as_str(), s2.as_str());
            println!("{}", result);
        }
    }

    #[test]
    fn longest_with_the_works() {
        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(
            x: &'a str,
            y: &'a str,
            ann: T,
        ) -> &'a str
            where
                T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let s1 = "hey";
        let s2 = "you";
        let it = longest_with_an_announcement(s1, s2, "tada!");
        println!("Boom {}", it);

    }
}