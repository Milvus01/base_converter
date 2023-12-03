use super::Language;
use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone)]
pub struct List {
    pub text: Vec<u8>,
    pub separator: char,
}
impl Default for List {
    fn default() -> Self {
        Self {
            text: Vec::default(),
            separator: ' ',
        }
    }
}
impl List {
    pub fn format_hex(&self) -> String {
        format!(
            "HEX: {}",
            self.text
                .iter()
                .map(|i| { format!("{:02X} ", i) })
                .collect::<String>()
        )
    }
    pub fn format_dec(&self) -> String {
        format!(
            "DEC: {}",
            self.text
                .iter()
                .map(|i| { format!("{} ", i) })
                .collect::<String>()
        )
    }
    pub fn format_bin(&self) -> String {
        format!(
            "BIN: {}",
            self.text
                .iter()
                .map(|i| { format!("{:08b} ", i) })
                .collect::<String>()
        )
    }
    pub fn format_ascii(&self) -> String {
        format!(
            "ASCII: {}",
            self.text
                .iter()
                .map(|i| { (*i as char).to_string() })
                .collect::<String>()
        )
    }
    fn general_number_input(&self, text: (&str, &str), base: u8, lang: &Language) -> Vec<u8> {
        let mut input = String::new();
        while input.len() <= 1 {
            print!(
                "{} {} {} {}.\n{} > ",
                lang.input_numbers[0],
                text.0,
                lang.input_numbers[1],
                if self.separator == ' ' {
                    format!("{}", lang.spaces)
                } else {
                    format!("`{}`", self.separator)
                },
                text.1
            );
            stdout().flush().unwrap();
            stdin().read_line(&mut input).expect(lang.err_input);
        }
        let vec: Vec<u8> = input
            .trim()
            .split(self.separator)
            .map(|x| match u8::from_str_radix(x, base as u32) {
                Ok(number) => number,
                Err(_) => 255,
            })
            .collect();
        vec
    }
    pub fn ask_hex(&mut self, lang: &Language) {
        self.text.drain(..);
        self.text
            .append(&mut self.general_number_input((lang.hex_input, "HEX"), 16, lang));
    }
    pub fn ask_dec(&mut self, lang: &Language) {
        self.text.drain(..);
        self.text
            .append(&mut self.general_number_input((lang.dec_input, "DEC"), 10, lang));
    }
    pub fn ask_bin(&mut self, lang: &Language) {
        self.text.drain(..);
        self.text
            .append(&mut self.general_number_input((lang.bin_input, "BIN"), 2, lang));
    }
    pub fn ask_ascii(&mut self, lang: &Language) {
        let mut input = String::new();
        while input.is_empty() {
            print!("{}\nASCII > ", lang.ascii_input);
            stdout().flush().unwrap();
            stdin().read_line(&mut input).expect(lang.err_input);
            input.pop();
        }
        let mut vec_ascii: Vec<u8> = input.as_bytes().into();
        self.text.drain(..);
        self.text.append(&mut vec_ascii);
    }
    pub fn config_separator(&mut self, lang: &Language) {
        let mut input = String::new();
        println!("{}", lang.separator_input);
        stdin().read_line(&mut input).expect(lang.err_input);
        match *input.as_bytes().first().unwrap() as char {
            '\n' => {
                println!("{}", lang.err_separator);
                return;
            }
            x if x.is_digit(16) => println!(
                "{} `{}` {}",
                lang.err_separator_nb[0], x, lang.err_separator_nb[1]
            ),
            x => self.separator = x,
        }
    }
}
