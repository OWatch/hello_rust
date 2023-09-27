#[cfg(test)]
mod tests {
    use num::complex::Complex;

    #[test]
    fn test_declaration() {
        let a = 1;
        let b: i32 = 2;
        let c = 3i32;

        println!("{} {} {}", a, b, c);
    }

    #[test]
    fn test_mutable() {
        let mut a = 1;
        println!("{}", a);
        a = 2;
        println!("{}", a);

        let _b = 3;
    }

    #[test]
    fn test_num() {
        let a = Complex { re: 2.1, im: -1.2 };
        let b = Complex::new(11.1, 22.2);
        let result = a + b;

        println!("{} + {}i", result.re, result.im)
    }

    #[test]
    fn test_char() {
        let a = 'a';
        let b = '富';
        let c = '😻';

        println!(
            "{} {} {}",
            std::mem::size_of_val(&a),
            std::mem::size_of_val(&b),
            std::mem::size_of_val(&c)
        )
    }
}
