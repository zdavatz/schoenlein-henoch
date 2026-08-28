// PDF-Ausgabe des Informationsblatts.
// Copyright (C) 2026 Zeno R.R. Davatz - GPL-3.0, siehe LICENSE.
//
// Pure Rust, kein Chrome und kein WeasyPrint: `genpdf` schreibt ueber
// `printpdf`, die DejaVu-Sans-Familie wird eingebettet. Dieselbe Pipeline wie
// in ~/.software/adhs-expert.
//
// Hyperlinks sind der heikle Teil - genpdf 0.2 kennt keine. Deshalb steht
// jeder Verweis allein auf seiner Zeile und in einer Schriftgroesse, die
// sonst nirgends im Dokument vorkommt (siehe LINK_SIZES). Nach dem Rendern
// laeuft `add_links` den Inhaltsstrom Seite fuer Seite durch, sammelt die
// Grundlinien aller Textzeilen in genau diesen Groessen ein und legt darueber
// `/Link`-Annotationen mit `lopdf`.

use std::path::Path;

use anyhow::{anyhow, Result};
use genpdf::elements::{
    Break, FrameCellDecorator, LinearLayout, Paragraph, TableLayout, UnorderedList,
};
use genpdf::style::{Color, Style};
use genpdf::{Alignment, Element, Margins};

use crate::inhalt::{Block, Span, Tabelle, Verweis};
use crate::inhalt::{DOKUMENT, FUSS, KOPFZEILE, QUELLEN, STAND, TITEL, TITEL2, UNTERTITEL};

// --- Schriftgroessen -------------------------------------------------------
//
// 9 und 11 gehoeren ausschliesslich den Verweiszeilen. Sobald eine andere
// Zeile in einer dieser Groessen gesetzt wird, verschiebt sich die Zuordnung
// der Links und jeder Verweis zeigt aufs falsche Ziel. Die Pruefung am Ende
// von `render` faengt das ab; sie darf nie entfernt werden.
const S_H1: u8 = 19;
const S_H2: u8 = 14;
const S_H3: u8 = 12;
const S_TEXT: u8 = 10;
const S_KLEIN: u8 = 8;
const S_KOPF: u8 = 7;
/// Verweise in Adressen und Kaesten.
const LINK_GROSS: u8 = 11;
/// Verweise im Quellenverzeichnis.
const LINK_KLEIN: u8 = 9;
const LINK_SIZES: [u8; 2] = [LINK_KLEIN, LINK_GROSS];

const INK: Color = Color::Rgb(0x1d, 0x21, 0x29);
const BORDEAUX: Color = Color::Rgb(0x7a, 0x1f, 0x2b);
const ROT: Color = Color::Rgb(0xa8, 0x32, 0x32);
const GRAU: Color = Color::Rgb(0x5a, 0x60, 0x68);
const HELLGRAU: Color = Color::Rgb(0x8a, 0x8f, 0x98);

const RAND_MM: f64 = 18.0;
const ZEILENABSTAND: f64 = 1.4;
const A4_WIDTH_PT: f64 = 595.276;
const MARGIN_PT: f64 = RAND_MM * 72.0 / 25.4;
/// Mittlere Zeichenbreite von DejaVu Sans in em - nur fuer die Klickflaeche.
const AVG_ADVANCE_EM: f64 = 0.55;
/// Laenger gesetzte Verweise wuerden umbrechen; ein Umbruch ergaebe zwei
/// Zeilen in Linkgroesse und damit eine Verschiebung der Zuordnung.
const MAX_LINK_CHARS: usize = 76;

fn grund() -> Style {
    Style::new().with_font_size(S_TEXT).with_color(INK)
}

/// Kuerzt lange Adressen in der Mitte. Verlinkt wird immer das Original,
/// die Klickflaeche bemisst sich nach dem angezeigten Text.
fn link_text(url: &str) -> String {
    let n = url.chars().count();
    if n <= MAX_LINK_CHARS {
        return url.to_string();
    }
    let keep = MAX_LINK_CHARS - 1;
    let vorn = keep / 2;
    let hinten = keep - vorn;
    let zeichen: Vec<char> = url.chars().collect();
    let mut s: String = zeichen[..vorn].iter().collect();
    s.push('…');
    s.extend(&zeichen[n - hinten..]);
    s
}

