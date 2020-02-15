pub trait Oops<T> {
    fn oops(self, msg: &str) -> std::io::Result<T>;
}

impl<T> Oops<T> for Option<T> {
    fn oops(self, msg: &str) -> std::io::Result<T> {
        self.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn third_element(slice: &[usize]) -> std::io::Result<&usize> {
        slice.iter().nth(3).oops("No third element")
    }

    #[test]
    fn test_oops() {
        let v: std::io::Result<()> = None.oops("_");
        assert!(v.is_err());

        let slice = &[1, 2];
        assert!(slice.iter().nth(3).oops("No 3rd element").is_err());

        assert!(third_element(&[1, 2, 3]).is_err());
    }
}
