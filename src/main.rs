use dioxus::prelude::*;
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectGroupProps, SelectListProps, SelectOptionProps, SelectProps,
    SelectTriggerProps, SelectValueProps,
};

use std::collections::HashMap;
use std::vec;

#[derive(PartialEq, Props, Clone)]
struct DepartmentManager {
    empl_in_dep: HashMap<String, Option<String>>,
    empl_names: Vec<String>,
    dprt_names: Vec<String>,
}

impl DepartmentManager {
    fn new() -> DepartmentManager {
        let mut dep_manager = DepartmentManager {
            empl_in_dep: HashMap::new(),
            empl_names: Vec::new(),
            dprt_names: Vec::new(),
        };

        dep_manager.add_department("Front desk".to_string());
        dep_manager.add_department("IT".to_string());
        dep_manager.add_department("Accounting".to_string());

        dep_manager.add_employee("John Smith".to_string());
        dep_manager.add_employee("Ivan Ivanenko".to_string());
        dep_manager.add_employee("Jane Doe".to_string());

        dep_manager
    }

    fn add_department(&mut self, department_name: String) {
        self.dprt_names.push(department_name);
    }

    fn add_employee(&mut self, employee_name: String) {
        self.empl_names.push(employee_name);
    }

    fn remove_department(&mut self, index: usize) {
        self.dprt_names.remove(index);
    }
    fn remove_employee(&mut self, index: usize) {
        self.empl_names.remove(index);
    }

    fn assign_employee_to_department(&mut self, employee: String, department: Option<String>) {
        self.empl_in_dep.insert(employee, department);
    }

    fn employees(&self) -> Vec<String> {
        self.empl_names.clone()
    }

    fn departments(&self) -> Vec<String> {
        self.dprt_names.clone()
    }
}

static DEP_MANAGER: GlobalSignal<DepartmentManager> = Signal::global(DepartmentManager::new);

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

    let mut input_dep = use_signal(|| String::new());
    let mut input_empl = use_signal(|| String::new());

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

                                for (index, department) in DEP_MANAGER.read().departments().into_iter().enumerate() {
                                    tr {
                                        td {
                                            div {
                                                style: "display: flex; justify-content: space-between; align-items: center; width: 100%;",
                                                "{department}"
                                                button { onclick: move |_event| DEP_MANAGER.write().remove_department(index),
                                                "X"}
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
                                placeholder: "Enter department name",
                                oninput: move |event| input_dep.set(event.value().clone()),
                            }
                            button {  onclick: move |_event| DEP_MANAGER.write().add_department(input_dep().clone()),
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

                                for (index, employee) in DEP_MANAGER.read().employees().into_iter().enumerate() {
                                    tr {
                                        td {
                                            div {
                                                style: "display: flex; justify-content: space-between; align-items: center; width: 100%;",
                                                "{employee}"
                                                button { onclick: move |_event| DEP_MANAGER.write().remove_employee(index),
                                                "X"}
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
                                placeholder: "Enter employee name",
                                oninput: move |event| input_empl.set(event.value().clone()),
                            }
                            button {  onclick: move |_event| DEP_MANAGER.write().add_employee(input_empl().clone()),
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

                                        select::Select::<String> {
                                        placeholder: "Select department",
                                        on_value_change: move |value: Option<String>| {
                                            DEP_MANAGER.write().assign_employee_to_department(employee.clone(), value);
                                        },
                                        select::SelectTrigger { aria_label: "Select Trigger", width: "12rem", select::SelectValue {} }
                                        select::SelectList { aria_label: "Select Demo",
                                            select::SelectGroup {
                                                select::SelectGroupLabel { "Departments" }

                                                for (index, department) in DEP_MANAGER.read().departments().into_iter().enumerate() {
                                                    {dep_option(index, department)}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            h1 { "Amount of departments: {DEP_MANAGER.read().departments().len():?}" }
            h1 { "Amount of employees: {DEP_MANAGER.read().employees().len():?}"}
        }
    }
}
