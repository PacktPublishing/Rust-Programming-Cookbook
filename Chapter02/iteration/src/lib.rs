#[cfg(test)]
mod tests {

    #[test]
    fn getting_the_iterator() {
        let v = vec![10, 10, 10];

        // borrowed iterator (v remains as is)
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), None);

        // owned iterator (consumes v)
        // this is the same as calling into_iter()
        for i in v {
            assert_eq!(i, 10);
        }
    }

    fn count_files(path: &String) -> usize {
        path.len()
    }

    #[test]
    fn data_transformations() {
        let v = vec![10, 10, 10];
        let hexed = v.iter().map(|i| format!("{:x}", i));
        assert_eq!(
            hexed.collect::<Vec<String>>(),
            vec!["a".to_string(), "a".to_string(), "a".to_string()]
        );
        assert_eq!(v.iter().fold(0, |p, c| p + c), 30);

        // as an example: directories and their file count
        let dirs = vec![
            "/home/alice".to_string(),
            "/home/bob".to_string(),
            "/home/carl".to_string(),
            "/home/debra".to_string(),
        ];

        // get the no of files with some function
        //  based on the directory name
        let file_counter = dirs.iter().map(count_files);

        // for easier handling, merge both collections into one
        let dir_file_counts: Vec<(&String, usize)> = dirs.iter().zip(file_counter).collect();

        // sorry for the messy string handling here ...
        // "hello" is a &str (a slice) so either we work
        // with &&str (slice reference) or &String (String reference)
        // We opted for the latter.
        assert_eq!(
            dir_file_counts,
            vec![
                (&"/home/alice".to_string(), 11),
                (&"/home/bob".to_string(), 9),
                (&"/home/carl".to_string(), 10),
                (&"/home/debra".to_string(), 11)
            ]
        )
    }

    #[test]
    fn data_filtering() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        // a simple filter to only get the even elements.
        // confirmed by an "all" function where a predicate has to apply to all elements
        assert!(data.iter().filter(|&n| n % 2 == 0).all(|&n| n % 2 == 0));
        // similarly, find and position can be used to find the *first* occurance of an element
        assert_eq!(data.iter().find(|&&n| n == 5), Some(&5));
        assert_eq!(data.iter().find(|&&n| n == 0), None);
        assert_eq!(data.iter().position(|&n| n == 5), Some(4));

        // we can also simply skip a number of elements
        assert_eq!(data.iter().skip(1).next(), Some(&2));
        let mut data_iter = data.iter().take(2);
        assert_eq!(data_iter.next(), Some(&1));
        assert_eq!(data_iter.next(), Some(&2));
        assert_eq!(data_iter.next(), None);

        // another handy use is for splitting data (e.g. for machine learning)
        // this splits the original Vec<T> into a training (~80%) and validation set (~20%)
        let (validation, train): (Vec<i32>, Vec<i32>) = data
            .iter()
            .partition(|&_| (rand::random::<f32>() % 1.0) > 0.8);

        assert!(train.len() > validation.len());
    }

}
