use std::fmt::Display;

use calsc_utils::{
    fs::get_line_between_positions,
    str::{print_blank_line, print_line},
};
use colored::Colorize;

use crate::span::Span;

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = match get_line_between_positions(self.start.clone(), self.end.clone()) {
            Ok(v) => v,
            Err(e) => vec!["Cannot read file!".to_string(), format!("{}", e)],
        };

        let mut line_index = self.start.line;

        for line in lines {
            writeln!(f, "    {}    {}", "|".bright_magenta(), line)?;

            let underline;

            if self.start.line == self.end.line {
                underline = print_line(self.start.column, self.end.column, self.kind.get_char());
            } else if line_index == self.start.line {
                underline = print_line(self.start.column, line.len(), self.kind.get_char());
            } else if line_index == self.end.line {
                underline = print_line(0, self.end.column, self.kind.get_char());
            } else {
                underline = print_line(0, line.len(), self.kind.get_char());
            }

            writeln!(
                f,
                "    {}    {}",
                "|".bright_magenta(),
                underline.bright_yellow()
            )?;

            line_index += 1;
        }

        if let Some(label) = &self.label {
            let space = print_blank_line(self.start.column);
            writeln!(f, "    {}    {}{}", "|".bright_magenta(), space, label)?;
        }

        Ok(())
    }
}
