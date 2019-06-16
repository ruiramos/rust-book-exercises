// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io::{self, Write};

enum Operation {
    Add(String, String),
    List(Option<String>),
    Delete(String),
    Help,
    Quit,
}

struct Person {
    name: String,
}

fn main() {
    let mut map: HashMap<String, Vec<Person>> = HashMap::new();

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read from stdin");

        match parse_command(input) {
            Some(Operation::Quit) => break,
            Some(Operation::Add(name, department)) => {
                println!("Adding {} to {}", name, department);
                let person = Person { name: name.clone() };
                let element: &mut Vec<Person> = map.entry(department).or_insert(Vec::new());
                element.push(person);
            }
            Some(Operation::List(filter)) => {
                if let Some(dept) = filter {
                    // list with a dept filter
                    if let Some(people) = map.get(&dept) {
                        println!("People in {}:", dept);
                        let mut sortedp: Vec<String> =
                            people.iter().map(|p| p.name.clone()).collect();
                        sortedp.sort();
                        for p in sortedp {
                            println!("  {}", p);
                        }
                    } else {
                        println!("Department {} not found", dept);
                    }
                } else {
                    // list without aditional filtering
                    println!("List of departments:");
                    for (dept, people) in &map {
                        println!("# {}", dept);
                        let mut sortedp: Vec<String> =
                            people.iter().map(|p| p.name.clone()).collect();
                        sortedp.sort();
                        for p in sortedp {
                            println!("  {}", p);
                        }
                    }
                }
                println!("---");
            }
            Some(Operation::Delete(dept)) => {
                let result = map.remove(&dept);
                match result {
                    Some(_) => println!("Department {} removed", dept),
                    None => println!("Department {} not found", dept),
                }
            }
            Some(Operation::Help) => show_help(),
            _ => println!("Command not found, sorry. Try 'help' or 'quit'."),
        }
    }
}

fn parse_command(input: String) -> Option<Operation> {
    let mut tokens = input.split_whitespace();

    if let Some(command) = tokens.next() {
        match command {
            "add" => {
                let mut temp_vec: Vec<String> = Vec::new();

                while let Some(token) = tokens.next() {
                    temp_vec.push(String::from(token));
                }

                match temp_vec.len() {
                    3 => {
                        if temp_vec[1] == "to" {
                            Some(Operation::Add(temp_vec[0].clone(), temp_vec[2].clone()))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            "delete" => {
                if let Some(name) = tokens.next() {
                    Some(Operation::Delete(String::from(name)))
                } else {
                    None
                }
            }
            "list" => {
                if let Some(list) = tokens.next() {
                    Some(Operation::List(Some(String::from(list))))
                } else {
                    Some(Operation::List(None))
                }
            }
            "help" => Some(Operation::Help),
            "quit" => Some(Operation::Quit),
            "exit" => Some(Operation::Quit),
            _ => None,
        }
    } else {
        None
    }
}

fn show_help() {
    println!("Commands: add <name> to <team>, delete <team>, list <team>?, quit");
}
