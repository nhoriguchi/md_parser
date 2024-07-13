// TODO: タグや優先度への対応、ID

use chrono::NaiveDateTime;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(unused_variables)]
#[derive(PartialEq)]
struct Section {
    id: usize,
    file: String,
    lineno: usize,
    title: String,
    full_title: String,
    level: usize,
    content: String,
    todo: bool,
    wip: bool,
    wait: bool,
    done: bool,
    dont: bool,
    timestamps: Vec<String>,
}

impl Section {
    fn new(
        id: usize,
        file: String,
        lineno: usize,
        title: String,
        full_title: String,
        level: usize,
    ) -> Self {
        Self {
            id,
            file,
            lineno,
            title,
            full_title,
            level,
            content: String::new(),
            todo: false,
            wip: false,
            wait: false,
            done: false,
            dont: false,
            timestamps: Vec::new(),
        }
    }

    fn add_content(&mut self, line: &str) {
        self.content.push_str(line);
        self.content.push('\n');
    }

    fn contains_keyword(&self, keyword: &str) -> bool {
        self.content.contains(keyword)
    }

    fn parse_timestamps(&mut self) {
        let pattern = r"\(\d{4}/\d{2}/\d{2} \d{2}:\d{2}\)";
        let re = Regex::new(pattern).unwrap();

        for mat in re.find_iter(&self.content) {
            self.timestamps.push(mat.as_str().to_string());
        }
        self.timestamps.sort();
        if self.timestamps.is_empty() {
            self.timestamps.push("(2000/01/01 00:00)".to_string());
        }
    }

    fn parse_status(&mut self) {
        if self.contains_keyword("*TODO*") {
            self.todo = true;
        }
        if self.contains_keyword("*WIP*") {
            self.wip = true;
        }
        if self.contains_keyword("*WAIT*") {
            self.wait = true;
        }
        if self.contains_keyword("*DONE*") {
            self.done = true;
        }
        if self.contains_keyword("*DONT*") {
            self.dont = true;
        }
        self.parse_timestamps();
    }

    fn print_summary_line(&self) {
        let binding = self.file.clone();
        let path = Path::new(&binding);
        let basename = path.file_name().unwrap().to_string_lossy();

        let created_at =
            NaiveDateTime::parse_from_str(self.timestamps.first().unwrap(), "(%Y/%m/%d %H:%M)")
                .unwrap();
        println!(
            "  {:.6}:L{:<4}: {} {}",
            basename,
            self.lineno,
            created_at.format("%y%m%d_%H%M").to_string(),
            self.short_title()
        );
    }

    fn short_title(&self) -> String {
        let mut result = self.full_title.clone();
        let pattern = r"\(\d{4}/\d{2}/\d{2} \d{2}:\d{2}\) ?";
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, "").to_string();
        let pattern = r"\*(TODO|WIP|WAIT|DONE|DONT)\* ?";
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, "").to_string();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_title() {
        let my_section = Section::new("".to_string(), 10, "test title".to_string(), 2);
        assert_eq!(my_section.short_title(), "test title");
        let mut my_section = Section::new(
            "".to_string(),
            10,
            "(2024/07/13 04:38) test title".to_string(),
            2,
        );
        assert_eq!(my_section.short_title(), "test title");
        my_section = Section::new(
            "".to_string(),
            10,
            "(2024/07/13 04:38) *TODO* test title".to_string(),
            2,
        );
        assert_eq!(my_section.short_title(), "test title");
        my_section = Section::new("".to_string(), 10, "*TODO* test title".to_string(), 2);
        assert_eq!(my_section.short_title(), "test title");
    }
}

fn show_markdown_section_summary(sections: &Vec<Section>) {
    println!("WAIT items:");
    for section in sections {
        if section.wait {
            section.print_summary_line();
        }
    }

    println!("");
    println!("WIP items:");
    for section in sections {
        if section.wip {
            section.print_summary_line();
        }
    }

    println!("");
    println!("TODO items:");
    for section in sections {
        if section.todo {
            section.print_summary_line();
        }
    }

    let key = "SHOW_CLOSED";
    let mut show_closed = false;
    if let Ok(val) = env::var(key) {
        if val == "true" {
            show_closed = true;
        }
    }
    if show_closed == false {
        return;
    }

    println!("");
    println!("DONT items:");
    for section in sections {
        if section.dont {
            section.print_summary_line();
        }
    }

    println!("");
    println!("DONE items:");
    for section in sections {
        if section.done {
            section.print_summary_line();
        }
    }
}

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: {} <mdfile> [<mdfile> ...]", args[0]);
        std::process::exit(1);
    }

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut global_sections = Vec::new();

    for arg in args.iter() {
        let sections = parse_markdown_file(&arg);
        global_sections.extend(sections);
    }

    global_sections.sort_by(|a, b| {
        b.timestamps
            .last()
            .unwrap()
            .cmp(&a.timestamps.last().unwrap())
    });

    show_markdown_section_summary(&global_sections);
}

fn parse_markdown_file<'a>(file_path: &str) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    let mut current_section: Option<Section> = None;

    let mut section_map = HashMap::<usize, Section>::new();

    let markdown = fs::read_to_string(file_path).expect("Could not read file");

    let heading_re = Regex::new(r"^(#{1,6})\s+(.*)").unwrap();
    let mut section_stack: Vec<usize> = Vec::new();
    let mut id = 1;
    let mut prevlevel = 0;
    for (index, line) in markdown.lines().enumerate() {
        if let Some(caps) = heading_re.captures(line) {
            let level = caps[1].len();
            let title = caps[2].trim().to_string();
            let mut full_title = String::new();

            // Finalize previous section.
            if current_section != None {
                let section = current_section.unwrap();
                sections.push(section.clone());

                // println!("XXX3 id:{} level {}/{} {:?}", id, prevlevel, level, section_stack);
                if prevlevel >= level {
                    // println!("XXX4 truncated {:?} to {} items", section_stack, level - 1);
                    if level > 1 {
                        section_stack.truncate(level - 1);
                    } else {
                        section_stack = Vec::new();
                    }
                    // println!("XXX4B truncated stack {:?}", section_stack);
                } else {
                    section_stack.push(id);
                }
                section_map.insert(id, section);
                // println!("XXX5 {:?}", section_stack);

                let mut ve: Vec<_> = section_stack
                    .iter()
                    .map(|x| section_map.get(x).unwrap().title.clone())
                    .collect();
                ve.push(title.clone());
                full_title = ve.join(" / ");
                // println!("XXX7 level {}/{}, {}", prevlevel, level, full_title)
            }

            if id == 1 {
                full_title = title.clone();
            }

            current_section = Some(Section::new(
                id,
                file_path.to_string(),
                index + 1,
                title,
                full_title,
                level,
            ));

            id += 1;
            prevlevel = level;
        }

        if let Some(ref mut section) = current_section {
            section.add_content(line);
        }
    }

    // Finalize the last section.
    let section = current_section.unwrap();
    sections.push(section.clone());

    for section in &mut sections {
        section.parse_status();
    }
    sections.sort_by(|a, b| {
        b.timestamps
            .last()
            .unwrap()
            .cmp(&a.timestamps.last().unwrap())
    });

    sections
}