// ---------------------------------------------------------------------------
// Absaetze
// ---------------------------------------------------------------------------

/// Zeichen, die nie am Zeilenanfang stehen duerfen.
const NACHKLAPP: [char; 8] = [',', '.', ';', ':', '!', '?', ')', '\u{bb}'];

fn absatz(sp: &[Span], basis: Style) -> Paragraph {
    // genpdf zerlegt jedes Textstueck einzeln in Woerter und darf an jeder
    // Grenze zwischen zwei Stuecken umbrechen; ein fuehrendes Leerzeichen
    // wird dabei ein Wort fuer sich. Beides ergibt haessliche Zeilenanfaenge -
    // eine Zeile, die mit einem Leerzeichen beginnt, oder ein Semikolon, das
    // von seinem Wort abgerissen wird. Deshalb wandern fuehrende Leerzeichen
    // und Satzzeichen vorher ans Ende des vorangehenden Stuecks.
    let mut teile: Vec<(String, Style)> = Vec::new();
    for s in sp {
        let (text, stil) = match s {
            Span::T(t) => ((*t).to_string(), basis),
            Span::B(t) => ((*t).to_string(), basis.bold()),
            Span::I(t) => ((*t).to_string(), basis.italic()),
            // Messwerte duerfen nicht umbrechen.
            Span::N(t) => (t.replace(' ', "\u{00a0}"), basis),
        };
        let mut rest = text.as_str();
        if let Some((vorher, _)) = teile.last_mut() {
            while let Some(c) = rest.chars().next() {
                if c == ' ' || NACHKLAPP.contains(&c) {
                    vorher.push(c);
                    rest = &rest[c.len_utf8()..];
                } else {
                    break;
                }
            }
        }
        if !rest.is_empty() {
            teile.push((rest.to_string(), stil));
        }
    }

    let mut p = Paragraph::default();
    for (text, stil) in teile {
        p.push_styled(text, stil);
    }
    p
}

fn ueberschrift(text: &str, groesse: u8, farbe: Color) -> impl Element {
    Paragraph::new(text.to_string())
        .styled(Style::new().with_font_size(groesse).with_color(farbe).bold())
        .padded(Margins::trbl(if groesse >= S_H2 { 4 } else { 2 }, 0, 1, 0))
}

fn verweiszeile(v: &Verweis, groesse: u8) -> impl Element {
    Paragraph::new(link_text(v.text))
        .styled(Style::new().with_font_size(groesse).with_color(BORDEAUX))
}

// ---------------------------------------------------------------------------
// Tabellen
// ---------------------------------------------------------------------------

fn tabelle(t: &Tabelle) -> TableLayout {
    let mut tab = TableLayout::new(t.gewichte.to_vec());
    tab.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    let kopfstil = Style::new().with_font_size(S_KLEIN).with_color(GRAU).bold();
    let mut kopf: Vec<Box<dyn Element>> = Vec::new();
    for k in t.kopf {
        kopf.push(Box::new(
            Paragraph::new(k.to_string())
                .styled(kopfstil)
                .padded(Margins::trbl(1, 1, 1, 1)),
        ));
    }
    tab.push_row(kopf).expect("Kopfzeile der Tabelle");

    for zeile in t.zeilen {
        let mut row: Vec<Box<dyn Element>> = Vec::new();
        for cell in zeile.iter() {
            row.push(Box::new(
                absatz(cell, grund()).padded(Margins::trbl(1, 1, 1, 1)),
            ));
        }
        tab.push_row(row).expect("Tabellenzeile");
    }
    tab
}

fn chronik(zeilen: &[(&str, &[Span])]) -> TableLayout {
    let mut tab = TableLayout::new(vec![16, 84]);
    tab.set_cell_decorator(FrameCellDecorator::new(false, false, false));
    let jahrstil = Style::new().with_font_size(S_TEXT).with_color(BORDEAUX).bold();
    for (jahr, was) in zeilen {
        let row: Vec<Box<dyn Element>> = vec![
            Box::new(
                Paragraph::new(jahr.to_string())
                    .styled(jahrstil)
                    .padded(Margins::trbl(0, 2, 3, 0)),
            ),
            Box::new(absatz(was, grund()).padded(Margins::trbl(0, 0, 3, 0))),
        ];
        tab.push_row(row).expect("Chronikzeile");
    }
    tab
}

