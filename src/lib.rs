#[cfg(test)]
mod tests{
    use super::*;
    use super::future::*;


    #[test]
    fn test_get_returns_value() {
        let f = Future::<_,FutureError>::finish(3);
        assert_eq!(f.unwrap(), Ok(3));
        f.finished(4);
        assert_eq!(f.unwrap(), Ok(4));
    }
}

pub mod future;