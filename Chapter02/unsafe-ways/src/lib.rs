#![allow(dead_code)]
use std::slice;

fn split_into_equal_parts<T>(slice: &mut [T], parts: usize) -> Vec<&mut [T]> {
    let len = slice.len();
    assert!(parts <= len);
    let step = len / parts;
    unsafe {
        let ptr = slice.as_mut_ptr();

        (0..step + 1)
            .map(|i| {
                let offset = (i * step) as isize;
                let a = ptr.offset(offset);
                slice::from_raw_parts_mut(a, step)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str_to_bytes_horribly_unsafe() {
        let bytes = unsafe { std::mem::transmute::<&str, &[u8]>("Going off the menu") };
        assert_eq!(
            bytes,
            &[
                71, 111, 105, 110, 103, 32, 111, 102, 102, 32, 116, 104, 101, 32, 109, 101, 110,
                117
            ]
        );
    }

    #[test]
    fn test_split_into_equal_parts() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(
            split_into_equal_parts(&mut v, 3),
            &[&[1, 2], &[3, 4], &[5, 6]]
        );
    }
}
