#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn exploring_vec() {
        // a Vec<T> is almost always initialized using a macro
        assert_eq!(vec![0; 3], [0, 0, 0]);
        let mut v: Vec<i32> = vec![];

        // a Vec<T> is defined by a triple (pointer, capacity, length)
        assert_eq!(mem::size_of::<Vec<i32>>(), mem::size_of::<usize>() * 3);

        // empty vectors point to no memory (yet)
        assert_eq!(mem::size_of_val(&*v), 0);

        v.push(10);

        // a vector will also over-allocate on insert
        // *by how much is an implementation detail and may change!*
        assert_eq!(mem::size_of::<Vec<i32>>(), mem::size_of::<i32>() * 6);

        // vectors support indexing
        assert_eq!(v[0], 10);
        
        // vectors have some convenience methods
        v.insert(0, 11);
        v.push(12);
        assert_eq!(v, [11, 10, 12]);
        assert!(!v.is_empty());

        // ... like one to create a heap in only a few lines
        assert_eq!(v.swap_remove(0), 11);
        assert_eq!(v, [12, 10]);

        // ... or a stack
        assert_eq!(v.pop(), Some(10));
        assert_eq!(v, [12]);

        // vectors also support regular removals
        assert_eq!(v.remove(0), 12);
        
        // and can go back to occupying no memory..
        v.shrink_to_fit();
        assert_eq!(mem::size_of_val(&*v), 0);
    }

    struct Point(f32, f32);

    #[test]
    fn exploring_tuples() {
        let mut my_tuple: (i32, usize, f32) = (10, 0, -3.42);

        // members can be accessed like that
        assert_eq!(my_tuple.0, 10);
        assert_eq!(my_tuple.1, 0);
        assert_eq!(my_tuple.2, -3.42);

        my_tuple.0 = 100;
        assert_eq!(my_tuple.0, 100);



        // tuples can be unpacked
        let (_val1, _val2, _val3) = my_tuple;

        // structs can be based on tuples too
        let point = Point(1.2, 2.1);
        assert_eq!(point.0, 1.2);
        assert_eq!(point.1, 2.1);
    }


    #[test]
    fn exploring_arrays() {
        // arrays use a familiar signature
        // (type declarations are not necessary)
        let mut arr: [usize; 3] = [0; 3];
        assert_eq!(arr, [0, 0, 0]);

        let arr2: [usize; 5] = [1,2,3,4,5];
        assert_eq!(arr2, [1,2,3,4,5]);

        arr[0] = 1;
        assert_eq!(arr, [1, 0, 0]);
        assert_eq!(arr[0], 1);
        assert_eq!(mem::size_of_val(&arr), mem::size_of::<usize>() * 3);
    }
}
