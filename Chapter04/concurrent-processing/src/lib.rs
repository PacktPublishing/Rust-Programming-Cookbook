use rayon::prelude::*;

///
/// Sum of squared errors, a statistical error measure that squares and sums up the differences between predicted values and their ground truths.
///
///
pub fn ssqe(y: &[f32], y_predicted: &[f32]) -> Option<f32> {
    if y.len() == y_predicted.len() {
        let y_iter = y.par_iter();
        let y_pred_iter = y_predicted.par_iter();

        Some(
            y_iter
                .zip(y_pred_iter)
                .map(|(y, y_pred)| (y - y_pred).powi(2))
                .reduce(|| 0.0, |a, b| a + b),
        ) // or sum()
    } else {
        None
    }
}


pub fn ssqe_sequential(y: &[f32], y_predicted: &[f32]) -> Option<f32> {
    if y.len() == y_predicted.len() {
        let y_iter = y.iter();
        let y_pred_iter = y_predicted.iter();

        Some(
            y_iter
                .zip(y_pred_iter)
                .map(|(y, y_pred)| (y - y_pred).powi(2))
                .sum()
        ) 
    } else {
        None
    }
}


pub fn seq_count_alpha_nums(corpus: &str) -> usize {
    corpus.chars().filter(|c| c.is_alphanumeric()).count()
}


pub fn par_count_alpha_nums(corpus: &str) -> usize {
    corpus.par_chars().filter(|c| c.is_alphanumeric()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_sq_errors() {
        assert_eq!(
            ssqe(&[1.0, 1.0, 1.0, 1.0], &[2.0, 2.0, 2.0, 2.0]),
            Some(4.0)
        );
        assert_eq!(
            ssqe(&[-1.0, -1.0, -1.0, -1.0], &[-2.0, -2.0, -2.0, -2.0]),
            Some(4.0)
        );
        assert_eq!(
            ssqe(&[-1.0, -1.0, -1.0, -1.0], &[2.0, 2.0, 2.0, 2.0]),
            Some(36.0)
        );
        assert_eq!(
            ssqe(&[1.0, 1.0, 1.0, 1.0], &[2.0, 2.0, 2.0, 2.0]),
            Some(4.0)
        );
        assert_eq!(
            ssqe(&[1.0, 1.0, 1.0, 1.0], &[2.0, 2.0, 2.0, 2.0]),
            Some(4.0)
        );
    }


    #[test]
    fn test_sum_of_sq_errors_seq() {
        assert_eq!(
            ssqe_sequential(&[1.0, 1.0, 1.0, 1.0], &[2.0, 2.0, 2.0, 2.0]),
            Some(4.0)
        );
        assert_eq!(
            ssqe_sequential(&[-1.0, -1.0, -1.0, -1.0], &[-2.0, -2.0, -2.0, -2.0]),
            Some(4.0)
        );
        assert_eq!(
            ssqe_sequential(&[-1.0, -1.0, -1.0, -1.0], &[2.0, 2.0, 2.0, 2.0]),
            Some(36.0)
        );
        assert_eq!(
            ssqe_sequential(&[1.0, 1.0, 1.0, 1.0], &[2.0, 2.0, 2.0, 2.0]),
            Some(4.0)
        );
        assert_eq!(
            ssqe_sequential(&[1.0, 1.0, 1.0, 1.0], &[2.0, 2.0, 2.0, 2.0]),
            Some(4.0)
        );
    }
}
