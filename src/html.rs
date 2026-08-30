// HTML-Ausgabe des Informationsblatts.
// Copyright (C) 2026 Zeno R.R. Davatz - GPL-3.0, siehe LICENSE.

use std::fmt::Write as _;

use crate::inhalt::{Block, Dokument, Span, Tabelle, Verweis};
use crate::inhalt::KOPFZEILE;

const CSS: &str = include_str!("blatt.css");

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn spans(out: &mut String, sp: &[Span]) {
    for s in sp {
        match s {
            Span::T(t) => out.push_str(&esc(t)),
            Span::B(t) => {
                let _ = write!(out, "<strong>{}</strong>", esc(t));
            }
            Span::I(t) => {
                let _ = write!(out, "<em>{}</em>", esc(t));
            }
            Span::L(t, url) => {
                let _ = write!(out, "<a href=\"{}\">{}</a>", esc(url), esc(t));
            }
            Span::N(t) => {
                // Messwerte duerfen nicht am Schraegstrich oder vor der
                // Einheit umbrechen.
                let _ = write!(out, "<span class=\"nb\">{}</span>", esc(t).replace(' ', "&nbsp;"));
            }
        }
    }
}

fn zellen(out: &mut String, tag: &str, cell: &[Span], klasse: &str) {
    let _ = write!(out, "<{tag}{klasse}>");
    spans(out, cell);
    let _ = write!(out, "</{tag}>");
}

fn tabelle(out: &mut String, t: &Tabelle) {
    let _ = write!(out, "<table{}>\n", if t.chronik { " class=\"chronik\"" } else { "" });
    if !t.kopf.is_empty() {
        out.push_str("<thead><tr>");
        for k in t.kopf {
            let _ = write!(out, "<th>{}</th>", esc(k));
        }
        out.push_str("</tr></thead>\n");
    }
    out.push_str("<tbody>\n");
    for zeile in t.zeilen {
        out.push_str("<tr>");
        for (i, cell) in zeile.iter().enumerate() {
            let klasse = match (i, t.chronik) {
                (0, true) => " class=\"jahr\"",
                (_, true) => " class=\"was\"",
                (0, false) => " class=\"feld\"",
                _ => "",
            };
            zellen(out, "td", cell, klasse);
        }
        out.push_str("</tr>\n");
    }
    out.push_str("</tbody>\n</table>\n\n");
}

fn verweis(out: &mut String, v: &Verweis, klasse: &str) {
    let _ = write!(
        out,
        "<p class=\"{klasse}\"><a href=\"{}\">{}</a></p>\n",
        esc(v.url),
        esc(v.text)
    );
}

fn blocks(out: &mut String, bs: &[Block]) {
    for b in bs {
        match b {
            Block::H2(t) => {
                let _ = write!(out, "\n<h2>{}</h2>\n", esc(t));
            }
            Block::H3(t) => {
                let _ = write!(out, "<h3>{}</h3>\n", esc(t));
            }
            Block::P(sp) => {
                out.push_str("<p>");
                spans(out, sp);
                out.push_str("</p>\n");
            }
            Block::Klein(sp) => {
                out.push_str("<p class=\"klein\">");
                spans(out, sp);
                out.push_str("</p>\n");
            }
            Block::Liste(items) => {
                out.push_str("<ul>\n");
                for it in *items {
                    out.push_str("  <li>");
                    spans(out, it);
                    out.push_str("</li>\n");
                }
                out.push_str("</ul>\n");
            }
            Block::Tab(t) => tabelle(out, t),
            Block::Lead { werte, blocks: inner } => {
                out.push_str("<div class=\"lead\">\n");
                let _ = write!(out, "  <span class=\"werte\">{}</span>\n", esc(werte));
                blocks(out, inner);
                out.push_str("</div>\n\n");
            }
            Block::Alarm { titel, blocks: inner } => {
                out.push_str("<div class=\"alarm\">\n");
                let _ = write!(out, "  <h3>{}</h3>\n", esc(titel));
                blocks(out, inner);
                out.push_str("</div>\n\n");
            }
            Block::Adresse { name, rolle, zeilen, links } => {
                out.push_str("<div class=\"adresse\">\n");
                let _ = write!(out, "  <p class=\"name\">{}</p>\n", esc(name));
                if !rolle.is_empty() {
                    out.push_str("  <p class=\"rolle\">");
                    spans(out, rolle);
                    out.push_str("</p>\n");
                }
                for z in *zeilen {
                    out.push_str("  <p class=\"kontakt\">");
                    spans(out, z);
                    out.push_str("</p>\n");
                }
                for v in *links {
                    out.push_str("  ");
                    verweis(out, v, "kontakt");
                }
                out.push_str("</div>\n\n");
            }
        }
    }
}

pub fn render(d: &Dokument) -> String {
    let mut out = String::with_capacity(96 * 1024);
    out.push_str("<!DOCTYPE html>\n<!--\n");
    out.push_str(
        "  IgA-Vaskulitis (Purpura Schoenlein-Henoch) - Informationsblatt\n\
         \x20 Copyright (C) 2026 Zeno R.R. Davatz\n\
         \x20\n\
         \x20 Erzeugt von src/main.rs - diese Datei nicht von Hand aendern.\n\
         \x20 Freie Software unter der GNU General Public License v3, siehe LICENSE.\n\
         \x20 Kein Ersatz fuer eine aerztliche Beurteilung.\n",
    );
    out.push_str("-->\n<html lang=\"de-CH\">\n<head>\n<meta charset=\"utf-8\">\n");
    let _ = write!(out, "<title>{} – {}</title>\n", esc(d.titel), esc(d.untertitel));
    // Die Kopfzeile steht im CSS als Vorgabe des Vaskulitis-Blattes; jedes
    // andere Dokument setzt hier seine eigene ein.
    debug_assert!(CSS.contains(KOPFZEILE));
    let css = CSS.replace(KOPFZEILE, d.kopfzeile);
    let _ = write!(out, "<style>\n{}</style>\n</head>\n<body>\n\n", css);

    let _ = write!(out, "<h1>{}<br>{}</h1>\n", esc(d.titel), esc(d.titel2));
    let _ = write!(out, "<p class=\"untertitel\">{}</p>\n", esc(d.untertitel));
    let _ = write!(out, "<p class=\"stand\">{}</p>\n\n", esc(d.stand));

    blocks(&mut out, d.blocks);

    out.push_str("\n<h2>Quellen</h2>\n<ul class=\"quellen\">\n");
    for (label, v) in d.quellen {
        let _ = write!(
            out,
            "  <li>{} · <a href=\"{}\">{}</a></li>\n",
            esc(label),
            esc(v.url),
            esc(v.text)
        );
    }
    out.push_str("</ul>\n\n</body>\n</html>\n");
    out
}
