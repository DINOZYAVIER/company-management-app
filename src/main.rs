use dioxus::prelude::*;

struct Department {
    name: String,
}

struct Employee {
    name: String,
    department: Department,
}

fn main() {
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
