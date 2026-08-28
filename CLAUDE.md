# CLAUDE.md

Hinweise für Claude Code (claude.ai/code) zur Arbeit an diesem Repositorium.

## Überblick

Erzeugt ein **HTML und ein PDF** zur **IgA-Vaskulitis (Purpura
Schönlein-Henoch)**: Entdeckungsgeschichte, Krankheitsbild, Verlauf der
letzten Wochen, Abklärungen, Behandlung, Abführmittel, die Sprechstunden in
Zürich, Quellen.

Die **Entdeckungsgeschichte steht zuoberst**, vor der Ausgangslage – so
gewünscht. Wer Abschnitte umstellt, lässt sie vorn.

Kommentare, Dokumententexte und Commit-Messages sind auf Deutsch
(Schweizer Rechtschreibung: **ss statt ß**).

## Build und Ausführung

```bash
make          # cargo build --release && ./target/release/infoblatt
make pruef    # Seitenbilder zur Sichtprüfung
make open
```

Beides – HTML und PDF – entsteht im selben Lauf. `FONT_DIR` überschreibt das
Schriftverzeichnis (Vorgabe `./fonts`). Der Build läuft offline, sofern
`genpdf 0.2`, `lopdf 0.34` und `anyhow` im Cargo-Cache liegen.

**`iga-vaskulitis.pdf` ist eingecheckt** – es soll ohne Rust-Toolchain
herunterladbar sein. Wer den Inhalt ändert, baut neu und commitet das PDF
mit; sonst steht im Repositorium eine alte Fassung. Das HTML bleibt
draussen, es ist nur eine Zwischenstufe.

## Architektur

