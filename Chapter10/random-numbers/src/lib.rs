#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use rand::SeedableRng;
    use rand_distr::{Bernoulli, Distribution, Normal, Uniform};

    const SAMPLES: usize = 10_000;

    #[test]
    fn test_distributions() {
        // seed for predictable outcomes
        let mut rng: StdRng = SeedableRng::from_seed([42; 32]);

        // Uniform distribution is the default
        // this is the same as thread_rng().gen_range(1, 100)
        let uniform = Uniform::new_inclusive(1, 100);
        let total_uniform: u32 = uniform.sample_iter(&mut rng).take(SAMPLES).sum();
        assert!((50.0 - (total_uniform as f32 / (SAMPLES as f32)).round()).abs() <= 2.0);

        let bernoulli = Bernoulli::new(0.8).unwrap();
        let total_bernoulli: usize = bernoulli
            .sample_iter(&mut rng)
            .take(SAMPLES)
            .filter(|s| *s)
            .count();

        assert_eq!(
            ((total_bernoulli as f32 / SAMPLES as f32) * 10.0)
                .round()
                .trunc(),
            8.0
        );

        let normal = Normal::new(2.0, 0.5).unwrap();
        let total_normal: f32 = normal.sample_iter(&mut rng).take(SAMPLES).sum();
        assert_eq!((total_normal / (SAMPLES as f32)).round(), 2.0);
    }

    #[test]
    fn test_sequences() {
        // seed for predictable outcomes
        let mut rng: StdRng = SeedableRng::from_seed([42; 32]);

        let emoji = "😄🙃🤪🙄😭😱".chars();
        let chosen_one = emoji.clone().choose(&mut rng).unwrap();
        assert_eq!(chosen_one, '🙃');

        let chosen = emoji.choose_multiple(&mut rng, 3);
        assert_eq!(chosen, ['😱', '🙃', '😭']);

        let mut three_wise_monkeys = vec!['🙈','🙉', '🙊'];
        three_wise_monkeys.shuffle(&mut rng);
        three_wise_monkeys.shuffle(&mut rng); // in this case, the first time won't change anything
        assert_eq!(three_wise_monkeys, ['🙈', '🙊', '🙉']);

        let mut three_wise_monkeys = vec!['🙈', '🙉', '🙊'];
        let partial = three_wise_monkeys.partial_shuffle(&mut rng, 2);
        assert_eq!(partial.0, ['🙊', '🙉']);
    }

    #[test]
    fn test_rngs() {
        // seed for predictable outcomes
        let mut rng: StdRng = SeedableRng::from_seed([42; 32]);
        assert_eq!(rng.gen::<u8>(), 152);

        let mut small_rng = SmallRng::from_rng(&mut rng).unwrap();
        assert_eq!(small_rng.gen::<u8>(), 174);

        let mut pcg = rand_pcg::Pcg32::from_rng(&mut rng).unwrap();
        assert_eq!(pcg.gen::<u8>(), 135);
    }
}
