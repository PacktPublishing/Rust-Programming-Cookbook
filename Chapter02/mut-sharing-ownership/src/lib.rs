#![feature(test)]

//pub mod list;

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_regular_push(b: &mut Bencher) {
        let mut v = vec![];
        b.iter(|| {
            for _ in 0..1_000 {
                v.push(10);
            }
        });
    }

    #[bench]
    fn bench_refcell_push(b: &mut Bencher) {
        let v = RefCell::new(vec![]);
        b.iter(|| {
            for _ in 0..1_000 {
                v.borrow_mut().push(10);
            }
        });
    }

    #[bench]
    fn bench_cell_push(b: &mut Bencher) {
        let v = Cell::new(vec![]);
        b.iter(|| {
            for _ in 0..1_000 {
                let mut vec = v.take();
                vec.push(10);
                v.set(vec);
            }
        });
    }

    use std::borrow::Cow;
    use std::ptr::eq;

    fn min_sum_cow(min: i32, v: &mut Cow<[i32]>) {
        let sum: i32 = v.iter().sum();
        if sum < min {
            v.to_mut().push(min - sum);
        }
    }

    #[test]
    fn handling_cows() {
        let v = vec![10, 20, 30];

        // cows - Copy on Writes - encapsulate the cloning process
        // we'll wrap the Vec<T> into a Cow
        let mut cow = Cow::from(&v);

        // the memory locations are now the same
        assert!(eq(&v[..], &*cow));

        // ... until we pass it mutably into a mutating function
        min_sum_cow(70, &mut cow);

        // on some cases, the Cow will NOT contain the
        // original value!
        assert_eq!(v, vec![10, 20, 30]);
        assert_eq!(cow, vec![10, 20, 30, 10]);

        // both pointers are not equal either
        assert!(!eq(&v[..], &*cow));

        // retrieve the owned value. this is a clone operation
        let v2 = cow.into_owned();

        // let's repeat without mutation
        let mut cow2 = Cow::from(&v2);
        min_sum_cow(70, &mut cow2);

        // they are now equal ...
        assert_eq!(cow2, v2);

        // ... and point to the same memory location
        assert!(eq(&v2[..], &*cow2));
    }

    use std::cell::{Cell, RefCell};

    fn min_sum_refcell(min: i32, v: &RefCell<Vec<i32>>) {
        let sum: i32 = v.borrow().iter().sum();
        if sum < min {
            v.borrow_mut().push(min - sum);
        }
    }

    fn min_sum_cell(min: i32, v: &Cell<Vec<i32>>) {
        // we first take the Vec<T> ownership
        let mut vec = v.take();

        // work with it ...
        let sum: i32 = vec.iter().sum();

        // change if needed
        if sum < min {
            vec.push(min - sum);
        }
        // then we put it back! no mut required
        v.set(vec);
    }

    #[test]
    fn about_cells() {
        // we allocate memory and use a RefCell to dynamically
        // manage ownership
        let ref_cell = RefCell::new(vec![10, 20, 30]);

        // mutable borrows are fine,
        min_sum_refcell(70, &ref_cell);

        // they are equal!
        assert!(ref_cell.borrow().eq(&vec![10, 20, 30, 10]));

        // cells are a bit different
        let cell = Cell::from(vec![10, 20, 30]);

        // pass the immutable cell into the function
        min_sum_cell(70, &cell);

        // unwrap
        let v = cell.into_inner();

        // check the contents, and they changed!
        assert_eq!(v, vec![10, 20, 30, 10]);
    }

    #[test]
    #[should_panic]
    fn failing_cells() {
        let ref_cell = RefCell::new(vec![10, 20, 30]);

        // multiple borrows are fine
        let _v = ref_cell.borrow();
        min_sum_refcell(60, &ref_cell);

        // ... until they are mutable borrows
        min_sum_refcell(70, &ref_cell); // panics!
    }
}
