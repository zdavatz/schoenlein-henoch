# CLAUDE.md

Hinweise für Claude Code (claude.ai/code) zur Arbeit an diesem Repositorium.

## Überblick

Erzeugt **zwei Blätter**, je als HTML und PDF. Das Hauptblatt zur
**IgA-Vaskulitis (Purpura Schönlein-Henoch)**: Entdeckungsgeschichte, Krankheitsbild, Verlauf der
letzten Wochen, Abklärungen, Behandlung, Abführmittel, die Sprechstunden in
Zürich, Quellen.

Die **Entdeckungsgeschichte steht zuoberst**, vor der Ausgangslage – so
gewünscht. Wer Abschnitte umstellt, lässt sie vorn.

Kommentare, Dokumententexte und Commit-Messages sind auf Deutsch
(Schweizer Rechtschreibung: **ss statt ß**).

## Build und Ausführung

```bash
make              # cargo build --release && ./target/release/infoblatt
make pruef        # Seitenbilder des Hauptblatts zur Sichtprüfung
make pruef-hunger # dasselbe für das Begleitblatt
make open
```

Alle vier Dateien entstehen im selben Lauf. `FONT_DIR` überschreibt das
Schriftverzeichnis (Vorgabe `./fonts`). Der Build läuft offline, sofern
`genpdf 0.2`, `lopdf 0.34` und `anyhow` im Cargo-Cache liegen.

**Beide PDFs sind eingecheckt** – sie sollen ohne Rust-Toolchain
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
| `src/inhalt.rs` | **Der gesamte Text des Hauptblatts**, als Daten. Wer Inhalt ändert, ändert ihn nur hier. |
| `src/hunger.rs` | Dasselbe für das Begleitblatt zum Kostaufbau nach langem Hungern |
| `src/html.rs` | HTML-Ausgabe |
| `src/blatt.css` | Gestaltung der HTML-Ausgabe, per `include_str!` eingebunden |
| `src/pdf.rs` | PDF-Satz und Link-Overlay |
| `src/main.rs` | schreibt beide Dateien |
| `fonts/` | DejaVu Sans, wird ins PDF eingebettet |

**Zwei Dokumente, eine Pipeline.** `Dokument` in `src/inhalt.rs` bündelt
Titel, Kolumnentitel, Blocks und Quellen; `html::render(&d)` und
`pdf::render(&d, …)` setzen jedes Dokument, das ihnen gereicht wird. Ein
drittes Blatt ist eine weitere Datei nach dem Muster von `src/hunger.rs`, ein
`mod`, ein `schreibe(…)`-Aufruf und zwei Dateinamen in `src/main.rs`. Zwei
Fallen dabei:

- **`GEZEICHNET` und `WORT_ZU_BREIT` sind global** und leben länger als ein
  Dokument. `pdf::render()` leert beide zu Beginn. Ohne das zählt die
  Link-Prüfung die Striche des vorigen Blattes mit und bricht mit «die
  Zuordnung wäre verschoben» ab – bei zwei Blättern der erste Fehler, den man
  sieht.
- **Der lebende Kolumnentitel steht im CSS**, als `content:`-Zeile der
  `@page`-Regel in `src/blatt.css`. `html::render` ersetzt darin `KOPFZEILE`
  durch die Kopfzeile des jeweiligen Dokuments; die Vorgabe im CSS ist die des
  Vaskulitis-Blattes. Wer den Text im CSS ändert, muss `KOPFZEILE` mitziehen,
  sonst greift die Ersetzung ins Leere (ein `debug_assert` fängt das ab).

Acht Dinge, die beim Bauen Zeit gekostet haben:

- **Links liegen hinter dem Wort, nicht auf einer eigenen Zeile.** Das
  frühere Verfahren – jeder Verweis allein auf seiner Zeile, erkannt an einer
  reservierten Schriftgrösse – ist weg. Es zwang dazu, nackte URLs unter die
  Absätze zu setzen, und das war der Sache nicht angemessen: Wer liest, soll
  das Wort anklicken.
- **`Fliesstext` in `src/pdf.rs` setzt Absätze selbst**, statt genpdfs
  `Paragraph` zu nehmen. Nur so ist bekannt, wo ein Wort zu liegen kommt. Der
  Umbruch ist derselbe wie in genpdf – gebrochen wird an Leerzeichen, das
  Leerzeichen bleibt beim Wort davor – und die Breiten kommen aus
  `Style::str_width`, also aus derselben Quelle wie bei genpdf, samt Kerning.
  Wer `umbrechen()` anfasst, ändert den Satz des ganzen Dokuments.
- **Die Klickfläche ist die Unterstreichung.** Unter jedes verlinkte Wort
  zeichnet `Fliesstext` eine Linie in `LINK_MARKE`; `draw_line` schreibt sie
  mit **Seitenkoordinaten** in den Inhaltsstrom, und genau daran findet
  `add_links()` die Rechtecke. Die Linie ist zugleich die Unterstreichung,
  die den Link sichtbar macht – Anzeige und Messpunkt sind dasselbe, deshalb
  können sie nicht auseinanderlaufen.
- **`LINK_MARKE` darf nirgends sonst als Strichfarbe vorkommen.** Als
  Füllfarbe für Text ist sie unbedenklich, als Strichfarbe nicht: Jeder
  Strich in dieser Farbe wird zu einer Link-Annotation. Tabellenlinien nehmen
  `LINIE`, Rahmen die Vorgabe Schwarz.
- **printpdf schreibt Farben mit zwei Nachkommastellen.** `0x7b` wird zu
  `0.48`, nicht zu `0.4824`. Der Farbvergleich in `add_links()` rundet
  deshalb auf zwei Stellen – mit einer engeren Toleranz findet er die eigene
  Farbe nicht, und es gibt null Treffer statt einer Fehlermeldung.
