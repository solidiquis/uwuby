#[no_mangle]
extern "C" fn fib(n: usize) -> usize {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    }

    let mut i = 0;
    let mut j = 1;

    for _ in 0..(n - 1) {
        let sum = i + j;
        i = j;
        j = sum;
    }

    j
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(50), 12586269025);
    }
}
