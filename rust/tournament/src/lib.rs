use std::collections::HashMap;

struct Record {
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

pub fn tally(match_results: &str) -> String {
    let header = String::from("Team                           | MP |  W |  D |  L |  P");
    if match_results.len() == 0 {
        return header
    }

    let mut records = HashMap::new();

    for game in match_results.lines() {
        let result: Vec<&str> = game.split(";").collect();
        match result[2] {
            "win" => { 
                let winning_team = records.entry(result[0]).or_insert(Record::new());
                *winning_team.matches_played += 1;
                *winning_team.wins += 1;
                *winning_team.points += 3;
                let losing_team = records.entry(result[1]).or_insert(Record::new());
                *losing_team.matches_played += 1;
                *losing_team.losses += 1;
            },
            "loss" => { 
                let winning_team = records.entry(result[1]).or_insert(Record::new());
                *winning_team.matches_played += 1;
                *winning_team.wins += 1;
                *winning_team.points += 3;
                let losing_team = records.entry(result[0]).or_insert(Record::new());
                *losing_team.matches_played += 1;
                *losing_team.losses += 1;
            },
            "draw" => { 
                let team_one = records.entry(result[0]).or_insert(Record::new());
                *team_one.matches_played += 1;
                *team_one.draws += 1;
                *team_one.points += 1;
                let team_two = records.entry(result[1]).or_insert(Record::new());
                *team_two.matches_played += 1;
                *team_two.draws += 1;
                *team_two.points += 1;
            },
            _ => { continue },
        }
    }
    
    String::new()
}

impl Record {
    fn new() -> Self {
        Record {
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }
}
