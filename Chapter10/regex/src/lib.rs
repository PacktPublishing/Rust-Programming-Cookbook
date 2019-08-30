#[cfg(test)]
mod tests {

    use regex::Regex;
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[test]
    fn simple_parsing() {
        let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();

        assert!(re.is_match("1999-12-01"));
        let date = re.captures("2019-02-27").unwrap();

        assert_eq!("2019", &date["y"]);
        assert_eq!("02", &date["m"]);
        assert_eq!("27", &date["d"]);

        let fun_dates: Vec<(i32, i32, i32)> = (1..12).map(|i| (2000 + i, i, i * 2)).collect();

        let multiple_dates: String = fun_dates
            .iter()
            .map(|d| format!("{}-{:02}-{:02} ", d.0, d.1, d.2))
            .collect();

        for (match_, expected) in re.captures_iter(&multiple_dates).zip(fun_dates.iter()) {
            assert_eq!(match_.get(1).unwrap().as_str(), expected.0.to_string());
            assert_eq!(
                match_.get(2).unwrap().as_str(),
                format!("{:02}", expected.1)
            );
            assert_eq!(
                match_.get(3).unwrap().as_str(),
                format!("{:02}", expected.2)
            );
        }
    }

    #[test]
    fn reshuffle_groups() {
        let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();

        let fun_dates: Vec<(i32, i32, i32)> = (1..12).map(|i| (2000 + i, i, i * 2)).collect();

        let multiple_dates: String = fun_dates
            .iter()
            .map(|d| format!("{}-{:02}-{:02} ", d.0, d.1, d.2))
            .collect();

        let european_format = re.replace_all(&multiple_dates, "$d.$m.$y");

        assert_eq!(european_format.trim(), "02.01.2001 04.02.2002 06.03.2003 08.04.2004 10.05.2005 12.06.2006 14.07.2007 16.08.2008 18.09.2009 20.10.2010 22.11.2011");
    }
    #[test]
    fn count_groups() {
        let counter: HashMap<String, i32> = HashMap::new();

        let phone_numbers = "+49 (1234) 45665
        +43(0)1234/45665 43
        +1 314-CALL-ME
        +44 1234 45665
        +49 (1234) 44444
        +44 12344 55538";

        let re = Regex::new(r"(\+[\d]{1,4})").unwrap();

        let prefixes = re
            .captures_iter(&phone_numbers)
            .map(|match_| match_.get(1))
            .filter(|m| m.is_some())
            .fold(RefCell::new(counter), |c, prefix| {
                {
                    let mut counter_dict = c.borrow_mut();
                    let prefix = prefix.unwrap().as_str().to_string();
                    let count = counter_dict.get(&prefix).unwrap_or(&0) + 1;
                    counter_dict.insert(prefix, count);
                }
                c
            });

        let prefixes = prefixes.into_inner();
        assert_eq!(prefixes.get("+49"), Some(&2));
        assert_eq!(prefixes.get("+1"), Some(&1));
        assert_eq!(prefixes.get("+44"), Some(&2));
        assert_eq!(prefixes.get("+43"), Some(&1));
    }
}
