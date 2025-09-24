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

struct Department {
    name: String,
}

impl Department {
    fn new(department_name: String) -> Department {
        Department {
            name: department_name,
        }
    }
}

struct Employee {
    name: String,
    department: Option<Department>,
}

impl Employee {
    fn new(employee_name: String, department: Option<Department>) -> Employee {
        Employee {
            name: employee_name,
            department,
        }
    }
    fn set_department(mut self, new_department: Department) {
        self.department = Some(new_department);
    }
}

fn parse_command(command: String) -> Result<(), CommandParseError> {
    let words: Vec<&str> = command.split(' ').collect();
    if words.len() != 4 {
        return Err(CommandParseError::WrongAmountOfArguments);
    }
    if (words[0] == "add") && (words[2] == "to") {
        return Ok(());
    }
    Err(CommandParseError::InvalidCommand)
}

fn main() {
    let _empls: Vec<Employee> = Vec::new();
    let _dprts: Vec<Department> = Vec::new();

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            // display sets the layout mode of the element
            display: "flex",
            // justify-content centers the element horizontally
            justify_content: "center",
            input {
                type: "string"
            },
            button {
                //onclick: move |_| "Increment"
            }

        }
    }
}
