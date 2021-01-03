pub trait Oops<T> {
    fn oops(self, msg: &str) -> std::io::Result<T>;

    fn lazy_oops<F: FnOnce() -> String>(self, lazy_msg: F) -> std::io::Result<T>;
}

impl<T> Oops<T> for Option<T> {
    fn oops(self, msg: &str) -> std::io::Result<T> {
        self.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, msg))
    }

    fn lazy_oops<F: FnOnce() -> String>(self, lazy_msg: F) -> std::io::Result<T> {
        match self {
            Some(x) => Ok(x),
            _ => self.oops(&lazy_msg()),
        }
    }
}

impl<T, E> Oops<T> for Result<T, E> {
    fn oops(self, msg: &str) -> std::io::Result<T> {
        self.ok().oops(msg)
    }

    fn lazy_oops<F: FnOnce() -> String>(self, lazy_msg: F) -> std::io::Result<T> {
        match self {
            Ok(x) => Ok(x),
            _ => self.oops(&lazy_msg()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

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

    #[test]
    fn test_oops_result() {
        let v: std::result::Result<(), String> = Err("hello world".into());
        assert!(v.is_err());

        let oops: std::io::Result<()> = v.oops("oh no");
        assert_eq!(oops.err().unwrap().kind(), std::io::ErrorKind::Other)
    }

    #[test]
    fn test_oops_lazy() {
        // Closure is not evaluated for Ok
        let v: std::result::Result<(), ()> = Ok(());
        v.lazy_oops(|| panic!("Impossibru")).unwrap();

        // Closure is convenient for format!
        (0..10).for_each(|v| {
            Ok::<(), ()>(())
                .lazy_oops(|| format!("Something happened when {}", v))
                .unwrap();
        });

        // Closure is evaluated with Err
        let q: std::result::Result<(), ()> = Err(());
        assert_eq!(
            q.lazy_oops(|| "oh no".into()).err().unwrap().kind(),
            std::io::ErrorKind::Other
        );
    }

    #[test]
    fn test_doc() {
        use std::io::Result;

        fn third_element(slice: &[usize]) -> Result<&usize> {
            // Using oops to add context to a None
            slice.iter().nth(3).oops("No third element")
        }

        fn parse_batch(slice: &[&str]) -> Result<Vec<usize>> {
            slice
                .iter()
                .map(|v| {
                    v.parse::<usize>()
                        // Using lazy_oops to add debug messages
                        .lazy_oops(|| format!("Failed to parse {} from {:?}", v, slice))
                })
                .collect()
        }

        assert_eq!(
            // No third element
            third_element(&[1, 2, 3]).err().unwrap().kind(),
            std::io::ErrorKind::Other
        );

        assert_eq!(
            // Failed to parse lo from ["2", "3", "7", "lo", "11"]
            parse_batch(&["2", "3", "7", "lo", "11"])
                .err()
                .unwrap()
                .kind(),
            std::io::ErrorKind::Other
        );
    }
}
