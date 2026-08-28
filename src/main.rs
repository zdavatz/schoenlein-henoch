// Informationsblatt zur IgA-Vaskulitis (Purpura Schoenlein-Henoch).
// Copyright (C) 2026 Zeno R.R. Davatz
//
// Dieses Programm ist freie Software: Sie koennen es weitergeben und/oder
// veraendern, unter den Bedingungen der GNU General Public License, wie von
// der Free Software Foundation veroeffentlicht, entweder Version 3 der
// Lizenz oder (nach Ihrer Wahl) jeder spaeteren Version. Siehe LICENSE.
//
// Kein Ersatz fuer eine aerztliche Beurteilung.
//
//   cargo run --release
//   cargo run --release -- --html raus.html --pdf raus.pdf
//
// Schriftverzeichnis ueber $FONT_DIR (Vorgabe: ./fonts).

mod html;
mod inhalt;
mod pdf;

use std::env;
use std::path::PathBuf;

use anyhow::Result;

const DEFAULT_HTML: &str = "iga-vaskulitis.html";
const DEFAULT_PDF: &str = "iga-vaskulitis.pdf";
const DEFAULT_FONT_DIR: &str = "fonts";

fn arg(args: &[String], name: &str, vorgabe: &str) -> PathBuf {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(vorgabe))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let html_out = arg(&args, "--html", DEFAULT_HTML);
    let pdf_out = arg(&args, "--pdf", DEFAULT_PDF);
    let font_dir = env::var("FONT_DIR").unwrap_or_else(|_| DEFAULT_FONT_DIR.to_string());

    let html = html::render();
    std::fs::write(&html_out, &html)?;
    println!("→ {} ({} B)", html_out.display(), html.len());

    let links = pdf::render(&pdf_out, &font_dir)?;
    let bytes = std::fs::metadata(&pdf_out)?.len();
    println!("→ {} ({bytes} B, {links} Links)", pdf_out.display());
    Ok(())
}
