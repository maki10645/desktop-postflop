use postflop_solver::*;
use serde::Serialize;
use std::sync::Mutex;
use std::fs::{read_to_string, remove_file, OpenOptions};
use std::io::{ BufWriter, Write};


#[derive(Serialize)]
pub struct FrontEndLoadConfig {
    pub range: [Vec<f32>; 2],
    pub flop: [Card; 3],
    pub turn: Card,
    pub river: Card,
    pub starting_pot: i32,
    pub effective_stack: i32,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct FrontEndTreeConfig {
    startingPot: i32,
    effectiveStack: i32,
    rakePercent: f64,
    rakeCap: f64,
    donkOption: bool,
    oopFlopBet: String,
    oopFlopRaise: String,
    oopTurnBet: String,
    oopTurnRaise: String,
    oopTurnDonk: String,
    oopRiverBet: String,
    oopRiverRaise: String,
    oopRiverDonk: String,
    ipFlopBet: String,
    ipFlopRaise: String,
    ipTurnBet: String,
    ipTurnRaise: String,
    ipRiverBet: String,
    ipRiverRaise: String,
    addAllInThreshold: f64,
    forceAllInThreshold: f64,
    mergingThreshold: f64,
    expectedBoardLength: i32,
    addedLines: String,
    removedLines: String,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct FrontEndConfig {
    status: bool,
    config_str: String,
}

#[tauri::command]
pub fn save_post_solver_result(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    path: String,
    frontendconfig: String,
) -> bool {
    let game = &*game_state.lock().unwrap();

    if path.is_empty() {
        return false;
    }

    if cfg!(debug_assertions) { println!("saved_frontendconfig: {:?}", frontendconfig); }

    match save_data_to_file(game, &frontendconfig, path, Some(4)) {
        Ok(ok) => {
            _ = ok;
            return true;
        }
        Err(err) => {
            _ = err;
            if cfg!(debug_assertions) { println!("save error!! {:?}", err); }
            return false;
        }
    }
}


#[tauri::command]
pub fn save_post_solver_result2(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    path: String,
    frontcfgpath: String,
) -> bool {

    if path.is_empty() {
        return false;
    }

    let str = read_to_string(&frontcfgpath);
    if cfg!(debug_assertions) { println!("save_post_solver_result2 ... {:?}  ",str); }
    match str{
        Ok(str) => { 
            if cfg!(debug_assertions) { println!("saved_frontendconfig2: {:?}", str); }
            let frontendconfig = str;
            let game = &*game_state.lock().unwrap();
            match save_data_to_file(game, &frontendconfig, path, Some(4)) {
                Ok(ok) => {
                    _ = ok;
                    return true;
                }
                Err(err) => {
                    _ = err;
                    if cfg!(debug_assertions) { println!("save error!! {:?}", err); }
                    return false;
                }
            }
        }
        Err(err) => {_ = err; dbg!("read_to_string") ;return false}
    }


}


#[tauri::command]
pub fn load_post_solver_result(
    game_state: tauri::State<Mutex<PostFlopGame>>,
    path: String,
) -> FrontEndConfig {
    let old_game = &mut *game_state.lock().unwrap();
    let mut ret: FrontEndConfig = FrontEndConfig {
        status: false,
        config_str: "".to_string(),
    };

    if old_game.is_solved() && old_game.is_ready() {
        finalize(old_game);
    }

    if path.is_empty() {
        return ret;
    }

    match load_data_from_file(path, None) {
        Ok(game) => {
            //let game = psr;
            //let game = load_data_from_file(path, None).unwrap();
            let frontendconfig: String = game.1;
            if cfg!(debug_assertions) { println!("loaded_frontendconfig: {:?}", frontendconfig); }
            let _ = std::mem::replace(old_game, game.0);

            ret.status = true;
            ret.config_str = frontendconfig;

            return ret;
        }
        Err(err) => {
            ret.status = false;
            ret.config_str = err.to_string();

            if cfg!(debug_assertions) { println!("{:?}", ret.config_str); }
            return ret;
        }
    }
}

#[tauri::command]
pub fn load_card_config(game_state: tauri::State<Mutex<PostFlopGame>>) -> FrontEndLoadConfig {
    let game = game_state.lock().unwrap();
    let card_config = game.card_config();

    let mut result = FrontEndLoadConfig {
        range: [
            Vec::with_capacity(52 * 51 / 2),
            Vec::with_capacity(52 * 51 / 2),
        ],
        flop: card_config.flop,
        turn: card_config.turn,
        river: card_config.river,
        starting_pot: game.tree_config().starting_pot,
        effective_stack: game.tree_config().effective_stack,
    };

    for i in 0..2 {
        for j in 0..(52 * 51 / 2) {
            result.range[i].push(card_config.range[i].raw_data()[j])
        }
    }

    result
}

#[tauri::command]
pub fn load_frontend_config(game_state: tauri::State<Mutex<PostFlopGame>>) -> FrontEndTreeConfig {
    let game = game_state.lock().unwrap();
    let tree_config = game.tree_config();
    let added_lines = game.added_lines();
    let removed_lines = game.removed_lines();

    //println!("flop_bet_sizes[0]... {:?}",tree_config.flop_bet_sizes[0]);
    //println!("flop_bet_sizes[1]... {:?}",tree_config.flop_bet_sizes[0]);
    let mut flop_bet: [String; 2] = [String::new(), String::new()];
    let mut flop_raise: [String; 2] = [String::new(), String::new()];
    let mut turn_bet: [String; 2] = [String::new(), String::new()];
    let mut turn_raise: [String; 2] = [String::new(), String::new()];
    let mut river_bet: [String; 2] = [String::new(), String::new()];
    let mut river_raise: [String; 2] = [String::new(), String::new()];

    let mut turn_donk: String = String::new();
    let mut river_donk: String = String::new();
    let mut donk_option: bool = false;

    //bet size string
    //[0]oop [1]ip
    let mut player = 0;
    for flop in &tree_config.flop_bet_sizes {
        for b in &flop.bet {
            match flop_bet[player].len() {
                0 => flop_bet[player] = betsize_tostring(*b),
                _ => flop_bet[player] = format!("{}, {}", flop_bet[player], betsize_tostring(*b)),
            }
        }
        for r in &flop.raise {
            match flop_raise[player].len() {
                0 => flop_raise[player] = betsize_tostring(*r),
                _ => {
                    flop_raise[player] = format!("{}, {}", flop_raise[player], betsize_tostring(*r))
                }
            }
        }
        player += 1;
    }

    let mut player = 0;
    for turn in &tree_config.turn_bet_sizes {
        for b in &turn.bet {
            match turn_bet[player].len() {
                0 => turn_bet[player] = betsize_tostring(*b),
                _ => turn_bet[player] = format!("{}, {}", turn_bet[player], betsize_tostring(*b)),
            }
        }
        for r in &turn.raise {
            match turn_raise[player].len() {
                0 => turn_raise[player] = betsize_tostring(*r),
                _ => {
                    turn_raise[player] = format!("{}, {}", turn_raise[player], betsize_tostring(*r))
                }
            }
        }
        player += 1;
    }

    let mut player = 0;
    for river in &tree_config.river_bet_sizes {
        for b in &river.bet {
            match river_bet[player].len() {
                0 => river_bet[player] = betsize_tostring(*b),
                _ => river_bet[player] = format!("{}, {}", river_bet[player], betsize_tostring(*b)),
            }
        }
        for r in &river.raise {
            match river_raise[player].len() {
                0 => river_raise[player] = betsize_tostring(*r),
                _ => {
                    river_raise[player] =
                        format!("{}, {}", river_raise[player], betsize_tostring(*r))
                }
            }
        }
        player += 1;
    }

    //donk size string
    //for tdonk in &tree_config.turn_donk_sizes {
    if let Some(tdonk) = &tree_config.turn_donk_sizes {
        for b in &tdonk.donk {
            match turn_donk.len() {
                0 => turn_donk = betsize_tostring(*b),
                _ => turn_donk = format!("{}, {}", turn_donk, betsize_tostring(*b)),
            }
        }
    }

    if let Some(rdonk) = &tree_config.river_donk_sizes {
        for b in &rdonk.donk {
            match river_donk.len() {
                0 => river_donk = betsize_tostring(*b),
                _ => river_donk = format!("{}, {}", river_donk, betsize_tostring(*b)),
            }
        }
    }

    if turn_donk.len() != 0 || river_donk.len() != 0 {
        donk_option = true;
    }

    if cfg!(debug_assertions) {
        println!("flop_bet {:?}", flop_bet);
        println!("flop_raise {:?}", flop_raise);

        println!("turn_bet {:?}", turn_bet);
        println!("turn_raise {:?}", turn_raise);

        println!("river_bet {:?}", river_bet);
        println!("river_raise {:?}", river_raise);

        println!("turn_donk {:?}", turn_donk);
        println!("river_donk {:?}", river_donk);

        println!("game.added_lines {:?}", added_lines);
        println!("game.removed_lines {:?}", removed_lines);
    }

    let result = FrontEndTreeConfig {
        startingPot: tree_config.starting_pot,
        effectiveStack: tree_config.effective_stack,
        rakePercent: tree_config.rake_rate,
        rakeCap: tree_config.rake_cap,
        donkOption: donk_option,
        oopFlopBet: flop_bet[0].to_string(),
        oopFlopRaise: flop_raise[0].to_string(),
        oopTurnBet: turn_bet[0].to_string(),
        oopTurnRaise: turn_raise[0].to_string(),
        oopTurnDonk: turn_donk.to_string(),
        oopRiverBet: river_bet[0].to_string(),
        oopRiverRaise: river_raise[0].to_string(),
        oopRiverDonk: river_donk.to_string(),
        ipFlopBet: flop_bet[1].to_string(),
        ipFlopRaise: flop_raise[1].to_string(),
        ipTurnBet: turn_bet[1].to_string(),
        ipTurnRaise: turn_raise[1].to_string(),
        ipRiverBet: river_bet[1].to_string(),
        ipRiverRaise: river_raise[1].to_string(),
        addAllInThreshold: tree_config.add_allin_threshold,
        forceAllInThreshold: tree_config.force_allin_threshold,
        mergingThreshold: tree_config.merging_threshold,
        expectedBoardLength: 0,
        addedLines: "".to_string(),
        removedLines: "".to_string(),
    };

    result
}

#[tauri::command]
pub fn save_uint8array_to_desig_path(path:String, chunk:Vec<u8> ) -> bool {

    if cfg!(debug_assertions) {
        println!("save_uint8array_to_desig_path");
        println!("{:?} {:?}", path, chunk);
    }

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path);

    match file{
        Ok(f) =>{
            let mut bw = BufWriter::new(f);
            match bw.write(&chunk){
                Ok(ok) => { _=ok; bw.flush().unwrap(); return true;},
                Err(err) => { _=err ; return false; },
            }

        },
        Err(err) =>{
            _=err;
            return false;
        }
    }

}


#[tauri::command]
pub fn delete_desig_path(path:String) -> bool {

    if cfg!(debug_assertions) {
        println!("save_udelete_desig_pathint8array_to_desig_path");
        println!("{:?}", path);
    }

    match remove_file(path){
        Ok(ok) => { _=ok; return true; },
        Err(err) => { _=err; return false; }
    }
}


#[tauri::command]
pub fn test_test(test1:String, test2:String ) -> bool {
    if cfg!(debug_assertions) { println!("test1 {:?}, test2 {:?}"  ,test1, test2); }

    let str = read_to_string(&test1).unwrap();
    if cfg!(debug_assertions) { println!("test1 contents {:?} " , str); }

    return true;
}

#[tauri::command]
pub fn test_test2(test1:String, test2:Vec<u8> ) -> bool {
    if cfg!(debug_assertions) { println!("test1 {:?}, test2 {:?}"  ,test1, test2); }

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(test1);

        if cfg!(debug_assertions) { println!("a"); }
    match file{
        Ok(f) =>{
            let mut bw = BufWriter::new(f);
            match bw.write(&test2){
                Ok(ok) =>   { if cfg!(debug_assertions) {println!("b")}; _=ok; bw.flush().unwrap(); return true;},
                Err(err) => { if cfg!(debug_assertions) {println!("c")}; _=err; return false; },
            }

        },
        Err(err) =>{
            _=err;
            if cfg!(debug_assertions) { println!("d, {:?}", err.to_string()); }
            return false;
        }
    } 
    //return false;
}

#[allow(unused_assignments)]
fn betsize_tostring(b: BetSize) -> String {
    let mut ret: String = "".to_string();

    match b {
        BetSize::PotRelative(bs) => ret = format!("{}", (bs * 100 as f64).to_string()),

        BetSize::PrevBetRelative(bs) => ret = format!("{}x", bs.to_string()),

        BetSize::Additive(bs, cap) => match cap {
            0 => ret = format!("{}c", bs.to_string()),
            _ => ret = format!("{}c{}r", bs.to_string(), cap.to_string()),
        },

        BetSize::Geometric(sz, cap) => match sz {
            0 => match cap {
                f64::INFINITY => ret = format!("e"),
                _ => ret = format!("e{}", (cap * 100 as f64).to_string()),
            },

            _ => match cap {
                f64::INFINITY => ret = format!("{}e", sz.to_string()),
                _ => ret = format!("{}e{}", sz.to_string(), (cap * 100 as f64).to_string()),
            },
        },

        BetSize::AllIn => ret = "a".to_string(),
    }

    ret
}

