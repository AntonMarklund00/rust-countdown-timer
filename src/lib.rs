use std::error::Error;
use std::{io, time, thread, fs};
use std::collections::HashMap;
use colored::Colorize;

struct Time{
    hours: i32,
    minutes: i32,
    seconds: i32
}

impl Time{
    fn get_time_in_seconds(&self) -> i32{
        self.hours*3600 + self.minutes*60 + self.seconds
    }
}

pub fn run(_args: Vec<String>) -> Result<(), Box<dyn Error>>{


    let time = get_time(_args);
    validate_time(&time);

    let mut time_in_seconds = time.get_time_in_seconds();


    let mut ascii_hash: HashMap<String, String> = HashMap::new();

    while time_in_seconds > -1{
        let countdown_thread = thread::spawn(|| {
            thread::sleep(time::Duration::from_secs(1));
        });

        let _time_left = time_in_seconds.clone();

        ascii_hash = print_time_left(_time_left, &mut ascii_hash);

        countdown_thread.join().map_err(|err| println!("{:?}", err)).ok();

        clear_screen();
        
        time_in_seconds = time_in_seconds-1;
    }

    Ok(())
}

fn get_time(args: Vec<String>) -> Time{
    let mut time = Time{
        hours: 0,
        minutes: 0,
        seconds: 0
    };

    let args_length = args.len();
    if args_length < 2{
        println!("Please select amount of hours!");
        time.hours = get_start_time();
        println!("Please select amount of minutes!");
        time.minutes = get_start_time();
        println!("Please select amount of seconds!");
        time.seconds = get_start_time();
    }else if args_length == 4{
        time.hours = args[1].parse::<i32>().unwrap();
        time.minutes = args[2].parse::<i32>().unwrap();
        time.seconds = args[3].parse::<i32>().unwrap();
    }else{
        panic!("Unvalid input")
    }
    time
}

fn validate_time(time: &Time) {
    if time.hours > 99 || time.minutes > 59 || time.seconds > 59 {
        panic!("Unvalid input")
    }
}

fn get_start_time() -> i32{
    let start_time: i32; 
    let mut input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    let trimmed = input_value.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => start_time = i,
        Err(..) => start_time = get_start_time(),
    };

    start_time
}

fn clear_screen(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn print_time_left(mut time: i32, ascii_hash: &mut HashMap<String, String>) -> HashMap<String, String>{
    let mut time_string_vector: Vec<String> = Vec::new();

    let hours = time / 3600;
    time = time - (hours * 3600);
    let minutes = time / 60;
    time = time - (minutes / 60);
    let seconds = time % 60;

    let time_string = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    for digit in time_string.to_string().split("") {
        if digit == "" { continue }


        let mut big_digit = "".to_string();
        if ascii_hash.contains_key(digit) {
            big_digit += &ascii_hash.get(digit).unwrap().to_string();
        }else{
            big_digit += &get_time_in_big_ascii(&digit);
            ascii_hash.insert(digit.to_string(), big_digit.clone());
        }

        if time_string_vector.len() < 1 {
            time_string_vector = 
                vec![
                    "".to_string();
                    big_digit.split("\n").count()
                ];
        }

        for (j, y) in big_digit.split("\n").enumerate() {
            time_string_vector[j] = time_string_vector[j].to_string() + &*y.to_string() + &*" ".repeat(10 - y.len());
        }

    }

    print_string_vector(time_string_vector);
    ascii_hash.clone()  
}

fn get_time_in_big_ascii(mut filename : &str) -> String{
    if filename == ":" { filename = "seperate"}
    fs::read_to_string("digits/".to_string() + &*filename.to_string() + ".txt")
        .expect("Failed to read file")
}

fn print_string_vector(vector: Vec<String>){
    for x in vector {
        println!("{}", format!("{}", x).purple());
    }
}
