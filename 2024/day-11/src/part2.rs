use num_traits::Euclid;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let nums = input
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut cache: HashMap<u64, u64> = nums.iter().fold(HashMap::default(), |mut cache, &num| {
        update_cache(&mut cache, num, 1);
        cache
    });

    for _ in 0..25 {
        let mut cache_i: HashMap<u64, u64> = HashMap::default();

        for (num, count) in cache.into_iter() {
            match num {
                0 => {
                    update_cache(&mut cache_i, 1, count);
                }
                n => {
                    let num_digits = n.checked_ilog10().unwrap_or(0) + 1;
                    if num_digits % 2 == 0 {
                        let (left, right) = n.div_rem_euclid(&10u64.pow(num_digits / 2));
                        update_cache(&mut cache_i, left, count);
                        update_cache(&mut cache_i, right, count);
                    } else {
                        update_cache(&mut cache_i, n * 2024, count);
                    }
                }
            }
        }
        cache = cache_i;
    }

    Ok(cache.values().sum::<u64>().to_string())
}

fn update_cache(cache: &mut HashMap<u64, u64>, num: u64, count: u64) {
    cache
        .entry(num)
        .and_modify(|v| {
            *v += count;
        })
        .or_insert(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
