use dioxus::prelude::*;
use std::vec;

enum CommandParseError {
    InvalidCommand,
    WrongAmountOfArguments,
}

enum CommandParseAction {
    AddEmployeeToDepartment,
    RemoveEmployeeFromDepartment,
    CreateEmployee,
    CreateDepartment,
    RemoveEmployee,
    RemoveDepartment,
}

#[derive(PartialEq, Props, Clone)]
struct Departments {
    names: Vec<String>,
}

impl Departments {
    fn new() -> Departments {
        Departments { names: Vec::new() }
    }
    fn add_department(&mut self, department_name: String) {
        self.names.push(department_name);
    }
}

#[derive(PartialEq, Props, Clone)]
struct Employees {
    names: Vec<String>,
}

impl Employees {
    fn new() -> Employees {
        Employees { names: Vec::new() }
    }
    fn add_employee(&mut self, employee_name: String) {
        self.names.push(employee_name);
    }
}

fn parse_command(command: String) -> Result<CommandParseAction, CommandParseError> {
    let words: Vec<&str> = command.split(' ').collect();
    if words.len() != 4 {
        return Err(CommandParseError::WrongAmountOfArguments);
    }
    if (words[0] == "add") && (words[2] == "to") {
        return Ok(CommandParseAction::AddEmployeeToDepartment);
    }
    Err(CommandParseError::InvalidCommand)
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut empls = Employees::new();
    let mut dprts = Departments::new();

    dprts.add_department("Front desk".to_string());
    dprts.add_department("IT".to_string());
    dprts.add_department("Accounting".to_string());

    empls.add_employee("John Smith".to_string());

    rsx! {
        div {
            display: "flex",
            justify_content: "center",
            flex_direction: "column",
            align_items: "center",
            h1 { "Departments" }
            ul {
                for department in {dprts.names} {
                    li { "{department}" }
                }
            }
            h1 { "Employees" }
            ul {
                for employee in {empls.names} {
                    li { "{employee}" }
                }
            }
            button {
                "Add Employee"
            }
        }
    }
}
