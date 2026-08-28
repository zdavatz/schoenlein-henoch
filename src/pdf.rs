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
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{anyhow, Result};
use genpdf::elements::{LinearLayout, Paragraph, UnorderedList};
use genpdf::error::Error as GenError;
use genpdf::fonts::FontCache;
use genpdf::render::Area;
use genpdf::style::{Color, Style};
use genpdf::{Alignment, Context, Element, Margins, Mm, Position, RenderResult, Size};

use crate::inhalt::{Block, Span, Tabelle, Verweis};
use crate::inhalt::{DOKUMENT, KOPFZEILE, QUELLEN, STAND, TITEL, TITEL2, UNTERTITEL};

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

/// Zerlegt die Textstuecke eines Absatzes in Paare aus Text und Stil.
///
/// genpdf zerlegt jedes Stueck einzeln in Woerter und darf an jeder Grenze
/// zwischen zwei Stuecken umbrechen; ein fuehrendes Leerzeichen wird dabei ein
/// Wort fuer sich. Beides ergibt haessliche Zeilenanfaenge - eine Zeile, die
/// mit einem Leerzeichen beginnt, oder ein Semikolon, das von seinem Wort
/// abgerissen wird. Deshalb wandern fuehrende Leerzeichen und Satzzeichen
/// vorher ans Ende des vorangehenden Stuecks.
fn stuecke(sp: &[Span], basis: Style) -> Vec<(String, Style)> {
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
    teile
}

