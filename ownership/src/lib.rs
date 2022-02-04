#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move() {
        let x = 5;
        let y = x;

        assert_eq!(x, y);
    }

    #[test]
    fn test_move_string() {
        let x = String::from("hello");
        let y = x.clone();

        assert_eq!(x, y);
    }

    #[test]
    fn take_ownership_string() {
        fn takes_ownership(s: String) {
            println!("{}", s)
        }
        let s = String::from("hello");
        takes_ownership(s);
    }

    #[test]
    fn make_copy_int() {
        fn make_copy(s: i32) {
            println!("{}", s)
        }
        let i = 5;
        make_copy(i);
    }

    #[test]
    fn give_ownership_string() {
        fn give_ownership() -> String {
            let s = String::from("hello");
            s
        }
        let s1 = give_ownership();
        println!("s1 = {}", s1)
    }

    #[test]
    fn take_give_back_string() {
        fn take_give_back(s: String) -> String {
            s
        }
        let s = String::from("hello");
        let s1 = take_give_back(s);
        println!("s1 = {}", s1)
    }

    #[test]
    fn references() {
        fn calculate_length(s: &String) -> usize {
           s.len()
        }
        let s = String::from("hello");
        let length = calculate_length(&s);
        println!("The length of {} is {}.", s, length)
    }

    #[test]
    fn mutable_reference() {
        fn change(s: &mut String) {
           s.push_str(", world")
        }
        let mut s = String::from("hello");
        change(&mut s);
        println!("{}", s)
    }

    #[test]
    fn mutable_reference_scope() {
        let mut s = String::from("hello");
        let c1 = &s;
        let c2 = &s;
        println!("{}, {}", c1, c2); // last used here hence mutable reference is allowed below

        let c3 = &mut s;
        println!("{}", c3)
    }

}