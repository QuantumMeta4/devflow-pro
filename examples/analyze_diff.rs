use devflow_pro::windsurf::{
    interface::{AnalysisContext, WindsurfIntegrationImpl},
    WindsurfPlugin,
};
use std::{fs, path::PathBuf};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = PathBuf::from(&args[1]);
    let file_content = fs::read_to_string(&file_path)?;

    let plugin = WindsurfPlugin::default();
    let integration = WindsurfIntegrationImpl::new(plugin)?;

    let mut ctx = AnalysisContext {
        content: file_content,
        position: None,
        file_path: file_path.to_string_lossy().into_owned(),
        visible_range: None,
    };

    integration.analyze(&mut ctx).await?;
    println!("Analysis complete");

    Ok(())
}
