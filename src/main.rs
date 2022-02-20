use std::{io, time, thread, fs};

fn main() {

    let mut start_time = get_start_time();
    
    while start_time != 0{
        clear_screen();
        print_time_left(start_time);
        thread::sleep(time::Duration::from_secs(1));
        start_time = start_time-1;
    }
    
}

fn get_start_time() -> i32{
    let start_time: i32; 
    let mut input_value = String::new();
    println!("{}", "Please enter the countdown start time");
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

fn print_time_left(time: i32){
    let mut time_string_vector: Vec<String> = Vec::new();

    for digit in time.to_string().split("") {
        if digit == "" { continue }

        let big_digit = get_time_in_big_ascii(&digit);

        if time_string_vector.len() < 1 {
            time_string_vector = vec!["".to_string(); big_digit.split("\n").count()];
        }

        for (j, y) in big_digit.split("\n").enumerate() {
            time_string_vector[j] = time_string_vector[j].to_string() + &*y.to_string() + &*" ".repeat(10 - y.len());
        }
    }

    print_string_vector(time_string_vector);
}

fn get_time_in_big_ascii(filename : &str) -> String{
    fs::read_to_string("digits/".to_string() + &*filename.to_string() + ".txt")
        .expect("Failed to read file")
}

fn print_string_vector(vector: Vec<String>){
    for x in vector {
        println!("{}", x);
    }
}