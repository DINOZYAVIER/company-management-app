use dioxus::prelude::*;
use std::vec;

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
            department: department,
        }
    }
}

fn main() {
    let empls: Vec<Employee> = Vec::new();
    let dprts: Vec<Department> = Vec::new();

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
            }
        }
    }
}
