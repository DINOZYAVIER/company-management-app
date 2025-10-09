use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Props, Clone)]
pub struct DepartmentManager {
    pub empl_in_dep: HashMap<String, Option<String>>,
    pub empl_names: Vec<String>,
    pub dprt_names: Vec<String>,
}

impl DepartmentManager {
    pub fn new() -> DepartmentManager {
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

    pub fn add_department(&mut self, department_name: String) {
        self.dprt_names.push(department_name);
    }

    pub fn add_employee(&mut self, employee_name: String) {
        self.empl_names.push(employee_name);
    }

    pub fn remove_department(&mut self, index: usize) {
        self.dprt_names.remove(index);
    }

    pub fn remove_employee(&mut self, index: usize) {
        self.empl_names.remove(index);
    }

    pub fn assign_employee_to_department(&mut self, employee: String, department: Option<String>) {
        self.empl_in_dep.insert(employee, department);
    }

    pub fn employees(&self) -> Vec<String> {
        self.empl_names.clone()
    }

    pub fn departments(&self) -> Vec<String> {
        self.dprt_names.clone()
    }
}
