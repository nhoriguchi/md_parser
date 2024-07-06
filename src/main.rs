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
    }

    fn parse_status(&mut self) {
        if self.contains_keyword("*TODO*") {
            self.todo = true;
        }
        if self.contains_keyword("*WIP*") {
            self.wip = true;
        }
        if self.contains_keyword("*DONE*") {
            self.done = true;
        }
        self.parse_timestamps();
    }

    fn print_summary_line(&self) {
        println!("  L{}: {}", self.lineno, self.title);
    }
}

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <markdown-file-path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let mut sections_with_keyword = find_sections_with_keyword(file_path);

    for section in &mut sections_with_keyword {
        section.parse_status();
    }

    println!("WIP items:");
    for section in &mut sections_with_keyword {
        if section.wip {
            section.print_summary_line();
        }
    }
    println!("");

    println!("TODO items:");
    for section in &mut sections_with_keyword {
        if section.todo {
            section.print_summary_line();
        }
    }
}

fn find_sections_with_keyword( file_path: &str) -> Vec<Section> {
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

    sections
}
