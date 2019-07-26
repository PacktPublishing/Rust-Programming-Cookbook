
///
/// Our almost generic statistics toolkit
/// 
pub struct StatisticsToolkit<'a> {
    base: &'a [f64],
}

impl<'a> StatisticsToolkit<'a> {

    ///
    /// Create a new instance if the slice is larger than 2 elements
    /// 
    pub fn new(base: &'a [f64]) -> Option<StatisticsToolkit> {
        if base.len() < 3 {
            None
        } else {
            Some(StatisticsToolkit { base: base })
        }
    }

    ///
    /// Computes the variance
    /// 
    pub fn var(&self) -> f64 {
        let mean = self.mean();

        let ssq: f64 = self.base.iter().map(|i| (i - mean).powi(2)).sum();
        return ssq / self.base.len() as f64;
    }


    ///
    /// Computes the standard deviation
    /// 
    pub fn std(&self) -> f64 {
        self.var().sqrt()
    }

    ///
    /// Computes the arithmetic mean
    /// 
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.base.iter().sum();

        sum / self.base.len() as f64
    }

    ///
    /// Computes the median, but clones the base slice for this.
    /// 
    pub fn median(&self) -> f64 {
        let mut clone = self.base.to_vec();

        // .sort() is not implemented for floats
        clone.sort_by(|a, b| a.partial_cmp(b).unwrap()); 

        let m = clone.len() / 2;
        if clone.len() % 2 == 0 {
            clone[m]
        } else {
            (clone[m] + clone[m - 1]) / 2.0
        }
    }
}

// declaring a lifetime is optional here, since the compiler automates this

///
/// Compute the arithmetic mean
/// 
pub fn mean<'a>(numbers: &'a [f32]) -> Option<f32> {
    if numbers.len() > 0 {
        let sum: f32 = numbers.iter().sum();
        Some(sum / numbers.len() as f32)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    ///
    /// a normal distribution created with numpy, with mu = 42 and sigma = 3.14 
    /// 
    fn numpy_normal_distribution() -> Vec<f64> {
        vec![
            43.67221552, 46.40865622, 43.44603147, 43.16162571, 40.94815816,
       44.585914  , 45.84833022, 37.77765835, 40.23715928, 48.08791899,
       44.80964938, 42.13753315, 38.80713956, 39.16183586, 42.61511209,
       42.25099062, 41.2240736 , 44.59644304, 41.27516889, 36.21238554
        ]
    }

    #[test]
    fn mean_tests() {
        // testing some aspects of the mean function
        assert_eq!(mean(&vec![1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(mean(&vec![]), None);
        assert_eq!(mean(&vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]), Some(0.0));
    }

    #[test]
    fn statisticstoolkit_new() {
        // require >= 3 elements in an array for a plausible normal distribution
        assert!(StatisticsToolkit::new(&vec![]).is_none());
        assert!(StatisticsToolkit::new(&vec![2.0, 2.0]).is_none());

        // a working example
        assert!(StatisticsToolkit::new(&vec![1.0, 2.0, 1.0]).is_some());

        // not a normal distribution, but we don't mind
        assert!(StatisticsToolkit::new(&vec![2.0, 1.0, 2.0]).is_some());
    }

    #[test]
    fn statisticstoolkit_statistics() {
         // simple best case test
        let a_sample = vec![1.0, 2.0, 1.0];
        let nd = StatisticsToolkit::new(&a_sample).unwrap();
        assert_eq!(nd.var(), 0.2222222222222222);
        assert_eq!(nd.std(), 0.4714045207910317);
        assert_eq!(nd.mean(), 1.3333333333333333);
        assert_eq!(nd.median(), 1.0);

        // no variance
        let a_sample = vec![1.0, 1.0, 1.0];
        let nd = StatisticsToolkit::new(&a_sample).unwrap();
        assert_eq!(nd.var(), 0.0);
        assert_eq!(nd.std(), 0.0);
        assert_eq!(nd.mean(), 1.0);
        assert_eq!(nd.median(), 1.0);


        // double check with a real libray
        let a_sample = numpy_normal_distribution();
        let nd = StatisticsToolkit::new(&a_sample).unwrap();
        assert_eq!(nd.var(), 8.580276516670548);
        assert_eq!(nd.std(), 2.9292109034124785);
        assert_eq!(nd.mean(), 42.36319998250001);
        assert_eq!(nd.median(), 42.61511209);


        // skewed distribution
        let a_sample = vec![1.0, 1.0, 5.0];
        let nd = StatisticsToolkit::new(&a_sample).unwrap();
        assert_eq!(nd.var(), 3.555555555555556);
        assert_eq!(nd.std(), 1.8856180831641267);
        assert_eq!(nd.mean(), 2.3333333333333335);
        assert_eq!(nd.median(), 1.0);

        // median with even collection length
        let a_sample = vec![1.0, 2.0, 3.0, 4.0] ;
        let nd = StatisticsToolkit::new(&a_sample).unwrap();
        assert_eq!(nd.var(), 1.25);
        assert_eq!(nd.std(), 1.118033988749895);
        assert_eq!(nd.mean(), 2.5);
        assert_eq!(nd.median(), 3.0);
    }
}
