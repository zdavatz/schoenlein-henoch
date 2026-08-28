# IgA-Vaskulitis (Purpura Schönlein-Henoch)

Ein Informationsblatt zum Krankheitsbild: wie die Krankheit entdeckt wurde,
was sie ist, was bei dieser Patientin abgeklärt gehört und wo in Zürich die
Sprechstunden dafür sind.

Das Blatt ist entstanden, weil zu dieser Krankheit im deutschen Sprachraum
vor allem Material für Kinder zu finden ist. Bei Erwachsenen, und erst recht
im hohen Alter, verläuft sie anders und wird härter beurteilt. Genau diese
Unterschiede stehen hier zusammengetragen.

## Der Anlass

Ein konkreter, anonymisierter Fall: eine Patientin von 84 Jahren, deren
Hämoglobin innert kurzer Zeit von 108 auf 83 g/l gefallen ist, mit einem
massiven Ausschlag an den Beinen in der Vorgeschichte. Dazu seit Wochen
kein Appetit, starke Bauchschmerzen, wenig Schlaf, Erbrechen nach jeder
Nahrungsaufnahme, kaum Flüssigkeit und seit etwa vier bis fünf Wochen gar
kein Stuhlgang mehr. Die «Ausgangslage» im Dokument beschreibt diesen Fall.
Namen kommen keine vor.

Vier Punkte, an denen sich das Blatt aufhängt, weil sie leicht falsch
verstanden werden:

- **Beim Hämoglobin ist tief schlechter, nicht besser.** Die Verwechslung
  ist das HbA1c, der Langzeit-Blutzucker – gleicher Wortstamm, gegenläufige
  Richtung.
- **Nicht der Wert ist der Befund, sondern das Tempo.** Ein Abfall um
  25 g/l innert Wochen spricht für Blutverlust, in erster Linie aus dem
  Darm. Eine Blutarmut durch Entzündung braucht Monate.
- **Die Krankheit ist nicht ansteckend**, aber der Funke ist häufig ein
  Infekt – bei 50 bis 90 Prozent geht einer voraus, oft mit Streptokokken
  der Gruppe A. Zwischen Infekt und Ausbruch liegen ein bis drei Wochen.
- **Wochenlang kein Stuhlgang und Erbrechen nach jedem Essen ist keine
  Verstopfung.** Das ist das Bild einer Passagestörung, und in dieser Lage
  ist ein Abführmittel durch den Mund nicht die Antwort, sondern in allen
  Fachinformationen eine Gegenanzeige.
- **Die Infusion füllt das Blutvolumen wieder auf** – richtig bei einer
  Patientin, die kaum trinkt. Sie verdünnt aber auch: Der gemessene
  Hämoglobinwert kann danach weiter fallen, ohne dass zusätzlich Blut
  verloren geht.

## Was drinsteht

Das Blatt beginnt mit der Entdeckungsgeschichte von Heberden 1801 bis zur
Chapel-Hill-Nomenklatur 2012 und geht von dort in den Fall.

Krankheitsbild und die vier betroffenen Bereiche · Hämoglobinwert lesen und
was die Infusion mit ihm macht ·
Ansteckung und Auslöser · warum das Alter den Verlauf ändert · was ein
früherer Schub bedeutet · der Verlauf der letzten Wochen und was er bedeutet ·
Herkunft des Blutverlusts · eine Tabelle der notwendigen Abklärungen ·
Behandlung · eine Übersicht der Abführmittel zum Trinken, die es in der
Schweiz gibt, mit Wirkstoff und Süssstoff je Präparat und den beiden
aromafreien Alternativen · Adressen in Zürich · was zum Termin mitgehört ·
Fragen, die sich zu stellen lohnen · Quellen.

**Keine allgemeinen Ratschläge.** Was hier steht, ist belegt und auf diese
Patientin zugeschnitten. Generische Warnzeichenlisten, Haushaltstipps und
Angaben, die Kinder betreffen, sind bewusst nicht drin.

Belegt ist alles mit der zitierten Literatur, im Dokument am Schluss
verlinkt. Die Arzneimittelangaben stammen aus den Fachinformationen auf
[ch.oddb.org](https://ch.oddb.org). Die Adressen sind öffentliche Angaben
von Universitätsspital und Kinderspital Zürich, überprüft am
28. August 2026.

## Bauen

```bash
make          # erzeugt iga-vaskulitis.html und iga-vaskulitis.pdf
make open     # baut und öffnet das PDF
make pruef    # Seitenbilder zur Sichtprüfung
```

Oder von Hand:

```bash
cargo run --release
cargo run --release -- --html raus.html --pdf raus.pdf
```

Gebraucht wird eine Rust-Toolchain (entwickelt mit 1.93). Beide Ausgaben
entstehen im selben Lauf. Das fertige
[`iga-vaskulitis.pdf`](iga-vaskulitis.pdf) liegt im Repositorium, damit man
es ohne Toolchain herunterladen und ausdrucken kann; es wird bei jeder
inhaltlichen Änderung neu gebaut und mitcommittet. Das HTML ist eine
Zwischenstufe und bleibt draussen.

Das PDF entsteht ohne Chrome und ohne WeasyPrint: [genpdf](https://crates.io/crates/genpdf)
schreibt über printpdf, [lopdf](https://crates.io/crates/lopdf) legt die
Hyperlinks nach, DejaVu Sans wird eingebettet – dieselbe Pipeline wie in
[adhs-expert](https://github.com/zdavatz/adhs-expert).

## Keine ärztliche Beurteilung

Das Blatt fasst den Stand der veröffentlichten Literatur zusammen. Es ist
dafür gedacht, zum Arzttermin mitgenommen zu werden, damit die
entscheidenden Fragen gestellt und die richtigen Untersuchungen angeordnet
werden. Es ersetzt keine ärztliche Beurteilung, und Sprechstunden wie
Zuständigkeiten ändern sich.

## Lizenz

GPL-3.0. Siehe [LICENSE](LICENSE).