fn absatz(sp: &[Span], basis: Style) -> Paragraph {
    let mut p = Paragraph::default();
    for (text, stil) in stuecke(sp, basis) {
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

/// Innenabstand der Zellen in Millimetern.
const ZELL_X: f64 = 1.6;
const ZELL_Y: f64 = 1.0;

const LINIE: Color = Color::Rgb(0xc9, 0xcc, 0xd1);

/// Gesetzt, sobald ein einzelnes Wort breiter ist als seine Spalte. genpdf
/// laesst ein solches Wort **stillschweigend weg** - der Text fehlt dann im
/// PDF, ohne dass irgendetwas fehlschlaegt. `render` bricht deshalb am Ende
/// ab, wenn dieses Flag steht.
static WORT_ZU_BREIT: AtomicBool = AtomicBool::new(false);

/// Schaetzt die Hoehe einer Zelle, indem sie genpdfs Wortumbruch nachbildet:
/// gebrochen wird an Leerzeichen, das Leerzeichen bleibt beim Wort davor.
fn zellhoehe(fc: &FontCache, sp: &[Span], basis: Style, breite: f64) -> f64 {
    let mut zeilen = 1usize;
    let mut x = 0.0f64;
    let mut zh = 0.0f64;
    for (text, stil) in stuecke(sp, basis) {
        zh = zh.max(f64::from(stil.line_height(fc)));
        for wort in text.split_inclusive(' ') {
            let w = f64::from(stil.str_width(fc, wort));
            if w > breite {
                WORT_ZU_BREIT.store(true, Ordering::Relaxed);
                eprintln!(
                    "Wort passt nicht in die Spalte: «{}» braucht {:.1} mm, verfügbar sind {:.1} mm",
                    wort.trim(),
                    w,
                    breite
                );
            }
            if x + w > breite && x > 0.0 {
                zeilen += 1;
                x = w;
            } else {
                x += w;
            }
        }
    }
    zh * zeilen as f64
}

/// Haelt einen Kasten zusammen: passt er nicht mehr auf die Seite, wird der
/// Umbruch davor gesetzt statt mittendrin.
///
/// Ohne das steht die Ueberschrift eines Warnkastens allein am Seitenfuss und
/// der Inhalt auf der naechsten Seite. genpdf kennt kein «keep together»,
/// aber `area.size().height` verraet die verbleibende Hoehe - passt die
/// geschaetzte Hoehe nicht mehr hinein, geben wir eine leere Flaeche mit
/// `has_more` zurueck und genpdf faengt eine neue Seite an.
struct Zusammenhalten<E: Element> {
    inhalt: E,
    /// Was hier zusammengehalten wird - nur fuer MASS_DEBUG.
    was: &'static str,
    /// Geschaetzte Hoehe in Millimetern.
    hoehe: f64,
    /// Genau ein erzwungener Umbruch, sonst schoebe ein Kasten, der auf keine
    /// Seite passt, endlos weiter.
    umbruch_offen: bool,
}

impl<E: Element> Element for Zusammenhalten<E> {
    fn render(
        &mut self,
        context: &Context,
        area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, GenError> {
        if std::env::var("MASS_DEBUG").is_ok() && self.umbruch_offen {
            eprintln!(
                "{}: geschätzt {:.1} mm, verbleibend {:.1} mm{}",
                self.was,
                self.hoehe,
                f64::from(area.size().height),
                if self.hoehe > f64::from(area.size().height) { "  -> Umbruch" } else { "" }
            );
        }
        if self.umbruch_offen && self.hoehe > f64::from(area.size().height) {
            self.umbruch_offen = false;
            return Ok(RenderResult {
                size: Size::new(area.size().width, Mm::from(0.0)),
                has_more: true,
            });
        }
        self.umbruch_offen = false;
        let r = self.inhalt.render(context, area, style)?;
        if std::env::var("MASS_DEBUG").is_ok() {
            eprintln!("  {} tatsächlich {:.1} mm", self.was, f64::from(r.size.height));
        }
        Ok(r)
    }
}

/// Schaetzt die Hoehe eines Kastens. Muss den Aufbau in `baue()` nachbilden:
/// Titelzeile, dann je Absatz seine Zeilen plus 2 mm Abstand, dazu die
/// Innen- und Aussenraender des Rahmens.
fn kastenhoehe(
    fc: &FontCache,
    titel: &'static str,
    bs: &'static [Block],
    style: Style,
    farbe: Color,
) -> f64 {
    // Satzbreite abzueglich Seitenrand, Rahmen und Innenabstand.
    let breite = 210.0 - 2.0 * RAND_MM - 2.0 * 3.0;
    let titelstil = style.and(Style::new().with_font_size(S_H3).with_color(farbe).bold());
    let mut h = zellhoehe(fc, &[Span::T(titel)], titelstil, breite) + 2.0;
    for b in bs {
        h += match b {
            Block::P(sp) => zellhoehe(fc, sp, style.and(grund()), breite) + 2.0,
            Block::Klein(sp) => {
                zellhoehe(fc, sp, style.and(Style::new().with_font_size(S_KLEIN)), breite) + 2.0
            }
            // Kaesten enthalten in diesem Dokument nur Absaetze. Kaeme etwas
            // anderes dazu, waere die Schaetzung zu klein und der Kasten
            // wuerde wieder umbrochen - unschoen, aber nicht falsch.
            _ => 0.0,
        };
    }
    // Innenabstand oben/unten, Rahmen, Abstand darunter.
    h + 2.0 + 1.0 + 4.0 + 2.0
}

/// Satzbreite in Millimetern.
const SATZBREITE: f64 = 210.0 - 2.0 * RAND_MM;

/// Hoechstens so viel wird einer Ueberschrift als Gefolge zugerechnet. Ohne
/// Deckel schoebe eine Ueberschrift vor einem langen Abschnitt jede halbwegs
/// gefuellte Seite um.
const MITNEHMEN_MAX: f64 = 46.0;

fn ueberschriftshoehe(fc: &FontCache, t: &'static str, groesse: u8, style: Style) -> f64 {
    let stil = style.and(Style::new().with_font_size(groesse).bold());
    let oben = if groesse >= S_H2 { 4.0 } else { 2.0 };
    zellhoehe(fc, &[Span::T(t)], stil, SATZBREITE) + oben + 1.0
}

/// Was einer Ueberschrift folgt und mit ihr auf dieselbe Seite gehoert.
/// Gedeckelt, damit ein langer Abschnitt nicht die ganze Seite verschiebt.
fn folgehoehe(fc: &FontCache, b: &'static Block, style: Style) -> f64 {
    let h = match b {
        Block::P(sp) => zellhoehe(fc, sp, style.and(grund()), SATZBREITE) + 2.0,
        Block::Klein(sp) => {
            zellhoehe(fc, sp, style.and(Style::new().with_font_size(S_KLEIN)), SATZBREITE) + 2.0
        }
        Block::Liste(items) => items
            .first()
            .map(|it| zellhoehe(fc, it, style.and(grund()), SATZBREITE - 4.0) + 1.0)
            .unwrap_or(0.0),
        Block::Tab(t) => {
            // Kopfzeile und erste Zeile - weniger sagt nichts aus.
            let anteil = |i: usize| {
                SATZBREITE * t.gewichte[i] as f64 / t.gewichte.iter().sum::<usize>() as f64
                    - 2.0 * ZELL_X
            };
            let mut h = 0.0f64;
            if !t.kopf.is_empty() {
                let kstil = style.and(Style::new().with_font_size(S_KLEIN).bold());
                for (i, k) in t.kopf.iter().enumerate() {
                    h = h.max(zellhoehe(fc, &[Span::T(k)], kstil, anteil(i)));
                }
                h += 2.0 * ZELL_Y;
            }
            if let Some(z) = t.zeilen.first() {
                let mut zh = 0.0f64;
                for (i, zelle) in z.iter().enumerate() {
                    zh = zh.max(zellhoehe(fc, zelle, style.and(grund()), anteil(i)));
                }
                h += zh + 2.0 * ZELL_Y;
            }
            h
        }
        Block::Adresse { name, rolle, zeilen, links } => {
            adresshoehe(fc, name, rolle, zeilen, links.len(), style)
        }
        // Kaesten halten sich selbst zusammen; die Ueberschrift davor soll
        // deswegen nicht die halbe Seite leer lassen.
        Block::Lead { .. } | Block::Alarm { .. } => MITNEHMEN_MAX,
        Block::H2(_) | Block::H3(_) => 0.0,
    };
    h.min(MITNEHMEN_MAX)
}

fn adresshoehe(
    fc: &FontCache,
    name: &'static str,
    rolle: &'static [Span],
    zeilen: &'static [&'static [Span]],
    links: usize,
    style: Style,
) -> f64 {
    let breite = SATZBREITE - 2.0;
    let mut h = zellhoehe(fc, &[Span::T(name)], style.and(grund().bold()), breite);
    if !rolle.is_empty() {
        h += zellhoehe(fc, rolle, style.and(grund()), breite);
    }
    for z in zeilen {
        h += zellhoehe(fc, z, style.and(grund()), breite);
    }
    let linkstil = style.and(Style::new().with_font_size(LINK_GROSS));
    h += links as f64 * f64::from(linkstil.line_height(fc));
    h + 4.0
}

/// Tabelle, die eine Zeile nie ueber einen Seitenumbruch reisst.
///
/// genpdfs `TableLayout` bricht mitten in der Zeile um: die linke Spalte steht
/// dann leer auf der Folgeseite und der Satz der rechten geht darunter weiter.
/// `Element::render` bekommt in `area.size().height` die auf der Seite noch
/// verbleibende Hoehe - damit laesst sich jede Zeile vorher messen und der
/// Umbruch selbst setzen. Die Kopfzeile wird auf jeder Seite wiederholt.
struct Zeilentabelle {
    t: &'static Tabelle,
    idx: usize,
    /// Ein Umbruch, bevor ueberhaupt eine Zeile steht, ist genau einmal
    /// erlaubt - naemlich wenn die Tabelle am Seitenende beginnt. Ohne diese
    /// Schranke koennte eine Zeile, die auf keine Seite passt, endlos
    /// weiterschieben.
    leerumbruch: bool,
}

impl Zeilentabelle {
    fn neu(t: &'static Tabelle) -> Zeilentabelle {
        Zeilentabelle { t, idx: 0, leerumbruch: true }
    }

    /// Grundstil einer Zelle. In der Chronik steht die Jahreszahl links.
    fn basis(&self, spalte: usize) -> Style {
        if self.t.chronik && spalte == 0 {
            Style::new().with_font_size(S_TEXT).with_color(BORDEAUX).bold()
        } else {
            grund()
        }
    }
}

impl Element for Zeilentabelle {
    fn render(
        &mut self,
        context: &Context,
        mut area: Area<'_>,
        style: Style,
    ) -> Result<RenderResult, GenError> {
        let mut result = RenderResult::default();
        result.size.width = area.size().width;
        if self.idx >= self.t.zeilen.len() {
            return Ok(result);
        }

        let fc = &context.font_cache;
        let seite = f64::from(area.size().height);
        let mut y = 0.0f64;

        // Kopfzeile, auf jeder Seite wiederholt.
        let kopfhoehe = if self.t.kopf.is_empty() {
            0.0
        } else {
            let basis = Style::new().with_font_size(S_KLEIN).with_color(GRAU).bold();
            let mess = style.and(basis);
            let spalten = area.split_horizontally(self.t.gewichte);
            let mut h = 0.0f64;
            for (i, sp) in spalten.iter().enumerate() {
                let breite = f64::from(sp.size().width) - 2.0 * ZELL_X;
                h = h.max(zellhoehe(fc, &[Span::T(self.t.kopf[i])], mess, breite));
            }
            h += 2.0 * ZELL_Y;
            for (i, sp) in spalten.iter().enumerate() {
                let mut ca = sp.clone();
                ca.add_offset(Position::new(ZELL_X, ZELL_Y));
                ca.set_width(Mm::from(f64::from(sp.size().width) - 2.0 * ZELL_X));
                Paragraph::new(self.t.kopf[i].to_string())
                    .styled(basis)
                    .render(context, ca, style)?;
            }
            let breite = f64::from(area.size().width);
            area.draw_line(
                vec![Position::new(0.0, h), Position::new(breite, h)],
                Style::new().with_color(LINIE),
            );
            area.add_offset(Position::new(0.0, h));
            y += h;
            h
        };

        while self.idx < self.t.zeilen.len() {
            let zeile = self.t.zeilen[self.idx];
            let spalten = area.split_horizontally(self.t.gewichte);

            let mut h = 0.0f64;
            for (i, zelle) in zeile.iter().enumerate() {
                let mess = style.and(self.basis(i));
                let breite = f64::from(spalten[i].size().width) - 2.0 * ZELL_X;
                h = h.max(zellhoehe(fc, zelle, mess, breite));
            }
            h += 2.0 * ZELL_Y;

            if y + h > seite {
                let schon_gesetzt = y > kopfhoehe + 0.01;
                if schon_gesetzt || self.leerumbruch {
                    if !schon_gesetzt {
                        self.leerumbruch = false;
                    }
                    result.has_more = true;
                    break;
                }
                // Sonst durch: eine Zeile, die auch auf einer leeren Seite
                // nicht passt, muss hier gesetzt werden.
            }

            for (i, zelle) in zeile.iter().enumerate() {
                let mut ca = spalten[i].clone();
                ca.add_offset(Position::new(ZELL_X, ZELL_Y));
                ca.set_width(Mm::from(f64::from(spalten[i].size().width) - 2.0 * ZELL_X));
                absatz(zelle, self.basis(i)).render(context, ca, style)?;
            }

            if self.t.linien {
                let breite = f64::from(area.size().width);
                area.draw_line(
                    vec![Position::new(0.0, h), Position::new(breite, h)],
                    Style::new().with_color(LINIE),
                );
            }

            area.add_offset(Position::new(0.0, h));
            y += h;
            self.idx += 1;
        }

        result.size.height = Mm::from(y);
        Ok(result)
    }
}

// ---------------------------------------------------------------------------
// Bloecke
// ---------------------------------------------------------------------------

fn baue(bs: &'static [Block], ziel: &mut LinearLayout, fc: &FontCache, style: Style) {
    for (i, b) in bs.iter().enumerate() {
        let folgt = || bs.get(i + 1).map(|n| folgehoehe(fc, n, style)).unwrap_or(0.0);
        match b {
            Block::H2(t) => ziel.push(Zusammenhalten {
                was: "H2", hoehe: ueberschriftshoehe(fc, t, S_H2, style) + folgt(),
                inhalt: ueberschrift(t, S_H2, BORDEAUX),
                umbruch_offen: true,
            }),
            Block::H3(t) => ziel.push(Zusammenhalten {
                was: "H3", hoehe: ueberschriftshoehe(fc, t, S_H3, style) + folgt(),
                inhalt: ueberschrift(t, S_H3, INK),
                umbruch_offen: true,
            }),
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
            Block::Tab(t) => ziel.push(Zeilentabelle::neu(t).padded(Margins::trbl(1, 0, 3, 0))),
            Block::Lead { werte, blocks } => {
                let mut innen = LinearLayout::vertical();
                innen.push(
                    Paragraph::new(werte.to_string())
                        .styled(Style::new().with_font_size(S_H3).with_color(BORDEAUX).bold())
                        .padded(Margins::trbl(0, 0, 2, 0)),
                );
                baue(blocks, &mut innen, fc, style);
                ziel.push(Zusammenhalten {
                    was: "Lead", hoehe: kastenhoehe(fc, werte, blocks, style, BORDEAUX),
                    inhalt: innen
                        .padded(Margins::trbl(2, 3, 1, 3))
                        .framed()
                        .padded(Margins::trbl(0, 0, 4, 0)),
                    umbruch_offen: true,
                });
            }
            Block::Alarm { titel, blocks } => {
                let mut innen = LinearLayout::vertical();
                innen.push(
                    Paragraph::new(titel.to_string())
                        .styled(Style::new().with_font_size(S_H3).with_color(ROT).bold())
                        .padded(Margins::trbl(0, 0, 2, 0)),
                );
                baue(blocks, &mut innen, fc, style);
                ziel.push(Zusammenhalten {
                    was: "Alarm", hoehe: kastenhoehe(fc, titel, blocks, style, ROT) + 3.0,
                    inhalt: innen
                        .padded(Margins::trbl(2, 3, 1, 3))
                        .framed()
                        .padded(Margins::trbl(3, 0, 4, 0)),
                    umbruch_offen: true,
                });
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
                ziel.push(Zusammenhalten {
                    was: "Adresse", hoehe: adresshoehe(fc, name, rolle, zeilen, links.len(), style),
                    inhalt: innen.padded(Margins::trbl(0, 0, 4, 2)),
                    umbruch_offen: true,
                });
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

    // Der Schriftcache wird nur zum Messen gebraucht; die Leihe endet vor dem
    // naechsten `doc.push`.
    let grundstil = Style::new()
        .with_font_size(S_TEXT)
        .with_line_spacing(ZEILENABSTAND);
    let mut korpus = LinearLayout::vertical();
    baue(DOKUMENT, &mut korpus, doc.font_cache(), grundstil);
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

    doc.render_to_file(out)
        .map_err(|e| anyhow!("PDF schreiben {}: {}", out.display(), e))?;

    if WORT_ZU_BREIT.load(Ordering::Relaxed) {
        return Err(anyhow!(
            "Mindestens ein Wort ist breiter als seine Tabellenspalte (siehe Meldungen oben). \
             genpdf lässt ein solches Wort stillschweigend weg – der Text fehlt dann im PDF. \
             Spaltengewichte in src/inhalt.rs anpassen oder kürzer formulieren."
        ));
    }

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
