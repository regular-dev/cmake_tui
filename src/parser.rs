use regex::Regex;
use std::process;

#[derive(PartialEq)]
pub enum CmakeTypeVal {
    Boolean,
    String,
    Path,
}

pub struct FlagEntry {
    pub name: String,
    pub type_val: CmakeTypeVal,
    pub value: String,
    pub desc: String,
}

pub struct Parser {
    pub entries: Vec<FlagEntry>,
    pub cmake_path: String,
}

impl Parser {
    pub fn new(path: &str) -> Parser {
        Parser {
            entries: Vec::new(),
            cmake_path: String::from(path),
        }
    }

    pub fn parse_folder(&mut self) {
        let cmake_output = process::Command::new("cmake")
            .arg("-LH")
            .arg(self.cmake_path.as_str())
            .output()
            .expect("failed to execute process");

        let string_out = String::from_utf8_lossy(&cmake_output.stdout);

        let list_out = string_out.split("\n");
        let reg_exp_split = Regex::new(r"[:]|[=]").unwrap();
        let reg_exp_check = Regex::new(r"\w+:\w+=").unwrap();
        let reg_exp_desc_check = Regex::new(r"(//)").unwrap();
        // TODO : check for regExp \W : \W = \= eq

        self.entries.clear();

        let mut it_desc = "";

        for i in list_out {
            let check_desc = reg_exp_desc_check.is_match(i);

            if check_desc {
                it_desc = &i[3..i.len()];
            }

            let check_reg = reg_exp_check.is_match(i);

            if !check_reg {
                continue;
            }

            let captures: Vec<&str> = reg_exp_split.split(i).collect();

            if captures.len() == 3 {
                self.entries.push(FlagEntry {
                    name: captures[0].to_string(),
                    type_val: CmakeTypeVal::Boolean,
                    value: captures[2].to_string(),
                    desc: String::from(it_desc),
                });

                it_desc = "";

                if captures[1] == "STRING" {
                    self.entries.last_mut().unwrap().type_val = CmakeTypeVal::String;
                }

                if captures[1] == "PATH" {
                    self.entries.last_mut().unwrap().type_val = CmakeTypeVal::Path;
                }
            }
        }
    }

    pub fn run_cmake(&self, opts_val: &Vec<String>) -> process::Command {
        let mut opts: Vec<String> = Vec::new();

        for (num, _i) in self.entries.iter().enumerate() {
            if self.entries[num].value != opts_val[num] {
                opts.push(format!("-D{}={}", self.entries[num].name, opts_val[num]));
            }
        }

        let mut cmake_runner = process::Command::new("cmake");
        cmake_runner.arg(self.cmake_path.as_str());
        cmake_runner.args(opts);

        cmake_runner
    }
}
