use serde_derive::{Deserialize, Serialize};
use std::fs::*;
// use serde_json;
use super::menus_manager::MenuList;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language<'a> {
    pub language: &'a str,
    pub welcome: &'a str,
    #[serde(borrow)]
    pub global_menu: MenuList<'a>,
    pub convert_menu: MenuList<'a>,
    pub config_menu: MenuList<'a>,
    pub lang_menu_title: &'a str,
    pub err_number: &'a str,
    pub pause: &'a str,
    pub not_implemented: &'a str,
    pub input_numbers: [&'a str; 2],
    pub spaces: &'a str,
    pub err_input: &'a str,
    pub hex_input: &'a str,
    pub dec_input: &'a str,
    pub bin_input: &'a str,
    pub ascii_input: &'a str,
    pub separator_input: &'a str,
    pub err_separator: &'a str,
    pub err_separator_nb: [&'a str; 2],
}
impl<'a> Language<'a> {
    pub fn load(&self) -> Result<Vec<&str>, ()> {
        let list = read_dir("./")
            .into_iter()
            .map(|path| path)
            .collect::<Vec<ReadDir>>();
        for i in list.into_iter() {
            println!("{:?}", i);
            /* let mut file = File::open(i).unwrap();
            let mut raw = String::new();
            file.read_to_string(&mut raw);
            serde_json::from_str(raw.as_str()); */
        }
        Ok(vec![self.clone().language])
    }
}
impl<'a> Default for Language<'a> {
    fn default() -> Self {
        Self {
            language: "en",
            welcome: "Welcome in base_converter.\nPlease read the instructions and navigate in menus by answer the questions.\n",
            global_menu: MenuList {
                title: "Global menu",
                items: vec!["Quit", "Convert", "Configure"],
            },
            convert_menu: MenuList {
                title: "Convert menu",
                items: vec!["Quit", "Back", "Hex", "Dec", "Bin", "ASCII"],
            },
            config_menu: MenuList {
                title: "Configuration menu",
                items: vec!["Quit", "Back", "Change language", "Change separator"],
            },
            lang_menu_title: "Language configuration menu",
            err_number: "ERROR please enter a valid number",
            pause: "Press enter to continue...",
            not_implemented: "This function has not yet been implemated, please wait for future versions...",
            input_numbers: ["Please enter", "separated by"],
            spaces: "spaces",
            err_input: "ERROR during input",
            hex_input: "hexadecimal numbers (between 00 and ff)",
            dec_input: "decimal numbers (between 0 and 255)",
            bin_input: "binary numbers (between 00000000 and 11111111)",
            ascii_input: "Please enter text.",
            separator_input: "Please enter a single character (if you enter several only the first will be considered)",
            err_separator: "You have not enter a character.",
            err_separator_nb:["You have enter", "and you can't use it because it is a number (include hexadecimal)"] 
        }
    }
}
