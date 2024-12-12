use itertools::Itertools;
use petgraph::{algo::condensation, prelude::*, visit::IntoNodeReferences};
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let blocks = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<(i32, i32), char>>();

    let mut block_graph: UnGraphMap<(i32, i32, char), ()> = UnGraphMap::new();

    for ((x, y), c) in blocks.iter() {
        let block = block_graph.add_node((*x, *y, *c));

        for direction in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let next_block = (x + direction.0, y + direction.1);
            if blocks.get(&next_block).is_some_and(|&next_c| *c == next_c) {
                block_graph.add_edge(block, (next_block.0, next_block.1, *c), ());
            };
        }
    }

    let condensed_graph = condensation(block_graph.clone().into_graph::<NodeIndex>(), false);
    let sum = condensed_graph
        .node_references()
        .map(|(_, neighbours)| {
            neighbours
                .iter()
                .map(|neighbour| {
                    [(0, 1), (1, 0), (0, -1), (-1, 0)]
                        .iter()
                        .circular_tuple_windows()
                        .fold(0, |sum, (a, b)| {
                            let a_is_neighbour = blocks
                                .get(&(a.0 + neighbour.0, a.1 + neighbour.1))
                                .is_some_and(|c| *c == neighbour.2);
                            let b_is_neighbour = blocks
                                .get(&(b.0 + neighbour.0, b.1 + neighbour.1))
                                .is_some_and(|c| *c == neighbour.2);
                            let a_b_is_not_neighbour = blocks
                                .get(&(a.0 + b.0 + neighbour.0, a.1 + b.1 + neighbour.1))
                                .is_some_and(|c| *c != neighbour.2);
                            if a_is_neighbour && b_is_neighbour && a_b_is_not_neighbour {
                                // have interior corner
                                sum + 1
                            } else if !a_is_neighbour && !b_is_neighbour {
                                //have exterior corner
                                sum + 1
                            } else {
                                sum
                            }
                        })
                })
                .sum::<usize>()
                * neighbours.len()
        })
        .sum::<usize>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
