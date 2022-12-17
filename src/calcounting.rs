use std::mem::*;

/// Computes which one elf carries the most food
/// and how much calories that food equals to.
pub fn top_one_elf_with_most_food(calories_list: &str) -> i32 {
    let top_cal_sums_with_indices = compute_cal_count_for_top_n_elves(calories_list, 1);
    return top_cal_sums_with_indices.first().unwrap().clone();
}

/// Computes which three elves carry the most food
/// and how much calories that food equals to.
pub fn top_three_elves_with_most_food(calories_list: &str) -> i32 {
    let top_cal_sums_with_indices = compute_cal_count_for_top_n_elves(calories_list, 3);
    return top_cal_sums_with_indices.into_iter().sum();
}

/// Computes which n elves carry the most food
/// and how much calories that food equals to.
fn compute_cal_count_for_top_n_elves(calories_list: &str, n: i32) -> Vec<i32> {
    // first, split at line breaks:
    let raw_cals = String::from(calories_list);
    let calories: Vec<&str> = raw_cals.split("\n").collect();

    // how many food groups / elves are there?
    let empty_lines: Vec<&str> = calories
        .clone()
        .into_iter()
        .filter(|&x| String::from(x).is_empty())
        .collect();
    let elf_count: usize = empty_lines.len() + 1;

    // okay, then take the calories per food group / elf:
    let cal_sums = compute_cals_per_group(&calories, elf_count);

    // sort the sums:
    let mut sorted_cal_sums_with_indices = cal_sums.clone();
    sorted_cal_sums_with_indices.sort();
    sorted_cal_sums_with_indices.reverse();

    // okidoki, which are the n highest cal counts?
    return sorted_cal_sums_with_indices
        .into_iter()
        .take(n as usize)
        .collect();
}

/// Computes the calories per food group / elf.
fn compute_cals_per_group(calories: &Vec<&str>, elf_count: usize) -> Vec<i32> {
    let (_, cal_sums) =
        calories
            .iter()
            .fold((0, vec![0; elf_count]), |acc: (usize, Vec<i32>), &cal| {
                let (i, mut cal_sums) = acc;

                // we increase our food group counter (i) whenever
                // we stumble over an empty line:
                return if String::from(cal).is_empty() {
                    (i + 1 as usize, cal_sums)

                // otherwise, we add the "current" cal count to
                // our accumulator:
                } else {
                    let x: i32 = String::from(cal).parse().unwrap();
                    let new_cal_sum: i32 = cal_sums.get(i).unwrap() + x;
                    let _r = replace(&mut cal_sums[i], new_cal_sum);
                    (i, cal_sums)
                };
            });
    return cal_sums;
}

#[cfg(test)]
mod tests {
    use crate::calcounting::*;

    fn supply_example_cals() -> &'static str {
        return "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    }

    #[test]
    fn count_top_one_elf_cals_works() {
        let cal_count = top_one_elf_with_most_food(supply_example_cals());
        assert_eq!(cal_count, 24000);
    }

    #[test]
    fn count_top_three_elves_cals_works() {
        let cal_count = top_three_elves_with_most_food(supply_example_cals());
        assert_eq!(cal_count, 45000);
    }
}
