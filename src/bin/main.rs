use anyhow::Context;
use clap::Parser;
use n_puzzle::n_puzzle::NPuzzle;
use pathfinding::prelude::{astar, bfs, dfs, idastar, iddfs};
use std::num::NonZeroU8;

/// Enum representing the available search algorithms.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum SearchAlgorithm {
    Bfs,
    Dfs,
    IdDfs,
    AStar,
    IdAStar,
}

/// Enum representing the available heuristics.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Heuristic {
    NumIncorrect,
    Taxicab,
}

/// Argument structure for use with the `clap` crate.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct CliArgs {
    /// The search algorithm to use.
    #[arg(short, long, default_value = "a-star")]
    algorithm: SearchAlgorithm,

    /// The heuristic to use.
    #[arg(short = 'r', long, default_value = "taxicab")]
    heuristic: Heuristic,

    /// The pieces of the puzzle, specified as a comma-separated list of non-zero unsigned integers.
    #[arg(short, long, required = true, value_delimiter = ',', value_parser = parse_nonzero_u8)]
    pieces: Vec<NonZeroU8>,

    /// The x-coordinate of the blank position.
    #[arg(short, long, required = true)]
    x_blank: usize,

    /// The y-coordinate of the blank position.
    #[arg(short, long, required = true)]
    y_blank: usize,
}

/// Parses a string into a `NonZeroU8`.
fn parse_nonzero_u8(s: &str) -> anyhow::Result<NonZeroU8> {
    s.parse::<u8>()
        .with_context(|| format!("Failed to parse '{}' as a number", s))?
        .try_into()
        .with_context(|| format!("Failed to convert '{}' to a positive number", s))
}

fn main() -> anyhow::Result<()> {
    // Parse command-line arguments.
    let args = CliArgs::parse();
    println!("{:?}", args);

    let size = match args.pieces.len() {
        8 => 3,
        15 => 4,
        _ => anyhow::bail!(
            "Expected 8 or 15 pieces, but got {}; pass pieces in via the --pieces flag",
            args.pieces.len()
        ),
    };

    anyhow::ensure!(
        args.x_blank < size,
        "Expected x_blank to be less than {size}, but got {}; pass x_blank in via the --x_blank flag", args.x_blank
    );
    anyhow::ensure!(
        args.y_blank < size,
        "Expected y_blank to be less than {size}, but got {}; pass y_blank in via the --y_blank flag", args.y_blank
    );

    let blank_position = (args.x_blank, args.y_blank);
    let puzzle =
        NPuzzle::new(size, args.pieces, blank_position).context("Failed to create puzzle")?;

    let heuristic_fn = match args.heuristic {
        Heuristic::NumIncorrect => NPuzzle::num_incorrect,
        Heuristic::Taxicab => NPuzzle::taxicab_distance,
    };

    let result = match args.algorithm {
        SearchAlgorithm::Bfs => bfs(&puzzle, NPuzzle::successors, NPuzzle::success).map(|path| {
            let cost = path.len() - 1;
            (path, cost)
        }),
        SearchAlgorithm::Dfs => dfs(puzzle, NPuzzle::successors, NPuzzle::success).map(|path| {
            let cost = path.len() - 1;
            (path, cost)
        }),
        SearchAlgorithm::IdDfs => {
            iddfs(puzzle, NPuzzle::successors, NPuzzle::success).map(|path| {
                let cost = path.len() - 1;
                (path, cost)
            })
        }
        SearchAlgorithm::AStar => astar(
            &puzzle,
            NPuzzle::successors_with_costs,
            heuristic_fn,
            NPuzzle::success,
        ),
        SearchAlgorithm::IdAStar => idastar(
            &puzzle,
            NPuzzle::successors_with_costs,
            heuristic_fn,
            NPuzzle::success,
        ),
    };

    let (path, cost) = result.unwrap();
    for node in path {
        println!("{node}");
    }
    println!("This cost of this solution (the # of moves) was {cost}.");

    Ok(())
}
