use crate::config::Config;
use genetic_algorithm::compete::CompeteWrapper;
use genetic_algorithm::crossover::CrossoverWrapper;
use genetic_algorithm::extension::ExtensionWrapper;
use genetic_algorithm::fitness::{Fitness, FitnessValue};
use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::mutate::MutateWrapper;
use genetic_algorithm::strategy::evolve::EvolveBuilder;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

#[derive(Clone, Debug)]
pub struct Builder<G: Genotype, F: Fitness<Genotype = G>> {
    pub evolve_builder: Option<
        EvolveBuilder<G, MutateWrapper, F, CrossoverWrapper, CompeteWrapper, ExtensionWrapper>,
    >,
    pub evolve_fitness_to_micro_second_factor: FitnessValue,
    pub rounds: usize,
    pub target_population_sizes: Vec<usize>,
    pub max_stale_generations_options: Vec<Option<usize>>,
    pub max_chromosome_age_options: Vec<Option<usize>>,
    pub target_fitness_score_options: Vec<Option<FitnessValue>>,
    pub mutates: Vec<MutateWrapper>,
    pub crossovers: Vec<CrossoverWrapper>,
    pub competes: Vec<CompeteWrapper>,
    pub extensions: Vec<ExtensionWrapper>,
}

impl<G: Genotype, F: Fitness<Genotype = G>> Builder<G, F> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Config<G, F>, TryFromBuilderError> {
        self.try_into()
    }

    pub fn with_evolve_builder(
        mut self,
        evolve_builder: EvolveBuilder<
            G,
            MutateWrapper,
            F,
            CrossoverWrapper,
            CompeteWrapper,
            ExtensionWrapper,
        >,
    ) -> Self {
        self.evolve_builder = Some(evolve_builder);
        self
    }
    pub fn with_rounds(mut self, rounds: usize) -> Self {
        self.rounds = rounds;
        self
    }
    pub fn with_evolve_fitness_to_micro_second_factor(
        mut self,
        evolve_fitness_to_micro_second_factor: FitnessValue,
    ) -> Self {
        self.evolve_fitness_to_micro_second_factor = evolve_fitness_to_micro_second_factor;
        self
    }
    pub fn with_target_population_sizes(mut self, target_population_sizes: Vec<usize>) -> Self {
        self.target_population_sizes = target_population_sizes;
        self
    }
    pub fn with_max_stale_generations_options(
        mut self,
        max_stale_generations_options: Vec<Option<usize>>,
    ) -> Self {
        self.max_stale_generations_options = max_stale_generations_options;
        self
    }
    pub fn with_max_chromosome_age_options(
        mut self,
        max_chromosome_age_options: Vec<Option<usize>>,
    ) -> Self {
        self.max_chromosome_age_options = max_chromosome_age_options;
        self
    }
    pub fn with_target_fitness_score_options(
        mut self,
        target_fitness_score_options: Vec<Option<FitnessValue>>,
    ) -> Self {
        self.target_fitness_score_options = target_fitness_score_options;
        self
    }
    pub fn with_mutates(mut self, mutates: Vec<MutateWrapper>) -> Self {
        self.mutates = mutates;
        self
    }
    pub fn with_crossovers(mut self, crossovers: Vec<CrossoverWrapper>) -> Self {
        self.crossovers = crossovers;
        self
    }
    pub fn with_competes(mut self, competes: Vec<CompeteWrapper>) -> Self {
        self.competes = competes;
        self
    }
    pub fn with_extensions(mut self, extensions: Vec<ExtensionWrapper>) -> Self {
        self.extensions = extensions;
        self
    }
}

impl<G: Genotype, F: Fitness<Genotype = G>> Default for Builder<G, F> {
    fn default() -> Self {
        Self {
            evolve_builder: None,
            evolve_fitness_to_micro_second_factor: 1_000_000,
            rounds: 0,
            target_population_sizes: vec![],
            max_stale_generations_options: vec![None],
            max_chromosome_age_options: vec![None],
            target_fitness_score_options: vec![None],
            mutates: vec![],
            crossovers: vec![],
            competes: vec![],
            extensions: vec![],
        }
    }
}
