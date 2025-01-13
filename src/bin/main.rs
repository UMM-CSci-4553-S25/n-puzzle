use anyhow::Context;
use clap::Parser;
use pathfinding::prelude::{astar, bfs, dfs, idastar, iddfs};
use std::num::NonZeroU8;

use n_puzzle::n_puzzle::NPuzzle;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum SearchAlgorithm {
    Bfs,
    Dfs,
    IdDfs,
    AStar,
    IdaStar,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Heuristic {
    NumIncorrect,
    Taxicab,
}

// Argument structure for use with the `clap` crate
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct CliArgs {
    #[arg(short, long, default_value = "a-star")]
    algorithm: SearchAlgorithm,
    #[arg(short, long, default_value = "taxicab")]
    heuristic: Heuristic,
    #[arg(short, long, required = true, value_delimiter = ',')]
    pieces: Vec<NonZeroU8>,
    #[arg(short, long, required = true, value_delimiter = ',')]
    x_blank: usize,
    #[arg(short, long, required = true, value_delimiter = ',')]
    y_blank: usize,
}

fn main() -> anyhow::Result<()> {
    let CliArgs {
        algorithm,
        heuristic,
        pieces,
        x_blank,
        y_blank,
    } = CliArgs::parse();

    let size = match pieces.len() {
        8 => 3,
        15 => 4,
        _ => anyhow::bail!(
            "Expected 8 or 15 pieces, but got {}; pass pieces in via the --pieces flag",
            pieces.len()
        ),
    };

    anyhow::ensure!(
        x_blank < size,
        "Expected x_blank to be less than {size}, but got {x_blank}; pass x_blank in via the --x_blank flag"
    );
    anyhow::ensure!(
        y_blank < size,
        "Expected y_blank to be less than {size}, but got {y_blank}; pass y_blank in via the --y_blank flag"
    );

    let blank_position = (x_blank, y_blank);
    let puzzle = NPuzzle::new(size, pieces, blank_position).context("Failed to create puzzle")?;

    let heuristic_fn = match heuristic {
        Heuristic::NumIncorrect => NPuzzle::num_incorrect,
        Heuristic::Taxicab => NPuzzle::taxicab_distance,
    };

    let result = match algorithm {
        SearchAlgorithm::Bfs => bfs(&puzzle, NPuzzle::successors, NPuzzle::success).map(|path| {
            let length = path.len();
            (path, length)
        }),
        SearchAlgorithm::Dfs => dfs(puzzle, NPuzzle::successors, NPuzzle::success).map(|path| {
            let length = path.len();
            (path, length)
        }),
        SearchAlgorithm::IdDfs => {
            iddfs(puzzle, NPuzzle::successors, NPuzzle::success).map(|path| {
                let length = path.len();
                (path, length)
            })
        }
        SearchAlgorithm::AStar => astar(
            &puzzle,
            NPuzzle::successors_with_costs,
            heuristic_fn,
            NPuzzle::success,
        ),
        SearchAlgorithm::IdaStar => idastar(
            &puzzle,
            NPuzzle::successors_with_costs,
            heuristic_fn,
            NPuzzle::success,
        ),
    };

    // Example from Wikipedia
    // let pieces =
    //     [12, 1, 2, 15, 11, 6, 5, 8, 7, 10, 9, 4, 13, 14, 3].map(|v| NonZeroU8::new(v).unwrap());
    // let blank_position = (3, 0);
    // Example from https://www.instructables.com/How-To-Solve-The-15-Puzzle/
    // let pieces =
    //     [1, 5, 10, 9, 15, 4, 14, 12, 2, 8, 13, 11, 7, 3, 6].map(|v| NonZeroU8::new(v).unwrap());
    // let blank_position = (1, 1);
    // let pieces = [7, 8, 5, 3, 1, 4, 6, 2].map(|v| NonZeroU8::new(v).unwrap());
    // let blank_position = (0, 2);
    // let puzzle = NPuzzle::new(4, pieces, blank_position).unwrap();

    // let result = bfs(&puzzle, NPuzzle::successors, NPuzzle::success);

    // let result = astar(
    //     &puzzle,
    //     NPuzzle::successors_with_costs,
    //     // NPuzzle::num_incorrect,
    //     NPuzzle::taxicab_distance,
    //     NPuzzle::success,
    // );

    // let result = idastar(
    //     &puzzle,
    //     NPuzzle::successors_with_costs,
    //     // NPuzzle::num_incorrect,
    //     NPuzzle::taxicab_distance,
    //     NPuzzle::success,
    // );

    let result = result.unwrap();
    for node in result.0 {
        println!("{node}");
    }
    println!("This solution passed through {} nodes.", result.1);

    Ok(())
}
