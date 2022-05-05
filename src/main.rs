mod app;

use std::time::Duration;

use app::{App, ErrorKind};

use clokwerk::{AsyncScheduler, TimeUnits};

use tokio;

async fn task() {

    let main_app = App::default();

    match main_app.post().await {
        Ok(_) => (),
        Err(_) => println!("Error occured!"),
    };

}

#[tokio::main]
async fn main() -> Result<(), ErrorKind> {

    let mut sched = AsyncScheduler::new();
    sched.every(30.minutes()).run(task);

    loop {
        sched.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
