use std::fmt::{Display, Formatter};
use crate::frameworks::Framework;

pub struct Nuxt;

impl Display for Nuxt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nuxt")
    }
}

impl Framework for Nuxt {
    fn name(&self) -> &str {
        "Nuxt"
    }

    fn get_modules(&self) -> Vec<String> {
        vec![
            "@nuxtjs/tailwindcss".to_string(),
            "@pinia/nuxt".to_string(),
            "@nuxtjs/color-mode".to_string(),
            "@vueuse/nuxt".to_string(),
            "shadcn-nuxt".to_string(),
        ]
    }

    fn init_command(&self, project_name: &str, package_manager: &str) -> Vec<String> {
        vec!["npx".to_string(), "nuxi@latest".to_string(), "init".to_string(), "--packageManager".to_string(), package_manager.to_string(), "--gitInit".to_string(), project_name.to_string()]
    }

    fn add_module_command(&self, module: &str) -> Vec<String> {
        vec!["npx".to_string(), "nuxi@latest".to_string(), "module".to_string(), "add".to_string(), module.to_string()]
    }
}