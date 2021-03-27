use core::panic;
use std::fs::File;

pub struct Writer {
    columns: u16,
    csv_writer: csv::Writer<File>,
}

#[allow(dead_code)]
impl Writer {
    pub fn new(file_path: &str, columns: u16) -> Self {
        let csv_writer = csv::Writer::from_path(file_path)
            .expect(&format!("Failed to open writer to file: {}", file_path));

        Self {
            columns,
            csv_writer,
        }
    }

    pub fn with_header(file_path: &str, header: &[&str]) -> Self {
        let columns = header.len();
        if columns > u16::MAX as usize {
            panic!(
                "Too many columns in the header. Max number is {}.",
                u16::MAX
            );
        }

        let columns = columns as u16;
        let mut csv_writer = csv::Writer::from_path(file_path)
            .expect(&format!("Failed to open writer to file: {}", file_path));

        if !csv_writer.write_record(header).is_ok() {
            panic!("Failed to write header row.");
        }

        Self {
            columns,
            csv_writer,
        }
    }

    pub fn write_row(&mut self, row: &[&str]) -> bool {
        if row.len() != self.columns as usize {
            return false;
        }

        self.csv_writer.write_record(row).is_ok()
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        let _ = self.csv_writer.flush();
    }
}
