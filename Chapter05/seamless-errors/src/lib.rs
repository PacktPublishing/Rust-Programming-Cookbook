
#[cfg(test)]
mod tests {

    #[test]
    fn positive_results() {

        let ok: Result<i32, f32> = Ok(42);

        assert_eq!(ok.and_then(|r| Ok(r + 1)), Ok(43));
        assert_eq!(ok.map(|r| r + 1), Ok(43));
        
        // Boolean operations with Results. Take a close look at what's returned
        assert_eq!(ok.and(Ok(43)), Ok(43));
        let err: Result<i32, f32> = Err(-42.0);
        assert_eq!(ok.and(err), err);
        assert_eq!(ok.or(err), ok);
    }

    #[test]
    fn negative_results() {
        let err: Result<i32, f32> = Err(-42.0);
        let ok: Result<i32, f32> = Ok(-41);

        assert_eq!(err.or_else(|r| Ok(r as i32 + 1)), ok);
        assert_eq!(err.map(|r| r + 1), Err(-42.0));
        assert_eq!(err.map_err(|r| r + 1.0), Err(-41.0));

        let err2: Result<i32, f32> = Err(43.0);
        let ok: Result<i32, f32> = Ok(42);
        assert_eq!(err.and(err2), err);
        assert_eq!(err.and(ok), err);
        assert_eq!(err.or(ok), ok);
    }
}
