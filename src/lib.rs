use anyhow::{bail, Context as _, Result};
use camino::Utf8PathBuf;
use log::*;
use mdbook::{
    book::{Book, Chapter},
    errors::Result as MdbookResult,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use serde::Deserialize;
use std::{collections::BTreeMap, fmt::Write};
use toml::value::Value;
use uuid::Uuid;

/// Configuration for the plugin
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub prefix: Utf8PathBuf,
}

#[derive(Clone, Debug)]
pub struct Instance {
    config: Config,
}

impl Instance {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn map(&self, book: Book) -> MdbookResult<Book> {
        Ok(book)
    }
}

#[derive(Default, Debug)]
pub struct MdbookLeptos;

impl Preprocessor for MdbookLeptos {
    fn name(&self) -> &str {
        "files"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> MdbookResult<Book> {
        let config = ctx.config.get_preprocessor(self.name()).unwrap();
        let config: Config = Value::Table(config.clone()).try_into().unwrap();
        let instance = Instance::new(config);
        instance.map(book)
    }
}

