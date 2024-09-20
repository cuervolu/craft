use super::NewArgs;
use anyhow::{Context, Result};
use inquire::{Text, Select, MultiSelect, required};

pub struct ProjectConfig {
    pub name: String,
    pub framework: String,
    pub framework_modules: Vec<String>,
    pub tauri_plugins: Vec<String>,
    pub package_manager: String,
}

pub fn create_project_config(args: &NewArgs) -> Result<ProjectConfig> {
    let name = if let Some(n) = &args.name {
        n.to_string()
    } else {
        Text::new("What is the name of your project?:")
            .with_formatter(&|s| s.trim().to_string())
            .with_validator(required!("This field is required"))
            .with_placeholder("my-tauri-app")
            .prompt()
            .context("Failed to get project name")?
    };

    let frameworks = vec!["Nuxt", "Next.js", "Leptos", "Qwik", "SvelteKit", "Trunk"];
    let framework = if let Some(f) = &args.framework {
        f.to_string()
    } else {
        Select::new("Which framework do you want to use?:", frameworks)
            .prompt()
            .context("Failed to select framework")?
            .to_string()
    };

    let framework_modules = if framework == "Nuxt" {
        let available_modules = vec![
            "@nuxtjs/tailwindcss",
            "@pinia/nuxt",
            "@nuxtjs/color-mode",
            "@vueuse/nuxt",
            "shadcn-nuxt",
        ];
        MultiSelect::new("Which Nuxt modules do you want to include?:", available_modules)
            .prompt()?
            .into_iter()
            .map(String::from)
            .collect()
    } else {
        vec![]
    };

    let package_managers = vec!["npm", "yarn", "pnpm", "bun"];
    let package_manager = if let Some(pm) = &args.package_manager {
        pm.to_string()
    } else {
        Select::new("Which package manager do you want to use?:", package_managers)
            .prompt()
            .context("Failed to select package manager")?
            .to_string()
    };

    let available_tauri_plugins = vec![
        "clipboard-manager", "dialog", "fs", "log", "notification",
        "os", "process", "shell", "store", "updater", "window-state"
    ];
    let tauri_plugins = MultiSelect::new("Which Tauri plugins do you want to include?:", available_tauri_plugins)
        .prompt()?
        .into_iter()
        .map(String::from)
        .collect();

    Ok(ProjectConfig {
        name,
        framework,
        framework_modules,
        tauri_plugins,
        package_manager,
    })
}