// ---------------------------------------------------------------------------
// Bloecke
// ---------------------------------------------------------------------------

fn baue(bs: &[Block], ziel: &mut LinearLayout) {
    for b in bs {
        match b {
            Block::H2(t) => ziel.push(ueberschrift(t, S_H2, BORDEAUX)),
            Block::H3(t) => ziel.push(ueberschrift(t, S_H3, INK)),
            Block::P(sp) => ziel.push(absatz(sp, grund()).padded(Margins::trbl(0, 0, 2, 0))),
            Block::Klein(sp) => ziel.push(
                absatz(sp, Style::new().with_font_size(S_KLEIN).with_color(GRAU))
                    .padded(Margins::trbl(0, 0, 2, 0)),
            ),
            Block::Liste(items) => {
                let mut ul = UnorderedList::with_bullet("·");
                for it in *items {
                    ul.push(absatz(it, grund()).padded(Margins::trbl(0, 0, 1, 0)));
                }
                ziel.push(ul.padded(Margins::trbl(0, 0, 2, 2)));
            }
            Block::Tab(t) => ziel.push(tabelle(t).padded(Margins::trbl(1, 0, 3, 0))),
            Block::Chronik(z) => ziel.push(chronik(z).padded(Margins::trbl(1, 0, 2, 0))),
            Block::Lead { werte, blocks } => {
                let mut innen = LinearLayout::vertical();
                innen.push(
                    Paragraph::new(werte.to_string())
                        .styled(Style::new().with_font_size(S_H3).with_color(BORDEAUX).bold())
                        .padded(Margins::trbl(0, 0, 2, 0)),
                );
                baue(blocks, &mut innen);
                ziel.push(
                    innen
                        .padded(Margins::trbl(2, 3, 1, 3))
                        .framed()
                        .padded(Margins::trbl(0, 0, 4, 0)),
                );
            }
            Block::Alarm { titel, blocks } => {
                let mut innen = LinearLayout::vertical();
                innen.push(
                    Paragraph::new(titel.to_string())
                        .styled(Style::new().with_font_size(S_H3).with_color(ROT).bold())
                        .padded(Margins::trbl(0, 0, 2, 0)),
                );
                baue(blocks, &mut innen);
                ziel.push(
                    innen
                        .padded(Margins::trbl(2, 3, 1, 3))
                        .framed()
                        .padded(Margins::trbl(3, 0, 4, 0)),
                );
            }
            Block::Adresse { name, rolle, zeilen, links } => {
                let mut innen = LinearLayout::vertical();
                innen.push(Paragraph::new(name.to_string()).styled(grund().bold()));
                if !rolle.is_empty() {
                    innen.push(absatz(rolle, Style::new().with_font_size(S_TEXT).with_color(GRAU)));
                }
                for z in *zeilen {
                    innen.push(absatz(z, Style::new().with_font_size(S_TEXT).with_color(GRAU)));
                }
                for v in *links {
                    innen.push(verweiszeile(v, LINK_GROSS));
                }
                ziel.push(innen.padded(Margins::trbl(0, 0, 4, 2)));
            }
        }
    }
}

/// Alle URLs in Satzreihenfolge. Muss mit `baue` und dem Quellenteil in
/// `render` uebereinstimmen.
fn urls() -> Vec<&'static str> {
    fn sammle(bs: &'static [Block], out: &mut Vec<&'static str>) {
        for b in bs {
            match b {
                Block::Lead { blocks, .. } | Block::Alarm { blocks, .. } => sammle(blocks, out),
                Block::Adresse { links, .. } => out.extend(links.iter().map(|v| v.url)),
                _ => {}
            }
        }
    }
    let mut out = Vec::new();
    sammle(DOKUMENT, &mut out);
    out.extend(QUELLEN.iter().map(|(_, v)| v.url));
    out
}

/// Angezeigte Laenge je Verweis, in derselben Reihenfolge - fuer die Breite
/// der Klickflaeche.
fn anzeigelaengen() -> Vec<usize> {
    fn sammle(bs: &'static [Block], out: &mut Vec<usize>) {
        for b in bs {
            match b {
                Block::Lead { blocks, .. } | Block::Alarm { blocks, .. } => sammle(blocks, out),
                Block::Adresse { links, .. } => {
                    out.extend(links.iter().map(|v| link_text(v.text).chars().count()))
                }
                _ => {}
            }
        }
    }
    let mut out = Vec::new();
    sammle(DOKUMENT, &mut out);
    out.extend(
        QUELLEN
            .iter()
            .map(|(_, v)| link_text(v.text).chars().count()),
    );
    out
}

