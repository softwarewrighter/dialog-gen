use crate::error::Result;
use crate::orchestrator::GeneratedDialog;
use std::fs;
use std::path::PathBuf;

pub struct OutputWriter {
    output_dir: PathBuf,
}

impl OutputWriter {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }

    /// Find next available output file number
    fn next_file_number(&self) -> Result<usize> {
        let mut max_num = 0;

        if let Ok(entries) = fs::read_dir(&self.output_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();

                // Extract number from "generated-dialogN.txt"
                if let Some(num_str) = name_str
                    .strip_prefix("generated-dialog")
                    .and_then(|s| s.strip_suffix(".txt"))
                    && let Ok(num) = num_str.parse::<usize>()
                {
                    max_num = max_num.max(num);
                }
            }
        }

        Ok(max_num + 1)
    }

    /// Write generated dialog to file
    pub fn write(&self, dialog: &GeneratedDialog) -> Result<PathBuf> {
        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;

        let num = self.next_file_number()?;
        let path = self.output_dir.join(format!("generated-dialog{}.txt", num));

        let mut content = String::new();
        for exchange in &dialog.exchanges {
            content.push_str(&format!("{}: {}\n\n", exchange.speaker, exchange.content));
        }

        fs::write(&path, content.trim_end())?;
        Ok(path)
    }
}
