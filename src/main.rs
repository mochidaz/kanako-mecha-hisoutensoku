mod app;

use std::time::Duration;

use app::{App, ErrorKind};

use clokwerk::{AsyncScheduler, TimeUnits};

use tokio;

async fn task() {

    let main_app = App::default();

    match main_app.post().await {
        Ok(_) => println!("Kanako is posting!"),
        Err(e) => println!("Error okkued! -> {}", e),
    };

}

#[tokio::main]
async fn main() -> Result<(), ErrorKind> {

    println!("Kanako is starting...");

    let mut sched = AsyncScheduler::new();
    sched.every(30.minutes()).run(task);

    println!("Kanako has been started!");

    loop {
        sched.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
