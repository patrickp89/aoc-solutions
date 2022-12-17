pub type ScoreComputer = fn(single_round: &str) -> i32;

/// Computes the score of a complete game of RPS when
/// picking X to Z as shapes (i.e. the first puzzle part).
pub fn compute_rps_score_with_xyz_as_shapes(rps_rounds_list: &str) -> i32 {
    let f = |rps_round: &str| compute_round_score_with_xyz_as_shapes(rps_round);
    return compute_rps_score(rps_rounds_list, f);
}

/// Computes the score of a complete game of RPS when
/// picking X to Z as desired outcomes (i.e. the second puzzle part).
pub fn compute_rps_score_with_xyz_as_outcomes(rps_rounds_list: &str) -> i32 {
    let f = |rps_round: &str| compute_round_score_with_xyz_as_outcomes(rps_round);
    return compute_rps_score(rps_rounds_list, f);
}

fn compute_rps_score(rps_rounds_list: &str, f: ScoreComputer) -> i32 {
    // first, split at line breaks:
    let raw_rps_rounds = String::from(rps_rounds_list);
    let rps_rounds: Vec<&str> = raw_rps_rounds.split("\n").collect();

    // okay, compute the score per round:
    let scores: Vec<i32> = rps_rounds
        .clone()
        .into_iter()
        .filter(|&x| !String::from(x).is_empty())
        .map(f)
        .collect();

    // and sum them up:
    return scores.iter().sum();
}

fn compute_round_score_with_xyz_as_shapes(rps_round: &str) -> i32 {
    let (picks, opponents_shape) = compute_picks_and_opponents_shape(String::from(rps_round));
    let (our_shape, outcome) = compute_our_shape_and_outcome(picks, opponents_shape);
    return compute_score(our_shape, outcome);
}

fn compute_round_score_with_xyz_as_outcomes(rps_round: &str) -> i32 {
    let (picks, opponents_shape) = compute_picks_and_opponents_shape(String::from(rps_round));
    let (our_shape, our_desired_outcome) =
        compute_our_shape_and_our_desired_outcome(picks, opponents_shape);
    return compute_score(our_shape, our_desired_outcome);
}

fn compute_picks_and_opponents_shape(rps_round: String) -> (Vec<String>, Shape) {
    // split at the whitespace:
    let raw_rps_round = String::from(rps_round);
    let raw_picks: Vec<&str> = raw_rps_round.split(" ").collect();
    let picks: Vec<String> = raw_picks.into_iter().map(|s| String::from(s)).collect();

    // okay, what did our opponent pick?
    let opponents_raw_shape: String = String::from(picks.clone().get(0).unwrap());
    let opponents_shape: Shape = match opponents_raw_shape.as_str() {
        "A" => Shape::ROCK,
        "B" => Shape::PAPER,
        "C" => Shape::SCISSORS,
        _ => panic!("Invalid input: {}!", opponents_raw_shape),
    };

    return (picks, opponents_shape);
}

fn compute_our_shape_and_outcome(picks: Vec<String>, opponents_shape: Shape) -> (Shape, Outcome) {
    // which RPS shape did we pick?
    let our_raw_shape = String::from(picks.get(1).unwrap());
    let our_shape: Shape = match our_raw_shape.as_str() {
        "X" => Shape::ROCK,
        "Y" => Shape::PAPER,
        "Z" => Shape::SCISSORS,
        _ => panic!("Invalid input: {}!", our_raw_shape),
    };

    // did we win (Rock defeats Scissors, Scissors defeats Paper,
    // and Paper defeats Rock)?
    let outcome = if our_shape == opponents_shape {
        Outcome::DRAW
    } else if (our_shape == Shape::ROCK && opponents_shape == Shape::SCISSORS)
        || (our_shape == Shape::SCISSORS && opponents_shape == Shape::PAPER)
        || (our_shape == Shape::PAPER && opponents_shape == Shape::ROCK)
    {
        Outcome::WIN
    } else {
        Outcome::LOSS
    };

    return (our_shape, outcome);
}

fn compute_our_shape_and_our_desired_outcome(
    picks: Vec<String>,
    opponents_shape: Shape,
) -> (Shape, Outcome) {
    // what is our desired outcome?
    let our_raw_desoutcome = String::from(picks.get(1).unwrap());
    let our_desired_outcome: Outcome = match our_raw_desoutcome.as_str() {
        "X" => Outcome::LOSS,
        "Y" => Outcome::DRAW,
        "Z" => Outcome::WIN,
        _ => panic!("Invalid input: {}!", our_raw_desoutcome),
    };

    // then what is the shape that we must pick in order to get the
    // desired outcome (Rock defeats Scissors, Scissors defeats Paper,
    // and Paper defeats Rock)?
    let our_shape: Shape = if our_desired_outcome == Outcome::DRAW {
        opponents_shape
    } else if our_desired_outcome == Outcome::WIN {
        match opponents_shape {
            Shape::ROCK => Shape::PAPER,
            Shape::PAPER => Shape::SCISSORS,
            Shape::SCISSORS => Shape::ROCK,
        }
    } else {
        // LOSS:
        match opponents_shape {
            Shape::PAPER => Shape::ROCK,
            Shape::SCISSORS => Shape::PAPER,
            Shape::ROCK => Shape::SCISSORS,
        }
    };

    return (our_shape, our_desired_outcome);
}

fn compute_score(our_shape: Shape, outcome: Outcome) -> i32 {
    // what partial score does the shape yield us?
    let score_for_shape = match our_shape {
        Shape::ROCK => 1,
        Shape::PAPER => 2,
        Shape::SCISSORS => 3,
    };

    // what partial score does the outcome yield us?
    let score_for_outcome = match outcome {
        Outcome::WIN => 6,
        Outcome::DRAW => 3,
        Outcome::LOSS => 0,
    };

    return score_for_shape + score_for_outcome;
}

#[derive(PartialEq)]
enum Shape {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(PartialEq)]
enum Outcome {
    WIN,
    LOSS,
    DRAW,
}

#[cfg(test)]
mod tests {
    use crate::rockpaperscissors::*;

    fn supply_example_rps_strategy_guide() -> &'static str {
        return "A Y
B X
C Z";
    }

    #[test]
    fn rps_score_computation_works() {
        let score = compute_rps_score_with_xyz_as_shapes(supply_example_rps_strategy_guide());
        assert_eq!(score, 15);
    }

    #[test]
    fn rps_score_computation_withdesoutcome_works() {
        let score = compute_rps_score_with_xyz_as_outcomes(supply_example_rps_strategy_guide());
        assert_eq!(score, 12);
    }
}
