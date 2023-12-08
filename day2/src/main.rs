use std::cmp::max;
use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::{Captures, Regex};

const BLUE_LIMIT: u32 = 14;
const GREEN_LIMIT: u32 = 13;
const RED_LIMIT: u32 = 12;
fn main(){
    task_one();
    task_two();
}
fn task_one() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file = File::open(file_path).expect("Unable to open file");
    let buffer = BufReader::new(file);


    let mut id_sum = 0;
    let reg = Regex::new(r"Game (\d+):(.*)").unwrap();
    let grn_reg = Regex::new(r"(\d+) green").unwrap();
    let red_reg = Regex::new(r"(\d+) red").unwrap();
    let blu_reg = Regex::new(r"(\d+) blue").unwrap();
    for line in buffer.lines(){
        let mut possible = true;
        for (_,[id,line_data]) in reg.captures_iter(line.unwrap().as_str()).map(|c| c.extract()){
            let id_num = id.parse::<u32>().unwrap();
            for data in (line_data.split(";")){
                let blu_val = regex_color_check(blu_reg.captures(data));
                let red_val = regex_color_check(red_reg.captures(data));
                let grn_val = regex_color_check(grn_reg.captures(data));
                if blu_val > BLUE_LIMIT || grn_val > GREEN_LIMIT || red_val > RED_LIMIT {
                    possible = false
                }
            }
            if possible{
                id_sum += id_num;
            }
        }
    }
    println!("TASK ONE!");
    println!("Sum of ids: {}",id_sum)


}

fn task_two(){
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file = File::open(file_path).expect("Unable to open file");
    let buffer = BufReader::new(file);


    let mut power_sum = 0;
    let reg = Regex::new(r"Game (\d+):(.*)").unwrap();
    let grn_reg = Regex::new(r"(\d+) green").unwrap();
    let red_reg = Regex::new(r"(\d+) red").unwrap();
    let blu_reg = Regex::new(r"(\d+) blue").unwrap();
    for line in buffer.lines(){

        for (_,[id,line_data]) in reg.captures_iter(line.unwrap().as_str()).map(|c| c.extract()){
            let mut min_red = 0;
            let mut min_grn = 0;
            let mut min_blu = 0;
            let id_num = id.parse::<u32>().unwrap();
            for data in (line_data.split(";")){
                let blu_val = regex_color_check(blu_reg.captures(data));
                let red_val = regex_color_check(red_reg.captures(data));
                let grn_val = regex_color_check(grn_reg.captures(data));

                min_blu = max(blu_val,min_blu);
                min_red = max(red_val,min_red);
                min_grn = max(grn_val,min_grn);

            }
            let power_val = min_blu * min_red * min_grn;
            power_sum += power_val;
        }
    }
    println!("TASK TWO!");
    println!("Sum of ids: {}",power_sum)


}
fn regex_color_check(a: Option<Captures>) -> u32{
    match a{
        Some(val) => {val.get(1).unwrap().as_str().parse::<u32>().unwrap()},
        None => 0
    }
}