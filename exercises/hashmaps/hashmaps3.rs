// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,   // 进球数
    goals_conceded: u8, // 失球数
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.

        // 这个问题要求我们学会根据HashMap中的key是否存在，采取插入还是更新操作
        // 总结：
        //  1. 直接插入：insert(k, v)，key不存在则插入，key存在则覆盖
        //  2. 检查key是否存在：contains_key(k)，存在返回true，不存在返回false
        //  3. 基于旧值更新：*map.entry(k).or_insert(0) += 5;
        //  4. 获取旧值后更新：
        //          if let Some(value) = scores.get_mut("Alice") {
        //              *value = 40;  // 直接修改值
        //          }
        //  5. 复杂更新：
        //          map.entry("Alice")
        //          .and_modify(|v| *v += 10)  // 如果键存在，执行闭包更新
        //          .or_insert(1);             // 如果键不存在，插入1
        // if let Some(team) = scores.get_mut(&team_1_name) {
        //     team.goals_scored += team_1_score;
        //     team.goals_conceded += team_2_score;
        // } else {
        //     scores.insert(team_1_name, Team {
        //         goals_scored: team_1_score,
        //         goals_conceded: team_2_score,
        //     });
        // }

        // if let Some(team) = scores.get_mut(&team_2_name) {
        //     team.goals_scored += team_2_score;
        //     team.goals_conceded += team_1_score;
        // } else {
        //     scores.insert(team_2_name, Team {
        //         goals_scored: team_2_score,
        //         goals_conceded: team_1_score,
        //     });
        // }

        scores.entry(team_1_name).
            and_modify(|v| {
                v.goals_scored += team_1_score;
                v.goals_conceded += team_2_score;
            }).or_insert(Team{
            goals_scored: team_1_score,
            goals_conceded: team_2_score,
        });

        scores.entry(team_2_name).
            and_modify(|v| {
                v.goals_scored += team_2_score;
                v.goals_conceded += team_1_score;
            }).or_insert(Team{
            goals_scored: team_2_score,
            goals_conceded: team_1_score,
        });


    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
