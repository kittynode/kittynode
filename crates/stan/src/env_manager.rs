use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

pub struct EnvManager {
    path: PathBuf,
    lines: Vec<String>,
    variables: HashMap<String, usize>, // key to line number mapping
}

impl EnvManager {
    pub fn new(env_path: &Path) -> io::Result<Self> {
        let file = File::open(env_path)?;
        let reader = io::BufReader::new(file);

        let mut lines = Vec::new();
        let mut variables = HashMap::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            lines.push(line.clone());

            if let Some((key, _)) = line.split_once('=') {
                variables.insert(key.trim().to_string(), i);
            }
        }

        Ok(EnvManager {
            path: env_path.to_path_buf(),
            lines,
            variables,
        })
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.variables.get(key).and_then(|&line_num| {
            self.lines.get(line_num).and_then(|line| {
                line.split_once('=')
                    .map(|(_, value)| value.trim().to_string())
            })
        })
    }

    pub fn set(&mut self, key: String, value: String) {
        match self.variables.get(&key) {
            Some(&line_num) => {
                self.lines[line_num] = format!("{}={}", key, value);
            }
            None => {
                self.lines.push(format!("{}={}", key, value));
                self.variables.insert(key, self.lines.len() - 1);
            }
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        for line in &self.lines {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }
    pub fn update_from_sample(&mut self, sample_path: &Path) -> io::Result<()> {
        let sample_file = File::open(sample_path)?;
        let reader = io::BufReader::new(sample_file);

        // Initialize a new set of lines for the updated .env content
        let mut new_lines = Vec::new();

        // Existing keys in the current .env
        let mut existing_keys = self.variables.clone();

        // Process lines from .env.sample
        for line in reader.lines().map_while(Result::ok) {
            let trimmed_line = line.trim();

            // Add comments and blank lines directly to new_lines
            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                new_lines.push(line);
                continue;
            }

            // Process key-value pairs
            if let Some((key, sample_value)) = line.split_once('=') {
                let key = key.trim();
                let sample_value = sample_value.trim();
                let existing_line_num = existing_keys.get(key);

                match (existing_line_num, sample_value.is_empty()) {
                    (Some(&line_num), true) => {
                        // Retain value from .env if sample value is empty
                        new_lines.push(self.lines[line_num].clone());
                    }
                    (Some(_), false) => {
                        // Update value if sample has non-empty value
                        new_lines.push(format!("{}={}", key, sample_value));
                    }
                    (None, _) => {
                        // Key is in .env.sample but not in .env, add it in correct position
                        new_lines.push(format!("{}={}", key, sample_value));
                    }
                }

                // Remove the processed key from existing_keys
                if existing_line_num.is_some() {
                    existing_keys.remove(key);
                }
            }
        }

        // Replace old lines with new_lines
        self.lines = new_lines;

        self.save()
    }
}
