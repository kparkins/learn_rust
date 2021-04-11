
use std::str::SplitWhitespace;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug)]
pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(String, String),
    List(String),
    Invalid,
    Exit,
}

fn parse_add(tokens: &mut SplitWhitespace) -> Command {
    let employee = match tokens.next() {
        Some(v) => String::from(v),
        None => return Command::Invalid, 
    };
    match tokens.next() {
        Some("to") => (),
        _ => return Command::Invalid,
    }
    let department = match tokens.next() {
        Some(v) => String::from(v),
        None => return Command::Invalid,
    };
    Command::Add(department, employee)
}

fn parse_list(tokens: &mut SplitWhitespace) -> Command {
    let department = match tokens.next() {
        Some(v) => String::from(v),
        None => String::new(), 
    };
    Command::List(department)
}

pub fn parse_command(s: &str) -> Command {
    let tokens = s.to_lowercase();
    let mut tokens = tokens.split_whitespace();

    match tokens.next() {
        Some("add") => parse_add(&mut tokens),
        Some("list") => parse_list(&mut tokens),
        Some("exit") => Command::Exit,
        _ => Command::Invalid, 
    }
}

impl Company {
    pub fn new() -> Company{
        Company {
            departments: HashMap::new(),
        }
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::Add(department, employee) => self.add_employee(&department, &employee),
            Command::List(department) => self.list_employees(&department),
            Command::Invalid => println!("invalid command"),
            _ => (),
        }
    }

    pub fn add_employee(&mut self, department: &str, employee: &str) {
        let key = department.to_string();
        let value = employee.to_string();
        match self.departments.entry(key) {
            Entry::Occupied(mut e) => {
                let entry = e.get_mut();
                entry.push(value);
            }
            Entry::Vacant(e) => { 
                e.insert(vec!(value));
            }
        }
    }

    fn list_department_employees(&self, department: &str) {
        let mut employees = match self.departments.get(department) {
            Some(e) => e.clone(),
            None => return,
        };
        employees.sort();
        employees.iter().for_each(|e| println!("{}", e));
    }

    pub fn list_employees(&self, department: &String) {
        match department.as_str() {
            "" => self.departments.iter().for_each(
                |(k,_)| {
                    self.list_department_employees(k)
                }
            ),
            v => self.list_department_employees(&v),
        }
    }
}