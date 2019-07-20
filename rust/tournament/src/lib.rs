use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Team {
    name: String,
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

    let mut teams = HashMap::new();

    for game in match_results.lines() {
        let result: Vec<&str> = game.split(";").collect();

        match result[2] {
            "win" => {
                let team_one = teams.entry(result[0]).or_insert(Team::new(result[0]));
                team_one.matches_played += 1;
                team_one.wins += 1;
                team_one.points += 3;

                let team_two = teams.entry(result[1]).or_insert(Team::new(result[1]));
                team_two.matches_played += 1;
                team_two.losses += 1;
            },
            "loss" => {
                let team_one = teams.entry(result[0]).or_insert(Team::new(result[0]));
                team_one.matches_played += 1;
                team_one.losses += 1;

                let team_two = teams.entry(result[1]).or_insert(Team::new(result[1]));
                team_two.matches_played += 1;
                team_two.wins += 1;
                team_two.points += 3;
            }, 
            "draw" => {
                let team_one = teams.entry(result[0]).or_insert(Team::new(result[0]));
                team_one.matches_played += 1;
                team_one.draws += 1;
                team_one.points += 1;

                let team_two = teams.entry(result[1]).or_insert(Team::new(result[1]));
                team_two.matches_played += 1;
                team_two.draws += 1;
                team_two.points += 1;
            },
            _ => { continue },
        }
    }

    let mut standings: Vec<&Team> = teams.values().collect();
    standings.sort_by(|a, b| {
        a.points.cmp(&b.points).reverse()
            .then(a.name.cmp(&b.name))
    });
    let mut table = header + "\n";
    for standing in standings {
        table += &standing.print();
        table += "\n";
    }

    table.trim_end().to_string()
}

impl Team {
    fn new(name: &str) -> Self {
        Team {
            name: name.to_string(),
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }

    fn print(&self) -> String {
        format!("{0: <30} | {1: >2} | {2: >2} | {3: >2} | {4: >2} | {5: >2}", self.name, self.matches_played, self.wins, self.draws, self.losses, self.points)
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        (&other.points).cmp(&self.points)
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.points.partial_cmp(&self.points)
    }
}

impl Eq for Team {}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}


