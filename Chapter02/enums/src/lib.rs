use std::io;

///
/// An enum to encapsulate errors without too much code
/// 
pub enum ApplicationError {
    Code { full:  usize, short: u16 },
    Message(String),
    IOWrapper(io::Error),
    Unknown
}

// Even enums can have an implementation!
impl ApplicationError {

    ///
    /// A function to quickly write the enum's name somewhere.
    /// 
    pub fn print_kind(&self,  mut to: &mut impl io::Write) -> io::Result<()> {
        // A simple match clause to react to each enum variant
        let kind = match self {
            ApplicationError::Code { full: _, short: _ } => "Code",
            ApplicationError::Unknown => "Unknown",
            ApplicationError::IOWrapper(_) => "IOWrapper",
            ApplicationError::Message(_) => "Message"
        };
        // the write trait lets us use any implementation and the write! macro
        // using the question mark operator lets the function return early in 
        //  case of an Err() result
        write!(&mut to, "{}", kind)?; 
        Ok(())
    }
}

///
/// An arbitrary function that simulates work and returns enum variants 
/// based on the input parameter.
/// 
pub fn do_work(choice: i32) -> Result<(), ApplicationError> {
    if choice < -100 {
        Err(ApplicationError::IOWrapper(io::Error::from(io::ErrorKind::Other)))
    } else if choice == 42 {
        Err(ApplicationError::Code { full: choice as usize, short: (choice % u16::max_value() as i32) as u16 } )
    } else if choice > 42 {
        Err(ApplicationError::Message(
            format!("{} lead to a terrible error", choice)
        ))
    } else {
        Err(ApplicationError::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::{ApplicationError, do_work};
    use std::io;

    #[test]
    fn test_do_work() {
        let choice = 10;
        if let Err(error) = do_work(choice) {
            match error {
                ApplicationError::Code { full: code, short: _ } => assert_eq!(choice as usize, code),
                // the following arm matches both variants (OR)
                ApplicationError::Unknown | ApplicationError::IOWrapper(_) => assert!(choice < 42),
                ApplicationError::Message(msg) => assert_eq!(format!("{} lead to a terrible error", choice), msg)
            }
        }
    }

    #[test]
    fn test_application_error_get_kind() {
        let mut target = vec![];
        let _ = ApplicationError::Code { full: 100, short: 100 }.print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "Code".to_string());
        
        let mut target = vec![];
        let _ = ApplicationError::Message("0".to_string()).print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "Message".to_string());
        
        let mut target = vec![];
        let _ = ApplicationError::Unknown.print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "Unknown".to_string());
        
        let mut target = vec![];
        let error = io::Error::from(io::ErrorKind::WriteZero);
        let _ = ApplicationError::IOWrapper(error).print_kind(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "IOWrapper".to_string());

    }
}
