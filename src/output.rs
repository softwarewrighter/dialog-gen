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

    /// Write generated dialog and metadata to files
    pub fn write(&self, dialog: &GeneratedDialog) -> Result<PathBuf> {
        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;

        let num = self.next_file_number()?;
        let dialog_path = self.output_dir.join(format!("generated-dialog{}.txt", num));
        let metadata_path = self.output_dir.join(format!("output-metadata{}.txt", num));

        // Write dialog
        let mut content = String::new();
        for exchange in &dialog.exchanges {
            content.push_str(&format!("{}: {}\n\n", exchange.speaker, exchange.content));
        }
        fs::write(&dialog_path, content.trim_end())?;

        // Write metadata
        let meta = &dialog.metadata;
        let metadata_content = format!(
            "Model: {}\n\
             Turns: {}\n\
             Temperature: {:.2}\n\
             \n\
             Prompt tokens: {}\n\
             Completion tokens: {}\n\
             Total tokens: {}\n\
             \n\
             Wall time: {:.2}s\n\
             Tokens/second: {:.1}\n",
            meta.model,
            meta.turns,
            meta.temperature,
            meta.total_prompt_tokens,
            meta.total_completion_tokens,
            meta.total_prompt_tokens + meta.total_completion_tokens,
            meta.total_wall_time.as_secs_f64(),
            meta.avg_tokens_per_second,
        );
        fs::write(&metadata_path, metadata_content)?;

        Ok(dialog_path)
    }

    /// Write edited podcast dialog to file
    pub fn write_edited(&self, dialog: &GeneratedDialog) -> Result<PathBuf> {
        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;

        let edited_path = self.output_dir.join("edited-podcast.txt");
        let metadata_path = self.output_dir.join("edited-metadata.txt");

        // Write edited dialog
        let mut content = String::new();
        for exchange in &dialog.exchanges {
            content.push_str(&format!("{}: {}\n\n", exchange.speaker, exchange.content));
        }
        fs::write(&edited_path, content.trim_end())?;

        // Write metadata
        let meta = &dialog.metadata;
        let metadata_content = format!(
            "Model: {}\n\
             Turns: {}\n\
             Temperature: {:.2}\n\
             \n\
             Prompt tokens: {}\n\
             Completion tokens: {}\n\
             Total tokens: {}\n\
             \n\
             Wall time: {:.2}s\n\
             Tokens/second: {:.1}\n",
            meta.model,
            meta.turns,
            meta.temperature,
            meta.total_prompt_tokens,
            meta.total_completion_tokens,
            meta.total_prompt_tokens + meta.total_completion_tokens,
            meta.total_wall_time.as_secs_f64(),
            meta.avg_tokens_per_second,
        );
        fs::write(&metadata_path, metadata_content)?;

        Ok(edited_path)
    }
}