- **`GEZEICHNET` hält die Ziele in Zeichenreihenfolge.** Bricht ein Link über
  zwei Zeilen um, zeichnet `Fliesstext` zwei Striche und legt das Ziel
  zweimal ab – die Zuordnung bleibt eins zu eins. `render()` bricht ab, wenn
  die Zahl der gefundenen Striche nicht zur Zahl der gezeichneten passt.
  **Diese Prüfung nie entfernen.**
- **Links prüft man nicht mit `grep /URI` auf der rohen Datei.** Erst
  entpacken: `qpdf --qdf --object-streams=disable ein.pdf raus.pdf`, dann
  `grep -c /URI raus.pdf`. Verlinkt sind auch `tel:` und `mailto:`.
- **`83 g/l` bricht am Schrägstrich um.** Deshalb `Span::N` um jede
  Wertangabe: im HTML `span.nb`, im PDF geschützte Leerzeichen. Wer neue
  Messwerte einfügt, muss das mitziehen.

**Inhalt wird per Skript geändert, nicht von Hand** – `src/inhalt.rs` ist
inzwischen über 1500 Zeilen, und der Text besteht aus verschachtelten
`Span`-Folgen. Bewährt hat sich ein kurzes Python-Skript mit einer Funktion,
die den Anker zählt und bei Abweichung abbricht, bevor irgendetwas
geschrieben wird:

```python
def rep(alt, neu, anzahl=1):
    global s, n
    if s.count(alt) != anzahl:
        sys.exit("ANKER %d-mal statt %d: %r" % (s.count(alt), anzahl, alt[:100]))
    s = s.replace(alt, neu); n += 1
```

Drei Fallen dabei, alle am 1. September 2026 zugeschlagen:

- **Der Commit läuft, auch wenn das Skript abbricht.** `python3 - <<'PY' … PY`
  und danach `git add -A && git commit` in derselben Bash-Zeile sind durch
  einen Zeilenumbruch getrennt, nicht durch `&&`. Bricht das Skript an einem
  Anker ab, schreibt es nichts – und der Commit geht trotzdem hinaus, mit
  einer Nachricht, die Änderungen behauptet, die nicht drin sind. **Entweder
  `&&` zwischen Skript und Commit setzen oder `git status --short` dazwischen
  lesen.**
- **Gleichlautende Anker.** «Fünf Punkte aus der Fachinformation…» steht
  sowohl im Novalgin- als auch im Pantoprazol-Abschnitt. Die Zählung fängt
  das ab; die Abhilfe ist, den vorangehenden, eindeutigen Satz in den Anker
  zu nehmen.
- **Eine Ersetzung muss die Span-Folge geschlossen zurücklassen.** Endet der
  neue Text mitten in einer Klammerfolge, entsteht `…") "), B("…`, und der
  Compiler meldet «unknown start of token» irgendwo weit entfernt. Deshalb
  nach **jeder** Skriptausführung `make` laufen lassen, bevor committet wird.

**Sichtprüfung Seite für Seite** nach jeder Änderung: `make pruef` und die
Bilder ansehen. Achtung: ab zehn Seiten wechselt die Nummerierung der
Ausgabedateien von `pruef-4.png` auf `pruef-04.png`.

**Tabellen setzt ein eigenes Element, nicht genpdfs `TableLayout`.** Dessen
Umbruch reisst Zeilen mittendurch: die linke Spalte steht leer auf der
Folgeseite, der Satz der rechten geht darunter weiter. `Zeilentabelle` in
`src/pdf.rs` misst deshalb jede Zeile vorher – `zellhoehe()` bildet genpdfs
Wortumbruch nach – und bricht selbst um, bevor eine Zeile nicht mehr passt;
`area.size().height` liefert dabei die auf der Seite verbleibende Höhe. Die
Kopfzeile wird auf jeder Folgeseite wiederholt – aber **erst gemessen, dann
gezeichnet**: Passt unter sie keine einzige Zeile mehr, bricht die Tabelle um,
bevor irgendetwas auf dem Papier steht. Sonst bleibt die Kopfzeile allein am
Seitenfuss stehen und wird auf der Folgeseite gleich noch einmal gesetzt. Wer
`zellhoehe()` anfasst, muss den Schalter `leerumbruch` mitdenken: Er lässt
genau einen Umbruch zu, bevor eine erste Zeile steht, und verhindert damit die
Endlosschleife bei einer Zeile, die auf keine Seite passt; sobald in einem
Durchgang mindestens eine Zeile gesetzt wurde, wird er wieder scharf
gestellt.

**Ein Aufzählungspunkt darf nicht allein am Seitenfuss stehen.** genpdfs
`UnorderedList` zeichnet das Zeichen «·», bevor es weiss, ob die erste Zeile
des Punktes noch auf die Seite passt – am 3. September 2026 stand so ein
einzelner Punkt am Fuss von Seite 5, der Text begann auf Seite 6. Seit dann
setzt `baue()` jeden Listenpunkt als eigene Einpunktliste in ein
`Zusammenhalten`, mit der gemessenen Höhe des Punktes, gedeckelt auf
`MITNEHMEN_MAX`, damit sehr lange Punkte weiter umbrechen dürfen. Die
Ränder oben/unten trägt nur der erste bzw. letzte Punkt. Prüfen lässt sich das
mit `pdftotext -f N -l N` je Seite: Die letzte nichtleere Zeile darf nie nur
«·» sein.