// ---------------------------------------------------------------------------
// Schriften und Link-Overlay
// ---------------------------------------------------------------------------

fn load_font_family(font_dir: &str) -> Result<genpdf::fonts::FontFamily<genpdf::fonts::FontData>> {
    let load = |file: &str| -> Result<genpdf::fonts::FontData> {
        let path = Path::new(font_dir).join(file);
        let data = std::fs::read(&path).map_err(|e| anyhow!("Schrift {}: {}", path.display(), e))?;
        genpdf::fonts::FontData::new(data, None).map_err(|e| anyhow!("Schrift {}: {}", file, e))
    };
    Ok(genpdf::fonts::FontFamily {
        regular: load("DejaVuSans.ttf")?,
        bold: load("DejaVuSans-Bold.ttf")?,
        italic: load("DejaVuSans-Oblique.ttf")?,
        bold_italic: load("DejaVuSans-BoldOblique.ttf")?,
    })
}

/// Legt ueber jede in einer Linkgroesse gesetzte Textzeile eine Annotation.
///
/// Der Inhaltsstrom wird Seite fuer Seite in Zeichenreihenfolge durchlaufen;
/// printpdf schreibt je Zeile `BT / TL / Td x y / Tf /F n / TJ [...]`, sodass
/// das letzte `Td` vor einem `TJ` die Grundlinie der Zeile angibt.
fn add_links(pdf: &Path, urls: &[&str], laengen: &[usize]) -> Result<usize> {
    use lopdf::{Dictionary, Document, Object, StringFormat};

    let mut doc = Document::load(pdf)?;
    let seiten: Vec<(u32, lopdf::ObjectId)> = doc.get_pages().into_iter().collect();

    let num = |o: &Object| -> Option<f64> {
        match o {
            Object::Real(r) => Some(*r as f64),
            Object::Integer(i) => Some(*i as f64),
            _ => None,
        }
    };

    let mut gesetzt = 0usize;
    for (_, page_id) in seiten {
        let content = doc.get_and_decode_page_content(page_id)?;

        let mut pos = (0.0f64, 0.0f64);
        let mut size = 0.0f64;
        let mut zeilen: Vec<(f64, f64, f64)> = Vec::new();
        for op in &content.operations {
            match op.operator.as_str() {
                "Td" | "TD" if op.operands.len() >= 2 => {
                    if let (Some(x), Some(y)) = (num(&op.operands[0]), num(&op.operands[1])) {
                        pos = (x, y);
                    }
                }
                "Tm" if op.operands.len() >= 6 => {
                    if let (Some(x), Some(y)) = (num(&op.operands[4]), num(&op.operands[5])) {
                        pos = (x, y);
                    }
                }
                "Tf" if op.operands.len() >= 2 => {
                    if let Some(s) = num(&op.operands[1]) {
                        size = s;
                    }
                }
                "Tj" | "TJ" => {
                    let ist_link = LINK_SIZES
                        .iter()
                        .any(|s| (size - *s as f64).abs() < 0.01);
                    let schon = zeilen
                        .last()
                        .map(|(x, y, _)| (*x, *y) == pos)
                        .unwrap_or(false);
                    if ist_link && !schon {
                        zeilen.push((pos.0, pos.1, size));
                    }
                }
                _ => {}
            }
        }
        if zeilen.is_empty() {
            continue;
        }

        let mut annots: Vec<Object> = Vec::new();
        for (x, y, size) in &zeilen {
            let Some(url) = urls.get(gesetzt) else { break };
            let chars = laengen.get(gesetzt).copied().unwrap_or(40);
            gesetzt += 1;

            let breite = chars as f64 * size * AVG_ADVANCE_EM;
            let rechts = (x + breite + 2.0).min(A4_WIDTH_PT - MARGIN_PT);

            let mut action = Dictionary::new();
            action.set("S", Object::Name(b"URI".to_vec()));
            action.set(
                "URI",
                Object::String(url.as_bytes().to_vec(), StringFormat::Literal),
            );

            let mut annot = Dictionary::new();
            annot.set("Type", Object::Name(b"Annot".to_vec()));
            annot.set("Subtype", Object::Name(b"Link".to_vec()));
            annot.set(
                "Rect",
                Object::Array(vec![
                    Object::Real((*x - 2.0) as f32),
                    Object::Real((*y - 2.0) as f32),
                    Object::Real(rechts as f32),
                    Object::Real((*y + size + 2.0) as f32),
                ]),
            );
            annot.set("Border", Object::Array(vec![0.into(), 0.into(), 0.into()]));
            annot.set("A", Object::Dictionary(action));
            annots.push(Object::Dictionary(annot));
        }

        if let Ok(page) = doc.get_object_mut(page_id).and_then(|o| o.as_dict_mut()) {
            page.set("Annots", Object::Array(annots));
        }
    }

    doc.save(pdf)?;
    Ok(gesetzt)
}

