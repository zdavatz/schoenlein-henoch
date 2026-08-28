// Inhalt des Informationsblatts zur IgA-Vaskulitis (Purpura Schoenlein-Henoch).
// Copyright (C) 2026 Zeno R.R. Davatz
//
// Dieses Programm ist freie Software: Sie koennen es weitergeben und/oder
// veraendern, unter den Bedingungen der GNU General Public License, wie von
// der Free Software Foundation veroeffentlicht, entweder Version 3 der
// Lizenz oder (nach Ihrer Wahl) jeder spaeteren Version. Siehe LICENSE.
//
// Kein Ersatz fuer eine aerztliche Beurteilung.
//
// Der gesamte Text steht hier als Daten; `html.rs` und `pdf.rs` sind zwei
// Ausgaben derselben Struktur. Wer Text aendert, aendert ihn nur hier.

/// Textstueck innerhalb eines Absatzes.
pub enum Span {
    /// Fliesstext.
    T(&'static str),
    /// Fett.
    B(&'static str),
    /// Kursiv.
    I(&'static str),
    /// Darf nicht umbrechen – Messwerte wie "83 g/l". Im HTML `span.nb`,
    /// im PDF mit geschuetzten Leerzeichen.
    N(&'static str),
}

/// Eine klickbare Zeile. Im PDF steht sie stets allein auf ihrer Zeile, weil
/// die Link-Annotationen ueber die Schriftgroesse zugeordnet werden.
pub struct Verweis {
    pub text: &'static str,
    pub url: &'static str,
}

pub struct Tabelle {
    /// Leer, wenn die Tabelle keine Kopfzeile hat.
    pub kopf: &'static [&'static str],
    /// Spaltengewichte fuer den PDF-Satz.
    pub gewichte: &'static [usize],
    pub zeilen: &'static [&'static [&'static [Span]]],
    /// Trennlinien unter den Zeilen.
    pub linien: bool,
    /// Chronik: erste Spalte als Jahreszahl, ohne Linien und ohne Kopf.
    pub chronik: bool,
}

pub enum Block {
    H2(&'static str),
    H3(&'static str),
    P(&'static [Span]),
    /// Randbemerkung, kleiner gesetzt.
    Klein(&'static [Span]),
    Liste(&'static [&'static [Span]]),
    Tab(&'static Tabelle),
    /// Kasten mit der Ausgangslage.
    Lead {
        werte: &'static str,
        blocks: &'static [Block],
    },
    /// Roter Warnkasten.
    Alarm {
        titel: &'static str,
        blocks: &'static [Block],
    },
    Adresse {
        name: &'static str,
        rolle: &'static [Span],
        zeilen: &'static [&'static [Span]],
        links: &'static [Verweis],
    },
}

pub const TITEL: &str = "IgA-Vaskulitis";
pub const TITEL2: &str = "(Purpura Schönlein-Henoch)";
pub const UNTERTITEL: &str =
    "Krankheitsbild, notwendige Abklärungen und die Spezialistinnen und Spezialisten in Zürich";
pub const STAND: &str =
    "Informationsblatt für die Patientin und ihre Angehörigen · Stand 28. August 2026 · zum Mitnehmen zum Arzttermin";
pub const KOPFZEILE: &str = "IgA-Vaskulitis (Purpura Schönlein-Henoch)";


use Block::*;
use Span::{B, I, N, T};

// ---------------------------------------------------------------------------
// Tabellen
// ---------------------------------------------------------------------------

static T_VERLAUF: Tabelle = Tabelle {
    kopf: &["Beobachtung", "Warum sie zählt"],
    gewichte: &[32, 68],
    zeilen: &[
        &[
            &[B("Wochenlang kein Appetit")],
            &[T(
                "Wochen ohne Nahrung heissen Gewichtsverlust und Mangelernährung. Das ist eine \
                 eigene Behandlungsaufgabe und nicht nur ein Begleitumstand. Beim Wiederaufbau \
                 der Ernährung nach langem Fasten droht zudem das Refeeding-Syndrom – \
                 Verschiebungen von Phosphat, Kalium und Magnesium, die überwacht gehören.",
            )],
        ],
        &[
            &[B("Starke Bauchschmerzen")],
            &[T(
                "Die Bauchbeteiligung ist bei dieser Krankheit häufig und der Ort, an dem sie \
                 gefährlich wird: Schleimhautblutung, Darmwandödem mit Passagestörung, seltener \
                 Einstülpung oder Durchblutungsstörung.",
            )],
        ],
        &[
            &[B("Wenig Schlaf pro Nacht")],
            &[T(
                "Schmerz, der den Schlaf über Wochen zerreisst, ist ein Mass für die Stärke der \
                 Beschwerden – und Schlafmangel verschlechtert bei einer 84-Jährigen Kreislauf, \
                 Orientierung und Sturzrisiko zusätzlich.",
            )],
        ],
        &[
            &[B("Erbrechen, jedes Mal nachdem eine Kleinigkeit gegessen wurde")],
            &[T(
                "Das ist der wichtigste Punkt der Liste. Erbrechen kurz nach dem Essen heisst, \
                 dass der Mageninhalt nicht weiterkommt. Erbrechen gehört zu den Leitzeichen \
                 einer Passagestörung; bei weiter unten liegendem Hindernis wird es gallig.",
            )],
        ],
        &[
            &[B("Sehr wenig getrunken, keine Lust zu trinken")],
            &[T(
                "Im Alter lässt das Durstgefühl nach – Austrocknung ist dann die Regel und nicht \
                 die Ausnahme. Sie verschlechtert die Nierenfunktion, verstärkt die Verstopfung \
                 und macht schwindlig. Und sie nimmt jedem Abführmittel die Grundlage: Diese \
                 Mittel brauchen Wasser, um zu wirken.",
            )],
        ],
        &[
            &[B("Seit etwa vier bis fünf Wochen gar kein Stuhlgang mehr")],
            &[T(
                "Ausbleibender Stuhl gehört zusammen mit Erbrechen und Bauchschmerz zum \
                 klassischen Bild des Darmverschlusses. Ausbleibender Stuhl ",
            ), B("und"), T(
                " ausbleibender Windabgang ist der Notfall, nicht die Sprechstunde.",
            )],
        ],
    ],
    linien: true,
    chronik: false,
};

static T_ABKLAERUNG: Tabelle = Tabelle {
    kopf: &["Bereich", "Untersuchung und wozu"],
    gewichte: &[24, 76],
    zeilen: &[
        &[
            &[B("Blutbild")],
            &[T("Hämoglobin im Verlauf, MCV, Retikulozyten – sie zeigen, ob das Knochenmark auf den Verlust antwortet. Dazu die Thrombozytenzahl und, wegen Novalgin, das "), B("Differentialblutbild"), T(" mit den neutrophilen Granulozyten.")],
        ],
        &[
            &[B("Eisen")],
            &[T("Ferritin "), B("und"), T(" Transferrinsättigung, immer zusammen mit CRP. Ferritin steigt bei Entzündung an – ein normaler Wert schliesst einen Eisenmangel dann nicht aus.")],
        ],
        &[
            &[B("Niere")],
            &[T("Kreatinin, eGFR, Albumin im Blut – der Verlauf, nicht nur der Einzelwert.")],
        ],
        &[
            &[B("Blutzerfall")],
            &[T("LDH, Haptoglobin, Bilirubin – trennt einen Zerfall der roten Blutkörperchen vom Blutverlust nach aussen.")],
        ],
        &[
            &[B("Urin")],
            &[T("Status und Sediment (Erythrozyten, Akanthozyten, Zylinder) sowie "), B("Protein-Kreatinin-Quotient im Spoturin"), T(". Die wichtigste wiederholte Kontrolle überhaupt.")],
        ],
        &[
            &[B("Immunologie")],
            &[B("ANCA"), T(" – schliesst die im Alter deutlich häufigere ANCA-assoziierte Vaskulitis aus, die anders und dringlicher behandelt wird. Dazu Komplement C3/C4, Kryoglobuline, Hepatitis B und C, Eiweisselektrophorese und Immunfixation zur Suche nach einem Paraprotein.")],
        ],
        &[
            &[B("Gerinnung")],
            &[T("Quick/INR und Thrombozytenzahl. Eine Purpura bei zu wenigen Blutplättchen ist eine ganz andere Krankheit mit anderer Behandlung.")],
        ],
        &[
            &[B("Ernährung")],
            &[T("Gewichtsverlauf, Albumin, Phosphat, Kalium, Magnesium. Nach Wochen ohne Nahrung sind diese Werte die Voraussetzung dafür, dass der Kostaufbau sicher beginnen kann.")],
        ],
        &[
            &[B("Bauch, dringlich")],
            &[T("Computertomografie des Bauches bei starken Bauchschmerzen, Erbrechen und ausbleibendem Stuhl – sie zeigt Wandschwellung, Passagestörung, Durchblutungsstörung und Komplikationen. Dieser Schritt kommt "), B("vor"), T(" jedem Abführmittel durch den Mund.")],
        ],
        &[
            &[B("Darm")],
            &[T("Magen- und Darmspiegelung. Beantwortet Blutungsquelle und Tumorfrage gemeinsam – deshalb der ergiebigste einzelne Schritt, sobald eine Passagestörung ausgeschlossen ist.")],
        ],
        &[
            &[B("Haut")],
            &[T("Biopsie einer frischen Läsion mit direkter Immunfluoreszenz auf IgA. Nur solange frische Flecken da sind.")],
        ],
        &[
            &[B("Nierengewebe")],
            &[T("Nierenbiopsie bei relevantem Eiweissverlust oder fallender Nierenfunktion. Sie entscheidet über die Therapie.")],
        ],
    ],
    linien: true,
    chronik: false,
};

static T_ABFUEHR: Tabelle = Tabelle {
    kopf: &["Präparat", "Wirkstoff", "Aroma und Süssstoff laut Fachinformation"],
    gewichte: &[26, 26, 48],
    zeilen: &[
        &[
            &[B("Movicol neutral"), T(", "), B("Movicol Junior neutral")],
            &[T("Macrogol 3350 mit Elektrolyten")],
            &[B("Keines."), T(" Für die neutrale Variante nennt die Fachinformation weder ein Aroma noch einen Süssstoff.")],
        ],
        &[
            &[B("Laxipeg aromafrei")],
            &[T("Macrogol 4000")],
            &[B("Hilfsstoffe: keine."), T(" Wörtlich so in der Fachinformation.")],
        ],
        &[
            &[T("Movicol")],
            &[T("Macrogol 3350 mit Elektrolyten")],
            &[T("Acesulfam-Kalium (E950), Limetten- und Zitronenaroma")],
        ],
        &[
            &[T("Movicol Chocolat")],
            &[T("Macrogol 3350 mit Elektrolyten")],
            &[T("Acesulfam-Kalium (E950), Schokoladenaroma, dazu Benzylalkohol im Aroma")],
        ],
        &[
            &[T("Transipeg, Transipeg forte")],
            &[T("Macrogol 3350 mit Elektrolyten")],
            &[T("Aspartam (E951), Acesulfam-Kalium")],
        ],
        &[
            &[T("Laxipeg banane")],
            &[T("Macrogol 4000")],
            &[T("Acesulfam-Kalium (E950), Bananenaroma")],
        ],
        &[
            &[T("Duphalac, Gatinar, Rudolac")],
            &[T("Lactulose-Sirup")],
            &[T("Kein Zusatz nötig – der Wirkstoff selbst ist ein Zucker. Duphalac nennt unter Hilfsstoffen: keine.")],
        ],
        &[
            &[T("Importal")],
            &[T("Lactitol")],
            &[T("Zuckeralkohol, gleiches Prinzip wie Lactulose.")],
        ],
    ],
    linien: true,
    chronik: false,
};

static T_DARMSPIEGELUNG: Tabelle = Tabelle {
    kopf: &["Vorbereitung auf die Darmspiegelung", "Wirkstoff", "Aroma und Süssstoff"],
    gewichte: &[26, 26, 48],
    zeilen: &[
        &[
            &[T("Moviprep, Moviprep Orange")],
            &[T("Macrogol 3350, Natriumsulfat, Ascorbat")],
            &[T("Aspartam (E951), 0,233 g je Beutel, Acesulfam-Kalium, Zitronenaroma – dazu literweise zu trinken")],
        ],
        &[
            &[T("Picoprep, CitraFleet")],
            &[T("Natriumpicosulfat, Magnesiumoxid, Citronensäure")],
            &[T("Saccharin-Natrium (E954), Orangenaroma – wirkt zusätzlich stimulierend auf die Darmbewegung")],
        ],
        &[
            &[T("Plenvu, Clensia, Cololyt")],
            &[T("Macrogol mit Elektrolyten")],
            &[T("Dieselbe Gruppe; die Zusammensetzung steht in der jeweiligen Fachinformation.")],
        ],
    ],
    linien: true,
    chronik: false,
};

static T_INTERAKTION: Tabelle = Tabelle {
    kopf: &["Mit Novalgin zusammen", "Klasse", "Was der Check sagt"],
    gewichte: &[22, 24, 54],
    zeilen: &[
        &[
            &[B("Methotrexat")],
            &[B("X – kontraindiziert")],
            &[T("Erhöhtes Risiko für Blutbildveränderungen; die Kombination ist wegen des negativen Nutzen-Risiko-Profils kontraindiziert. Zur Schmerzstillung gehört ein Mittel mit geringerem Potenzial für Knochenmarksschäden. "), B("Die wichtigste Zeile dieser Tabelle:"), T(" Methotrexat ist in der Rheumatologie ein gängiges Mittel. Wer es nimmt, darf Novalgin nicht bekommen.")],
        ],
        &[
            &[B("Clozapin")],
            &[B("X – kontraindiziert")],
            &[T("Ebenfalls erhöhtes Risiko für Blutbildveränderungen. Die Fachinformation von Clozapin verbietet die gleichzeitige Anwendung von Mitteln, die eine Agranulozytose auslösen können.")],
        ],
        &[
            &[T("Carbamazepin")],
            &[T("C – regelmässige Überwachung")],
            &[T("Erhöhtes Agranulozytoserisiko, möglicherweise additive Knochenmarksdepression. Bei Kombination ist das Blutbild engmaschig zu kontrollieren.")],
        ],
        &[
            &[T("Niedrig dosiertes Aspirin (ASS 100)")],
            &[T("C – regelmässige Überwachung")],
            &[T("Verminderte gerinnungshemmende Wirkung; beide konkurrieren an derselben Bindungsstelle der COX-1. Massnahme: "), B("Aspirin 30 bis 60 Minuten vor Metamizol geben"), T(" – oder für die Schmerzen auf Paracetamol ausweichen.")],
        ],
        &[
            &[T("Bupropion")],
            &[T("C – regelmässige Überwachung")],
            &[T("Metamizol induziert CYP2B6 und CYP3A4 und erhöht damit den aktiven Abbaustoff von Bupropion.")],
        ],
    ],
    linien: true,
    chronik: false,
};

static T_CHRONIK: Tabelle = Tabelle {
    kopf: &[],
    gewichte: &[16, 84],
    zeilen: &[
        &[&[T("1801")], &[
        B("William Heberden, London."),
        T(" Beschreibt in seinen "), I("Commentaries"), T(" einen fünfjährigen Buben: Flecken an den Beinen, Bauchschmerzen, blutiger Stuhl, geschwollene Gelenke, Blut im Urin. Das vollständige Bild, 36 Jahre vor Schönlein – nur zieht niemand einen Schluss daraus. Eines bleibt bemerkenswert: Schon der allererste beschriebene Fall war ein Kind. Das prägt die Literatur bis heute und ist der Grund, warum über alte Patientinnen so wenig zu finden ist."),
        ]],
        &[&[T("1837")], &[
        B("Johann Lukas Schönlein, Zürich."),
        T(" Schönlein war 1832 in Würzburg im Zug der Demagogenverfolgung nach dem Hambacher Fest seiner Ämter enthoben worden; die eben gegründete Universität Zürich holte ihn 1833. In seinen Zürcher Jahren prägt er den Begriff "), I("Peliosis rheumatica"), T(" – griechisch "), I("pelios"), T(", fahl-blauschwarz. Sein Beitrag ist nicht die Beobachtung der Flecken, die kannte man, sondern die Verknüpfung: Hautblutung und Gelenkschmerz sind eine Krankheit und nicht zwei. Geschrieben hat er es nie selbst. Er publizierte fast nichts; seine Studenten schrieben die Vorlesungen mit und gaben sie heraus, von einer der Ausgaben distanzierte er sich ausdrücklich. Im selben Jahr findet er in Zürich den Erreger des Favus – einer der ersten Nachweise überhaupt, dass ein Mikroorganismus eine menschliche Krankheit verursacht, Jahrzehnte vor Koch und Pasteur."),
        ]],
        &[&[T("1868–1899")], &[
        B("Eduard Heinrich Henoch, Berlin."),
        T(" Henoch hörte Schönlein als Student und leitete später die Kinderabteilung der Charité. 1868 beschreibt er die Verbindung von Kolik, blutigem Durchfall, Gelenkschmerz und Ausschlag – der Bauch kommt dazu. 1874 legt er vier Kinderfälle mit dem vollständigen Quartett vor. 1899 betont er, wie häufig eine Nephritis dazugehört. Damit ist das Bild fertig, und zwar in genau der Reihenfolge, in der die Medizin die Krankheit bis heute begreift: erst was man sieht, dann was weh tut, zuletzt was gefährlich ist."),
        ]],
        &[&[T("1914–1948")], &[
        B("Die Allergie-Epoche."),
        T(" Osler vermutet 1914, es steckten anaphylaktische Vorgänge dahinter; Frank und Glanzmann prägen daraufhin den Namen «anaphylaktoide Purpura». Im Mechanismus falsch, in der Denkrichtung richtig – das Immunsystem war im Spiel. 1948 spannt Douglas Gairdner im "), I("Quarterly Journal of Medicine"), T(" die beiden Namen endgültig zusammen. Ab da heisst sie Schönlein-Henoch-Syndrom."),
        ]],
        &[&[T("1968–1973")], &[
        B("Der Beweis."),
        T(" Jean Berger beschreibt 1968 in Paris eine Nierenkrankheit mit IgA-Ablagerungen im Mesangium – gegen die damalige Lehre, nach der IgG das schädigende Immunglobulin sei. 1973 zeigen Baart de la Faille-Kuyper und Mitarbeiter im "), I("Lancet"), T(" dasselbe IgA in den Hautgefässen "), B("und"), T(" im Nierenmesangium von Schönlein-Henoch-Patienten. Damit ist klar, was 172 Jahre lang nur ein Muster von Symptomen war: eine Krankheit der IgA-Immunkomplexe. Und es erklärt, warum ausgerechnet Haut, Darm und Niere zusammen erkranken – sie haben nichts gemeinsam ausser der Grösse ihrer Gefässe."),
        ]],
        &[&[T("2012")], &[
        B("Der Name fällt."),
        T(" Die Chapel Hill Consensus Conference benennt die Vaskulitiden nach ihrer Ursache statt nach ihren Entdeckern. Aus Purpura Schönlein-Henoch wird IgA-Vaskulitis. Beide Namen laufen bis heute nebeneinander; im Klinikalltag hört man das alte oft."),
        ]],
    ],
    linien: false,
    chronik: true,
};

// ---------------------------------------------------------------------------
// Das Dokument
// ---------------------------------------------------------------------------

pub static DOKUMENT: &[Block] = &[
    H2("Wie die Krankheit entdeckt wurde"),
    P(&[T("Der Doppelname führt in die Irre. Keiner der beiden hat die Krankheit als Erster gesehen, und was sie wirklich ist, wusste bis 1973 niemand.")]),
    Tab(&T_CHRONIK),
    Klein(&[T("Von Heberdens Buben bis zum Fluoreszenzmikroskop vergingen 172 Jahre. Das Kortison, das weiter unten im Abschnitt zur Behandlung steht, war schon zwei Jahrzehnte in Gebrauch, bevor man wusste, wogegen es eigentlich wirkt.")]),

    Lead {
        werte: "Ausgangslage: Patientin, 84 Jahre · Hämoglobin von 108 auf 83 g/l gefallen",
        blocks: &[
            P(&[T("Drei Dinge daran sind wichtig, und sie sind nicht dasselbe.")]),
            P(&[
                B("Der Wert."), T(" "), N("83 g/l"), T(" ist eine mittelschwere Blutarmut; der Normbereich für Frauen liegt bei etwa "), N("117–157 g/l"), T(". Die üblichen Transfusionsgrenzen liegen bei "), N("70 g/l"), T(", bei bekannter Herzkrankheit bei "), N("80 g/l"), T(". 83 liegt knapp darüber – also nicht automatisch transfusionsbedürftig, aber in dem Bereich, in dem der Zustand der Patientin entscheidet und nicht die Zahl."),
            ]),
            P(&[
                B("Das Tempo."), T(" Der Abfall um "), N("25 g/l"), T(" ist der eigentliche Befund. Eine Blutarmut durch Entzündung oder Nierenschwäche entwickelt sich über Monate. Ein Abfall innert Wochen spricht für Blutverlust – bei dieser Krankheit in erster Linie aus dem Darm. Das gehört rasch abgeklärt und nicht in eine Sprechstunde in sechs Wochen."),
            ]),
            P(&[
                B("Der Bauch."), T(" Seit Wochen kein Appetit, starke Bauchschmerzen, wenig Schlaf, immer wieder Erbrechen – jedes Mal, nachdem eine Kleinigkeit gegessen wurde –, kaum Flüssigkeit, und seit etwa vier bis fünf Wochen gar kein Stuhlgang mehr. Diese Kombination ist keine gewöhnliche Verstopfung; sie ist das Bild einer Passagestörung und der dringlichste Teil dieses Blattes. Siehe den Abschnitt «Was seit Wochen läuft»."),
            ]),
            P(&[
                B("Und ein Vorlauf."), T(" Es gab schon einmal einen massiven Ausschlag an den Beinen, nach einer Phase starker Belastung. Damit ist das heutige Bild wahrscheinlich kein Erstereignis – siehe den Abschnitt «Ein früherer Schub»."),
            ]),
        ],
    },
    H2("Was seit Wochen läuft"),
    P(&[T("Was die Patientin seit Wochen erlebt, gehört zusammen auf ein Blatt. Einzeln klingt jeder Punkt nach einer Unannehmlichkeit; zusammen ergeben sie etwas anderes.")]),
    Tab(&T_VERLAUF),
    P(&[
        T("Zusammengenommen ist das die klassische Kombination einer "), B("Passagestörung"), T(": Erbrechen kurz nach dem Essen, ausbleibender Stuhl über Wochen, starker Bauchschmerz. Bei einer IgA-Vaskulitis ist der naheliegende Mechanismus ein Darmwandödem – die entzündete, geschwollene Wand behindert die Passage. Es kommen aber auch die Erklärungen infrage, die bei einer 84-Jährigen ohnehin auf der Liste stehen, ein Tumor voran. Beides klärt dasselbe: ein Bild vom Bauch."),
    ]),
    P(&[
        T("Daraus folgen zwei Dinge. Erstens: Ein Abführmittel durch den Mund ist in dieser Lage nicht die Antwort, sondern das Gegenteil davon – die Fachinformationen aller dieser Mittel führen Darmverschluss und Obstruktion als Gegenanzeige. Zweitens: Wer seit Wochen weder isst noch trinkt, trocknet aus und verliert Gewicht."),
    ]),
    P(&[
        B("Die Infusion ist deshalb richtig."), T(" Wenn durch den Mund nichts drin bleibt, füllt sie das Blutvolumen wieder auf – kristalloide Lösungen sind bei Austrocknung und Blutverlust das Mittel erster Wahl. Das ist bei dieser Patientin gleich dreifach von Nutzen: Der Kreislauf bekommt wieder Volumen, gegen Schwindel und Schwarzwerden beim Aufstehen; die Niere bekommt wieder Durchblutung, und sie ist bei dieser Krankheit ohnehin das gefährdete Organ; und der Körper braucht Wasser, damit ein Abführmittel überhaupt wirken kann. Für Nahrung gilt dasselbe in eigener Rechnung – nach Wochen ohne Essen gehört der Kostaufbau überwacht."),
    ]),

    H2("Den Hämoglobinwert richtig lesen"),
    P(&[
        T("Ein verbreitetes Missverständnis, das hier viel ändert: "), B("Tief ist nicht besser, tief ist schlechter."), T(" Hämoglobin ist der rote Blutfarbstoff, der den Sauerstoff transportiert. Weniger Hämoglobin heisst weniger Sauerstoff in den Geweben. Der Normbereich für Frauen liegt bei rund "), N("117–157 g/l"), T("; 108 war bereits zu tief, 83 ist eine deutliche Blutarmut."),
    ]),
    P(&[
        T("Die Verwechslung dahinter ist naheliegend: "), B("HbA1c"), T(", das «Hämoglobin A1c», ist der Langzeit-Blutzuckerwert – und dort ist tief tatsächlich besser. Dieser Wert misst aber, wie stark der Zucker das Hämoglobin verzuckert hat, nicht wie viel Hämoglobin überhaupt vorhanden ist. Gleicher Wortstamm, gegenläufige Richtung."),
    ]),
    P(&[T("Die einzige Lage, in der ein Arzt Hämoglobin absichtlich senkt, ist die Polyzythämie: zu viel davon, das Blut wird zu dickflüssig, dann Aderlass. Das ist das Gegenteil dieser Situation.")]),
    P(&[
        B("Und was die Infusion mit dem Wert macht."), T(" Hämoglobin ist eine Konzentration, kein Vorrat: gemessen wird, wie viel Farbstoff in einem Liter Blut steckt. Wer seit Wochen kaum trinkt, hat weniger Flüssigkeit im Kreislauf – das dickt das Blut ein und lässt den Wert "), B("besser"), T(" aussehen, als er ist. Läuft dann die Infusion, verdünnt sich das Blut wieder, und das Hämoglobin kann weiter fallen, ohne dass ein einziger Tropfen zusätzlich verloren gegangen wäre. Dasselbe gilt umgekehrt für eine frische Blutung: Der Wert sinkt erst mit der Verdünnung, über 24 bis 72 Stunden. Zwei praktische Folgen: "), N("83 g/l"), T(", bei einer ausgetrockneten Patientin gemessen, ist eher zu günstig als zu schlecht – und ein Abfall unter laufender Infusion ist zuerst Verdünnung und nicht automatisch eine neue Blutung. Auseinanderhalten lässt sich das nur im Verlauf, zusammen mit Puls, Blutdruck und dem Aussehen des Stuhls."),
    ]),
    P(&[
        T("Was "), N("83 g/l"), T(" mit 84 Jahren praktisch bedeutet: Der Körper gleicht den Mangel mit höherem Puls und schnellerer Atmung aus. Daraus werden Müdigkeit, Schwindel beim Aufstehen, Kurzatmigkeit und Sturzgefahr – bei vorgeschädigtem Herz auch Angina pectoris. Genau darum liegt die Transfusionsgrenze bei bekannter Herzkrankheit bei 80 statt "), N("70 g/l"), T("."),
    ]),

    H2("Was die Krankheit ist"),
    P(&[T("Die IgA-Vaskulitis – der ältere Name Purpura Schönlein-Henoch ist noch geläufig – ist eine Entzündung der kleinsten Blutgefässe. Antikörper der Klasse IgA lagern sich in den Gefässwänden ab, das Immunsystem reagiert darauf, und die Gefässe werden durchlässig und brüchig. Weil solche kleinen Gefässe überall im Körper liegen, betrifft die Krankheit vier Bereiche, klassisch in dieser Kombination:")]),
    Liste(&[
        &[B("Haut."), T(" Tastbare Purpura: rötlich-violette Flecken, die sich beim Darüberstreichen erhaben anfühlen und auf Druck nicht verblassen. Typisch an Unterschenkeln und Gesäss. Das ist meist das erste und sichtbarste Zeichen – und das harmloseste.")],
        &[B("Gelenke."), T(" Schmerzen und Schwellungen, meist Sprung- und Kniegelenke. Vorübergehend, ohne bleibenden Schaden.")],
        &[B("Magen-Darm-Trakt."), T(" Kolikartige Bauchschmerzen, Übelkeit, Erbrechen, Blut im Stuhl. Dieselbe Gefässentzündung schädigt die Darmschleimhaut; vor allem im Zwölffingerdarm entstehen Geschwüre, die bluten. Schwillt die Wand an, stört das die Passage.")],
        &[B("Niere."), T(" Der Punkt, der über den Verlauf entscheidet. Blut und Eiweiss im Urin, oft ohne dass die Patientin etwas davon spürt. Deshalb ist die regelmässige Urinkontrolle aussagekräftiger als das Befinden.")],
    ]),
    P(&[T("Die Diagnose stützt sich auf das klinische Bild und, wo möglich, auf eine Hautbiopsie mit direkter Immunfluoreszenz: Der Nachweis von IgA in der Gefässwand ist der Beweis. Entscheidend dabei – die Probe muss aus einer "), B("frischen"), T(" Hautstelle stammen, jünger als etwa 48 Stunden. An abgeheilten Flecken findet sich nichts mehr.")]),

    H2("Was den Schub ausgelöst haben kann"),
    P(&[T("In den entzündeten Gefässen sitzen keine Bakterien, sondern "), B("Immunkomplexe aus körpereigenen Antikörpern"), T(". Die Krankheit ist keine Infektion und "), B("nicht übertragbar"), T(" – niemand im Haushalt ist gefährdet, es braucht keine Isolation.")]),
    P(&[T("Ein grosser Teil der Fälle beginnt "), B("ein bis drei Wochen nach einem Infekt"), T(", meist der oberen Atemwege; die Angaben schwanken je nach Studie zwischen etwa 50 und 90 Prozent. Am häufigsten genannt wird "), B("Streptococcus der Gruppe A"), T(", derselbe Erreger, der die eitrige Angina verursacht; bei über 30 Prozent der Patienten mit Nierenbeteiligung liessen sich Streptokokken kulturell nachweisen. Der Infekt ist also oft der Auslöser, aber nicht die Krankheit – wenn der Ausschlag erscheint, ist die Angina meist längst vorbei.")]),
    P(&[T("Daraus folgt der Punkt, der im Alltag am meisten zählt: "), B("Antibiotika behandeln die Vaskulitis nicht."), T(" Sie behandeln eine Infektion, falls gerade eine besteht – aber sie bringen weder den Ausschlag noch die Bauchschmerzen noch die Nierenbeteiligung zum Verschwinden. Deshalb steht im Abschnitt zur Behandlung Kortison und kein Antibiotikum.")]),
    P(&[T("Bei einer 84-jährigen Patientin verschiebt sich die Auslöserfrage. Der Infekt als Auslöser ist die Regel "), I("bei Kindern"), T(". Im Alter stehen zwei andere Möglichkeiten weiter vorn: "), B("Medikamente"), T(" und eine noch unentdeckte "), B("Tumorerkrankung"), T(" – siehe den nächsten Abschnitt. Die Frage «welcher Infekt war es?» ist hier weniger ergiebig als «welche Medikamente sind in den letzten Wochen neu dazugekommen?» und «wurde nach einem Tumor gesucht?».")]),

    H2("Warum das Alter den Unterschied macht"),
    P(&[T("Fast alles, was man über diese Krankheit liest, stammt aus der Kinderheilkunde. Dort ist sie häufig, verläuft meist harmlos und heilt von selbst aus. Bei Erwachsenen ist sie selten – rund 0,8 bis 1,8 Fälle auf 100'000 Personen im Jahr – und sie verhält sich anders:")]),
    Liste(&[
        &[T("Nieren- und Darmbeteiligung sind häufiger und ausgeprägter als bei Kindern.")],
        &[T("Alter über 65 Jahre, Eiweissverlust über den Urin, nachlassende Nierenfunktion und Blut im Urin sind die ungünstigen Zeichen.")],
        &[T("Eine Darmblutung ist nicht nur ein Problem für sich, sondern ein Warnzeichen für die Niere. In einer Untersuchung an Erwachsenen mit Nierenbeteiligung landeten die Betroffenen mit Darmblutung weit häufiger in einem dauerhaften Nierenversagen und weit seltener in einer Ausheilung als die ohne.")],
        &[T("Bei älteren Menschen kann eine IgA-Vaskulitis die Begleiterscheinung einer noch unentdeckten Krebserkrankung sein. In einer Übersichtsarbeit entwickelten rund 16 Prozent der erwachsenen Betroffenen einen Tumor, zu gut zwei Dritteln solide Tumoren – Lunge, Prostata, Brust –, zum übrigen Drittel Lymphome und Leukämien. Verbindliche Suchempfehlungen gibt es nicht, aber im hohen Alter ist die Frage zu stellen.")],
    ]),
    P(&[T("Der letzte Punkt, der Hämoglobin-Abfall und der seit Wochen fehlende Stuhlgang hängen zusammen: Ein Bild vom Bauch und anschliessend eine Magen- und Darmspiegelung beantworten dieselben Fragen in einem Durchgang – wo es blutet, was die Passage behindert, und ob etwas dahintersteckt.")]),

    H2("Ein früherer Schub"),
    P(&[T("Tastbare Purpura an den Unterschenkeln ist die Signatur dieser Krankheit. Ist das schon einmal aufgetreten, ist das heutige Bild wahrscheinlich ein "), B("Rezidiv"), T(" – und das ändert die Einordnung.")]),
    P(&[T("Bei Erwachsenen kehrt die IgA-Vaskulitis in etwa "), B("43 Prozent"), T(" der Fälle wieder, gut ein Viertel davon mehrfach; nur rund die Hälfte erreicht eine vollständige Remission. Wiederkehren ist bei Erwachsenen die Regel und nicht die Ausnahme, und höheres Alter beim Erstauftreten ist selbst ein Vorhersagefaktor für Rückfälle. Die Tumorfrage bleibt dabei auf dem Tisch: Die Suche wird ausdrücklich bei "), B("neu aufgetretener oder wiederkehrender"), T(" IgA-Vaskulitis im Alter empfohlen – ein Rezidiv entlastet nicht.")]),
    H3("Die Frage, auf die es ankommt"),
    P(&[B("Wurde damals der Urin kontrolliert?"), T(" Daran entscheidet sich, wie lange die Niere schon unter Beschuss steht. Eine Nierenbeteiligung tritt oft erst Wochen nach dem Ausschlag auf und macht keine Beschwerden – sie zeigt sich nur im Urin. Wurde der erste Schub als Hautsache abgetan und hat niemand Urinstatus und Protein-Kreatinin-Quotient bestimmt, kann die Niere seither still gelitten haben. Das würde erklären, warum das Bild heute schwerer ist.")]),
    P(&[T("Und falls damals eine Diagnose gestellt wurde: Gab es eine Hautbiopsie? Ein IgA-Nachweis in der Gefässwand von damals wäre heute Gold wert – er erspart die Frage, ob es dieselbe Krankheit ist.")]),
    H3("Zum Stress als Auslöser"),
    P(&[T("Für die IgA-Vaskulitis speziell ist psychischer Stress "), B("kein belegter Auslöser"), T(". Belegt sind Infekte, Medikamente und Tumoren. Es gibt eine Untersuchung, nach der Vaskulitis-Patienten den Stress sehr häufig als Ursache annehmen, ohne dass sich das wissenschaftlich stützen liess – und nach der diese Überzeugung mit mehr Erschöpfung und Funktionseinbussen einherging. Für andere Vaskulitiden, etwa die ANCA-assoziierte, gibt es Hinweise auf Stress als Schubauslöser.")]),
    P(&[T("Plausibel, aber nicht gesichert, ist die indirekte Kette: Stress schwächt die Infektabwehr, und Infekte sind der belegte Auslöser. Eine belastende Phase mit einer verschleppten Erkältung darin ist naheliegend – nur erinnert man hinterher den Stress und nicht das Halsweh. Die praktische Folge: Die Stress-Erklärung darf die Suche nach dem tatsächlichen Auslöser nicht ersetzen. Hier heisst das vor allem: Welche Medikamente sind neu dazugekommen?")]),
    H3("Was bei wiederkehrender Beinpurpura dazugehört"),
    P(&[T("Ein wiederholtes Auftreten macht drei Untersuchungen wichtiger, die in der Tabelle weiter oben ohnehin stehen:")]),
    Liste(&[
        &[B("Eiweisselektrophorese und Immunfixation"), T(" – eine IgA-Paraprotein-assoziierte Vaskulitis (MGUS, Myelom) sieht klinisch gleich aus und kommt im Alter vor.")],
        &[B("Kryoglobuline, Hepatitis B und C"), T(" – die kryoglobulinämische Vaskulitis ist der klassische Nachahmer bei rezidivierender Beinpurpura.")],
        &[B("Medikamentenanamnese über Jahre"), T(" – eine arzneimittelbedingte leukozytoklastische Vaskulitis kehrt wieder, solange das auslösende Mittel wiederkehrt.")],
    ]),
    H3("Nicht verwechseln: Purpura senilis"),
    P(&[T("Die Purpura senilis ist bei 84-Jährigen sehr häufig: flache, nicht erhabene Blutungen, meist an Unterarmen und Handrücken, aus brüchigen Gefässen und oft unter Blutverdünnern. Sie ist harmlos und hat mit der Vaskulitis nichts zu tun. Das Unterscheidungsmerkmal ist einfach – "), B("tastbar erhaben und an den Unterschenkeln"), T(" spricht für die Vaskulitis, "), B("flach und an den Unterarmen"), T(" für Altershaut. Existieren Fotos des damaligen Ausschlags, klärt das die Frage in Sekunden.")]),

    H2("Woher der Blutverlust kommen kann"),
    Liste(&[
        &[B("Blutung aus dem Magen-Darm-Trakt."), T(" Die naheliegendste Erklärung. Die Vaskulitis schädigt die Schleimhaut, am häufigsten im Zwölffingerdarm. Die Blutung muss nicht sichtbar sein – sie kann über Wochen sickern, ohne dass der Stuhl auffällt. Kommt seit Wochen ohnehin kein Stuhl, fällt dieser Hinweis ganz weg.")],
        &[B("Medikamente."), T(" Kortison zusammen mit Schmerzmitteln vom NSAR-Typ (Ibuprofen, Diclofenac, Naproxen) erhöht das Geschwürrisiko erheblich; NSAR belasten zusätzlich die Niere. Blutverdünner verstärken jede vorhandene Blutungsquelle. Die vollständige Medikamentenliste gehört auf den Tisch, rezeptfreie Mittel eingeschlossen.")],
        &[B("Eine zweite, unabhängige Quelle."), T(" Mit 84 Jahren ist ein Dickdarmtumor oder ein Magengeschwür statistisch häufiger als die Vaskulitis selbst. Beides würde den Blutverlust erklären – und zusammen mit der ausbleibenden Passage womöglich auch den Rest.")],
        &[B("Die Niere."), T(" Bei nachlassender Nierenfunktion bildet der Körper weniger Erythropoetin, das Hormon für die Blutbildung. Das erklärt eine langsam sinkende Kurve, keinen Sturz um "), N("25 g/l"), T(" in kurzer Zeit.")],
        &[B("Novalgin."), T(" Metamizol kann das Knochenmark treffen: Leukopenie selten, Agranulozytose sehr selten, aplastische Anämie und Panzytopenie ohne Häufigkeitsangabe. Als alleinige Erklärung unwahrscheinlich, aber mit einem Differentialblutbild in einem Schritt geprüft – siehe den Abschnitt zu Novalgin.")],
        &[B("Mangelernährung."), T(" Wochen ohne Nahrung liefern zu wenig Eisen, Folsäure und Vitamin B12. Als alleinige Erklärung für dieses Tempo zu langsam, als verstärkender Faktor real – und mit drei Laborwerten geprüft.")],
        &[B("Entzündungsanämie."), T(" Ebenso: als Grundrauschen möglich, für diesen Verlauf zu langsam.")],
        &[B("Verdünnung durch die Infusion."), T(" Sie senkt den gemessenen Wert, ohne dass Blut verloren geht – siehe oben. Zusammen mit häufigen Blutentnahmen erklärt das einen Teil des Abfalls und ist vor der grossen Abklärung in einer Minute geprüft.")],
    ]),

    H2("Was abgeklärt gehört"),
    Tab(&T_ABKLAERUNG),

    H2("Zur Behandlung"),
    P(&[T("Die Therapie führen die Spezialisten; hier steht nur, was den Rahmen erklärt.")]),
    P(&[T("Sind allein Haut und Gelenke betroffen, wird oft beobachtet und nur gegen die Beschwerden behandelt. Sobald Darm oder Niere beteiligt sind, kommt Kortison zum Einsatz, üblicherweise Prednison um 1 mg pro Kilogramm Körpergewicht mit anschliessendem Ausschleichen. Eine frühe Kortisonbehandlung senkt die Wahrscheinlichkeit eines bleibenden Nierenschadens deutlich. Bei schwerer Nierenbeteiligung kommen zusätzliche Immunsuppressiva in Frage – das entscheidet die Nierenbiopsie.")]),
    P(&[T("Im hohen Alter gilt: die niedrigste wirksame Dosis, und die Nebenwirkungen von Anfang an mitbehandeln – Magenschutz, Blutzucker, Knochendichte, Infektrisiko.")]),
    P(&[T("Drei Punkte, die sich anzusprechen lohnen:")]),
    Liste(&[
        &[B("NSAR meiden."), T(" Ibuprofen, Diclofenac, Naproxen belasten Niere und Magenschleimhaut gleichzeitig – beides genau dort, wo die Krankheit ohnehin angreift. Paracetamol ist die verträglichere Alternative. Sie hat "), B("Novalgin"), T(" bekommen; das ist kein NSAR und für kolikartige Schmerzen naheliegend, bringt aber eigene Auflagen mit – siehe den nächsten Abschnitt.")],
        &[B("Blutdruck einstellen."), T(" Bei Eiweiss im Urin schützt ein ACE-Hemmer oder ein Sartan die Niere zusätzlich.")],
        &[B("Was durch den Mund nicht geht, geht durch die Vene."), T(" Solange Erbrechen jede Mahlzeit beendet, ist auch die Aufnahme von Medikamenten unsicher. Das gehört bei der Verordnung mitbedacht.")],
    ]),
    P(&[T("Und die Nachkontrolle: Urin und Blutdruck regelmässig über mindestens sechs bis zwölf Monate, bei Erwachsenen eher länger. Eine Nierenbeteiligung kann auch dann noch auftreten, wenn der Ausschlag längst verschwunden ist. Genau das ist der häufigste Fehler – die Kontrollen enden, sobald die Haut wieder sauber aussieht.")]),

    H2("Novalgin"),
    P(&[T("Novalgin ist "), B("Metamizol"), T(" (Novaminsulfon, Dipyron), ein Pyrazolonderivat und "), B("kein NSAR"), T(". Es wirkt schmerzstillend, fiebersenkend und "), B("krampflösend"), T(" – gerade das Letzte passt zu kolikartigen Bauchschmerzen. Als Wahl ist es nachvollziehbar: Es belastet Magenschleimhaut und Niere nicht auf demselben Weg wie Ibuprofen oder Diclofenac. Dieselbe Substanz heisst in der Schweiz auch Minalgin, Metamizol-Mepha, Metamizol Spirig HC oder Novaminsulfon Sintetica; es gibt sie als Tabletten, Tropfen und Ampullen.")]),
    P(&[T("Fünf Punkte aus der Fachinformation, die in dieser Lage zählen:")]),
    Liste(&[
        &[B("Das Blutbild gehört dazu."), T(" Metamizol kann eine "), B("Agranulozytose"), T(" auslösen – sehr selten, unter 1 von 10'000, aber sie kann tödlich verlaufen. Sie ist nicht dosisabhängig, kann jederzeit auftreten, auch nach früher problemloser Einnahme, und noch kurz nach dem Absetzen. Bei Fieber, Schüttelfrost, Halsschmerzen oder schmerzhaften Stellen der Schleimhaut in Mund, Nase oder Rachen ist die Behandlung zu unterbrechen und sofort ein "), B("vollständiges Blutbild mit Differentialblutbild"), T(" zu machen. Ohne Häufigkeitsangabe stehen ausserdem aplastische Anämie und Panzytopenie in der Liste, beide auch mit tödlichem Ausgang. In den deutschen Meldungen traten zwei Drittel der Fälle innert sechs Wochen auf, knapp ein Drittel innert sieben Tagen.")],
        &[B("Damit gehört Novalgin auf die Liste der Erklärungen für den Hämoglobin-Abfall."), T(" Nicht als wahrscheinlichste, aber als eine, die ein Differentialblutbild in einem Schritt mitprüft. Die Fachinformation nennt ausdrücklich "), B("Blässe"), T(" als Anzeichen, mit dem man zum Arzt soll – daneben Krankheitsgefühl, Infektionszeichen, andauerndes Fieber, Hämatome und Blutungen.")],
        &[B("Blutdruck."), T(" Gelegentlich – 1 von 1000 bis 1 von 100 – löst Metamizol einen isolierten Blutdruckabfall aus. Die Fachinformation verlangt vorher ausdrücklich die «Optimierung des hämodynamischen Status bei Patienten mit vorbestehender Hypotonie mit Volumenmangel, Dehydratation, instabilem Kreislauf». Das beschreibt genau diese Patientin und ist ein weiterer Grund für die Infusion. In die Vene darf höchstens 1 ml pro Minute laufen.")],
        &[B("Niere."), T(" Sehr selten ein akutes Nierenversagen, «vor allem wenn bereits eine Nierenerkrankung vorliegt», dazu eine akute interstitielle Nephritis ohne Häufigkeitsangabe. Bei einer Krankheit, über deren Verlauf die Niere entscheidet, ist das kein Nebensatz. Hohe Dosen sind bei eingeschränkter Nierenfunktion zu vermeiden, und im Alter ist die Nierenfunktion ohnehin mitzudenken.")],
        &[B("Der Kreatininwert kann falsch sein."), T(" Metamizol stört Labortests, die auf der Trinder-Reaktion beruhen – ausdrücklich auch die Messung des "), B("Kreatinins"), T(". Wer die Nierenfunktion im Verlauf beurteilt, muss das wissen.")],
    ]),
    P(&[T("Zwei Dinge, die im Alltag Verwirrung stiften:")]),
    Liste(&[
        &[B("Roter Urin unter Novalgin muss kein Blut sein."), T(" Metamizol bildet Rubazonsäure, ein harmloses Abbauprodukt, das den Harn rot färbt und nach dem Absetzen verschwindet. Weil dieses Blatt die Urinkontrolle in den Mittelpunkt stellt, zählt das doppelt – und ebenso, dass die Frage nicht mit blossem Auge beantwortet wird, sondern mit Streifen und Sediment.")],
        &[B("Magen-Darm-Blutungen stehen trotzdem in der Liste."), T(" Ohne Häufigkeitsangabe, aber die Fachinformation nennt gastrointestinale Blutungen, Ulzerationen und Perforationen unter den unerwünschten Wirkungen. «Kein NSAR» heisst also nicht «für den Magen unbedenklich».")],
    ]),
    P(&[T("Zuerst zu klären sind zwei Gegenanzeigen: eine "), B("eingeschränkte Knochenmarksfunktion oder Blutbildungsstörung"), T(" und eine frühere Agranulozytose unter Pyrazolonen. Und falls sie niedrig dosiertes Aspirin zum Herzschutz nimmt: Metamizol schwächt dessen Wirkung auf die Blutplättchen ab.")]),

    H2("Interaktionscheck"),
    P(&[T("Geprüft mit "), B("SDIF"), T(", dem Swiss Drug Interaction Finder: Er wertet die Interaktionsangaben aus den Schweizer Fachinformationen aus und gleicht sie mit der EPha-Datenbank ab, die jede Kombination von A bis X einstuft – A keine Massnahmen, C regelmässige Überwachung, D Kombination vermeiden, X kontraindiziert. Der Lauf vom 28. August 2026 mit Novalgin, Kortison, Macrogol, Lactulose und niedrig dosiertem Aspirin ergab Folgendes.")]),
    P(&[
        B("Der Vorbehalt zuerst:"), T(" Geprüft ist nur, was auf diesem Blatt steht. Die vollständige Medikamentenliste kennt nur die Patientin selbst, und sie gehört zum Termin mitgebracht – rezeptfreie Mittel eingeschlossen. Ein maschineller Check kann nur vergleichen, was man ihm gibt."),
    ]),
    Tab(&T_INTERAKTION),
    H3("Was bei Kortison dazukommt"),
    Liste(&[
        &[B("Aspirin und Kortison: Klasse C."), T(" Erhöhtes Risiko einer Blutung im Magen-Darm-Trakt durch additive Schädigung der Magenschleimhaut. Das Risiko steigt ausdrücklich mit höherem Lebensalter, mit einer Vorgeschichte von Geschwüren und unter gleichzeitiger Blutverdünnung – alle drei gehören hier geprüft. Empfohlene Massnahme: klinische Überwachung auf Geschwüre und die "), B("vorbeugende Gabe eines Protonenpumpenhemmers"), T(". Das deckt sich mit dem Magenschutz, der im Abschnitt zur Behandlung ohnehin steht.")],
        &[B("Lactulose und Kortison."), T(" Kortikosteroide senken das Kalium, und die Duphalac-Fachinformation nennt Lactulose als Mittel, das diesen Kaliumverlust theoretisch verstärkt. Bei einer Patientin, deren Kalium nach Wochen ohne Nahrung ohnehin überwacht gehört, ist das ein weiteres Argument für Macrogol.")],
    ]),
    H3("Ohne Treffer"),
    P(&[T("Novalgin mit Macrogol, mit Lactulose, mit Paracetamol, mit einem Protonenpumpenhemmer und mit einem ACE-Hemmer: kein Treffer. Macrogol hat in der EPha-Datenbank überhaupt keinen Eintrag – es wird nicht aufgenommen und interagiert praktisch nicht. Auch das spricht für Movicol neutral.")]),
    H3("Ein Fund am Rande, der hier zählt"),
    P(&[T("Beim Auflösen des Namens «Paracetamol» griff der Check auf Kombinationspräparate zu, die zusätzlich Tramadol oder Codein enthalten; die dortigen Warnungen betrafen den Opioid-Anteil und nicht das Paracetamol. Als Interaktion ist das ein Fehlalarm – als Hinweis ist es der praktisch wichtigste des ganzen Laufs: "), B("Opioide verstopfen."), T(" Die Duphalac-Fachinformation führt Opiate ausdrücklich unter den Substanzen, die die Wirkung von Lactulose abschwächen, weil sie selbst obstipierend wirken. Bei einer Patientin, die seit Wochen keinen Stuhlgang hat, ist ein opioidhaltiges Schmerzmittel – Codein, Tramadol – das Falsche. Novalgin enthält keines, und das ist ein Punkt zu seinen Gunsten.")]),
    P(&[T("Zwei weitere Treffer des Laufs sind ebenfalls keine: Der Check meldete «kontraindiziert» zwischen Aspirin und Movicol, weil er im Namen «Macrogol, Kombinationen» das Wort «Kombinationen» als Wirkstoff las, und er ordnete Novalgin einer Regel für Aspirin und NSAR zu, obwohl Metamizol keines von beiden ist. Wer maschinell prüft, muss die Treffer nachlesen; die Fachinformation entscheidet, nicht die Trefferliste.")]),

    H2("Abführmittel: welche es gibt und warum sie süss sind"),
    P(&[T("Vorbemerkung, weil sie in dieser Lage alles andere überwiegt: Solange Erbrechen nach jedem Essen und wochenlang fehlender Stuhlgang nicht abgeklärt sind, ist die Frage nach dem richtigen Abführmittel die zweite Frage. Die erste steht im Abschnitt oben. Was hier folgt, gilt für die Zeit danach – und für den Fall, dass ein Mittel bereits verordnet ist und schlecht vertragen wird.")]),
    P(&[T("Dass ein Abführmittel zum Trinken süss ist, ist kein Zufall des Herstellers. Bei der einen Gruppe ist der Wirkstoff selbst ein Zucker; bei der anderen ist die Süsse ein Zusatz – und den gibt es auch ohne.")]),
    P(&[
        B("Lactulose ist der Zucker."), T(" Duphalac, Gatinar und Rudolac sind Lactulose-Sirup. Die Fachinformation nennt unter Hilfsstoffen: keine. Süss ist hier nicht ein Zusatz, sondern der Wirkstoff, und daran lässt sich nichts ändern. Lactulose wird nicht aufgenommen, sondern im Dickdarm von Bakterien "), B("vergoren"), T(" – dabei entstehen Gase. Blähungen sind deshalb kein Nebeneffekt, sondern das Stoffwechselprodukt. In den Zulassungsstudien war Durchfall sehr häufig (13,1 Prozent), Flatulenz, Bauchschmerzen, Übelkeit und Erbrechen häufig. Importal (Lactitol) ist ein Zuckeralkohol und funktioniert nach demselben Prinzip."),
    ]),
    P(&[
        B("Macrogol ist von sich aus geschmacklos."), T(" Es ist ein inertes Polymer: Es wird weder aufgenommen noch von Bakterien verstoffwechselt, sondern bindet osmotisch Wasser und geht unverändert durch. Keine Vergärung, entsprechend "), B("deutlich weniger Gas"), T(". Die Süsse der gängigen Präparate kommt aus Aroma und Süssstoff – und genau die gibt es auch weggelassen:"),
    ]),
    Tab(&T_ABFUEHR),
    P(&[T("Und die Mittel, die zur Vorbereitung einer Darmspiegelung literweise getrunken werden – die süssesten von allen:")]),
    Tab(&T_DARMSPIEGELUNG),
    P(&[
        B("Was man konkret verlangen kann:"), T(" "), B("Movicol neutral"), T(" oder "), B("Laxipeg aromafrei"), T(". Beide sind in der Schweiz zugelassen und kassenpflichtig, beide enthalten weder Aroma noch Süssstoff. Von Movicol neutral gibt es auch eine Junior-Packung, wenn eine kleinere Dosis leichter fällt."),
    ]),
    P(&[T("Der Cochrane-Vergleich gibt Macrogol gegenüber Lactulose ohnehin durchweg den Vorzug: bessere Stuhlfrequenz, bessere Stuhlform, weniger Bauchschmerzen, weniger Bedarf an Zusatzmitteln. Der Wechsel löst also nicht nur das Geschmacksproblem.")]),
    H3("Vier Punkte aus den Fachinformationen, die hier besonders zählen"),
    Liste(&[
        &[B("Bei Darmverschluss verboten."), T(" Alle diese Mittel führen intestinale Obstruktion, Ileus und Perforation als Gegenanzeige. Duphalac verlangt darüber hinaus ausdrücklich, dass schmerzhafte Bauchsymptome unklarer Ursache "), B("vor"), T(" Behandlungsbeginn abgeklärt werden, um eine nicht diagnostizierte Obstruktion auszuschliessen. Bei seit Wochen fehlendem Stuhlgang ist das keine Formalie.")],
        &[B("Ohne Flüssigkeit keine Wirkung."), T(" Duphalac empfiehlt während einer Abführbehandlung 1,5 bis 2 Liter am Tag. Für Movicol steht ausdrücklich, dass die zubereitete Lösung die reguläre Flüssigkeitszufuhr "), B("nicht ersetzt"), T("; ein Beutel wird in 125 ml Wasser gelöst. Wer kaum trinkt, dem hilft das Mittel wenig – und die Austrocknung trifft ausgerechnet die Niere.")],
        &[B("Im Alter die kleinere Dosis."), T(" Für Menschen über 65 genügt bei Movicol laut Fachinformation normalerweise ein Beutel täglich statt ein bis zwei.")],
        &[B("Die neutrale Variante lässt sich vorbereiten."), T(" Die zubereitete Lösung von Movicol neutral ist im Kühlschrank 24 Stunden haltbar, die der aromatisierten Varianten nur 6 – die neutrale kann also am Vorabend angesetzt und kalt getrunken werden.")],
    ]),
    Alarm {
        titel: "Erbrechen von Galle ist kein Geschmacksproblem",
        blocks: &[
            P(&[T("Grün-gelbe Galle zu erbrechen heisst, dass Darminhalt rückwärts läuft. Bei Bauchbeteiligung einer Vaskulitis und gleichzeitig fallendem Hämoglobin ist das ein Warnzeichen für einen "), B("Subileus"), T(" – die entzündete, geschwollene Darmwand behindert die Passage. Ein Darmwandödem mit Passagestörung ist eine bekannte Komplikation genau dieser Krankheit.")]),
            P(&[T("Trifft das zu, ist ein Abführmittel durch den Mund nicht nur wirkungslos, sondern falsch: Man drückt Flüssigkeit gegen einen Engpass. Dann hilft auch der Wechsel auf Macrogol nichts. Dazu kommen die Aspirationsgefahr beim Erbrechen und der Flüssigkeitsverlust – der trifft ausgerechnet die Niere, das Organ, das hier ohnehin gefährdet ist.")]),
            P(&[B("Zwei Fragen gehören heute geklärt, nicht nächste Woche.")]),
            P(&[B("Wofür ist das Abführmittel gedacht?"), T(" Ist es die Vorbereitung auf die Darmspiegelung, ist Erbrechen unter der Spülung zwar häufig – aber mit Galle und Aufstossen gehört die Vorbereitung abgebrochen und die Ärztin informiert, nicht durchgezogen. Ist es gegen gewöhnliche Verstopfung, gilt dieselbe Abklärungspflicht.")]),
            P(&[B("Gehen Winde und Stuhl noch ab?"), T(" Wenn nicht, ist das der Notfall und nicht die Sprechstunde.")]),
        ],
    },

    H2("Die Adressen in Zürich"),
    P(&[T("Eine eigene Sprechstunde für die Purpura Schönlein-Henoch gibt es nicht. Zuständig sind zwei Fächer gemeinsam – Rheumatologie für die Vaskulitis, Nephrologie für die Niere. Bei einer Darmblutung oder einer Passagestörung kommt die Gastroenterologie dazu.")]),
    H3("Rheumatologie"),
    Adresse {
        name: "Vaskulitis-Sprechstunde, Klinik für Rheumatologie, Universitätsspital Zürich",
        rolle: &[T("Leitung: PD Dr. med. Carmen-Marina Mihai")],
        zeilen: &[
            &[T("Post: Universitätsspital Zürich, Disposition, Klinik für Rheumatologie, Rämistrasse 100, 8091 Zürich")],
            &[T("Sprechzimmer: G5, Empfang J, The Circle 59, 8058 Zürich-Flughafen")],
        ],
        links: &[
            Verweis { text: "Anmeldung Disposition Rheumatologie: +41 44 255 26 87", url: "tel:+41442552687" },
            Verweis { text: "dispo.ruz@usz.ch", url: "mailto:dispo.ruz@usz.ch" },
            Verweis { text: "usz.ch/sprechstunde/vaskulitis", url: "https://www.usz.ch/sprechstunde/vaskulitis/" },
        ],
    },
    H3("Nephrologie"),
    Adresse {
        name: "Dr. med. Stephanie Damm, Oberärztin",
        rolle: &[T("Klinik für Nephrologie USZ · Schwerpunkt Glomerulonephritis und Vaskulitis, Nierenbeteiligung bei rheumatischen Erkrankungen. Für diesen Fall die fachlich genaueste Adresse.")],
        zeilen: &[&[T("Rämistrasse 100, 8091 Zürich")]],
        links: &[
            Verweis { text: "+41 44 255 33 84", url: "tel:+41442553384" },
            Verweis { text: "stephanie.damm@usz.ch", url: "mailto:stephanie.damm@usz.ch" },
            Verweis { text: "usz.ch/fachbereich/nephrologie/team", url: "https://www.usz.ch/fachbereich/nephrologie/team/" },
        ],
    },
    Adresse {
        name: "Prof. Dr. med. Britta George, Klinikdirektorin",
        rolle: &[T("Klinik für Nephrologie USZ · Schwerpunkt glomeruläre Erkrankungen, Nierentransplantation, genetische Nierenerkrankungen")],
        zeilen: &[],
        links: &[
            Verweis { text: "+41 44 255 33 84", url: "tel:+41442553384" },
            Verweis { text: "britta.george@usz.ch", url: "mailto:britta.george@usz.ch" },
        ],
    },
    H2("Was zum Termin mitgehört"),
    Liste(&[
        &[T("Alle Hämoglobinwerte mit Datum – die Kurve sagt mehr als der letzte Punkt")],
        &[T("Ein Verlaufsblatt zum Bauch: seit wann kein Appetit, seit wann kein Stuhlgang, wie oft Erbrechen und in welchem Abstand zum Essen, wie viel getrunken wird")],
        &[T("Das Gewicht, wenn möglich mit einem früheren Wert zum Vergleich")],
        &[T("Die vollständige Medikamentenliste, rezeptfreie Schmerz- und Abführmittel eingeschlossen – mit dem Namen des Abführmittels auf der Packung")],
        &[T("Fotos des Ausschlags mit Datum. Purpura heilt oft schneller ab, als ein Termin zustande kommt.")],
        &[T("Angaben zum früheren Schub: wann, wie lange, was wurde gemacht, wurde der Urin kontrolliert, gab es eine Biopsie")],
        &[T("Bisherige Urin- und Nierenwerte")],
    ]),

    H2("Fragen, die sich lohnen"),
    Liste(&[
        &[T("Seit vier bis fünf Wochen kein Stuhlgang und Erbrechen nach jedem Essen – ist ein Darmverschluss ausgeschlossen, und mit welcher Untersuchung?")],
        &[T("Ist eine Computertomografie des Bauches geplant, und wann?")],
        &[T("Wie wird die Flüssigkeitszufuhr sichergestellt, wenn kaum getrunken wird und Erbrechen dazukommt?")],
        &[T("Nach Wochen ohne Nahrung: Wie wird der Kostaufbau begleitet, und werden Phosphat, Kalium und Magnesium dabei kontrolliert?")],
        &[T("Sie bekommt Novalgin: Wird darunter das Differentialblutbild kontrolliert, und sind Knochenmarkserkrankung und frühere Agranulozytose ausgeschlossen?")],
        &[T("Nimmt sie Methotrexat, Clozapin oder Carbamazepin? Die ersten beiden sind zusammen mit Novalgin kontraindiziert, beim dritten steigt das Agranulozytoserisiko.")],
        &[T("Falls Aspirin zum Herzschutz läuft: Wird es 30 bis 60 Minuten vor Novalgin gegeben, und ist unter Kortison ein Magenschutz verordnet?")],
        &[T("Kann der Kreatininwert durch Metamizol verfälscht sein – und wie wird die Nierenfunktion dann beurteilt?")],
        &[T("Wofür ist das Abführmittel verordnet – gegen Verstopfung oder als Vorbereitung der Darmspiegelung? Und ist es in dieser Lage überhaupt zulässig?")],
        &[T("Kann statt eines aromatisierten Präparats Movicol neutral oder Laxipeg aromafrei verschrieben werden – ohne Aroma und ohne Süssstoff?")],
        &[T("Ist eine Magen- und Darmspiegelung geplant – und wann?")],
        &[T("Wie hoch ist der Eiweissverlust im Urin, gemessen als Protein-Kreatinin-Quotient?")],
        &[T("Wurde ANCA bestimmt, also die im Alter häufigere Vaskulitisform ausgeschlossen?")],
        &[T("Wurde eine Hautbiopsie gemacht, solange frische Flecken vorhanden waren?")],
        &[T("Ist bei diesem Verlauf eine Nierenbiopsie angezeigt?")],
        &[T("Welche der aktuellen Medikamente belasten Magen oder Niere?")],
        &[T("Wie oft und über welchen Zeitraum wird der Urin kontrolliert?")],
        &[T("Ist bei einer Erstmanifestation in diesem Alter eine Tumorsuche vorgesehen?")],
        &[T("Wurde nach dem früheren Ausschlag an den Beinen der Urin kontrolliert – und mit welchem Ergebnis?")],
        &[T("Ging ein Infekt voraus – und welche Medikamente sind in den letzten Wochen neu dazugekommen?")],
    ]),

];

// ---------------------------------------------------------------------------
// Quellen
// ---------------------------------------------------------------------------
//
// Die Arzneimittelangaben stammen aus den Fachinformationen auf ch.oddb.org
// (Open Drug Database, ywesee GmbH). Bewusst nicht compendium.ch: dieselbe
// Fachinfo steht auf ch.oddb.org und ist dort frei zugaenglich.

pub static QUELLEN: &[(&str, Verweis)] = &[
    ("Vaskulitis-Sprechstunde, Universitätsspital Zürich",
     Verweis { text: "https://www.usz.ch/sprechstunde/vaskulitis/", url: "https://www.usz.ch/sprechstunde/vaskulitis/" }),
    ("Team der Klinik für Nephrologie, Universitätsspital Zürich",
     Verweis { text: "https://www.usz.ch/fachbereich/nephrologie/team/", url: "https://www.usz.ch/fachbereich/nephrologie/team/" }),
    ("Notfall, Universitätsspital Zürich",
     Verweis { text: "https://www.usz.ch/notfall/", url: "https://www.usz.ch/notfall/" }),
    ("Immunoglobulin A (IgA) Vasculitis in the Elderly. PMC9978861",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9978861/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9978861/" }),
    ("Shimamura et al.: Gastrointestinal bleeding is associated with renal prognosis in adult patients with IgA vasculitis with nephritis. J Gen Fam Med 2020",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6942937/", url: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6942937/" }),
    ("Toker et al.: IgA vasculitis and malignancy – a systematic review. J Eur Acad Dermatol Venereol 2024",
     Verweis { text: "https://onlinelibrary.wiley.com/doi/10.1111/jdv.19411", url: "https://onlinelibrary.wiley.com/doi/10.1111/jdv.19411" }),
    ("The association between adult IgA vasculitis and cancer – a prospective observational study. PMC11893613",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC11893613/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC11893613/" }),
    ("IgA-Vaskulitis im Erwachsenenalter. Medical Tribune Schweiz",
     Verweis { text: "https://medical-tribune.ch/news/medizin/4000106652/iga-vaskulitis-im-erwachsenenalter/", url: "https://medical-tribune.ch/news/medizin/4000106652/iga-vaskulitis-im-erwachsenenalter/" }),
    ("IgA Vasculitis (Henoch-Schönlein Purpura). StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK537252/", url: "https://www.ncbi.nlm.nih.gov/books/NBK537252/" }),
    ("Pathogenesis of IgA Vasculitis: An Up-To-Date Review. Front Immunol 2021",
     Verweis { text: "https://www.frontiersin.org/journals/immunology/articles/10.3389/fimmu.2021.771619/full", url: "https://www.frontiersin.org/journals/immunology/articles/10.3389/fimmu.2021.771619/full" }),
    ("IgA vasculitis: refractory and relapsing disease course in the adult population. Clin Kidney J 2021",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/34345419/", url: "https://pubmed.ncbi.nlm.nih.gov/34345419/" }),
    ("Predictive factors of relapse in adult Henoch-Schönlein purpura",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/22441366/", url: "https://pubmed.ncbi.nlm.nih.gov/22441366/" }),
    ("Causal Attributions about Disease-Onset and Relapse in Patients with Systemic Vasculitis. PMC4008683",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC4008683/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC4008683/" }),
    ("Zum Bild der Passagestörung: Small Bowel Obstruction. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK448079/", url: "https://www.ncbi.nlm.nih.gov/books/NBK448079/" }),
    ("Zur Infusion: Crystalloid Solutions in Intravenous Fluid Therapy. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK500033/", url: "https://www.ncbi.nlm.nih.gov/books/NBK500033/" }),
    ("Zur Verdünnung des Hämoglobinwerts: Acute Anemia. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK537232/", url: "https://www.ncbi.nlm.nih.gov/books/NBK537232/" }),
    ("Zum Kostaufbau nach langem Fasten: Refeeding Syndrome. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK564513/", url: "https://www.ncbi.nlm.nih.gov/books/NBK564513/" }),
    ("Zum nachlassenden Durstgefühl im Alter: Adult Dehydration. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK555956/", url: "https://www.ncbi.nlm.nih.gov/books/NBK555956/" }),
    ("Lee-Robichaud H et al.: Lactulose versus Polyethylene Glycol for Chronic Constipation. Cochrane Database Syst Rev 2010; CD007570",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/20614462/", url: "https://pubmed.ncbi.nlm.nih.gov/20614462/" }),
    ("SDIF – Swiss Drug Interaction Finder, Interaktionscheck auf Basis der Schweizer Fachinformationen und EPha",
     Verweis { text: "https://sdif.oddb.org/?tab=check", url: "https://sdif.oddb.org/?tab=check" }),
    ("EPha.ch – die abgestuften Interaktionsdaten hinter dem Check",
     Verweis { text: "https://epha.ch", url: "https://epha.ch" }),
    ("Fachinformation Novalgin Tropfen (Metamizol), ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/16952", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/16952" }),
    ("Fachinformation Novalgin-F Tabletten (Metamizol), ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/16951", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/16951" }),
    ("Metamizole-induced agranulocytosis (MIA): a mini review. PMC10435429",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC10435429/", url: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC10435429/" }),
    ("Zum zeitlichen Verlauf: Metamizole-associated agranulocytosis, Analyse deutscher Spontanmeldungen 1990–2012",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/26169297/", url: "https://pubmed.ncbi.nlm.nih.gov/26169297/" }),
    ("Fachinformation Movicol / Movicol neutral / Movicol Chocolat, ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/58420", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/58420" }),
    ("Fachinformation Laxipeg banane / Laxipeg aromafrei, ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/62765", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/62765" }),
    ("Fachinformation Transipeg / Transipeg forte, ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/53282", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/53282" }),
    ("Fachinformation Duphalac (Lactulose), ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/32894", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/32894" }),
    ("Fachinformation Gatinar (Lactulose), ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/37585", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/37585" }),
    ("Fachinformation Rudolac (Lactulose), ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/51067", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/51067" }),
    ("Fachinformation Importal (Lactitol), ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/52785", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/52785" }),
    ("Fachinformation Moviprep, ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/57900", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/57900" }),
    ("Fachinformation Picoprep, ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/62754", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/62754" }),
    ("Fachinformation Cololyt, ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/48205", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/48205" }),
    ("Alle Präparate im Direktvergleich: Macrogol-Kombinationen (A06AD65) auf ch.oddb.org",
     Verweis { text: "https://ch.oddb.org/de/gcc/search/zone/drugs/search_query/movicol", url: "https://ch.oddb.org/de/gcc/search/zone/drugs/search_query/movicol" }),
    ("Zur Geschichte: Johann Lukas Schönlein, Deutsche Biographie",
     Verweis { text: "https://www.deutsche-biographie.de/sfz79016.html", url: "https://www.deutsche-biographie.de/sfz79016.html" }),
    ("Schönlein's Allgemeine und specielle Pathologie und Therapie, 1837 (Digitalisat)",
     Verweis { text: "https://archive.org/details/drjlschnleinsall01schn", url: "https://archive.org/details/drjlschnleinsall01schn" }),
    ("Gairdner D.: The Schönlein-Henoch syndrome (anaphylactoid purpura). Q J Med 1948; 17: 95-122",
     Verweis { text: "https://academic.oup.com/qjmed/article-abstract/17/2/95/1534443", url: "https://academic.oup.com/qjmed/article-abstract/17/2/95/1534443" }),
    ("Osler W.: The visceral lesions of purpura and allied conditions. BMJ 1914",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/20767027/", url: "https://pubmed.ncbi.nlm.nih.gov/20767027/" }),
    ("Berger J., Hinglais N.: Les dépôts intercapillaires d'IgA-IgG. J Urol Nephrol 1968 – Übersicht",
     Verweis { text: "https://www.ajkd.org/article/S0272-6386(11)00809-2/fulltext", url: "https://www.ajkd.org/article/S0272-6386(11)00809-2/fulltext" }),
    ("Jennette JC et al.: 2012 Revised International Chapel Hill Consensus Conference Nomenclature of Vasculitides. Arthritis Rheum 2013",
     Verweis { text: "https://onlinelibrary.wiley.com/doi/10.1002/art.37715", url: "https://onlinelibrary.wiley.com/doi/10.1002/art.37715" }),
];