**Kästen, Überschriften und Adressen hält `Zusammenhalten` zusammen.**
Dasselbe Prinzip: Höhe vorher schätzen, mit `area.size().height` vergleichen
und, wenn es nicht mehr reicht, eine leere Fläche mit `has_more` zurückgeben
– dann fängt genpdf eine neue Seite an. Eine Überschrift rechnet sich
dabei an, was ihr folgt (`folgehoehe()`, gedeckelt auf `MITNEHMEN_MAX`),
sonst steht sie allein am Seitenfuss. `umbruch_offen` lässt genau einen
erzwungenen Umbruch zu und verhindert die Endlosschleife.

**Ein Wort, das breiter ist als seine Tabellenspalte, lässt genpdf
stillschweigend weg** – der Text fehlt im PDF, und nichts schlägt fehl. Genau
das ist mit «kontraindiziert» in der Interaktionstabelle passiert. `zellhoehe()`
meldet solche Wörter jetzt auf stderr und setzt `WORT_ZU_BREIT`; `render()`
bricht am Ende damit ab. **Diese Prüfung nie entfernen** – sie ist neben der
Link-Zählung die zweite Absicherung gegen still verlorenen Inhalt. Abhilfe:
Spaltengewichte in `src/inhalt.rs` anpassen oder kürzer formulieren.

**`MASS_DEBUG=1` zeigt die Schätzungen** – je Element geschätzte Höhe,
verbleibende Höhe und ob umbrochen wurde, danach die tatsächlich gesetzte
Höhe. Die Schätzung lag zuletzt rund 1,5 % über dem Satz. Wer `baue()`
ändert, prüft damit, ob `kastenhoehe()` noch zum Aufbau passt: Die beiden
müssen dieselben Abstände zählen.

Was `genpdf` 0.2 weiter nicht kann: Hintergrundfarben. Kästen sind deshalb
gerahmt statt hinterlegt.

Eine Folge des Zusammenhaltens ist, dass Seiten halb leer bleiben können –
der Ausgangslage-Kasten braucht rund 130 mm und rückt lieber auf die
nächste Seite, als sich zerreissen zu lassen. Das ist gewollt.

## Inhaltliches

- **Jede medizinische Aussage gehört belegt**, nicht aus dem Gedächtnis
  geschrieben. Der Beleg hängt hinter dem Wort im Satz, das Verzeichnis am
  Schluss führt die Literatur; neue Aussagen kommen mit einer neuen Quelle
  oder gar nicht.
- **Quellennummern nachprüfen, nicht abschreiben.** Eine PMC-Nummer führt
  immer auf *irgendeinen* Artikel, auch auf den falschen – ohne Fehlermeldung.
  In PR #1 zeigte `PMC5021885`, angeblich der 93-jährige Fall, auf eine Arbeit
  über mesenchymale Stromazellen bei myelodysplastischen Syndromen; richtig
  ist `PMC5031831`. Titel deshalb vor dem Einbauen abgleichen:

  ```bash
  curl -s "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esummary.fcgi?db=pmc&id=5031831&retmode=json" \
    | python3 -c "import json,sys; print(json.load(sys.stdin)['result']['5031831']['title'])"
  ```

  Für DOIs dasselbe über Crossref: `curl -s https://api.crossref.org/works/<doi>`.
  Beide laufen ohne Bot-Schutz – anders als `pubmed.ncbi.nlm.nih.gov` und
  `pmc.ncbi.nlm.nih.gov`, die WebFetch eine Cookie- oder Captcha-Seite
  zurückgeben; dort hilft nur der Browser.

- **Findet sich keine Quelle, fliegt die Aussage raus.** Die Tabellenzeile zu
  einer 99-jährigen Patientin aus PR #1 ist so verschwunden: In PubMed gibt es
  keinen solchen Fall, und der 93-jährige Fall bezeichnet sich selbst als den
  ältesten bekannten. Nicht stehenlassen und nicht mit einer ungefähr
  passenden Quelle zukleben.

- **Keine allgemeinen Ratschläge.** Alles im Blatt muss belegt *und* auf
  diese Patientin zugeschnitten sein. Herausgeflogen sind darum: die
  generische Warnzeichenliste «Was nicht warten darf», die Haushaltstipps
  gegen den süssen Geschmack (Strohhalm, Bouillon, Tagesmenge verteilen),
  der Hinweis auf den rektalen Weg, der Abschnitt zum Kinderspital samt
  Kispi-Wiki-Quelle, die Adresse der Notfallstation, der Abschnitt «Zur
  Anmeldung» und der Schlussabsatz mit dem Haftungshinweis. Auch die
  Überschrift «Ist das ansteckend? Nein» ist weg – die Frage stellt sich
  nicht; die belegten Angaben zum Infekt als Auslöser stehen weiter im
  Abschnitt «Was den Schub ausgelöst haben kann». Nichts davon wieder
  einbauen, ohne zu fragen.
