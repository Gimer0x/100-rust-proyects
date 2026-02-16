use notify::{Config, EventHandler, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;

/// Forwards watcher events into the channel so the main loop can receive them.
struct ChannelHandler(std::sync::mpsc::Sender<notify::Result<notify::Event>>);

impl EventHandler for ChannelHandler {
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        let _ = self.0.send(event);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file-or-directory-to-watch>", args[0]);
        return;
    }

    let path = &args[1];
    println!("Watching for changes in: {}", path);

    let (tx, rx) = channel();

    let config = Config::default().with_poll_interval(Duration::from_secs(1));
    let mut watcher = RecommendedWatcher::new(ChannelHandler(tx), config).unwrap();

    watcher.watch(Path::new(path), RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(e) => {
                    if let EventKind::Modify(_) = e.kind {
                        println!("File modified: {:?}", e.paths);
                    } else {
                        println!("Event: {:?}", e);
                    }
                }
                Err(e) => eprintln!("watch error: {:?}", e),
            },
            Err(_) => break,
        }
    }
}
