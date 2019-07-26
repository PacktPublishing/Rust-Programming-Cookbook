#[cfg(test)]
mod tests {

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    struct MyError;

    #[test]
    fn transposing() {
        let this: Result<Option<i32>, MyError> = Ok(Some(42));
        let other: Option<Result<i32, MyError>> = Some(Ok(42));
        assert_eq!(this, other.transpose());

        let this: Result<Option<i32>, MyError> = Err(MyError);
        let other: Option<Result<i32, MyError>> = Some(Err(MyError));
        assert_eq!(this, other.transpose());

        assert_eq!(None::<Result<i32, MyError>>.transpose(), Ok(None::<i32>));
    }


    #[test]
    fn conversion() {
        let opt = Some(42);
        assert_eq!(opt.ok_or(MyError), Ok(42));

        let res: Result<i32, MyError> = Ok(42);
        assert_eq!(res.ok(), opt);
        assert_eq!(res.err(), None);
        
        let opt: Option<i32> = None;
        assert_eq!(opt.ok_or(MyError), Err(MyError));

        let res: Result<i32, MyError> = Err(MyError);
        assert_eq!(res.ok(), None);
        assert_eq!(res.err(), Some(MyError));
    }
}
