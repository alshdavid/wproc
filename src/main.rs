use std::{path::PathBuf, sync::mpsc::channel, thread};

use clap::Parser;

mod watcher;

#[derive(Debug, Parser)]
struct Command {
  /// Command to run in sub shell
  command: Vec<String>,

  /// File/Folder to watch
  #[arg(short='w', long = "watch")]
  watch: PathBuf,
}

fn main() -> anyhow::Result<()> {
  let cmd = Command::parse();

  dbg!(&cmd);

  let w = watcher::Watcher::new(cmd.watch)?;

  let (tx, rx) = channel::<()>();

  let mut command = cmd.command;
  thread::spawn(move || -> anyhow::Result<()> {
    let arg0 = command.remove(0);
    let args = command.clone();
    let mut child = None::<std::process::Child>;

    while let Ok(_) = rx.recv() {
      if let Some(mut child) = child.take() {
        child.kill()?;
      }
      let mut proc = std::process::Command::new(&arg0);
      proc.args(&args);
      proc.stdout(std::process::Stdio::inherit());
      proc.stderr(std::process::Stdio::inherit());
      child = Some(proc.spawn()?);
    }

    Ok(())
  });

  tx.send(())?;

  while let Ok(update) = w.rx_watch.recv() {
    println!("Update");
    dbg!(&update);
    tx.send(())?;
  }

  Ok(())
}
