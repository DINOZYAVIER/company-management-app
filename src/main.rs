use dioxus::prelude::*;
use std::collections::HashMap;
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
struct DepartmentManager {
    empl_in_dep: HashMap<String, String>,
    empl_names: Vec<String>,
    dprt_names: Vec<String>,
}

impl DepartmentManager {
    fn new() -> DepartmentManager {
        DepartmentManager {
            empl_in_dep: HashMap::new(),
            empl_names: Vec::new(),
            dprt_names: Vec::new(),
        }
    }

    fn add_department(&mut self, department_name: String) {
        self.dprt_names.push(department_name);
    }

    fn add_employee(&mut self, employee_name: String) {
        self.empl_names.push(employee_name);
    }

    fn employees(&self) -> Vec<String> {
        self.empl_names.clone()
    }

    fn departments(&self) -> Vec<String> {
        self.dprt_names.clone()
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
    let mut dep_manager = DepartmentManager::new();

    dep_manager.add_department("Front desk".to_string());
    dep_manager.add_department("IT".to_string());
    dep_manager.add_department("Accounting".to_string());

    dep_manager.add_employee("John Smith".to_string());
    dep_manager.add_employee("Ivan Ivanenko".to_string());
    dep_manager.add_employee("Jane Doe".to_string());

    rsx! {
        div {
            display: "flex",
            justify_content: "center",
            flex_direction: "column",
            align_items: "center",
            h1 { "Departments" }
            table {
                thead {
                    tr {
                        th { "Department Name" }
                    }
                }
                tbody {
                    for department in {dep_manager.departments()} {
                        tr {
                            td { "{department}" }
                        }
                    }
                }
            }
            button {
                "Add Department"
            }
            h1 { "Employees" }
            table {
                thead {
                    tr {
                        th { "Employee Name" }
                    }
                }
                tbody {
                for employee in {dep_manager.employees()} {
                        tr {
                            td { "{employee}" }
                        }
                    }
                }
            }
            button {
                "Add Employee"
            }
        }
    }
}
