# CLAUDE.md

Hinweise für Claude Code (claude.ai/code) zur Arbeit an diesem Repositorium.

## Überblick

Erzeugt ein PDF-Informationsblatt zur **IgA-Vaskulitis (Purpura
Schönlein-Henoch)**: Krankheitsbild, Abklärungen, Warnzeichen, die
Sprechstunden in Zürich, Entdeckungsgeschichte, Quellen.

Kommentare, Dokumententexte und Commit-Messages sind auf Deutsch
(Schweizer Rechtschreibung: **ss statt ß**).

## Build und Ausführung

```bash
make          # weasyprint src/iga-vaskulitis.html iga-vaskulitis.pdf
make open
```

`WARNING: Unable to subset font with Harfbuzz` erscheint bei jedem Lauf und
ist harmlos; das Makefile filtert sie weg.

## Architektur

**Eine einzige HTML-Datei mit `@page`-CSS, gerendert von WeasyPrint.**
Bewusst nicht die genpdf/Rust-Pipeline aus
[adhs-expert](https://github.com/zdavatz/adhs-expert) – hier braucht es
Tabellen, Kästen und echte Hyperlinks, und die bekommt man in CSS
geschenkt.

Vier Dinge, die beim Bauen Zeit gekostet haben:

- **Links prüft man nicht mit `grep /URI` auf der rohen Datei.** WeasyPrint
  komprimiert die Objektströme, `grep` findet null Treffer, obwohl alle
  Annotationen da sind. Erst entpacken:
  `qpdf --qdf --object-streams=disable ein.pdf raus.pdf`, dann
  `grep -c /URI raus.pdf`. Die 0 ist ein falscher Alarm, kein Fehler.
  Verlinkt sind auch `tel:` und `mailto:`.
- **`83 g/l` bricht am Schrägstrich um.** Deshalb die Klasse
  `.nb { white-space: nowrap; }` um jede Wertangabe, mit `&nbsp;` vor der
  Einheit. Wer neue Messwerte einfügt, muss das mitziehen.
- **Kein `break-before: page` von Hand.** Ein erzwungener Umbruch vor einem
  Abschnitt hinterlässt eine halbleere Seite. Zusammenhalten regelt
  `break-inside: avoid` auf `tr`, `.adresse`, `.alarm` und `.quellen`,
  `break-after: avoid` auf den Überschriften.
- **Sichtprüfung Seite für Seite** nach jeder Änderung:
  `pdftoppm -png -r 70 iga-vaskulitis.pdf pruef` und die Bilder ansehen.
  Achtung: ab zehn Seiten wechselt die Nummerierung der Ausgabedateien von
  `pruef-4.png` auf `pruef-04.png`.

## Inhaltliches

- **Jede medizinische Aussage gehört belegt**, nicht aus dem Gedächtnis
  geschrieben. Die Quellen stehen am Schluss des Dokuments verlinkt; neue
  Aussagen kommen mit einer neuen Quelle oder gar nicht.
- Die Angaben zu Erwachsenen und Betagten sind der Kern. Das meiste, was
  man auf Deutsch zu dieser Krankheit findet, betrifft Kinder und führt bei
  einer 84-jährigen Patientin in die Irre.
- Adressen und Zuständigkeiten veralten. Das Datum in `p.stand` und im
  Schlussabsatz beim Prüfen mitführen.

## Vertraulichkeit

**Dieses Repositorium ist öffentlich.**

- **Keine Namen.** Der Anlass ist ein realer Fall; die Person wird nirgends
  benannt. Alter, Laborwerte und Verlauf sind ausdrücklich in Ordnung,
  Namen und Angehörige nicht.
- Keine privaten Mailadressen, keine Zugangsdaten, keine
  Anwendungspasswörter in eingecheckten Dateien. Ein `.gitignore`-Eintrag
  ist eine Vorsichtsmassnahme, kein Schutz – ein `git add -f` genügt. Vor
  jedem Commit `git status` prüfen.
- Die Mailadressen in den Adressangaben sind öffentliche Kontaktangaben von
  Universitätsspital und Kinderspital Zürich, keine privaten.

## Lizenz

GPL-3.0. Neue Quelldateien tragen einen GPL-3.0-verträglichen Kopf, und
jede Abhängigkeit muss mit GPL-3.0 vereinbar sein.
