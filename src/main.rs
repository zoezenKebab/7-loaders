use std::{env, fmt, io};
use std::fmt::Debug;
use std::fs;
use colored::*;

fn main() {

        //println!{"{}", "█████████████████░░░░░░░░░░░ 43%".truecolor(125, 230, 125)};
        //println!{"{}", "████████████░░░░░░░░░░░░░░░░ 28%".truecolor(125, 125, 230)};
        //println!{"{}", "███████░░░░░░░░░░░░░░░░░░░░░ 28%".truecolor(230, 125, 125)};
        let file_path: String = ask_for_input("enter a valid file location".to_string());

        let content = fs::read_to_string(file_path.trim())
            .expect("failed to load file");

        let games_dict: Vec<&str> = content.split(".").collect();

        let mut games_dict_vec: Vec<Game> = vec![];
        let mut player_score_vec: Vec<&str> = vec![];

        eprint!("loading");
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
                let mut player_score = Score {
                    name: cells[0].parse().unwrap(),
                    face: cells[1].parse().unwrap(),
                    mer_pts: cells[2].parse().unwrap(),
                    arg_pts: cells[3].parse().unwrap(),
                    mil_pts: cells[4].parse().unwrap(),
                    ble_pts: cells[5].parse().unwrap(),
                    jau_pts: cells[6].parse().unwrap(),
                    ver_pts: cells[7].parse().unwrap(),
                    vio_pts: cells[8].parse().unwrap(),
                    sum_pts: 0,

                };

                player_score.sum_pts = sum(&player_score);
                player_score_parsed.push(player_score);
            }
            let game_struct = Game {
                players: num_players,
                winner: "None".to_string(),
                individual_score: player_score_parsed,
            };
            games_dict_vec.push(game_struct)
        }

        println!("done !");
        println!("{:#?}", games_dict_vec);
        println!("number of games compiled : {:?}", games_dict_vec.len());

        if ask_for_input("enter restart to restart".to_string()) == "restart".to_string() { main() }

}
fn ask_for_input(mut prompt : String) -> String {
    if prompt == "default".to_string() {
        prompt = "waiting for input".to_string()
    }
    println!("{prompt}");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    println!("user typed : {}", &input);
    return input.trim().to_string()
}



struct Score {
    name : String,
    face : String,
    mer_pts : i8,
    arg_pts : i8,
    mil_pts : i8,
    ble_pts : i8,
    jau_pts : i8,
    ver_pts : i8,
    vio_pts : i8,
    sum_pts : i8,
}


fn sum(score : &Score) -> i8 {
    let tot = score.mer_pts +
        score.arg_pts +
        score.mil_pts +
        score.ble_pts +
        score.jau_pts +
        score.ver_pts +
        score.vio_pts;

    return tot
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


struct Game {
    players: i8,
    winner: String,
    individual_score: Vec<Score>,
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