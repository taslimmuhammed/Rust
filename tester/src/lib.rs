pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 1);
        assert_eq!(result, 4, "Sorry we panicked");
    }
}


// -----------------------assertions--------------------------------
// assert!(condition: bool,*optional message here), it'll panic if the conditions is false
// assert_eq!(val1, val1) , to compare if two values are equal
// assert_ne!(val1, val1) , to compare if two values are not equal