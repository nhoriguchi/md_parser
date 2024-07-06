use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <markdown-file-path> <keyword>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let keyword = &args[2];

    // Markdownファイルを読み込む
    let markdown = fs::read_to_string(file_path).expect("Could not read file");

    let sections_with_keyword = find_sections_with_keyword(&markdown, keyword);

    for (heading, contains_keyword) in sections_with_keyword {
        println!("Section '{}' contains keyword '{}': {}", heading, keyword, contains_keyword);
    }
    // let html = parse_markdown(&markdown);
    // println!("{}", html);
}

fn find_sections_with_keyword(markdown: &str, keyword: &str) -> Vec<(String, bool)> {
    let mut sections: Vec<(String, bool)> = Vec::new();
    let mut section_stack: Vec<(String, usize)> = Vec::new(); // (section_title, heading_level)
    let mut current_text = String::new();

    let heading_re = Regex::new(r"^(#{1,6})\s+(.*)").unwrap();
    for line in markdown.lines() {
        if let Some(caps) = heading_re.captures(line) {
            let level = caps[1].len();
            let heading = caps[2].trim().to_string();

            // 現在のセクションを保存
            if !section_stack.is_empty() {
                let (current_section, _) = section_stack.last().unwrap();
                let contains_keyword = current_text.contains(keyword);
                sections.push((current_section.clone(), contains_keyword));
            }

            // 現在のテキストをクリア
            current_text.clear();

            // スタックを調整して新しいセクションを追加
            while !section_stack.is_empty() && section_stack.last().unwrap().1 >= level {
                section_stack.pop();
            }
            section_stack.push((heading.clone(), level));
        } else {
            current_text.push_str(line);
            current_text.push('\n');
        }
    }

    // 最後のセクションを保存
    if !section_stack.is_empty() {
        let (current_section, _) = section_stack.last().unwrap();
        let contains_keyword = current_text.contains(keyword);
        sections.push((current_section.clone(), contains_keyword));
    }

    sections
}

// fn find_sections_with_keyword_old(markdown: &str, keyword: &str) -> HashMap<String, bool> {
//     let mut sections: HashMap<String, bool> = HashMap::new();
//     let mut current_section = String::new();
//     let mut current_text = String::new();
// 
//     println!("<{}>", keyword);
// 
//     // // キーワードを正規表現パターンに変換
//     // let regex_pattern = regex::escape(keyword).replace(r"\*", ".*").replace(r"\?", ".");
//     // let keyword_re = Regex::new(&regex_pattern).expect("Invalid regex pattern");
//     let heading_re = Regex::new(r"^#{1,6}\s+.*").unwrap();
// 
//     for line in markdown.lines() {
//         // println!(">>> {} -- {}", line, line.contains(keyword));
//         if heading_re.is_match(line) {
//         // if line.contains(keyword) {
//             println!(">>> {} -- {}", line, line.contains(keyword));
//             if !current_section.is_empty() {
//                 let contains_keyword = current_text.to_lowercase().contains(keyword);
//                 sections.insert(current_section.clone(), contains_keyword);
//             }
//             current_section = line.trim().to_string();
//             current_text.clear();
//         } else {
//             current_text.push_str(line);
//             current_text.push('\n');
//         }
//     }
//     println!("? {}", current_section);
// 
//     if !current_section.is_empty() {
//         let contains_keyword = current_text.contains(keyword);
//         sections.insert(current_section, contains_keyword);
//     }
// 
//     sections
// }

// fn parse_markdown(markdown: &str) -> String {
//     let mut html = String::new();
// 
//     for line in markdown.lines() {
//         let line = line.trim();
//         if line.starts_with("# ") {
//             html.push_str(&format!("<h1>{}</h1>", &line[2..]));
//         } else if line.starts_with("## ") {
//             html.push_str(&format!("<h2>{}</h2>", &line[3..]));
//         } else if line.starts_with("### ") {
//             html.push_str(&format!("<h3>{}</h3>", &line[4..]));
//         } else {
//             let line = parse_bold(&line);
//             let line = parse_links(&line);
//             html.push_str(&format!("<p>{}</p>", line));
//         }
//     }
// 
//     html
// }
// 
// fn parse_bold(text: &str) -> String {
//     let re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
//     re.replace_all(text, "<strong>$1</strong>").to_string()
// }
// 
// fn parse_links(text: &str) -> String {
//     let re = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
//     re.replace_all(text, r#"<a href="$2">$1</a>"#).to_string()
// }