- **Arzneimittelangaben ausschliesslich über [ch.oddb.org](https://ch.oddb.org)
  belegen, nicht über compendium.ch.** Dieselbe Fachinformation steht auf
  ch.oddb.org und ist dort frei zugänglich. Die Fachinfo einer Zulassung
  liegt unter `https://ch.oddb.org/de/gcc/fachinfo/reg/<Swissmedic-Nr>`, die
  Suche unter `.../search/zone/drugs/search_query/<name>`. **Fällt die Seite
  aus** – am 3. September 2026 stand dort minutenlang «Wegen eines
  Software-Updates … nicht erreichbar» –, liegt derselbe Fachinfo-Text offline
  in der AmiKo-Datenbank von SDIF und lässt sich sofort lesen:

  ```bash
  sqlite3 ~/.software/sdif/db/amiko_db_full_idx_de.db \
    "select content from amikodb where regnrs='42933' and type='FI';"
  ```

  Damit ist die Arbeit nicht blockiert. Der **Link** gehört trotzdem im Browser
  geöffnet, bevor er ins Blatt kommt – die Offline-Datenbank sagt nichts
  darüber, auf welchem Dokument eine Nummer landet.
- **Der Beleg steht hinter dem Wort.** `Span::L("Wort", "URL")` macht ein
  Wort im Satz anklickbar – die Fachinformation hängt hinter dem Begriff, den
  sie belegt, und die Präparatenamen in den Tabellen führen auf ihre
  Zusammensetzung. Keine nackten URL-Zeilen im Fliesstext. Das
  Quellenverzeichnis führt je Präparat einen Registereintrag auf die
  vollständige Fachinformation.
- **Verlinkt wird das Kapitel, nicht die ganze Fachinformation:**
  `.../fachinfo/reg/<Nr>/chapter/<kapitel>`. Wer eine Aussage belegt, verlinkt
  das Kapitel, in dem sie steht – sonst muss der Leser eine zwanzigseitige
  Fachinfo durchsuchen. Die Kapitel heissen `composition`, `galenic_form`,
  `indications`, `usage` (Dosierung), `contra_indications`, `restrictions`
  (Warnhinweise), `interactions`, `pregnancy`, `driving_ability`,
  `unwanted_effects`, `overdose`, `effects`, `kinetic`, `preclinic`,
  `other_advice`, `iksnrs`, `packages`, `registration_owner`, `date`. Die
  Liste stammt aus der Kapitelnavigation der Seite selbst; im Zweifel dort
  nachsehen statt raten. Die Seite steht
  hinter einem Bot-Schutz (Anubis): `curl` und einfache Fetch-Werkzeuge
  bekommen die Challenge-Seite, ein echter Browser kommt durch.
- **Der Interaktionscheck kommt aus [SDIF](https://sdif.oddb.org/?tab=check)**, dem
  Werkzeug in `~/.software/sdif` – nicht aus dem Gedächtnis und nicht von
  einer fremden Website. Der Lauf, auf dem der Abschnitt beruht:

  ```bash
  cd ~/.software/sdif
  ./target/release/sdif check Novalgin Pantoprazol Movicol Duphalac Cardiax
  ```

  Spiricort war bis zum 29. August 2026 im Warenkorb und ist heraus – die
  Patientin hat nie Kortikoide bekommen. Wer es wieder aufnimmt, bekommt
  die beiden Treffer zurück, die im Blatt als «Was mit dem Kortison
  weggefallen ist» beschrieben sind.

  Die EPha-Einstufungen (A bis X) stehen zusätzlich in `db/interactions.db`
  und lassen sich direkt abfragen:

  ```bash
  sqlite3 db/interactions.db "select atc2, risk_class, effect, measures \
    from epha_interactions where atc1='N02BB02' and risk_class in ('C','D','X');"
  ```

  Der Warenkorb lässt sich in der URL mitgeben – im Blatt verlinkt als
  `?tab=check&drugs=N02BB02-A02BC02-A06AD65-A06AD11-B01AC06`. Die Codes sind
  ATC, nicht Marken: für die Macrogol-Klasse A06AD65 zeigt die Oberfläche
  irgendein Präparat der Klasse an, nicht zwingend Movicol.

  Die Namensauflösung ist die Fehlerquelle: «Paracetamol» landet auf einem
  Kombipräparat mit Tramadol, «Dafalgan» auf Co-Dafalgan mit Codein, und der
  ATC-Name «Macrogol, Kombinationen» erzeugt Treffer über das Wort
  «Kombinationen». Für niedrig dosiertes Aspirin `Cardiax` nehmen, nicht
  `Aspirin`. Jeden Treffer gegen die Fachinformation nachlesen, bevor er ins
  Blatt kommt.
- Die Angaben zu Erwachsenen und Betagten sind der Kern. Das meiste, was
  man auf Deutsch zu dieser Krankheit findet, betrifft Kinder und führt bei
  einer 84-jährigen Patientin in die Irre.
- **Angaben kommen tröpfchenweise und kippen Prämissen.** Der Fall wird
  während der Arbeit erzählt, und einzelne Sätze der Angehörigen heben
  ganze Abschnitte auf: aus «vier bis fünf Wochen ohne Stuhlgang» wurde
  eine Woche, aus «sie bekommt Kortison» wurde «sie hat nie Kortikoide
  bekommen». Wenn eine Prämisse fällt, reicht es **nicht**, den einen
  Absatz zu ändern. `grep` auf den Begriff über `src/inhalt.rs` und
  `README.md`, jede Fundstelle einzeln beurteilen – Kortison hing an
  einem Dutzend Stellen, vom Interaktionscheck über die Ödeme bis zur
  Erklärung der Magenentzündung. Und: Was wegfällt, gehört benannt statt
  stillschweigend gelöscht, damit nachvollziehbar bleibt, warum eine
  frühere Fassung etwas anderes sagte.
- **Eine widerlegte Aussage wird nicht durch ihr Gegenteil ersetzt.** Zum
  Spinat hiess es zuerst, die Oxalsäure hemme die Eisenaufnahme; die
  ETH-Arbeit zeigt, dass sie es nicht tut. Im Blatt steht deshalb beides:
  dass der Volksglaube falsch ist *und* dass Spinat trotzdem keine Antwort
  auf einen Blutverlust dieser Grössenordnung ist.
- **Die Nierenfunktion ist kein Befund für sich, sondern ein Filter über die
  ganze Medikamentenliste.** Bei einer GFR von 30 gehört jedes Mittel auf dem
  Blatt daraufhin nachgelesen – und die Fachinformationen sagen es meist
  ausdrücklich, nur nicht im Kapitel `unwanted_effects`, sondern in `usage`,
  `restrictions` oder `kinetic`. So steht der entscheidende Satz zu Morphin bei
  Niereninsuffizienz in der **Pharmakokinetik**, nicht bei den Nebenwirkungen.
  Wer nur die Nebenwirkungen liest, findet ihn nie.
- **Eine Medikamentenliste im Bericht ist eine Momentaufnahme, kein Stand.**
  Die Liste vom 30. August führt Novalgin nicht, obwohl es als Infusion läuft,
  und Co-Amoxicillin nicht, weil es vor dem Eintritt lief. Wer aus ihr auf das
  Jetzt schliesst, irrt in beide Richtungen. Beim Schreiben deshalb die
  **Zeitform** mitführen: «lief vor dem Eintritt» ist etwas anderes als
  «bekommt» – wer den Fall kennt, merkt den Unterschied sofort, und zu Recht.
  Und bei Risikofaktoren prüfen, welches **Zeitfenster** die zitierte Studie
  zählt: bei Clostridioides difficile etwa die Einnahme in den letzten dreissig
  Tagen, nicht die laufende. Ein Risikofaktor kann erfüllt sein, ohne dass das
  Mittel noch läuft – das ist kein Wortspiel, sondern der Unterschied zwischen
  einer haltbaren und einer angreifbaren Begründung.
- **Der Verabreichungsweg gehört zur Angabe.** «Sie bekommt Novalgin» und «sie
  bekommt Novalgin als Infusion» sind in der Fachinformation zwei verschiedene
  Sachverhalte: Die parenterale Gabe bringt eigene Auflagen mit (höchstens
  1 ml/min, erhöhtes Anaphylaxierisiko, vorher den Kreislauf stabilisieren).
  Dasselbe beim Thiamin, wo der orale Weg nachweislich versagt. Bei jeder
  Verordnung deshalb nach dem Weg fragen und ihn im Blatt mitführen – und die
  Fachinfo-Kapitel `restrictions` und `usage` daraufhin lesen, nicht nur
  `unwanted_effects`.
- **Eine Behandlung, die die Angehörigen einer Beschwerde zuordnen, ist oft
  gegen etwas anderes gerichtet.** «Sie bekommt Sauerstoff wegen der
  Bauchschmerzen» – Sauerstoff ist kein Schmerzmittel; er läuft, weil ein
  Messwert zu tief war. Solche Zuordnungen entstehen, weil beides gleichzeitig
  da ist, und sie sind die ergiebigsten Stellen im ganzen Verlauf: Dahinter
  steckt regelmässig ein Befund, den niemand erwähnt hat. Nicht die Zuordnung
  übernehmen, sondern fragen, was gemessen wurde.
- **Die Farbe ist eine Ortsangabe.** Erbrochenes und Stuhl melden über die
  Farbe, aus welcher Höhe sie kommen: schwarz oben heisst Blut in der Säure,
  gelb heisst Galle und damit Inhalt von unterhalb der Einmündung des
  Gallengangs, Miserere heisst Darminhalt. Eine Farbänderung ist deshalb nie
  bloss ein Detail, sondern eine Bewegung auf einer Skala – und die
  **Reihenfolge** über die Tage sagt mehr als jeder Einzelwert. Beim Notieren
  gehört die Richtung mit: von Darminhalt über Blut zu Galle ist etwas anderes
  als umgekehrt. Und was eine Farbe *nicht* sagt, gehört gleich daneben: Gelbes
  Erbrochenes nimmt die Frist für die Magenspiegelung weg, aber weder den
  Hämoglobinwert noch den Stuhl.
- **Die Seite ist eine Ursachenangabe**, so wie die Farbe eine Ortsangabe ist.
  Wo ein Befund seitengebunden auftritt, sortiert die Seite die Ursachen,
  bevor irgendein Laborwert vorliegt: Systemische Ursachen – Herzschwäche,
  Zirrhose, Nierenversagen, Eiweissmangel – machen **kleine Ergüsse auf beiden
  Seiten**, so sehr, dass die Übersichtsarbeit für sie ausdrücklich *keine*
  diagnostische Punktion verlangt. Bauchwasser, das durchs Zwerchfell steigt,
  käme ganz überwiegend **rechts**. Eine Lungenembolie macht einen **kleinen**
  Erguss. Ein einseitiger Liter links ist von allen dreien das Gegenteil und
  zeigt auf einen örtlichen Vorgang. Beim Notieren gehört deshalb die Seite mit,
  und zwar zu **jedem** seitenfähigen Befund; und wo eine frühere Angabe
  «beidseits» lautete und die neue «nur links», ist die **Asymmetrie** die
  Information, nicht die Menge.
- **Material aus einer Punktion hat eine Frist von Stunden.** Untersuchte
  Flüssigkeit lässt sich aufheben, ununtersuchte wird verworfen – und danach
  kostet dieselbe Antwort einen zweiten Eingriff. Sobald irgendwo punktiert,
  drainiert oder biopsiert wurde, ist die Frage «Ist das vollständige Programm
  angefordert?» deshalb dringlicher als jede Auswertung des Ergebnisses. Sie
  gehört an die Spitze der Fragenliste, mit der Frist dazu, nicht in den
  Fliesstext. Beim Erguss ist das Programm: Zellzahl mit Differenzierung,
  Gesamteiweiss, LDH, pH, Gram-Färbung, Kultur, Zytologie – und ein
  **Zellblock**, ohne den keine Immunfärbung möglich ist.
- **Ein laufendes Medikament kann nicht nur den Wert verschieben, sondern das
  Kriterium wechseln.** Unter einem Entwässerungsmittel dickt ein Transsudat
  ein und rutscht in den Exsudatbereich; die Light-Kriterien ordneten in der
  Arbeit dazu fünf Herzschwäche-Kranke falsch ein, vier davon nach Diuretikum.
  Dann gilt statt ihrer der Serum-Erguss-Albumin-Gradient. Das ist eine Stufe
  schärfer als die Regel zu den Bedingungen eines negativen Befundes: Es reicht
  nicht, das Ergebnis vorsichtiger zu lesen – es ist die **falsche Messlatte**.
  Deshalb bei jedem Schwellenwert im Blatt prüfen, ob ein Mittel auf der Liste
  ihn ungültig macht.
- **Bei einem Hausmittel zuerst die Evidenz *dafür* suchen – sie trägt das
  Argument oft besser als jede Warnung.** Der Umweg über Gegenanzeigen,
  Fallberichte und Lebensmittelrecht hat beim Flohsamen vier Seiten gekostet
  und den Leser in die Defensive gedrängt. Was am Ende trug, stand in der
  Wirksamkeitsliteratur selbst: die **Dosisschwelle** (Wirkung erst ab 10 g
  täglich – der Zeitungsartikel empfiehlt die halbe Menge), der **grösste
  Einzeleffekt der Meta-Analyse**, der nicht der Nutzen war, sondern die
  Blähung und damit ihre Hauptbeschwerde, und die **Benotung gegen das, was
  ohnehin schon läuft** (Macrogol Note A, Flohsamen Note B – sie bekommt
  Macrogol). Drei Zahlen aus Arbeiten, die für das Mittel sprechen, statt
  einer Anklage. Zwei Nebenregeln dazu:
  **«Derselbe Stoff» nachprüfen, nicht behaupten** – Agiolax mite enthält je
  Teelöffel 3,25 g ganzen Samen und nur 0,11 g Schale, der Migros-Beutel ist
  reine Schale; die Mengen stehen unter `galenic_form`, nicht unter
  `composition`, und ich hatte es erst falsch im Blatt. **Prüfen, ob
  Fallberichte dieselbe Zubereitung betreffen** wie das, worüber geredet wird.
  Und wenn ein Hausmittel für das **falsche Kompartiment** vorgeschlagen wird –
  Flohsamen gegen das Wasser in den Beinen –, entscheidet das Kapitel
  `kinetic` die Frage in einer Zeile: Bei Agiolax mite stehen Absorption,
  Verteilung, Abbau und Ausscheidung allesamt als «nicht zutreffend», der Stoff
  tritt also gar nicht in den Körper ein. Das ist kürzer und stärker als jede
  Erklärung.
- **Verhältnismässigkeit ist eine inhaltliche Frage, keine Formsache.** Der
  Flohsamen-Abschnitt wuchs auf vier Seiten, während die beiden Fragen mit
  echtem Einsatz offen blieben – welche Kategorie im zytologischen Befund
  steht, und ob die Pleuraflüssigkeit vom 2. September das vollständige
  Programm bekam, bevor sie verworfen wurde. Der Umfang eines Abschnitts
  bemisst sich am Einsatz, nicht daran, wie viel Material sich finden liess.
  Ein Warnsignal ist, wenn dieselbe Frage dreimal zurückkommt: Dann ist die
  Antwort zu lang und nicht zu kurz. Und wenn ein Einwand kommt («es macht
  keinen Sinn»), zuerst die eigene Argumentation auf **veraltete Prämissen**
  prüfen – hier stand «die Peristaltik fehlt» aus einem Befund von vorher,
  obwohl die Passage längst wieder offen war.
- **Kommt während des Versands eine neue Angabe, geht sie zuerst ins Blatt.**
  Am 2. September traf «das Erbrochene ist jetzt gelb» ein, als das
  Versandskript schon im Trockenlauf stand. Richtig ist dann nicht, die
  vorbereitete Mail abzuschicken und die Angabe nachzureichen, sondern
  anzuhalten: einarbeiten, bauen, prüfen, committen, dann senden. Ein PDF, dem
  die neueste Meldung fehlt, ist am Krankenbett schlechter als ein PDF, das
  zehn Minuten später kommt – und eine zweite Mail zur selben Sache ist genau
  das, was die Regel zur Mailfolge vermeiden will.
- **Hinter einer Laienauskunft steht oft eine abgestufte Fachsprache.** «Es
  wurden ein paar pathologische Zellen gefunden» ist keine Diagnose, sondern
  die Umschreibung einer Kategorie. Die Zytologie der Ergussflüssigkeiten
  kennt fünf Stufen, und zu jeder ist das Risiko ausgerechnet: Atypie unklarer
  Bedeutung 50,8 Prozent, «verdächtig» 91,3, «bösartig» 98,2. Zwischen der
  ersten und der zweiten liegt der Unterschied zwischen offen und beinahe
  entschieden – und die Umschreibung deckt beide. Wer mit der Umschreibung
  weiterrechnet, rechnet mit einem Faktor zwei. Die Regel gilt allgemein: Wo
  ein Befund abgestuft berichtet wird, ist die **Stufe** die Information, und
  sie gehört wörtlich erfragt, bevor irgendetwas daraus folgt.
- **Ein Gewicht ist eine Bilanz, und ein Ödemgewicht ist keine Rechengrundlage.**
  66 kg bei einem gewohnten Gewicht von 58 sind acht Liter Wasser – die Waage
  beantwortet die Ein- und Ausfuhrbilanz, bevor ein Bilanzblatt gelesen wird.
  Zugleich verdeckt sie das, was sie zeigen sollte: Wer wochenlang nichts isst
  und trotzdem zunimmt, sieht auf der Waage gut ernährt aus. Kalorien- und
  Urinschwellen rechnen deshalb auf das **Trockengewicht**, nicht auf das
  Gewicht mit den Ödemen. Wer eine neue Gewichtsangabe bekommt, prüft `grep`
  auf jede Stelle, die mit Kilogramm rechnet.
- **Der Kapitel-Link auf ch.oddb.org gehört geöffnet, nicht abgeleitet.** Die
  Kapitelnamen sind zwar eine feste Liste, aber sie sind nicht dort, wo man sie
  vermutet: Die Mengenangabe «Na⁺ 154 mmol/l» der Kochsalzlösung steht nicht
  unter `composition`, sondern unter `galenic_form`; `composition` führt nur
  Wirk- und Hilfsstoffe ohne Zahlen. Von vier geratenen Kapiteln war eines
  falsch. Die Prüfung geht nur im Browser (Anubis) und kostet einen Aufruf je
  Link.
- **Ein negativer Befund ist nur so viel wert wie die Bedingungen, unter denen
  er erhoben wurde.** Dieses Blatt ist dreimal auf dasselbe Muster gestossen:
  Der unauffällige Streifentest war unter laufender Infusion verdünnt; der
  unauffällige Urin war eine Momentaufnahme; die unauffällige Darmspiegelung
  war schlecht vorbereitet. Bei jedem «unauffällig» im Bericht deshalb die
  Bedingungen mitlesen und, wenn sie fehlen, danach fragen – sonst wird aus
  «nichts gesehen» ein «nichts vorhanden». Der Unterschied zwischen
  ausgeschlossen und unzureichend eingesehen ist der wichtigste Satz, den
  dieses Blatt an mehreren Stellen wiederholt.
- **Ein ärztlicher Bericht ist die beste Quelle über den Fall und die
  gefährlichste Datei im Verzeichnis.** Er trägt Name, Geburtsdatum, Adresse,
  die Namen mehrerer Ärzte und oft den Beruf der Patientin. Ins Blatt kommt
  ausschliesslich der Sachverhalt: Zahlen, Befunde, Daten, Verordnungen. **Der
  Beruf bleibt draussen** – eine 84-jährige Ärztin eines bestimmten Fachs in
  Zürich ist identifizierbar, auch ohne Namen. Und die **behandelnden Ärzte
  werden nicht genannt**: Die Adressen im Blatt sind öffentliche Kontaktangaben
  des Universitätsspitals, ein Zuweiser ist etwas anderes – sein Name verknüpft
  den Fall mit einer Praxis. Die Datei selbst gehört nicht ins Repositorium.
- **Die Arbeitsdiagnose kann unter dem ganzen Dokument wegkippen.** Der
  Zwischenbericht vom 30. August 2026 beurteilt den Fall als «unklares
  Krankheitsbild» mit drei Möglichkeiten – autoimmun, paraneoplastisch,
  Peritonealkarzinose. Das Blatt trägt den Namen der ersten. Richtig ist dann
  nicht, es umzubenennen oder stillschweigend weiterzuschreiben, sondern es an
  den Anfang zu stellen: Wer liest, liest die Ausarbeitung eines Zweiges,
  während zwei andere offen danebenstehen. **Am 3. September 2026 ist genau das
  eingetreten:** Der Gynäkologe fand ein Eierstockkarzinom. Das Blatt heisst
  weiter «IgA-Vaskulitis», die Diagnose steht im Ausgangslage-Kasten und in
  einem eigenen Abschnitt direkt danach, und die Abschnitte, die auf sie
  zugelaufen sind («Die Zellen im Bauchwasser», «Warum der Gynäkologe»),
  bleiben stehen mit einem datierten Nachtrag – nicht umgeschrieben. Vier
  Dinge, die dabei zählten: (1) **Bild, Zellen oder Gewebe** – «Karzinom» aus
  zweiter Hand kann eine Ultraschall-Annahme, eine Zytologie mit PAX8 oder
  eine Histologie sein; nur die dritte liefert Typ, Grad und den
  Reparaturdefekt für die Erhaltungstherapie. Das ist die erste Frage, vor
  jeder Prognosezahl. (2) **Was die Diagnose rückwirkend zusammenführt, gehört
  als Liste hin** – Bauchwasser, Ergüsse, Harnstau, Darmstillstand, Gewicht,
  Ödeme, Purpura –, mit dem Satz, dass eine Erklärung, die alles erklärt,
  besonders sorgfältig geprüft gehört. (3) **Behandlungen mit Zahlen, nicht
  mit Namen**, und jede an ihrer Bedingung gemessen: Carboplatin an der GFR
  (Calvert, Grenze <60), Bevacizumab am Darm (Perforation), PARP-Hemmer am
  Hämoglobin (Anämie 31 %), Operation an Alter und Wasser (CHORUS: 6 % gegen
  <1 % Tote nach 28 Tagen). Und der dritte Weg – Beschwerden behandeln ohne
  den Tumor – steht gleichberechtigt da, weil sie entscheidet, nicht das
  Blatt. (4) **Prognosezahlen sofort einschränken**, im selben Absatz: Sie
  stammen aus jüngeren, behandelten Gruppen und sagen etwas über die
  Krankheit, nichts über ihren Verlauf. Und zwei Symptome, die mit einer
  Krebsdiagnose die Klasse wechseln: geschwollene Beine werden zur
  Thrombosefrage (Eierstockkarzinom gehört zu den drei Krebsarten mit dem
  höchsten VTE-Risiko), Geruchsempfindlichkeit wird zum Krankheitszeichen
  (48 % vor jeder Behandlung) statt zur Vorliebe.
- **Den Interaktionscheck nie auf einem vermuteten Warenkorb stehen lassen.**
  Der Lauf vom 29. August rechnete mit Novalgin, Aspirin und zwei
  Abführmitteln; von alldem stand auf der tatsächlichen Liste nur Pantoprazol.
  Wer ohne die Verordnungsliste prüft, prüft eine Vermutung – das gehört im
  Abschnitt so benannt, und der Lauf gehört wiederholt, sobald die Liste da
  ist. Zweitens: **«Verordnet» ist nicht «gegeben».** Reservemedikamente
  («in R») stehen auf dem Blatt, ohne verabreicht worden zu sein; was zählt,
  ist das Verabreichungsprotokoll.
- **Eine Frage, die zweimal gestellt wird, ist ein eigenes Blatt.** Die Frage
  nach dem Kostaufbau nach langem Hungern kam zweimal; die Antwort passte nicht
  mehr in einen Abschnitt des Hauptblatts, ohne es zu verziehen. Daraus wurde
  `src/hunger.rs`. Das Hauptblatt behält den kurzen Abschnitt und verweist
  darauf – nicht umgekehrt. Wer den einen ändert, prüft den anderen: Thiamin,
  Phosphat und die vier Tage stehen in beiden.
- Adressen, Präparate und Zuständigkeiten veralten. Das Datum in `STAND`
  beim Prüfen mitführen – ebenso das Datum im Abschnitt zum Interaktions-
  check, das den Lauf datiert.

## Vertraulichkeit

**Dieses Repositorium ist öffentlich.**

- **Keine Namen.** Der Anlass ist ein realer Fall; die Person wird nirgends
  benannt. Alter, Laborwerte, Beschwerden und Verlauf sind ausdrücklich in
  Ordnung, Namen und Angehörige nicht.
- **Fotos aus dem Spital tragen Namen mit.** Das Etikett einer Blutkonserve
  führt Name, Geburtsdatum, Patientennummer und den Namen des Arztes; ein
  Pumpendisplay die Station. Aus solchen Bildern kommt ausschliesslich der
  Sachverhalt ins Blatt – «es läuft ein Erythrozytenkonzentrat» –, nie das
  Bild, nie ein Ausschnitt daraus, nie eine Angabe vom Etikett. Auch nicht
  in einen Commit, einen Dateinamen oder eine Mail.
- Keine privaten Mailadressen, keine Zugangsdaten, keine
  Anwendungspasswörter in eingecheckten Dateien. Ein `.gitignore`-Eintrag
  ist eine Vorsichtsmassnahme, kein Schutz – ein `git add -f` genügt. Vor
  jedem Commit `git status` prüfen.
- **Die Versandskripte bleiben lokal und gehen direkt über die Gmail-API**,
  nicht über die MCP-Schnittstelle. Diese schreibt beim Anlegen eines
  Entwurfs **jeden Link zu einer Google-Umleitung um**
  (`https://www.google.com/url?q=…&source=gmail&ust=…`); wer die
  MIME-Nachricht selbst baut und hochlädt, umgeht das. Dieselbe Mechanik wie
  `entwurf_curtins.py` in `~/.software/adhs-expert`:

  ```python
  SCOPES = ["https://www.googleapis.com/auth/gmail.compose"]   # Entwurf
  SCOPES = ["https://www.googleapis.com/auth/gmail.send"]      # Versand
  BASIS  = ~/.software/fundaziun-davaz   # token_compose.json, token_send.json,
                                         # client_secret_*.json
  ```

  `send_*.py` verschickt, `entwurf_*.py` legt an; beide Muster stehen im
  `.gitignore`, weil die Dateien Mailadressen, Patientenname und
  Geburtsdatum im Klartext tragen. Je Blatt ein Skript: `send_blatt.py`
  schickt das Hauptblatt, `send_hunger.py` das Begleitblatt.

  **Der Sende-Scope kann nicht nachlesen.** `gmail.send` erlaubt kein
  `messages.get` – wer mit `token_send.json` prüfen will, ob ein Anhang
  wirklich mitging, bekommt nichts zurück. Der Nachweis geht über den
  `sizeEstimate` der gesendeten Nachricht (über die MCP-Schnittstelle
  abfragbar), verglichen mit der base64-Grösse der Dateien, also
  `ceil(n/3)*4` je Anhang plus rund 150 KB für Text und Kopfzeilen. Das
  entscheidet die Frage «war der Anhang dabei?» ohne Lesezugriff.

  **Und die praktische Folge daraus: je Blatt eine Mail.** Zwei PDFs in
  einer Nachricht von 11 MB sehen im Mailprogramm aus wie eines – die
  Vorschau klappt sie zusammen, und der zweite Anhang gilt als nicht
  gesendet. Beim Versand mehrerer Blätter deshalb nicht bündeln.

  **Und bei rascher Folge kurz halten.** An einem Tag, an dem stündlich neue
  Angaben kommen, gehen leicht mehrere Mails an dieselben zwei Menschen. Die
  zweite und dritte wiederholen dann nicht die erste, sondern bringen die neue
  Sache – und schliessen mit einem Satz, was aus der vorigen unverändert gilt.
  Sonst muss der Leser raten, ob etwas zurückgenommen wurde. Es sind Angehörige
  am Krankenbett und keine Abonnenten. Der Name der Patientin darf in eine Mail
  an ihre behandelnden Ärzte – dorthin gehört er –, aber in keine Datei,
  die auch nur in die Nähe eines Commits kommt.
- Die Mailadressen in den Adressangaben sind öffentliche Kontaktangaben des
  Universitätsspitals Zürich, keine privaten.

## Lizenz

GPL-3.0. Neue Quelldateien tragen einen GPL-3.0-verträglichen Kopf, und
jede Abhängigkeit muss mit GPL-3.0 vereinbar sein.
