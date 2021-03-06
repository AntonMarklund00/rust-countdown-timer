use std::{io, time, thread, fs};
use std::collections::HashMap;

fn main() {

    println!("Please select amount of hours!");
    let start_time_hours = get_start_time();
    println!("Please select amount of minutes!");
    let start_time_minutes= get_start_time();
    println!("Please select amount of seconds!");
    let start_time_seconds= get_start_time();

    let mut time_in_seconds = convert_time_to_seconds(start_time_hours, start_time_minutes, start_time_seconds);

//     let start = Instant::now();


    let mut ascii_hash:HashMap<String, String> = HashMap::new();

    while time_in_seconds > -1{
        let countdown_thread = thread::spawn(|| {
            thread::sleep(time::Duration::from_secs(1));
        });

        countdown_thread.join().map_err(|err| println!("{:?}", err)).ok();

        clear_screen();
        
        ascii_hash = print_time_left(time_in_seconds, ascii_hash);
        time_in_seconds = time_in_seconds-1;

    }

    // let duration = start.elapsed();
    // println!("{:?}", duration);
    
}

fn convert_time_to_seconds(hours: i32, minutes: i32, seconds: i32) -> i32{
    hours*3600 + minutes*60 + seconds
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

fn print_time_left(mut time: i32, mut ascii_hash: HashMap<String, String>) -> HashMap<String, String>{
    let mut time_string_vector: Vec<String> = Vec::new();

    let hours = time / 3600;
    time = time - (hours * 3600);
    let minutes = time / 60;
    time = time - (minutes / 60);
    let seconds = time % 60;

    let time_string = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    for digit in time_string.to_string().split("") {
        if digit == "" { continue }

        let mut big_digit = String::new(); 

        if ascii_hash.contains_key(digit) {
            big_digit = ascii_hash.get(digit).unwrap().to_string();
        }else{
            big_digit = get_time_in_big_ascii(&digit);
            ascii_hash.insert(digit.to_string(), big_digit.clone());
        }

        if time_string_vector.len() < 1 {
            time_string_vector = vec!["".to_string(); 
            big_digit.split("\n").count()];
        }

        for (j, y) in big_digit.split("\n").enumerate() {
            time_string_vector[j] = time_string_vector[j].to_string() + &*y.to_string() + &*" ".repeat(10 - y.len());
        }
    }

    print_string_vector(time_string_vector);
    ascii_hash  
}

fn get_time_in_big_ascii(mut filename : &str) -> String{
    if filename == ":" { filename = "seperate"}
    fs::read_to_string("digits/".to_string() + &*filename.to_string() + ".txt")
        .expect("Failed to read file")
}

fn print_string_vector(vector: Vec<String>){
    for x in vector {
        println!("{}", x);
    }
}

