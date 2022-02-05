#[cfg(test)]
mod test {
    use std::ops::Mul;

    fn scalar_product(v: &[i32], w: &[i32]) -> i32 {
        let length = v.len();

        assert_eq!(length, w.len());

        (0..length).fold(0, |acc, i| acc + v[i] * w[i])
    }

    #[test]
    fn test_scalar_product() {
        let v = vec![1, 1, 1];
        let w = vec![1, 0, 1];
        let prod = scalar_product(&v, &w);
        assert_eq!(prod, 2);
    }

    fn generic_scalar_product<T>(v: &[T], w: &[T]) -> T
        where T: num_traits::Zero + Mul<Output=T> + Copy
    {
        let length = v.len();
        assert_eq!(length, w.len());

        (0..length).fold(T::zero(), |acc, i| acc + v[i] * w[i])
    }

    #[test]
    fn test_generic_scalar_product() {
        // Unsigned integers
        let x: Vec<u32> = vec![1, 1, 1];
        let y: Vec<u32> = vec![1, 0, 1];
        assert_eq!(generic_scalar_product(&x, &y), 2);

        // Signed integers
        let f: Vec<i32> = vec![1, 1, -1];
        let g: Vec<i32> = vec![1, 0, 1];
        assert_eq!(generic_scalar_product(&f, &g), 0);

        // Floats
        let v: Vec<f32> = vec![1., 1., 1.];
        let w: Vec<f32> = vec![1., 0., 1.];
        assert_eq!(generic_scalar_product(&v, &w), 2.);
    }

    fn bubble_sort(v: &mut [i32]) {
        let length = v.len();

        for _ in 0..length {
            for j in 0..length - 1 {
                if v[j] > v[j + 1] {
                    let temp = v[j + 1];
                    v[j + 1] = v[j];
                    v[j] = temp;
                }
            }
        }
    }

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![6, 5, 4, 3];
        let midpoint_index = v.len() / 2;
        let first_half = &mut v[0..midpoint_index];
        bubble_sort(first_half);
        assert_eq!(v, vec![5, 6, 4, 3]);
    }
}