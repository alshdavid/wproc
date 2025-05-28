use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use notify_debouncer_full::new_debouncer;
use notify_debouncer_full::notify::EventKind;
use notify_debouncer_full::notify::RecommendedWatcher;
use notify_debouncer_full::notify::RecursiveMode;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::Debouncer;
use notify_debouncer_full::RecommendedCache;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;

pub struct Watcher {
  pub rx_watch: Receiver<Vec<PathBuf>>,
  _debouncer: Arc<Debouncer<RecommendedWatcher, RecommendedCache>>,
}

impl Watcher {
  pub fn new(target_dir: PathBuf) -> anyhow::Result<Self> {
    let (tx_watch, rx_watch) = channel::<Vec<PathBuf>>();
    let (tx_debounce, rx_debounce) = std::sync::mpsc::channel::<DebounceEventResult>();

    thread::spawn({
      move || {
        while let Ok(Ok(mut result)) = rx_debounce.recv() {
          let mut paths = vec![];

          while let Some(ev) = result.pop() {
            match ev.event.kind {
              EventKind::Create(_) => {}
              EventKind::Modify(_) => {}
              EventKind::Remove(_) => {}
              _ => continue,
            }
            paths.extend(ev.paths.clone());
          }

          if !paths.is_empty() {
            println!("[CNG]");
            tx_watch.send(paths).unwrap();
          }
        }
      }
    });

    let mut debouncer = new_debouncer(Duration::from_millis(1000), None, tx_debounce)?;
    debouncer.watch(&target_dir, RecursiveMode::Recursive)?;

    Ok(Self {
      rx_watch,
      _debouncer: Arc::new(debouncer),
    })
  }
}