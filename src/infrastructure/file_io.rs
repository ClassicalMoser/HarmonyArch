//! File I/O utilities for common file operations

use std::io::{Write, Result};
use std::fs::File;

/// Common file writing operations
pub struct FileWriter;

impl FileWriter {
    /// Create a new file for writing
    pub fn create_file(filename: &str) -> Result<File> {
        File::create(filename)
    }

    /// Write a line to a file
    pub fn write_line(file: &mut File, content: &str) -> Result<()> {
        writeln!(file, "{}", content)
    }

    /// Write raw bytes to a file
    pub fn write_bytes(file: &mut File, bytes: &[u8]) -> Result<()> {
        file.write_all(bytes)
    }

    /// Write formatted content to a file
    pub fn write_formatted<F>(file: &mut File, formatter: F) -> Result<()>
    where
        F: FnOnce(&mut File) -> Result<()>,
    {
        formatter(file)
    }
} 