use regex::Regex;
use std::env;
use std::fs;

#[allow(dead_code)]

#[derive(Debug)]
#[allow(unused_variables)]
struct Section {
    file: String,
    lineno: usize,
    title: String,
    level: usize,
    content: String,
    todo: bool,
    wip: bool,
    wait: bool,
    done: bool,
    timestamps: Vec<String>,
}

impl Section {
    fn new(file: String, lineno: usize, title: String, level: usize) -> Self {
        Self {
            file,
            lineno,
            title,
            level,
            content: String::new(),
            todo: false,
            wip: false,
            wait: false,
            done: false,
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
        self.parse_timestamps();
    }

    fn print_summary_line(&self) {
        println!("  L{:<4}: {} {}", self.lineno, self.timestamps.last().unwrap(), self.title);
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <markdown-file-path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let sections = parse_markdown_file(file_path);

    show_markdown_section_summary(&sections);
}

fn parse_markdown_file(file_path: &str) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    let mut current_section: Option<Section> = None;

    let markdown = fs::read_to_string(file_path).expect("Could not read file");

    let heading_re = Regex::new(r"^(#{1,6})\s+(.*)").unwrap();
    for (index, line) in markdown.lines().enumerate() {
        if let Some(caps) = heading_re.captures(line) {
            let level = caps[1].len();
            let title = caps[2].trim().to_string();

            if let Some(section) = current_section.take() {
                sections.push(section);
            }

            current_section = Some(Section::new(file_path.to_string(), index + 1, title, level));
        }
        if let Some(ref mut section) = current_section {
            section.add_content(line);
        }
    }

    if let Some(section) = current_section {
        sections.push(section);
    }

    for section in &mut sections {
        section.parse_status();
    }

    sections.sort_by(|a, b| b.timestamps.last().unwrap().cmp(&a.timestamps.last().unwrap()));

    sections
}
