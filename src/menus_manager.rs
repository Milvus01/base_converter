use super::Language;
use serde_derive::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuList<'a> {
    pub title: &'a str,
    pub items: Vec<&'a str>,
}
impl<'a> MenuList<'a> {
    pub fn new(title: &'a str, items: Vec<&'a str>) -> Self {
        Self {
            title: title,
            items: items,
        }
    }
    pub fn print_menu(&self) {
        println!(" \u{1b}[34m{}:\u{1b}[39m", self.title); // title in blue
        for (i, j) in self.items.iter().enumerate() {
            println!("{:>5}. {}", i, j); // left 5 spaces before begin write text
        }
    }
    pub fn ask(&self, lang: &Language) -> Result<usize, String> {
        self.print_menu();
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect(lang.err_input);
        match input.trim().parse::<usize>() {
            Ok(x) => Ok(x),
            Err(_) => Err(format!("{}", lang.err_input)),
        }
    }
}
