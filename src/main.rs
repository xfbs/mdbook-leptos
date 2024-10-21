use anyhow::{bail, Result};
use clap::Parser;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_leptos::MdbookLeptos;
use std::io;
use std::path::PathBuf;

/// Preprocessor for mdBook which renders files from a directory as an interactive widget, with
/// syntax highlighting.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Options {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Check if the renderer is supported.
    Supports(SupportsCommand),
    /// Process a parsed book (default).
    Process,
    /// Install support for mdbook-files into the current mdbook project.
    Install(InstallCommand),
}

#[derive(Parser, Debug)]
pub struct SupportsCommand {
    pub renderer: String,
}

#[derive(Parser, Debug)]
pub struct InstallCommand {
    #[clap(long)]
    pub assets: Option<PathBuf>,
}

impl Options {
    fn run(&self, preprocessor: &dyn Preprocessor) -> Result<()> {
        match &self.command {
            Some(Command::Supports(command)) => {
                if preprocessor.supports_renderer(&command.renderer) {
                    Ok(())
                } else {
                    bail!("unknown renderer {}", command.renderer);
                }
            }
            None | Some(Command::Process) => {
                let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
                let output = preprocessor.run(&ctx, book)?;
                serde_json::to_writer(io::stdout(), &output)?;
                Ok(())
            }
            Some(Command::Install(_command)) => Ok(()),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let options = Options::parse();
    let renderer = MdbookLeptos::default();
    options.run(&renderer)
}
