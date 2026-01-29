/// Given a number n, return true if it is a prime number, and false otherwise.
///
/// # Example
///
/// ```
/// let test_prime = 2;
/// assert!(is_prime(test_prime));
///
/// let test_not_prime = 42;
/// assert!(!is_prime(test_not_prime));
/// ```
///
fn is_prime(n: usize) -> bool {
    todo!()
}

/// Given a number n, return the nth prime. Refer to the test cases below for more details.
///
/// For example, the 0th prime is 2, and the 1st prime is 3, then the 2nd prime is 5, etc.
///
/// # Example
///
/// ```
/// let n = 4;
/// assert_eq!(nth_prime(n), 11);
///
/// let n = 20;
/// assert_eq!(nth_prime(n), 73);
/// ```
fn nth_prime(n: usize) -> usize {
    todo!()
}


/// Returns the nth fibonacci number.
///
/// We consider the 0th fibonacci number to be 0, and the first to be 1.
///
/// # Example
///
/// ```
/// assert_eq!(fib(2), 1);
/// assert_eq!(fib(4), 3);
/// assert_eq!(fib(7), 13);
/// ```
///
/// # Note
///
/// There are a few ways to implement this. However, one obvious way might time out on Gradescope...
///
/// The easiest way to do this is to either use iteration instead of recursion, or to use some sort
/// of helper function (which you can define somewhere else in this file).
fn fib(n: usize) -> usize {
    todo!()
}

fn main() {

    let test_prime = 2; 
    assert!(is_prime(test_prime)); 
    let test_not_prime = 42; 
    assert!(!is_prime(test_not_prime));

    let n = 4;
    assert_eq!(nth_prime(n), 11);
    let n = 20;
    assert_eq!(nth_prime(n), 73);

    assert_eq!(fib(2), 1);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(7), 13);

}
