use std::fmt;
use std::fmt::Debug;
use std::ops::Index;
use indicatif;
use indicatif::{ProgressBar, ProgressStyle};

pub fn parse_txt(txt_source : String) -> Vec<Game> {
    eprint!("loading");

    let games_dict: Vec<&str> = txt_source.split(".").collect();
    let mut player_score_vec: Vec<&str> = vec![];
    let mut games_dict_vec: Vec<Game> = vec![];

    for single_game in &games_dict {
        let mut player_score_parsed: Vec<Score> = vec![];
        player_score_vec = single_game.split("\r\n").collect();

        let mut num_players: i8 = 0;

        //parse the player score strings
        for column in &player_score_vec {
            if column.is_empty() { continue }

            num_players += 1;
            let cells: Vec<&str> = column.split("/").collect();

            eprint!(".");

            let player_score = Score::new_from_column(cells);
            player_score_parsed.push(player_score);
        }
        let game_struct = Game {
            players: num_players,
            winner: determine_winner(&player_score_parsed),
            individual_score: player_score_parsed,
        };

        games_dict_vec.push(game_struct)
    }

    println!("done !");
    return games_dict_vec
}


pub struct Score {
    pub name : String,
    pub face : String,
    pub mer_pts : i8,
    pub arg_pts : i8,
    pub mil_pts : i8,
    pub ble_pts : i8,
    pub jau_pts : i8,
    pub ver_pts : i8,
    pub vio_pts : i8,
    pub sum_pts : i8,
}

impl Score {
    fn new_from_column(cells: Vec<&str>) -> Score {
        Score {
            name : cells[0].parse().expect("empty name"),
            face : cells[1].parse().expect("empty face"),
            mer_pts : cells[2].parse().expect("empty mer_pts"),
            arg_pts : cells[3].parse().expect("empty mil_pts"),
            mil_pts : cells[4].parse().expect("empty arg_pts"),
            ble_pts : cells[5].parse().expect("empty ble_pts"),
            jau_pts : cells[6].parse().expect("empty jau_pts"),
            ver_pts : cells[7].parse().expect("empty ver_pts"),
            vio_pts : cells[8].parse().expect("empty vio_pts"),
            sum_pts : Score::sum(cells)
        }
    }

    fn sum(score : Vec<&str>) -> i8 {
        let mut tot : i8 = 0;
        for str in &score {
            if str == &score[0] {continue}
            if str == &score[1] {continue}
            let cell: i8 = str.parse().expect("empty string in score");
            tot += cell;
        }
        return tot
    }

}

impl Debug for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Score")
            .field("name", &self.name)
            .field("face", &self.face)
            .field("mer_pts", &self.mer_pts)
            .field("arg_pts", &self.arg_pts)
            .field("mil_pts", &self.mil_pts)
            .field("ble_pts", &self.ble_pts)
            .field("jau_pts", &self.jau_pts)
            .field("ver_pts", &self.ver_pts)
            .field("vio_pts", &self.vio_pts)
            .field("sum_pts", &self.sum_pts)
            .finish()
    }
}

pub struct Game {
    pub players: i8,
    pub winner: String,
    pub individual_score: Vec<Score>,
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("players", &self.players)
            .field("winner", &self.winner)
            .field("players score", &self.individual_score)
            .finish()
    }
}

fn determine_winner(game: &Vec<Score>) -> String {
    let mut highest_tot: &i8 = &0;
    let mut highest_arg: &i8 = &0;
    let mut winner = String::new();
        for score in game {
            if &score.sum_pts > highest_tot {
                highest_tot = &score.sum_pts;
                highest_arg = &score.arg_pts;
                winner = score.name.clone();
            }
            if &score.sum_pts == highest_tot {
                if &score.arg_pts > highest_arg {
                    winner = score.name.clone();
                }
            }
        }

        return winner
}


pub struct AvgScore {
    pub face_avg : f32,
    pub mer_avg : f32,
    pub arg_avg : f32,
    pub mil_avg : f32,
    pub ble_avg : f32,
    pub jau_avg : f32,
    pub ver_avg : f32,
    pub vio_avg : f32,
    pub tot_avg : f32
}

impl AvgScore {
    pub fn new_empty() -> AvgScore {
        AvgScore {
            face_avg : 0f32,
            mer_avg : 0f32,
            arg_avg : 0f32,
            mil_avg : 0f32,
            ble_avg : 0f32,
            jau_avg : 0f32,
            ver_avg : 0f32,
            vio_avg : 0f32,
            tot_avg : 0f32,
        }
    }
    pub fn as_array(&self) -> [f32; 9]{
        [self.face_avg,
         self.mer_avg,
         self.arg_avg,
         self.mil_avg,
         self.ble_avg,
         self.jau_avg,
         self.ver_avg,
         self.vio_avg,
         self.tot_avg]
    }

    pub fn pretty_print(stat : f32, color_idx : usize) {
        let mut len = stat.to_string();
        if color_idx == 0 {len = 10.to_string()}
        let color_vec: Vec<&str> = vec!["gray", "white", "yellow", "red", "cyan", "yellow", "green", "magenta", "blue"];
        let tmp: String = "{bar:".to_string() + &*len + "." + color_vec[color_idx] + "/blue} {msg}";
        let prg = ProgressBar::new(100)
            .with_position(100)
            .with_tab_width(5)
            .with_message(stat.to_string());
        if color_idx == 0 {
            prg.set_position(stat as u64);
            let msg = stat.to_string() + " % Night faces";
            prg.set_message(msg)
        }
        prg.set_style(ProgressStyle::with_template(&tmp.as_str())
            .unwrap());
        prg.abandon()

    }
}

impl Debug for AvgScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Score")
            .field("face", &self.face_avg)
            .field("mer_avg", &self.mer_avg)
            .field("arg_avg", &self.arg_avg)
            .field("mil_avg", &self.mil_avg)
            .field("ble_avg", &self.ble_avg)
            .field("jau_avg", &self.jau_avg)
            .field("ver_avg", &self.ver_avg)
            .field("vio_avg", &self.vio_avg)
            .field("sum_avg", &self.tot_avg)
            .finish()
    }
}