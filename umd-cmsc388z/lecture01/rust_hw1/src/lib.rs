// Default function created with library
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// (1) A function that doubles an `i32` integer. The returned value is an `i32` integer.
// Simply add parameter value to itself
pub fn double_v1(n: i32) -> i32 {
    n + n
}

// (2) A function that takes the reference of an i32 integer as the input and returns an i64 integer.
// In this function, create a new i32 to take the doubled value, and then return its i64 version.
pub fn double_v2(n: &i32) -> i64 {
    let new_int: i64 = *n as i64;
    new_int * 2
}

// (3) A function that doubles an i32 integer in place.
pub fn double_v3(n: &mut i32) {
    *n *= 2
}

// Implement the integer square root function "sqrt(n)".
// In this function try some values and then return the largest value "m" such that "m * m <= n".
// For a 'harder' version, try to do it more efficiently than trying every possibility.
// Remember to write a unit test here (and on all future functions).

pub fn sqrt(n: usize) -> usize {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_double_v1() {
        assert_eq!(double_v1(2), 4);
        assert_eq!(double_v1(-3), -6);
    }

    #[test]
    fn test_double_v2() {
        assert_eq!(double_v2(&2), 4);
        assert_eq!(double_v2(&-3), -6);
    }

    #[test]
    fn test_double_v3() {
        let mut n: i32 = 2;
        double_v3(&mut n);
        assert_eq!(n, 4);
        
        n = -3;
        double_v3(&mut n);
        assert_eq!(n, -6);
    }
}
