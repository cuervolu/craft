pub mod new;
// pub mod list_templates;
// pub mod add_plugin;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Tauri project
    New(new::NewArgs),
    /// List available templates
    ListTemplates,
    /// Add a plugin to an existing project
    AddPlugin {
        /// Name of the plugin to add
        name: String,
    },
}