use anyhow::Result;
use clap::Parser;
use tauri_craft::{Cli, cmd::Commands, cmd};
// use tauri_craft::templates::TemplateManager;

fn main() -> Result<()> {
    let cli = Cli::parse();
    // let template_manager = TemplateManager::new()?;

    match &cli.command {
        Commands::New(args) => {
            cmd::new::execute(args)?;
        }
        Commands::ListTemplates => {
            // cmd::list_templates::execute(&template_manager)?;
        }
        Commands::AddPlugin { name } => {
            // cmd::add_plugin::execute(&template_manager, name)?;
        }
    }
    Ok(())
}