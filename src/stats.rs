use genetic_algorithm::fitness::FitnessValue;
use stats::{mean, stddev};
use std::time::Duration;

#[derive(Default)]
pub struct Stats {
    pub durations: Vec<Duration>,
    pub best_generations: Vec<usize>,
    pub best_fitness_scores: Vec<Option<FitnessValue>>,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn duration_count(&self) -> usize {
        self.durations.len()
    }
    pub fn duration_stddev_subsec_millis(&self) -> f64 {
        stddev(self.durations.iter().map(|c| c.subsec_millis()))
    }
    pub fn duration_mean_subsec_millis(&self) -> f64 {
        mean(self.durations.iter().map(|c| c.subsec_millis()))
    }

    pub fn duration_stddev_subsec_micros(&self) -> f64 {
        stddev(self.durations.iter().map(|c| c.subsec_micros()))
    }
    pub fn duration_mean_subsec_micros(&self) -> f64 {
        mean(self.durations.iter().map(|c| c.subsec_micros()))
    }

    pub fn best_generation_count(&self) -> usize {
        self.best_generations.len()
    }
    pub fn best_generation_stddev(&self) -> f64 {
        stddev(self.best_generations.clone().into_iter())
    }
    pub fn best_generation_mean(&self) -> f64 {
        mean(self.best_generations.clone().into_iter())
    }

    pub fn best_fitness_score_count(&self) -> usize {
        self.best_fitness_scores
            .iter()
            .filter(|s| s.is_some())
            .count()
    }
    pub fn best_fitness_score_stddev(&self) -> f64 {
        stddev(self.best_fitness_scores.iter().filter_map(|s| *s))
    }
    pub fn best_fitness_score_mean(&self) -> f64 {
        mean(self.best_fitness_scores.iter().filter_map(|s| *s))
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.duration_mean_subsec_millis() >= 1.0 {
            write!(
                f,
                "duration - count: {}, mean: {:.*}ms, stddev: {:.*}ms",
                self.duration_count(),
                1,
                self.duration_mean_subsec_millis(),
                1,
                self.duration_stddev_subsec_millis()
            )?;
        } else {
            write!(
                f,
                "duration - count: {}, mean: {:.*}us, stddev: {:.*}us",
                self.duration_count(),
                1,
                self.duration_mean_subsec_micros(),
                1,
                self.duration_stddev_subsec_micros()
            )?;
        }
        write!(f, " | ")?;
        write!(
            f,
            "best_generation - count: {}, mean: {:.*}, stddev: {:.*}",
            self.best_generation_count(),
            1,
            self.best_generation_mean(),
            1,
            self.best_generation_stddev()
        )?;
        write!(f, " | ")?;
        write!(
            f,
            "best_fitness_score - count: {}, mean: {:.*}, stddev: {:.*}",
            self.best_fitness_score_count(),
            1,
            self.best_fitness_score_mean(),
            1,
            self.best_fitness_score_stddev()
        )
    }
}
