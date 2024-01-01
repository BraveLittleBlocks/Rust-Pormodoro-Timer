use std::io;
use webbrowser;
use std::time::{Duration, Instant};

fn main() {
    loop {
        let youtube_url_start = "https://www.youtube.com/watch?v=-MaCJZIBKGs";
        let youtube_url_end = "https://youtu.be/2k0SmqbBIpQ?si=2jJzOp_ml2kAwvst&t=2U";
    
        //First input 
        println!("Choose a work timer duration:");
        println!("1. 5 minute");
        println!("2. 25 minutes");
        println!("3. Exit");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        let choice = input.trim().parse::<i32>().expect("Invalid input");
    
        let work_seconds = match choice {
            1 => 300,   // 5 minute
            2 => 1500,  // 25 minutes
            3 => break, // Exit the loop
            _ => {
                println!("Invalid option");
                continue; // Skip to the next iteration of the loop
            }
        };
        //Second input
        println!("Choose a Break timer duration:");
        println!("1. 5 minute");
        println!("2. 25 minutes");
        println!("3. Exit");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        let choice = input.trim().parse::<i32>().expect("Invalid input");
    
        let break_seconds = match choice {
            1 => 300,   // 5 minute
            2 => 600,  // 10 minutes
            3 => break, // Exit the loop
            _ => {
                println!("Invalid option");
                continue; // Skip to the next iteration of the loop
            }
        };

        webbrowser::open(&youtube_url_start).unwrap();

        //Work timer
        let target_time = Instant::now() + Duration::from_secs(work_seconds);
    
        while Instant::now() < target_time {
            // Print remaining time every second
            let remaining = target_time - Instant::now();
            println!("Time remaining: {} seconds", remaining.as_secs());
            std::thread::sleep(Duration::from_secs(1));
        }
        webbrowser::open(&youtube_url_end).unwrap(); // Open the URL after the countdown

        //break timer
        let target_time = Instant::now() + Duration::from_secs(break_seconds);
    
        while Instant::now() < target_time {
            // Print remaining time every second
            let remaining = target_time - Instant::now();
            println!("Time remaining: {} seconds", remaining.as_secs());
            std::thread::sleep(Duration::from_secs(1));
        }
    }
      
}



