use std::{fs, io, path::Path, sync::RwLock};

pub struct Logger;
impl Logger {
  fn log(&self, l_level: LogLevel, msg: String) {
    let res: io::Result<()> = match l_level {
      LogLevel::Info => {
        println!("12");
        fs::write(Path::new("./log.txt"), format!("[INFO]: {}\n", msg))
      }

      _ => todo!(),
    };


    match res {
      Err(err) => {
        eprintln!("Failed to log data: {}", err)
      }

      _ => {}
    };
  }

  pub fn info(&self, msg: String) {
    return self.log(LogLevel::Info, msg);
  }
}
enum LogLevel {
  Info,
  Warn,
  Error,
}

thread_local! {
  pub static LOGGER: RwLock<Logger> = RwLock::new(Logger);
}
