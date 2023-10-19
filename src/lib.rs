pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec: Vec<usize> = Vec::new();

        vec.push(1usize);
        vec.push(1);
        vec.push(2);

        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 3);
    }
}
