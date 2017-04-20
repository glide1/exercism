use std::fmt::Display;
use std::collections::BTreeMap;

fn team_line_format<T1: Display, T2: Display, T3: Display, T4: Display, T5: Display, T6: Display>
    (team: T1,
     played: T2,
     wins: T3,
     draws: T4,
     losses: T5,
     points: T6)
     -> String {
    format!("{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team,
            played,
            wins,
            draws,
            losses,
            points)
}

#[derive(Debug, PartialEq, Eq)]
struct Record(u32, u32, u32);

pub fn tally(input: &str) -> String {
    // let line_format = "{{:<31}}";

    let mut tree_map: BTreeMap<String, Record> = BTreeMap::new();

    for line in input.split('\n') {
        println!("line: {}", line);
        let entries: Vec<&str> = line.split(';').collect();
        if let Some(result) = entries.get(2) {
            let first = String::from(entries[0]);
            let second = String::from(entries[1]);
            match *result {
                "win" => {
                    tree_map.entry(first).or_insert(Record(0, 0, 0)).add_win();
                    tree_map.entry(second).or_insert(Record(0, 0, 0)).add_loss();
                }
                "loss" => {
                    tree_map.entry(first).or_insert(Record(0, 0, 0)).add_loss();
                    tree_map.entry(second).or_insert(Record(0, 0, 0)).add_win();
                }
                "draw" => {
                    tree_map.entry(first).or_insert(Record(0, 0, 0)).add_draw();
                    tree_map.entry(second).or_insert(Record(0, 0, 0)).add_draw();
                }
                _ => (), 
            }
        }
    }

    let mut standings: Vec<(String, Record)> =
        tree_map.into_iter().fold(Vec::new(), |mut vec, kv| {
            vec.push(kv);
            vec
        });

    standings.sort_by(|a, b| b.1.cmp(&a.1));

    let mut ret: Vec<String> = Vec::new();

    ret.push(team_line_format("Team", "MP", "W", "D", "L", "P"));
    for team in standings {
        ret.push(team_line_format(team.0,
                                  (team.1).played(),
                                  (team.1).0,
                                  (team.1).2,
                                  (team.1).1,
                                  (team.1).points()));
    }

    ret.join("\n")

}

impl Record {
    pub fn add_win(&mut self) {
        self.0 += 1;
    }
    pub fn add_loss(&mut self) {
        self.1 += 1;
    }
    pub fn add_draw(&mut self) {
        self.2 += 1;
    }
    pub fn points(&self) -> u32 {
        self.0 * 3 + self.2
    }
    pub fn played(&self) -> u32 {
        self.0 + self.1 + self.2
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))

    }
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> std::cmp::Ordering {
        self.points().cmp(&(other.points()))
    }
}