**Pure Rust, kein Chrome und kein WeasyPrint.** Der Vorgänger dieses
Repositoriums rief `weasyprint` auf eine von Hand gepflegte HTML-Datei auf;
das ist ersetzt. Die Pipeline ist jetzt dieselbe wie in
[adhs-expert](https://github.com/zdavatz/adhs-expert): `genpdf` schreibt über
`printpdf`, `lopdf` legt die Links nach, die DejaVu-Sans-Familie wird
eingebettet.

| Datei | Aufgabe |
|---|---|
| `src/inhalt.rs` | **Der gesamte Text**, als Daten. Wer Inhalt ändert, ändert ihn nur hier. |
| `src/html.rs` | HTML-Ausgabe, bindet `src/blatt.css` per `include_str!` ein |
| `src/pdf.rs` | PDF-Satz und Link-Overlay |
| `src/main.rs` | schreibt beide Dateien |
| `fonts/` | DejaVu Sans, wird ins PDF eingebettet |

Fünf Dinge, die beim Bauen Zeit gekostet haben:

- **Die Schriftgrössen 9 pt und 11 pt gehören ausschliesslich den
  Verweiszeilen.** `genpdf` 0.2 kennt keine Hyperlinks. Deshalb steht jeder
  Verweis allein auf seiner Zeile und in einer dieser beiden Grössen; nach
  dem Rendern läuft `add_links()` den Inhaltsstrom Seite für Seite durch,
  sammelt die Grundlinien aller Zeilen in genau diesen Grössen ein und legt
  `/Link`-Annotationen darüber. Sobald irgendwo sonst Text in 9 oder 11 pt
  gesetzt wird, verschiebt sich die Zuordnung und jeder Link zeigt aufs
  falsche Ziel. `render()` bricht mit Fehler ab, wenn die Zahl der gefundenen
  Zeilen nicht der Zahl der URLs entspricht – **diese Prüfung nie
  entfernen**, sie ist die einzige Absicherung. Fliesstext läuft auf 10,
  Kleingedrucktes auf 8, die Kopfzeile auf 7 pt.
- **`urls()` und `anzeigelaengen()` müssen dieselbe Reihenfolge liefern wie
  der Satz**: erst der Dokumentbaum (Kästen, Adressen, Kontaktzeilen), dann
  die Quellen. Wer die Reihenfolge in `baue()` ändert, muss beide mitziehen.
- **`link_text()` kürzt lange Adressen in der Mitte**; verlinkt wird immer
  das Original. `MAX_LINK_CHARS = 76` garantiert, dass eine URL nie umbricht
  – ein Umbruch ergäbe zwei Zeilen in Linkgrösse und damit wieder eine
  Verschiebung.
- **Links prüft man nicht mit `grep /URI` auf der rohen Datei.** Erst
  entpacken: `qpdf --qdf --object-streams=disable ein.pdf raus.pdf`, dann
  `grep -c /URI raus.pdf`. Verlinkt sind auch `tel:` und `mailto:`.
- **`83 g/l` bricht am Schrägstrich um.** Deshalb `Span::N` um jede
  Wertangabe: im HTML `span.nb`, im PDF geschützte Leerzeichen. Wer neue
  Messwerte einfügt, muss das mitziehen.

**Sichtprüfung Seite für Seite** nach jeder Änderung: `make pruef` und die
Bilder ansehen. Achtung: ab zehn Seiten wechselt die Nummerierung der
Ausgabedateien von `pruef-4.png` auf `pruef-04.png`.

`genpdf` 0.2 kennt kein «keep together» und keine Hintergrundfarben. Kästen
sind deshalb gerahmt statt hinterlegt, und ein Kasten, der nicht mehr auf die
Seite passt, wird umbrochen. Wenn das stört, ist der Weg derselbe wie in
adhs-expert: Höhen vorab messen und den Umbruch selbst setzen.

## Inhaltliches

- **Jede medizinische Aussage gehört belegt**, nicht aus dem Gedächtnis
  geschrieben. Die Quellen stehen am Schluss des Dokuments verlinkt; neue
  Aussagen kommen mit einer neuen Quelle oder gar nicht.
- **Keine allgemeinen Ratschläge.** Alles im Blatt muss belegt *und* auf
  diese Patientin zugeschnitten sein. Herausgeflogen sind darum: die
  generische Warnzeichenliste «Was nicht warten darf», die Haushaltstipps
  gegen den süssen Geschmack (Strohhalm, Bouillon, Tagesmenge verteilen),
  der Hinweis auf den rektalen Weg und der Abschnitt zum Kinderspital samt
  Kispi-Wiki-Quelle. Nichts davon wieder einbauen, ohne zu fragen.
- **Arzneimittelangaben ausschliesslich über [ch.oddb.org](https://ch.oddb.org)
  belegen, nicht über compendium.ch.** Dieselbe Fachinformation steht auf
  ch.oddb.org und ist dort frei zugänglich. Die Fachinfo einer Zulassung
  liegt unter `https://ch.oddb.org/de/gcc/fachinfo/reg/<Swissmedic-Nr>`, die
  Suche unter `.../search/zone/drugs/search_query/<name>`. Die Seite steht
  hinter einem Bot-Schutz (Anubis): `curl` und einfache Fetch-Werkzeuge
  bekommen die Challenge-Seite, ein echter Browser kommt durch.
- Die Angaben zu Erwachsenen und Betagten sind der Kern. Das meiste, was
  man auf Deutsch zu dieser Krankheit findet, betrifft Kinder und führt bei
  einer 84-jährigen Patientin in die Irre.
- Adressen, Präparate und Zuständigkeiten veralten. Das Datum in `STAND` und
  in `FUSS` beim Prüfen mitführen.

## Vertraulichkeit

**Dieses Repositorium ist öffentlich.**

- **Keine Namen.** Der Anlass ist ein realer Fall; die Person wird nirgends
  benannt. Alter, Laborwerte, Beschwerden und Verlauf sind ausdrücklich in
  Ordnung, Namen und Angehörige nicht.
- Keine privaten Mailadressen, keine Zugangsdaten, keine
  Anwendungspasswörter in eingecheckten Dateien. Ein `.gitignore`-Eintrag
  ist eine Vorsichtsmassnahme, kein Schutz – ein `git add -f` genügt. Vor
  jedem Commit `git status` prüfen.
- Die Mailadressen in den Adressangaben sind öffentliche Kontaktangaben von
  Universitätsspital und Kinderspital Zürich, keine privaten.

## Lizenz

GPL-3.0. Neue Quelldateien tragen einen GPL-3.0-verträglichen Kopf, und
jede Abhängigkeit muss mit GPL-3.0 vereinbar sein.
