use tokio::time;

#[tokio::main]
async fn main() {
    let bpm = 200;
    let time_signature = 4;
    
    let interval = time::interval(time::Duration::from_millis(bpm_to_ms(bpm)));
    tokio::pin!(interval);


    loop {
        let mut beat_counter = 0;
        for _i in 0..time_signature {
            interval.as_mut().tick().await;
            beat_counter += 1;
            println!("{}", beat_counter);
        }
    }
}

fn bpm_to_ms(bpm: u64) -> u64 {
    //  1 min => 60 seconds => 60000 ms
    60000 / bpm
}
