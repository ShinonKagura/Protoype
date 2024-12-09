use std::fs;
use std::path::PathBuf;
use std::sync::Once;
use chrono::{DateTime, Local, Duration};
use log::{LevelFilter, Log, Metadata, Record};
use anyhow::Result;

static INIT: Once = Once::new();

pub struct FileLogger {
    log_dir: PathBuf,
}

impl FileLogger {
    pub fn new(log_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&log_dir)?;
        let log_dir_display = log_dir.display().to_string();
        let logger = Self { log_dir };
        
        // Test write access
        let test_path = logger.get_log_file_path();
        let test_file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&test_path)?;
        test_file.sync_all()?;
        
        log::info!("FileLogger initialized with directory: {}", log_dir_display);
        Ok(logger)
    }

    fn get_log_file_path(&self) -> PathBuf {
        let now = Local::now();
        let filename = format!("smart_transfer_{}.log", now.format("%Y-%m-%d"));
        self.log_dir.join(filename)
    }

    fn write_log(&self, record: &Record) {
        let now = Local::now();
        let level = record.level();
        let target = record.target();
        let message = record.args();

        let log_entry = format!(
            "[{} {} {}] {}\n",
            now.format("%Y-%m-%d %H:%M:%S%.3f"),
            level,
            target,
            message
        );

        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.get_log_file_path())
        {
            use std::io::Write;
            if let Err(e) = file.write_all(log_entry.as_bytes()) {
                eprintln!("Failed to write log entry: {}", e);
            }
            if let Err(e) = file.sync_all() {
                eprintln!("Failed to sync log file: {}", e);
            }
        }
    }
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.write_log(record);
        }
    }

    fn flush(&self) {}
}

pub fn init_logging(log_dir: PathBuf) -> Result<()> {
    let mut result = Ok(());
    INIT.call_once(|| {
        // Ensure log directory exists
        if let Err(e) = fs::create_dir_all(&log_dir) {
            result = Err(anyhow::anyhow!("Failed to create log directory: {}", e));
            return;
        }

        log::info!("Initializing logging to directory: {}", log_dir.display());

        match FileLogger::new(log_dir) {
            Ok(logger) => {
                match log::set_boxed_logger(Box::new(logger)) {
                    Ok(()) => {
                        log::set_max_level(LevelFilter::Debug);
                        log::info!("Logging initialized successfully");
                    }
                    Err(e) => {
                        result = Err(anyhow::anyhow!("Failed to set logger: {}", e));
                    }
                }
            }
            Err(e) => {
                result = Err(anyhow::anyhow!("Failed to create logger: {}", e));
            }
        }
    });
    result
}

pub fn get_logs_directory() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        // During development, use project directory
        let project_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("logs");
        println!("Development mode: Using local logs directory: {:?}", project_dir);
        project_dir
    }

    #[cfg(not(debug_assertions))]
    {
        // In release mode, use app data directory
        let app_data = tauri::api::path::app_data_dir(&tauri::Config::default())
            .expect("Failed to get app data directory");
        app_data.join("logs")
    }
}

pub fn cleanup_old_logs(log_dir: PathBuf, days_to_keep: u64) -> Result<()> {
    let now = Local::now();
    let cleanup_date = now - Duration::days(days_to_keep as i64);

    for entry in fs::read_dir(log_dir)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Ok(metadata) = fs::metadata(&path) {
                let modified = metadata.modified().unwrap_or(std::time::UNIX_EPOCH);
                let modified_datetime: DateTime<Local> = DateTime::from(modified);
                if modified_datetime < cleanup_date {
                    fs::remove_file(path)?;
                }
            }
        }
    }
    Ok(())
}

pub mod macros {
    #[macro_export]
    macro_rules! log_operation_start {
        ($operation:expr) => {
            log::info!("Starting operation: {}", $operation);
        };
    }

    #[macro_export]
    macro_rules! log_operation_end {
        ($operation:expr, $result:expr) => {
            match $result {
                Ok(_) => log::info!("Operation completed successfully: {}", $operation),
                Err(e) => log::error!("Operation failed: {} - Error: {}", $operation, e),
            }
        };
    }

    #[macro_export]
    macro_rules! log_debug_value {
        ($name:expr, $value:expr) => {
            log::debug!("{}: {:?}", $name, $value);
        };
    }
}
