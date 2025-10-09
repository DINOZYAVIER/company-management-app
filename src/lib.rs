// lib.rs
pub mod app;
pub mod department_manager;

use department_manager::DepartmentManager;
use dioxus::prelude::*;

static DEP_MANAGER: GlobalSignal<DepartmentManager> = Signal::global(DepartmentManager::new);
