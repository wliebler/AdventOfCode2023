use std::env;
use std::io::{BufRead, BufReader};
use std::fs::{copy, File};
use regex::{Match, Regex};

fn main() {
    task_one();
    task_two();
}


fn task_one(){
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file = File::open(file_path).expect("Unable to open file");
    let buffer = BufReader::new(file);

    let rex = Regex::new(r"\d").unwrap();

    let mut sum = 0;
    let mut results : Vec<u32> = Vec::new();
    for line in buffer.lines(){
        let linedata = &line.unwrap();
        let mut matches = rex.find_iter(linedata);
        let first = matches.next().unwrap().as_str().parse::<u32>().unwrap();

        let last = match matches.last(){
            None => first,
            Some(val) => val.as_str().parse::<u32>().unwrap()
        };
        results.push(first * 10 + last);
        sum += (first * 10 + last);
    }
    println!("TASK ONE!!!");
    println!("{:?}",results);
    println!("Sum {}",sum);
    println!("==========");
}

fn task_two(){
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let file = File::open(file_path).expect("Unable to open file");
    let buffer = BufReader::new(file);

    let rex = Regex::new(r"\d").unwrap();

    let reg_num = Regex::new(r"(?:one|two|three|four|five|six|seven|eight|nine|)").unwrap();
    let mut sum = 0;
    let mut results : Vec<u32> = Vec::new();

    for line in buffer.lines(){
        let linedata = &line.unwrap();

        let string_num_matches = reg_num.find_iter(&linedata);
        let mut updated_ln_one = String::from(linedata);
        for s_num in string_num_matches{
            let num = match s_num.as_str(){
                "one" => "1e",
                "two" => "2o",
                "three" => "3e",
                "four" => "4r",
                "five" => "5e",
                "six" => "6x",
                "seven" => "7n",
                "eight" => "8t",
                "nine" => "9e",
                _ => ""
            };
            updated_ln_one = String::from(updated_ln_one.replacen(s_num.as_str(), num, 1));
        }

        let mut updated_ln = String::from(&updated_ln_one);
        for s_num in reg_num.find_iter(&updated_ln_one){
            let num = match s_num.as_str(){
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => ""
            };
            updated_ln = String::from(updated_ln.replacen(s_num.as_str(), num, 1));
        }

        let mut matches = rex.find_iter(updated_ln.as_str());
        let first = matches.next().unwrap().as_str().parse::<u32>().unwrap();

        let last = match matches.last() {
            None => first,
            Some(x) => x.as_str().parse::<u32>().unwrap()
        };

        results.push(first * 10 + last);
        sum += (first * 10) + last;
    }

    println!("TASK TWO!!!");
    println!("{:?}",results);
    println!("Sum {}",sum);
    println!("==========");
}