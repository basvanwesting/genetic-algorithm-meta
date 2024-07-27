# genetic-algorithm-meta

This library was extracted from the crate [genetic_algorithm](https://docs.rs/genetic_algorithm/latest/genetic_algorithm) and therefore depends on it.
Version numbers are kept in-sync to indicate compatibility.

One cool thing to do with genotypes is to make a meta-genotype of all the
Crossover/Mutate/Compete strategies and other Evolve parameters. This could be used to optimize
the parameters of some other genetic algorithm. Yes, a simple nested for loop would also work,
but where is the fun in that? But I wasn't able to find an elegant approach to creating such a
heterogene setup. It was tried with Trait objects, Any and Enums, but all didn't work well:

* Genotype wasn't allowed to become a Trait object due to it's other traits and generics.
* Any worked, but you still need to know all possible Genotypes up front for downcasting, defeating the flexible purpose
* Enum worked, but you still need to know all possible Genotypes up front for wrapping, defeating the flexible purpose

So, after some consideration I settled on using an nested index based Genotype
`MultiDiscreteGenotype<usize>` indices of external vectors of arbitrary types, which should
then be retrieved in the fitness function. Only one type is allowed per external vector, so the
Crossover/Mutate/Compete strategies all have a Dispatch implementation forwarding to the
underlying types (e.g. `CompeteTournament::new(4).into()`)

Currently implemented as a permutation, but with caching an evolve strategy could also be used for larger search spaces.

## Documentation

See [docs.rs](https://docs.rs/genetic_algorithm/latest/genetic_algorithm)

## Examples
Run with `cargo run --example [EXAMPLE_BASENAME] --release`

* [example/meta_evolve_binary](https://github.com/basvanwesting/genetic-algorithm-meta/blob/main/examples/meta_evolve_binary.rs) `cargo run --example meta_evolve_binary --release`
* [example/meta_evolve_milp](https://github.com/basvanwesting/genetic-algorithm-meta/blob/main/examples/meta_evolve_milp.rs) `cargo run --example meta_evolve_milp --release`
* [example/meta_evolve_monkeys](https://github.com/basvanwesting/genetic-algorithm-meta/blob/main/examples/meta_evolve_monkeys.rs) `cargo run --example meta_evolve_monkeys --release`
* [example/meta_evolve_nqueens](https://github.com/basvanwesting/genetic-algorithm-meta/blob/main/examples/meta_evolve_nqueens.rs) `cargo run --example meta_evolve_nqueens --release`

## "Quick" Usage

```rust
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::strategy::evolve::prelude::*;
use genetic_algorithm_meta::prelude::*;

let rounds = 10;
let target_population_sizes = vec![2, 4, 8];
let max_stale_generations_options = vec![Some(10)];
let max_chromosome_age_options = vec![Some(10)];
let target_fitness_score_options = vec![Some(0)];
let mutates = vec![
    MutateOnce::new(0.05).into(),
    MutateOnce::new(0.2).into(),
    MutateOnce::new(0.4).into(),
];
let crossovers = vec![
    CrossoverClone::new(true).into(),
    CrossoverSingleGene::new(false).into(),
    CrossoverSingleGene::new(true).into(),
    CrossoverSinglePoint::new(true).into(),
    CrossoverUniform::new(true).into(),
];
let competes = vec![
    CompeteElite::new().into(),
    CompeteTournament::new(3).into(),
    CompeteTournament::new(4).into(),
];
let extensions = vec![
    ExtensionNoop::new().into(),
    ExtensionMassExtinction::new(0.9, 0.1).into(),
    ExtensionMassGenesis::new(0.9).into(),
    ExtensionMassInvasion::new(0.9, 0.1).into(),
    ExtensionMassDegeneration::new(0.9, 10).into(),
];

let genotype = BinaryGenotype::builder()
    .with_genes_size(10)
    .build()
    .unwrap();
let fitness = CountTrue;
let evolve_builder = EvolveBuilder::new()
    .with_genotype(genotype)
    .with_fitness(fitness)
    .with_fitness_ordering(FitnessOrdering::Minimize);
let evolve_fitness_to_micro_second_factor = 1_000_000;

let config = MetaConfig::builder()
    .with_evolve_builder(evolve_builder)
    .with_evolve_fitness_to_micro_second_factor(evolve_fitness_to_micro_second_factor)
    .with_rounds(rounds)
    .with_target_population_sizes(target_population_sizes)
    .with_max_stale_generations_options(max_stale_generations_options)
    .with_max_chromosome_age_options(max_chromosome_age_options)
    .with_target_fitness_score_options(target_fitness_score_options)
    .with_mutates(mutates)
    .with_crossovers(crossovers)
    .with_competes(competes)
    .with_extensions(extensions)
    .build()
    .unwrap();

let permutate = MetaPermutate::new(&config).call();
println!();
println!("{}", permutate);

// meta-permutate population_size: 270

// [...]

// meta-permutate:
//   best_target_population_size: 2
//   best_max_stale_generations: Some(10)
//   best_max_chromosome_age: Some(10)
//   best_target_fitness_score: Some(0)
//   best_mutate: Some(MutateDispatch(Random, 0.4))
//   best_crossover: Some(CrossoverDispatch(Clone, true))
//   best_compete: Some(CompeteDispatch(Elite, 0))
//   best_extension: Some(ExtensionDispatch(Noop))
```

## Tests
Run tests with `cargo test`
