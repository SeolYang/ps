use std::{io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("입력 실패");

    let input_trim = input.trim();
    let input_case_num : f32 = match input_trim.parse() {
        Ok(val) => val,
        Err(_) => 0
    } as f32;

    input.clear();
    io::stdin().read_line(&mut input)
    .expect("입력 실패");

    let input_trim = input.trim();
    let mut input_scores = input_trim.split_whitespace().collect::<Vec<&str>>();
    input_scores.retain(|&val| val != "");

    let mut sum_score : f32 = 0.0;
    let mut max_score : f32 = 0.0;
    for score_str in input_scores
    {
        let score : f32 = match score_str.parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Faeild to convert score string to flot32 {}", score_str);
                0.0
            }
        };

        if max_score < score
        {
            max_score = score;
        }

        sum_score += score;
    }

    sum_score /= 0.01 * max_score;
    println!("{}", {sum_score/input_case_num})
}