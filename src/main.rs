use std::io::{stdin, stdout, Write};
use std::process::exit;
pub mod converter;
pub use converter::*;
pub mod menus_manager;
pub use menus_manager::*;
pub mod language;
pub use language::*;

// print!("\x1b[2J\x1b[1;1H"); // clear screen
/*
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'
*/
fn main() {
    let mut lang = Language::default();
    // _test();
    println!("{}", lang.welcome);
    let global_menu = lang.global_menu.clone();
    let mut my_list = List::default();
    loop {
        match global_menu.ask(&lang) {
            Ok(x) => match x {
                0 => exit(0),
                1 => convert(&mut my_list, &lang),
                2 => configure(&mut my_list, &mut lang),
                _ => println!("{}", lang.err_number),
            },
            Err(e) => println!("{e}"),
        }
    }
}
fn _test() {
    // -- print all the table -- //
    let list = List {
        text: (0..=255).collect(),
        ..Default::default()
    };
    println!("{}", list.format_hex());
    println!("{}", list.format_dec());
    println!("{}", list.format_bin());
    println!("{}", list.format_ascii());
}
pub fn pause(lang: &Language) {
    let stdin = stdin();
    write!(stdout(), "{}", lang.pause).unwrap();
    stdout().flush().unwrap();
    stdin.read_line(&mut String::new()).unwrap();
}
pub fn convert(list: &mut List, lang: &Language) {
    let convert_menu = &lang.convert_menu;
    loop {
        match convert_menu.ask(&lang) {
            Ok(x) => match x {
                0 => exit(0),
                1 => return,
                2 => {
                    list.ask_hex(&lang);
                    println!("{}", list.format_dec());
                    println!("{}", list.format_bin());
                    println!("{}", list.format_ascii());
                }
                3 => {
                    list.ask_dec(&lang);
                    println!("{}", list.format_hex());
                    println!("{}", list.format_bin());
                    println!("{}", list.format_ascii());
                }
                4 => {
                    list.ask_bin(&lang);
                    println!("{}", list.format_hex());
                    println!("{}", list.format_dec());
                    println!("{}", list.format_ascii());
                }
                5 => {
                    list.ask_ascii(&lang);
                    println!("{}", list.format_hex());
                    println!("{}", list.format_dec());
                    println!("{}", list.format_bin());
                }
                _ => println!("{}", lang.err_number),
            },
            Err(e) => println!("{e}"),
        }
    }
}
pub fn configure(list: &mut List, lang: &mut Language) {
    let config_menu = &lang.config_menu;
    loop {
        match config_menu.ask(&lang) {
            Ok(x) => match x {
                0 => exit(0),
                1 => return,
                2 => println!("{}", lang.not_implemented),
                3 => list.config_separator(&lang),
                _ => println!("{}", lang.err_number),
            },
            Err(e) => println!("{e}"),
        }
    }
}
pub fn change_language(lang: &mut Language) {
    let lang_menu = MenuList::new(lang.lang_menu_title, lang.load().unwrap());
    match lang_menu.ask(&lang) {
        Ok(x) => match x {
            _ => todo!()
        }
        Err(e) => println!("{e}")
    }






}
