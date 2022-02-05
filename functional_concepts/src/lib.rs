#[cfg(test)]
mod test {
    #[test]
    fn closure() {
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;

        assert!(equal_to_x(y));
    }

    #[test]
    fn test_fn() {
        let x = vec![1, 2, 3];
        let y = x.clone();

        let equal_to_x = |z: &Vec<i32>| z == &x;

        equal_to_x(vec![0, 2].as_ref());
        equal_to_x(vec![0, 1, 3].as_ref());
        equal_to_x(vec![0, 100].as_ref());

        assert_eq!(x, y);
    }

    #[test]
    fn test_fn_mut() {
        let mut x = vec![1, 2, 3];
        let y = x.clone();

        let mut push_to_x = |z| x.push(z);

        push_to_x(4);
        push_to_x(5);
        push_to_x(6);

        assert_ne!(x, y);
        assert_eq!(x, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_fn_once() {
        let x = vec![1, 2, 3];

        let move_x = move || x;

        move_x();
    }

    trait TypeName {
        fn type_name() -> String;
    }

    impl TypeName for usize {
        fn type_name() -> String {
            "usize".to_owned()
        }
    }

    impl TypeName for i32 {
        fn type_name() -> String {
            "i32".to_owned()
        }
    }

    impl TypeName for String {
        fn type_name() -> String {
            "String".to_owned()
        }
    }

    impl<T: TypeName> TypeName for Vec<T> {
        fn type_name() -> String {
            format!("Vec<{}>", T::type_name())
        }
    }

    impl<T1: TypeName, T2: TypeName> TypeName for (T1, T2) {
        fn type_name() -> String {
            format!("({},{})", T1::type_name(), T2::type_name())
        }
    }

    fn inspect_function<F, T1, T2>(_f: F)
        where
            F: Fn(T1) -> T2,
            T1: TypeName,
            T2: TypeName
    {
        println!("This function has the shape: {} -> {}",
                 T1::type_name(),
                 T2::type_name()
        );
    }

    #[test]
    fn test_inspect_function() {
        inspect_function(|n: usize| n.to_string());
        inspect_function(|mut v: Vec<_>| { v.push(10); v });
        inspect_function(|mut v: Vec<_>| { v.push(10usize); v });
    }
}