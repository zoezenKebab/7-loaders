use std::{fs, io};
use seven_loaders::*;
use dialoguer::{Confirm, Input, Select};

fn main() {

        let mut file_path: String = r"C:\Users\zenon\Desktop\valid save file.txt".to_string();

        let use_custom_path = Confirm::new()
            .with_prompt("use custom path ?")
            .interact()
            .unwrap();

        if use_custom_path {
                let custom_path = Input::new()
                    .with_prompt("enter custom path")
                    .interact_text()
                    .unwrap();
                file_path = custom_path
        }

        let content = fs::read_to_string(file_path.trim())
            .expect("failed to load file");

        //convert the string txt into a vec of games struct
        let dict = parse_txt(content);

        println!("number of games compiled : {:?}", dict.len());

        main_menu(dict)

}

fn main_menu(bulk : Vec<Game>) {
        let main_stats_menu = vec!["bulk stats", "wonder stats", "global stats", "quit"];
        loop {
                let selection = select_menu(&main_stats_menu, "select a stats menu");
                match selection {
                        0 => {
                                println!("{:#?}", bulk);
                                continue
                        }
                        1 => {
                                wonder_stats(&bulk)

                        }
                        2 => {
                                println!("not yet implemented !");
                                continue
                        }
                        3 => {
                                break
                        }
                        _ => {
                                println!("you're not supposed to see that");
                                continue
                        }
                }
        }
}

fn wonder_stats(bulk : &Vec<Game>) {
        let wonder_code =vec!["ALEX", "ARTE", "BABY", "GIZE", "HALI", "RHOD", "ZEUS"];
        for wdr in wonder_code {
                average_stats(find_wonder(bulk, wdr.clone().to_string()), wdr.to_string())
        }
        println!("press a key to continue");
        let _guess = io::stdin()
            .read_line(&mut "".to_string());

}

fn select_menu(menu : &Vec<&str>, prmt : &str) -> usize {
        return Select::new()
            .with_prompt(prmt)
            .items(&menu)
            .interact()
            .unwrap()

}


fn average_stats(focus : Vec<&Score>, name : String) {
        let mut tot = vec![0i16; 9];
        let mut face: Vec<String> = vec![];
        let mut avg = AvgScore::new_empty();
        for part in &focus {
                if part.face == "NUIT".to_string() { face.push(part.face.clone()); }
                tot[1] += part.mer_pts as i16;
                tot[2] += part.arg_pts as i16;
                tot[3] += part.mil_pts as i16;
                tot[4] += part.ble_pts as i16;
                tot[5] += part.jau_pts as i16;
                tot[6] += part.ver_pts as i16;
                tot[7] += part.vio_pts as i16;
                tot[8] += part.sum_pts as i16;
        }
        tot[0] = face.len() as i16;
        //println!("{}", tot[0]);
        let divider: f32 = focus.len() as f32;
        avg.face_avg = tot[0] as f32 / divider;
        avg.mer_avg = tot[1] as f32 / divider;
        avg.arg_avg = tot[2] as f32 / divider;
        avg.mil_avg = tot[3] as f32 / divider;
        avg.ble_avg = tot[4] as f32 / divider;
        avg.jau_avg = tot[5] as f32 / divider;
        avg.ver_avg = tot[6] as f32 / divider;
        avg.vio_avg = tot[7] as f32 / divider;
        avg.tot_avg = tot[8] as f32 / divider;

        println!("{}",name);
        println!("average over {:#?} games", divider);
        let mut color_idx = 0;
        for value in avg.as_array() {
                if color_idx == 0 {
                        AvgScore::pretty_print(value * 100f32, color_idx)
                }
                else {
                        AvgScore::pretty_print(value, color_idx)
                }
                color_idx += 1
        }
        println!("      ")
}

fn find_wonder(bulk : &Vec<Game>, focus : String) -> Vec<&Score> {
        let mut raw_scores : Vec<&Score> = vec![];
        let mut wdr_focus : Vec<&Score> = vec![];
        for single_game in bulk {
                for score in &single_game.individual_score {
                        raw_scores.push(score)
                }
        }
        for single_score in raw_scores {
                if single_score.name == focus {
                        wdr_focus.push(single_score)
                }
        }
        return wdr_focus
}