// ---------------------------------------------------------------------------
// Satz
// ---------------------------------------------------------------------------

pub fn render(out: &Path, font_dir: &str) -> Result<usize> {
    let family = load_font_family(font_dir)?;
    let mut doc = genpdf::Document::new(family);
    doc.set_title(format!("{TITEL} {TITEL2} – {UNTERTITEL}"));
    doc.set_minimal_conformance();
    doc.set_font_size(S_TEXT);
    doc.set_line_spacing(ZEILENABSTAND);

    let mut deco = genpdf::SimplePageDecorator::new();
    deco.set_margins(RAND_MM as u8);
    deco.set_header(move |page| {
        let mut p = Paragraph::default();
        if page > 1 {
            p.push_styled(
                format!("{KOPFZEILE} · Seite {page}"),
                Style::new().with_color(HELLGRAU).with_font_size(S_KOPF),
            );
        }
        p.aligned(Alignment::Right)
            .padded(Margins::trbl(0, 0, 5, 0))
    });
    doc.set_page_decorator(deco);

    // Titelblock
    doc.push(
        Paragraph::new(TITEL.to_string())
            .styled(Style::new().with_font_size(S_H1).with_color(BORDEAUX).bold()),
    );
    doc.push(
        Paragraph::new(TITEL2.to_string())
            .styled(Style::new().with_font_size(S_H1).with_color(BORDEAUX).bold()),
    );
    doc.push(
        Paragraph::new(UNTERTITEL.to_string())
            .styled(Style::new().with_font_size(S_H3).with_color(GRAU))
            .padded(Margins::trbl(2, 0, 1, 0)),
    );
    doc.push(
        Paragraph::new(STAND.to_string())
            .styled(Style::new().with_font_size(S_KLEIN).with_color(HELLGRAU))
            .padded(Margins::trbl(0, 0, 4, 0)),
    );

    let mut korpus = LinearLayout::vertical();
    baue(DOKUMENT, &mut korpus);
    doc.push(korpus);

    doc.push(ueberschrift("Quellen", S_H2, BORDEAUX));
    let mut quellen = LinearLayout::vertical();
    for (label, v) in QUELLEN {
        quellen.push(
            Paragraph::new(label.to_string())
                .styled(Style::new().with_font_size(S_KLEIN).with_color(GRAU)),
        );
        quellen.push(verweiszeile(v, LINK_KLEIN).padded(Margins::trbl(0, 0, 1, 0)));
    }
    doc.push(quellen);

    doc.push(Break::new(1));
    doc.push(
        absatz(FUSS, Style::new().with_font_size(S_KLEIN).with_color(HELLGRAU))
            .padded(Margins::trbl(2, 0, 0, 0)),
    );

    doc.render_to_file(out)
        .map_err(|e| anyhow!("PDF schreiben {}: {}", out.display(), e))?;

    let urls = urls();
    let laengen = anzeigelaengen();
    let gesetzt = add_links(out, &urls, &laengen)?;
    if gesetzt != urls.len() {
        return Err(anyhow!(
            "Link-Overlay: {} Zeilen in Linkgroesse gefunden, aber {} URLs erwartet – \
             die Zuordnung waere verschoben. Steht irgendwo Text in {} oder {} pt?",
            gesetzt,
            urls.len(),
            LINK_KLEIN,
            LINK_GROSS
        ));
    }
    Ok(gesetzt)
}
