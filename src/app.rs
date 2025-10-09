use crate::DEP_MANAGER;
use dioxus::prelude::*;
use dioxus_primitives::select;

pub fn department_list() -> Element {
    let mut input_dep = use_signal(String::new);

    rsx! {
        div {
            class: "container mx-auto p-4",
            h1 { class: "text-xl font-bold bg-gray-50 border-b border-gray-200 mb-2", "Departments" }
            div {
                class: "flex space-x-2",
                input {
                    r#type: "text",
                    class: "p-2 border border-gray-300 rounded",
                    placeholder: "Enter department name",
                    oninput: move |event| input_dep.set(event.value().clone()),
                }
                button {
                    class: "bg-blue-500 text-white p-2 rounded",
                    onclick: move |_event| DEP_MANAGER.write().add_department(input_dep().clone()),
                    "Add Department"
                }
            }
            table {
                class: "min-w-full bg-gray-50 border border-gray-300",
                tbody {
                    for (index, department) in DEP_MANAGER.read().departments().into_iter().enumerate() {
                        tr {
                            td {
                                class: "p-2 border border-gray-300",
                                div {
                                    class: "flex justify-between items-center",
                                    "{department}"
                                    button {
                                        class: "bg-red-500 text-white p-1 rounded",
                                        onclick: move |_event| DEP_MANAGER.write().remove_department(index),
                                        "X"
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

pub fn employee_list() -> Element {
    let mut input_empl = use_signal(String::new);

    rsx! {
        div {
            class: "container mx-auto p-4",
            h1 { class: "text-xl font-bold bg-gray-50 border-b border-gray-200 mb-2", "Employees" }
            div {
                class: "flex space-x-2",
                input {
                    r#type: "text",
                    class: "p-2 border border-gray-300 rounded",
                    placeholder: "Enter employee name",
                    oninput: move |event| input_empl.set(event.value().clone()),
                }
                button {
                    class: "bg-blue-500 text-white p-2 rounded",
                    onclick: move |_event| DEP_MANAGER.write().add_employee(input_empl().clone()),
                    "Add Employee"
                }
            }
            table {
                class: "min-w-full bg-gray-50 border border-gray-300",
                tbody {
                    for (index, employee) in DEP_MANAGER.read().employees().into_iter().enumerate() {
                        tr {
                            td {
                                class: "p-2 border border-gray-300",
                                div {
                                    class: "flex justify-between items-center",
                                    "{employee}"
                                    button {
                                        class: "bg-red-500 text-white p-1 rounded",
                                        onclick: move |_event| DEP_MANAGER.write().remove_employee(index),
                                        "X"
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

pub fn employee_assignment() -> Element {
    let dep_option = |i: usize, dep_name: String| {
        rsx! {
            select::SelectOption::<String> {
                class: "p-2 hover:bg-gray-100 cursor-pointer",
                index: i, value: dep_name.clone(), text_value: "{dep_name}",
                "{dep_name}"
                select::SelectItemIndicator {}
            }
        }
    };

    rsx! {
        table {
            class: "min-w-full bg-white border border-gray-300",
            thead {
                tr {
                    th { class: "bg-gray-100 p-4 text-left", "Employee Name" }
                }
            }
            tbody {
                for employee in DEP_MANAGER.read().employees() {
                    tr {
                        td {
                            class: "p-2 border border-gray-300",
                            div { class: "font-bold", "{employee}" },
                            div {
                                class: "flex items-center",
                                select::Select::<String> {
                                    placeholder: "Select department",
                                    on_value_change: move |value: Option<String>| {
                                        DEP_MANAGER.write().assign_employee_to_department(employee.clone(), value);
                                    },
                                    select::SelectTrigger {
                                        width: "12rem",
                                        class: "p-3 border border-gray-300 rounded bg-white m-1",
                                        select::SelectValue {}
                                    }
                                    select::SelectList {
                                        class: "bg-white border border-gray-300 rounded shadow-lg p-3 mt-1",
                                        select::SelectGroup {
                                            select::SelectGroupLabel { class: "font-bold p-2 bg-gray-50 border-b border-gray-200", "Departments" }
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
    }
}

pub fn App() -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        div {
            class: "container mx-auto p-4",
            table {
                class: "min-w-full bg-white border border-gray-300",
                tr {
                    td {
                        department_list {}
                    }
                    td {
                        employee_list {}
                    }
                }
            }
            employee_assignment {}
            h1 { "Amount of departments: {DEP_MANAGER.read().departments().len():?}" }
            h1 { "Amount of employees: {DEP_MANAGER.read().employees().len():?}" }
        }
    }
}
