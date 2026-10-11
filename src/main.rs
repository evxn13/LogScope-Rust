use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug, Hash, Eq, PartialEq)]
enum LogLevel {
    ERROR,
    WARN,
    INFO,
    DEBUG
}

// fn set_keyword(typ: String) -> String {
//     let mut keyword= String::new();
//     println!("Type keyword for {:?}:", typ);
//     io::stdin()
//         .read_line(&mut keyword)
//         .expect("Error.");
//     println!("\n");
//     return keyword.trim().to_string(); // "Erreur" "Error" "Err" etc...
// }

fn set_hashmap() -> HashMap<LogLevel, (String, i32)> {
    let mut levels: HashMap<LogLevel, (String, i32)> = HashMap::new();

    levels.insert(LogLevel::ERROR, ("ERROR".to_string(), 0));
    levels.insert(LogLevel::WARN, ("WARN".to_string(), 0));
    levels.insert(LogLevel::INFO, ("INFO".to_string(), 0));
    levels.insert(LogLevel::DEBUG, ("DEBUG".to_string(), 0));

    return levels;
}

fn set_keyword(mut levels: HashMap<LogLevel, (String, i32)>) -> HashMap<LogLevel, (String, i32)> {
    for (level, keyword) in levels.iter_mut() {
        println!("Type keyword for {:?}:", level);
        keyword.0.clear();
        io::stdin().read_line(&mut keyword.0).expect("Error.");
        keyword.0 = keyword.0.trim().to_string();
    }
    return levels; // "Erreur" "Error" "Err" etc...
}

fn set_mode() -> bool {
    loop {
        let mut user_input = String::new();
        println!("Type y/n to choose Automatic Mode (LogScope gonna detect all patern of your log, and take all .log in your current folder): ");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error.");
        user_input = user_input.trim().to_string();
        if  user_input == "y" || user_input == "yes"
        {return true;}
        else if user_input == "n" || user_input == "no"
        {return false;}
        else
        {println!("Error: choose y or n.\n");}
    }
}

fn main() {
    let log_analyse = fs::read_to_string("server.log").expect("Error reading log file");
    let auto_mode = set_mode();
    if auto_mode {
        
    }
    else
    {
        // let levels: HashMap<LogLevel, (String, i32)> = HashMap::new();
        let levels = set_hashmap();
        let mut levels = set_keyword(levels);
        for line in log_analyse.lines() {
            for (_, keyword) in levels.iter_mut() {
                if !keyword.0.is_empty() && line.contains(&keyword.0) {
                    keyword.1 += 1;
                }
            }   
        }
        println!("ERROR: {:?}\nWARN: {:?}\nINFO: {:?}\nDEBUG: {:?}\nIn the all LOG file(s)", levels.get(&LogLevel::ERROR).unwrap().1, levels.get(&LogLevel::WARN).unwrap().1, levels.get(&LogLevel::INFO).unwrap().1, levels.get(&LogLevel::DEBUG).unwrap().1);
    }
}
