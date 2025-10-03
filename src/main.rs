use dioxus::prelude::*;
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectGroupProps, SelectListProps, SelectOptionProps, SelectProps,
    SelectTriggerProps, SelectValueProps,
};

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
    empl_in_dep: HashMap<String, Option<String>>,
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

    fn assign_employee_to_department(&mut self, employee: String, department: Option<String>) {
        self.empl_in_dep.insert(employee, department);
    }

    fn get_department_for_employee(&mut self, employee: String) -> Option<String> {
        self.empl_in_dep.get(&employee).cloned()?
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
    let dep_option = |i: usize, dep_name: String| {
        rsx! {
            select::SelectOption::<String> { index: i, value: dep_name.clone(), text_value: "{dep_name}",
                "{dep_name}"
                select::SelectItemIndicator {}
            }
        }
    };

    static DEP_MANAGER: GlobalSignal<DepartmentManager> =
        Signal::global(|| DepartmentManager::new());

    DEP_MANAGER.write().add_department("Front desk".to_string());
    DEP_MANAGER.write().add_department("IT".to_string());
    DEP_MANAGER.write().add_department("Accounting".to_string());

    DEP_MANAGER.write().add_employee("John Smith".to_string());
    DEP_MANAGER
        .write()
        .add_employee("Ivan Ivanenko".to_string());
    DEP_MANAGER.write().add_employee("Jane Doe".to_string());
    //let mut collThing = use_DEP_MANAGER.write()(|dep_manager: DepartmentManager| dep_manager);
    //let mut dep_man_clos = use_signal(|| DEP_MANAGER.write().assign);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            display: "flex",
            justify_content: "center",
            flex_direction: "column",
            align_items: "center",
            table {
                tr {
                    th {
                        h1 { "Departments" }
                        table {
                            thead {
                                tr {
                                    th { "Department Name" }
                                }
                            }
                            tbody {
                                for department in {DEP_MANAGER.read().departments()} {
                                    tr {
                                        td {
                                            div {
                                                style: "display: flex; justify-content: space-between; align-items: center; width: 100%;",
                                                "{department}"
                                                button { "X" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            style: "display: flex; justify-content: center; align-items: center; width: 100%;",
                            input {
                                r#type: "text",
                                placeholder: "Enter department name"
                            }
                            button {
                                "Add Department"
                            }
                        }
                    }
                    th {
                        h1 { "Employees" }
                        table {
                            thead {
                                tr {
                                    th { "Employee Name" }
                                }
                            }
                            tbody {
                                for employee in {DEP_MANAGER.read().employees()} {
                                    tr {
                                        td {
                                            div {
                                                style: "display: flex; justify-content: space-between; align-items: center; width: 100%;",
                                                "{employee}"
                                                button { "X" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            style: "display: flex; justify-content: center; align-items: center; width: 100%;",
                            input {
                                r#type: "text",
                                placeholder: "Enter employee name"
                            }
                            button {
                                "Add Employee"
                            }
                        }
                    }
                }
            }
            table {
                thead {
                    tr {
                        th { "Employee Name" }
                    }
                }
                tbody {
                    for employee in DEP_MANAGER.read().employees() {
                        tr {
                            td {
                                div {
                                    style: "display: flex; justify-content: space-between; align-items: center; width: 100%;",
                                    "{employee}"

                                        select::Select::<String> { placeholder: "Select department",
                                        on_value_change: move |value: Option<String>| {
                                            //DEP_MANAGER.write().assign_employee_to_department(employee.clone(), value);
                                        },
                                        select::SelectTrigger { aria_label: "Select Trigger", width: "12rem", select::SelectValue {} }
                                        select::SelectList { aria_label: "Select Demo",
                                            select::SelectGroup {
                                                select::SelectGroupLabel { "Departments" }

                                                for (index, department) in DEP_MANAGER.read().departments().into_iter().enumerate() {
                                                    {dep_option(index, department)}
                                                }
                                            }
                                            select::SelectGroup {
                                                select::SelectGroupLabel { "Other" }
                                                select::SelectOption::<String> {
                                                    index: DEP_MANAGER.read().employees().len(),
                                                    value: "Other",
                                                    text_value: "Other",
                                                    select::SelectItemIndicator {}
                                                }
                                            }
                                        }
                                    }
                                    /*select::Select::<Option<String>> {
                                        value: "{DEP_MANAGER.write().employees().get(employee)}",
                                        select::SelectTrigger { aria_label: "Select Trigger", width: "12rem", select::SelectValue {} }
                                        select::SelectList { aria_label: "Department",
                                            select::SelectGroup {
                                                select::SelectGroupLabel { "Departments" }
                                            }
                                        }
                                    }*/
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
