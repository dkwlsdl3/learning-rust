use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Workout {
    date: String,
    exercise: String,
    weight_kg: f64,
    reps: u32,
    sets: u32,
}

#[derive(Debug, PartialEq)]
struct ExerciseSummary {
    total_sets: u32,
    total_volume: f64,
}

fn read_workout_file(path: &str) -> Result<String, String> {
    let readstr = std::fs::read_to_string(path);
    match readstr {
        Ok(content) => Ok(content),
        Err(err) => Err(err.to_string()),
    }
}

fn volume(reps: u32, sets: u32, weight_kg: f64) -> f64 {
    reps as f64 * sets as f64 * weight_kg
}

fn parse_workout(line: &str) -> Result<Workout, String> {
    let s: Vec<&str> = line.split(',').collect();
    if s.len() != 5 {
        return Err("에러다욧".to_string());
    }

    Ok(Workout {
        date: s[0].to_string(),
        exercise: s[1].to_string(),
        weight_kg: s[2]
            .parse::<f64>()
            .map_err(|_| "무게는 실수여야 합니다".to_string())?,
        reps: s[3]
            .parse::<u32>()
            .map_err(|_| "횟수는 정수여야 합니다".to_string())?,
        sets: s[4]
            .parse::<u32>()
            .map_err(|_| "세트는 정수여야 합니다".to_string())?,
    })
}

fn parse_workouts(contents: String) -> Result<Vec<Workout>, String> {
    let mut workouts: Vec<Workout> = Vec::new();

    for (line_index, line) in contents.lines().enumerate() {
        let res: Result<Workout, String> = parse_workout(line);
        match res {
            Ok(workout) => {
                println!(
                    "{} {}: {}kg x {}회 x {}세트 = {}",
                    workout.date,
                    workout.exercise,
                    workout.weight_kg,
                    workout.reps,
                    workout.sets,
                    volume(workout.reps, workout.sets, workout.weight_kg)
                );
                workouts.push(workout);
            }
            Err(error) => {
                let line_num = line_index + 1;
                return Err(format!("{line_num}행: {error}"));
            }
        }
    }

    Ok(workouts)
}

fn summarize_workouts(workouts: &[Workout]) -> HashMap<String, ExerciseSummary> {
    let mut sum_map: HashMap<String, ExerciseSummary> = HashMap::new();
    for workout in workouts {
        let summary = sum_map
            .entry(workout.exercise.clone())
            .or_insert(ExerciseSummary {
                total_sets: 0,
                total_volume: 0.0,
            });
        summary.total_sets += workout.sets;
        summary.total_volume += workout.reps as f64 * workout.sets as f64 * workout.weight_kg;
    }
    sum_map
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("사용법: cargo run -- <파일 경로>");
            return;
        }
    };

    let contents = match read_workout_file(&path.into_boxed_str()) {
        Ok(contents) => contents,
        Err(error) => {
            println!("{error}");
            return;
        }
    };
    match parse_workouts(contents) {
        Ok(workouts) => {
            println!("{}", workouts.len());
            let sum_map = summarize_workouts(&workouts);
            let highest = sum_map
                .iter()
                .max_by(|(_, left), (_, right)| left.total_volume.total_cmp(&right.total_volume));
            if let Some((exercise, summary)) = highest {
                println!(
                    "최고 볼륨 운동은? {}, {} kg",
                    exercise, summary.total_volume
                );
            }

            let mut rows: Vec<_> = sum_map.iter().collect();
            rows.sort_by(|(left_name, _), (right_name, _)| left_name.cmp(right_name));

            for (exercise, summary) in rows {
                println!(
                    "{}: 총 {}세트, 총 볼륨 {}kg",
                    exercise, summary.total_sets, summary.total_volume
                );
            }
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_workout;
    use super::parse_workouts;
    use super::summarize_workouts;
    use super::volume;
    use crate::Workout;

    #[test]
    fn volume_calculates_weight_times_reps_sets() {
        assert_eq!(volume(5, 5, 100.0), 2500.0)
    }

    #[test]
    fn normal_csv_line_become_workout() {
        assert_eq!(
            parse_workout("2025-05-05,데드,85,5,5"),
            Ok(Workout {
                date: "2025-05-05".to_string(),
                exercise: "데드".to_string(),
                weight_kg: 85.0,
                reps: 5,
                sets: 5,
            })
        )
    }

    #[test]
    fn rejects_row_with_wrong_field_count() {
        assert_eq!(
            parse_workout("2025-05-05,데드,85,5"),
            Err("에러다욧".to_string())
        )
    }

    #[test]
    fn rejects_non_numeric_weight() {
        assert_eq!(
            parse_workout("2025-05-05,데드,무거움,5,5"),
            Err("무게는 실수여야 합니다".to_string())
        )
    }

    #[test]
    fn rejects_invalid_row_with_line_number() {
        let input = r#"2025-05-05,데드,85,5,5
ㅇㅇㅇㅇ"#;
        assert_eq!(
            parse_workouts(input.to_string()),
            Err("2행: 에러다욧".to_string())
        );
    }

    #[test]
    fn summarizes_sets_and_volume_by_exercise() {
        let workouts = vec![
            Workout {
                exercise: "벤치프레스".to_string(),
                date: "2026-08-30".to_string(),
                weight_kg: 65.0,
                reps: 5,
                sets: 5,
            },
            Workout {
                exercise: "벤치프레스".to_string(),
                date: "2026-08-31".to_string(),
                weight_kg: 65.0,
                reps: 5,
                sets: 5,
            },
            Workout {
                exercise: "스쿼트".to_string(),
                date: "2026-08-31".to_string(),
                weight_kg: 90.0,
                reps: 5,
                sets: 5,
            },
        ];

        let summaries = summarize_workouts(&workouts);
        let sum = summaries.get("벤치프레스").unwrap();
        assert_eq!(sum.total_sets, 10);
        assert_eq!(sum.total_volume, 3250.0);
    }
}
