#![allow(unused_macros)]


macro_rules! print_debug {
    (stdout, $($o:expr),*) => {
        $(print!("{:?}", $o));*;
        println!();
    };
    (error, $($o:expr),*) => {
        $(eprint!("{:?}", $o));*;
        eprintln!();
    };
    ($stream:expr, $($o:expr),*) => {
        $(let _ = write!($stream, "{:?}", $o));*;
        let _ = writeln!($stream);
    }
}


#[cfg(test)]
mod tests {
    use std::io::Write;

    #[test]
    fn test_printer() {
        print_debug!(error, "hello std err");
        print_debug!(stdout, "hello std out");
        let mut v = vec![];
        print_debug!(&mut v, "a");
        assert_eq!(v, vec![34, 97, 34, 10]);

    }

    macro_rules! mock {
        ($type: ty, $name: ident, $ret_val: ty, $val: block) => {
            pub trait $name {
                fn $name(&self) -> $ret_val;
            }

            impl $name for $type {
                fn $name(&self) -> $ret_val $val
            }
        };
        ($name: ident, $($variant: ident => $type:ty),+) => {
            #[derive(PartialEq, Clone, Debug)]
            struct $name {
                $(pub $variant: $type),+
            }
        };
    }

    mock!(String, hello, &'static str, { "Hi!" });
    mock!(HelloWorld, greeting => String, when => u64);

    #[test]
    fn test_mock() {
        let mystr = "Hello".to_owned();
        assert_eq!(mystr.hello(), "Hi!");

        let g = HelloWorld { greeting: "Hello World".to_owned(), when: 1560887098 };

        assert_eq!(g.greeting, "Hello World");
        assert_eq!(g.when, 1560887098);
    }
}
