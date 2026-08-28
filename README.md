# IgA-Vaskulitis (Purpura Schönlein-Henoch)

Ein Informationsblatt zum Krankheitsbild: was die Krankheit ist, was beim
Verdacht abgeklärt gehört, welche Warnzeichen nicht warten dürfen, wo in
Zürich die Sprechstunden dafür sind – und wie die Krankheit entdeckt wurde.

Das Blatt ist entstanden, weil zu dieser Krankheit im deutschen Sprachraum
vor allem Material für Kinder zu finden ist. Bei Erwachsenen, und erst recht
im hohen Alter, verläuft sie anders und wird härter beurteilt. Genau diese
Unterschiede stehen hier zusammengetragen.

## Der Anlass

Ein konkreter, anonymisierter Fall: eine Patientin von 84 Jahren, deren
Hämoglobin innert kurzer Zeit von 108 auf 83 g/l gefallen ist, mit einem
massiven Ausschlag an den Beinen in der Vorgeschichte. Die «Ausgangslage»
im Dokument beschreibt diesen Fall. Namen kommen keine vor.

Drei Punkte, an denen sich das Blatt aufhängt, weil sie leicht falsch
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

## Was drinsteht

Krankheitsbild und die vier betroffenen Bereiche · Hämoglobinwert lesen ·
Ansteckung und Auslöser · warum das Alter den Verlauf ändert · was ein
früherer Schub bedeutet · Herkunft des Blutverlusts · eine Tabelle der
notwendigen Abklärungen · Behandlung · Abführmittel bei dieser Konstellation
(Macrogol statt Lactulose) · Adressen in Zürich · was zum Termin mitgehört ·
Fragen, die sich zu stellen lohnen · die Entdeckungsgeschichte von Heberden
1801 bis zur Chapel-Hill-Nomenklatur 2012 · Quellen.

Belegt ist alles mit der zitierten Literatur, im Dokument am Schluss
verlinkt. Die Adressen sind öffentliche Angaben von Universitätsspital und
Kinderspital Zürich, überprüft am 27. August 2026.

## Bauen

```bash
make          # erzeugt iga-vaskulitis.pdf
make open     # baut und öffnet
```

Oder von Hand:

```bash
weasyprint src/iga-vaskulitis.html iga-vaskulitis.pdf
```

Gebraucht wird [WeasyPrint](https://weasyprint.org/) (entwickelt mit 66.0).
Die Warnung `Unable to subset font with Harfbuzz` ist harmlos.

## Keine ärztliche Beurteilung

Das Blatt fasst den Stand der veröffentlichten Literatur zusammen. Es ist
dafür gedacht, zum Arzttermin mitgenommen zu werden, damit die
entscheidenden Fragen gestellt und die richtigen Untersuchungen angeordnet
werden. Es ersetzt keine ärztliche Beurteilung, und Sprechstunden wie
Zuständigkeiten ändern sich.

## Lizenz

GPL-3.0. Siehe [LICENSE](LICENSE).
