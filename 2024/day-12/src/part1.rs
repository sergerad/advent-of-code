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

    let mut block_graph: UnGraphMap<(i32, i32), ()> = UnGraphMap::new();

    for ((x, y), c) in blocks.iter() {
        let block = block_graph.add_node((*x, *y));

        for direction in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let next_block = (x + direction.0, y + direction.1);
            if blocks.get(&next_block).is_some_and(|next_c| c == next_c) {
                block_graph.add_edge(block, next_block, ());
            };
        }
    }

    let condensed_graph = condensation(block_graph.clone().into_graph::<NodeIndex>(), false);
    let sum = condensed_graph
        .node_references()
        .map(|(_, neighbours)| {
            let perimeter = neighbours
                .iter()
                .map(|neighbour| 4 - block_graph.neighbors(*neighbour).count())
                .sum::<usize>();
            neighbours.len() * perimeter
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
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
