use rayon;

///
/// Merges two collections into one. 
/// 
fn sorted_merge<T: Default + Clone + PartialOrd>(sorted_l: Vec<T>, sorted_r: Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = vec![Default::default(); sorted_l.len() + sorted_r.len()];

    let (mut i, mut j) = (0, 0);
    let mut k = 0;
    while i < sorted_l.len() && j < sorted_r.len() {
        if sorted_l[i] <= sorted_r[j] {
            result[k] = sorted_l[i].clone();
            i += 1;
        } else {
            result[k] = sorted_r[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < sorted_l.len() {
        result[k] = sorted_l[i].clone();
        k += 1;
        i += 1;
    }

    while j < sorted_r.len() {
        result[k] = sorted_r[j].clone();
        k += 1;
        j += 1;
    }
    result
}

///
/// Merge sort implementation using parallelism.
/// 
pub fn merge_sort_par<T>(collection: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + Default + Send + Sync,
{
    if collection.len() > 1 {
        let (l, r) = collection.split_at(collection.len() / 2);
        let (sorted_l, sorted_r) = rayon::join(|| merge_sort_par(l), || merge_sort_par(r));
        sorted_merge(sorted_l, sorted_r)
    } else {
        collection.to_vec()
    }
}


///
/// Regular, sequential merge sort implementation
/// 
pub fn merge_sort_seq<T: PartialOrd + Clone + Default>(collection: &[T]) -> Vec<T> {
    if collection.len() > 1 {
        let (l, r) = collection.split_at(collection.len() / 2);
        let (sorted_l, sorted_r) = (merge_sort_seq(l), merge_sort_seq(r));
        sorted_merge(sorted_l, sorted_r)
    } else {
        collection.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_seq() {
        assert_eq!(merge_sort_seq(&vec![9, 8, 7, 6]), vec![6, 7, 8, 9]);
        assert_eq!(merge_sort_seq(&vec![6, 8, 7, 9]), vec![6, 7, 8, 9]);
        assert_eq!(merge_sort_seq(&vec![2, 1, 1, 1, 1]), vec![1, 1, 1, 1, 2]);
    }

    #[test]
    fn test_merge_sort_par() {
        assert_eq!(merge_sort_par(&vec![9, 8, 7, 6]), vec![6, 7, 8, 9]);
        assert_eq!(merge_sort_par(&vec![6, 8, 7, 9]), vec![6, 7, 8, 9]);
        assert_eq!(merge_sort_par(&vec![2, 1, 1, 1, 1]), vec![1, 1, 1, 1, 2]);
    }
}
