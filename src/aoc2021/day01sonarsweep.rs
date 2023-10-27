pub type DepthAndDepthsCount = (i32, i32);

pub fn compute_increasing_depthscount(raw_depths_list: &str) -> i32 {
    let (_, c) = to_integer_depths(raw_depths_list)
        .into_iter()
        .fold((0, 0), accumulate_depths);

    return c - 1;
}

pub fn compute_windowed_increasing_depthscount(raw_depths_list: &str) -> i32 {
    let (_, c) = to_integer_depths(raw_depths_list)
        .as_slice()
        .windows(3)
        .map(|depth_window| sum_window(depth_window))
        .fold((0, 0), accumulate_depths);

    return c - 1;
}

fn accumulate_depths(acc: DepthAndDepthsCount, depth: i32) -> DepthAndDepthsCount {
    let (prior_depth, depths_count) = acc;
    return if depth > prior_depth {
        (depth, depths_count + 1)
    } else {
        (depth, depths_count)
    };
}

fn to_integer_depths(raw_depths_list: &str) -> Vec<i32> {
    return String::from(raw_depths_list)
        .split("\n")
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
}

fn sum_window(depth_window: &[i32]) -> i32 {
    return depth_window.into_iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::aoc2021::day01sonarsweep::*;

    fn supply_example_depths() -> &'static str {
        return "199
200
208
210
200
207
240
269
260
263";
    }

    #[test]
    fn count_increasing_depths() {
        let depths_count = compute_increasing_depthscount(supply_example_depths());
        assert_eq!(depths_count, 7);
    }

    #[test]
    fn count_sliding_window_increasing_depths() {
        let depths_count = compute_windowed_increasing_depthscount(supply_example_depths());
        assert_eq!(depths_count, 5);
    }
}
