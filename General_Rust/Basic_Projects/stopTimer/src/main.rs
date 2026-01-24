use std::time::{Instant, Duration};
use std::thread;
use std::io::{self, Write};

fn main(){
    println!("Rust Stop Watch program\n");
    println!("Use ctrl+c to stop");


    let start_timer = Instant::now();
    loop{
        let elapsed_time = start_timer.elapsed();
        let time_as_seconds = elapsed_time.as_secs();
        let time_as_milliseconds = elapsed_time.subsec_millis();
        print!("\rTime Elapsed: {}.{:03} seconds", time_as_seconds, time_as_milliseconds);
        io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(50));
    }

}

