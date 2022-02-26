use serde::Serialize;
use std::{
  io::{self, Read, BufReader, BufRead},
};

#[derive(Debug, Serialize)]
pub struct Entry {
  pub debug: usize,
  pub info: usize,
  pub warn: usize,
  pub error: usize,
  pub trace: usize,
  pub other: usize,
}

impl Entry {
  fn new() -> Self {
    Entry {
      debug: 0,
      info: 0,
      warn: 0,
      error: 0,
      trace: 0,
      other: 0
    }
  }
}

/// Line # in the logfile and the parsed log data for that line
#[derive(Debug, Serialize)]
pub struct LogEntry {
  pub total_lines: usize,
  pub line_severity: Entry,
}

impl LogEntry {
  pub fn new() -> Self {
    LogEntry {
      total_lines: 0,
      line_severity: Entry::new(),
    }
  }
}

pub fn parse_log_file<R: Read>(file: R) -> Result<LogEntry, io::Error> {
  let reader = BufReader::new(file);
  let mut log_info: LogEntry = LogEntry::new();

  for line in reader.lines() {
    match line? {
      s if s.contains("DEBUG") => log_info.line_severity.debug += 1,
      s if s.contains("INFO") => log_info.line_severity.info += 1,
      s if s.contains("ERROR") => log_info.line_severity.error += 1,
      s if s.contains("TRACE") => log_info.line_severity.trace += 1,
      s if s.contains("WARN") => log_info.line_severity.warn += 1,
      _ => log_info.line_severity.other += 1,
    }

    log_info.total_lines += 1;
  }

  Ok(log_info)
}