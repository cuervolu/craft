use std::fmt;

mod nuxt;
pub(crate) mod factory;

pub trait Framework: fmt::Display {
    fn name(&self) -> &str;
    fn get_modules(&self) -> Vec<String>;
    fn init_command(&self, project_name: &str, package_manager: &str) -> Vec<String>;
    fn add_module_command(&self, module: &str) -> Vec<String>;
}