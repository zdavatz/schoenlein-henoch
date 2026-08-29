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
    /// Verlinktes Wort mitten im Satz: Anzeigetext und Ziel. Im PDF liegt die
    /// Klickflaeche hinter dem Wort, erkennbar an der Unterstreichung.
    L(&'static str, &'static str),
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
    "Informationsblatt für die Patientin und ihre Angehörigen · Stand 29. August 2026 · zum Mitnehmen zum Arzttermin";
pub const KOPFZEILE: &str = "IgA-Vaskulitis (Purpura Schönlein-Henoch)";


use Block::*;
use Span::{B, I, L, N, T};

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
            &[B("Erbrechen, jedes Mal nachdem eine Kleinigkeit gegessen wurde – inzwischen bleibt wieder etwas drin")],
            &[T(
                "Das ist der wichtigste Punkt der Liste. Erbrechen kurz nach dem Essen heisst, \
                 dass der Mageninhalt nicht weiterkommt. Erbrechen gehört zu den Leitzeichen \
                 einer Passagestörung; bei weiter unten liegendem Hindernis wird es gallig. Dass Haferflocken jetzt wieder drinbleiben, ist die zweite grosse Entlastung nach dem Stuhlgang – und die Magenentzündung erklärt rückblickend, warum es vorher jedes Mal kam.",
            )],
        ],
        &[
            &[B("Sehr wenig getrunken, keine Lust zu trinken – inzwischen wieder Tee")],
            &[T(
                "Im Alter lässt das Durstgefühl nach – Austrocknung ist dann die Regel und nicht \
                 die Ausnahme. Sie verschlechtert die Nierenfunktion, verstärkt die Verstopfung \
                 und macht schwindlig. Und sie nimmt jedem Abführmittel die Grundlage: Diese \
                 Mittel brauchen Wasser, um zu wirken.",
            )],
        ],
        &[
            &[B("Der Magen ist gebläht, sie stösst Luft auf bis zum Würgen – auch unter Pantoprazol")],
            &[T(
                "Das ist der Punkt, an dem der Säureblocker die Antwort schuldig bleibt. Ein \
                 Völlegefühl mit Aufstossen, das unter voller Säurehemmung fortbesteht, hält die \
                 Frage offen, ob der Mageninhalt weiterkommt – dieselbe Frage wie beim Erbrechen, \
                 nur leiser gestellt. Sie gehört zu den Gründen, die Spiegelung nicht weiter zu \
                 verschieben.",
            )],
        ],
        &[
            &[B("Eine Woche gar kein Stuhlgang – inzwischen wieder in Gang")],
            &[T(
                "Ausbleibender Stuhl gehört zusammen mit Erbrechen und Bauchschmerz zum \
                 klassischen Bild des Darmverschlusses. Ausbleibender Stuhl ",
            ), B("und"), T(
                " ausbleibender Windabgang ist der Notfall, nicht die Sprechstunde. Dass die \
                 Passage wieder offen ist, entschärft genau diesen Punkt – siehe den Abschnitt \
                 «Was sich gebessert hat». Wie er aussieht, ist allerdings ein eigener Befund: \
                 schwarz und flüssig, mit weissem Schleim dazwischen – siehe den Abschnitt \
                 «Dunkler Stuhl: Galle oder Blut?».",
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
            &[T("Ferritin "), B("und"), T(" Transferrinsättigung, immer zusammen mit CRP. Ferritin steigt bei Entzündung an – ein normaler Wert schliesst einen Eisenmangel dann nicht aus. Wird Eisen verordnet, gehört die Frage nach dem Weg dazu: Unter einem Protonenpumpenhemmer ist die Aufnahme über den Mund unsicher – siehe den Abschnitt zu Pantoprazol.")],
        ],
        &[
            &[B("Niere")],
            &[T("Kreatinin, eGFR, Albumin im Blut – der Verlauf, nicht nur der Einzelwert. Dazu der "), B("Harnstoff aus derselben Entnahme"), T(": Wie er zum Kreatinin steht, ist ein Hinweis darauf, ob eine Blutungsquelle oben liegt – siehe den Abschnitt «Dunkler Stuhl: Galle oder Blut?».")],
        ],
        &[
            &[B("Harnwege")],
            &[T("Ultraschall der Nieren und ableitenden Harnwege, "), B("wiederholt"), T(" – er zeigt den Stau ohne Kontrastmittel und ohne Strahlung. Dazu, wegen sichtbaren Bluts im Urin und des Alters, die urologische Abklärung mit Blasenspiegelung und Bildgebung der oberen Harnwege; die Computertomografie des Bauches lässt sich als CT-Urografie fahren und beantwortet dann beide Fragen auf einmal.")],
        ],
        &[
            &[B("Blutzerfall")],
            &[T("LDH, Haptoglobin, Bilirubin – trennt einen Zerfall der roten Blutkörperchen vom Blutverlust nach aussen.")],
        ],
        &[
            &[B("Urin")],
            &[T("Status und Sediment (Erythrozyten, "), B("Akanthozyten"), T(", Erythrozytenzylinder) sowie "), B("Protein-Kreatinin-Quotient im Spoturin"), T(". Die wichtigste wiederholte Kontrolle überhaupt – und jetzt, wo Blut im Urin ist, die Untersuchung, die entscheidet, ob es aus der Niere oder aus den ableitenden Harnwegen kommt. Ein Streifentest allein beantwortet das nicht.")],
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
            &[T("Gewichtsverlauf, Albumin, Phosphat, Kalium, Magnesium. Nach Wochen ohne Nahrung sind diese Werte die Voraussetzung dafür, dass der Kostaufbau sicher beginnen kann. Magnesium steht unter Pantoprazol ein zweites Mal auf dieser Liste.")],
        ],
        &[
            &[B("Nach der Transfusion")],
            &[T("Hämoglobin am Tag danach. Eine Einheit hebt den Wert um rund "), N("10 g/l"), T("; bleibt der Anstieg aus oder fällt der Wert wieder, läuft die Blutung weiter. Dazu die Flüssigkeitsbilanz, weil die Infusion daneben weiterläuft.")],
        ],
        &[
            &[B("Bauch, dringlich")],
            &[T("Computertomografie des Bauches bei starken Bauchschmerzen, Erbrechen und ausbleibendem Stuhl – sie zeigt Wandschwellung, Passagestörung, Durchblutungsstörung und Komplikationen. Dieser Schritt kommt "), B("vor"), T(" jedem Abführmittel durch den Mund.")],
        ],
        &[
            &[B("Darm")],
            &[T("Magen- und Darmspiegelung. Beantwortet Blutungsquelle und Tumorfrage gemeinsam – deshalb der ergiebigste einzelne Schritt, sobald eine Passagestörung ausgeschlossen ist. Seit der Stuhl schwarz und flüssig ist, hat dieser Schritt eine Frist: Bei akuter Blutung im oberen Verdauungstrakt nennen beide Fachgesellschaften "), B("24 Stunden"), T(". Bei der Magenspiegelung gehören zwei Dinge dazu: Gewebeproben aus dem "), B("absteigenden Zwölffingerdarm"), T(", wo die Vaskulitis sitzt, und der Test auf Helicobacter pylori aus denselben Proben. Bei der Darmspiegelung muss das Gerät bis in den "), B("Krummdarm"), T("; dort ist der Befall am häufigsten. Die Darmspiegelung hat inzwischen stattgefunden und war unauffällig – siehe den eigenen Abschnitt dazu, samt den zwei Rückfragen, die trotzdem gehören.")],
        ],
        &[
            &[B("Dünndarm")],
&[T("Erst wenn oben nichts gefunden wird und es weiterblutet. Dann ist der Dünndarm dazwischen die verbleibende Strecke: Kapselendoskopie als erster Schritt, bei unauffälligem Befund die Computertomografie als Enterografie. Ob die Kapsel gefahrlos ist, hängt nicht am Transport, sondern daran, ob es eine Enge gibt – das beantwortet die Computertomografie mit.")],
        ],
        &[
            &[B("Schilddrüse")],
            &[T("TSH. Eine Unterfunktion kann einen Darmstillstand nachahmen und ist behandelbar – ein Wert aus der ohnehin laufenden Entnahme.")],
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
            &[L("Movicol neutral", "https://ch.oddb.org/de/gcc/fachinfo/reg/58420/chapter/composition"), T(", "), L("Movicol Junior neutral", "https://ch.oddb.org/de/gcc/fachinfo/reg/58420/chapter/composition")],
            &[T("Macrogol 3350 mit Elektrolyten")],
            &[B("Keines."), T(" Für die neutrale Variante nennt die Fachinformation weder ein Aroma noch einen Süssstoff.")],
        ],
        &[
            &[L("Laxipeg aromafrei", "https://ch.oddb.org/de/gcc/fachinfo/reg/62765/chapter/composition")],
            &[T("Macrogol 4000")],
            &[B("Hilfsstoffe: keine."), T(" Wörtlich so in der Fachinformation.")],
        ],
        &[
            &[L("Movicol", "https://ch.oddb.org/de/gcc/fachinfo/reg/58420/chapter/composition")],
            &[T("Macrogol 3350 mit Elektrolyten")],
            &[T("Acesulfam-Kalium (E950), Limetten- und Zitronenaroma")],
        ],
        &[
            &[L("Movicol Chocolat", "https://ch.oddb.org/de/gcc/fachinfo/reg/58420/chapter/composition")],
            &[T("Macrogol 3350 mit Elektrolyten")],
            &[T("Acesulfam-Kalium (E950), Schokoladenaroma, dazu Benzylalkohol im Aroma")],
        ],
        &[
            &[L("Transipeg, Transipeg forte", "https://ch.oddb.org/de/gcc/fachinfo/reg/53282/chapter/composition")],
            &[T("Macrogol 3350 mit Elektrolyten")],
            &[T("Aspartam (E951), Acesulfam-Kalium")],
        ],
        &[
            &[L("Laxipeg banane", "https://ch.oddb.org/de/gcc/fachinfo/reg/62765/chapter/composition")],
            &[T("Macrogol 4000")],
            &[T("Acesulfam-Kalium (E950), Bananenaroma")],
        ],
        &[
            &[L("Duphalac", "https://ch.oddb.org/de/gcc/fachinfo/reg/32894/chapter/composition"), T(", "), L("Gatinar", "https://ch.oddb.org/de/gcc/fachinfo/reg/37585/chapter/composition"), T(", "), L("Rudolac", "https://ch.oddb.org/de/gcc/fachinfo/reg/51067/chapter/composition")],
            &[T("Lactulose-Sirup")],
            &[T("Kein Zusatz nötig – der Wirkstoff selbst ist ein Zucker. Duphalac nennt unter Hilfsstoffen: keine.")],
        ],
        &[
            &[L("Importal", "https://ch.oddb.org/de/gcc/fachinfo/reg/52785/chapter/composition")],
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
            &[L("Moviprep, Moviprep Orange", "https://ch.oddb.org/de/gcc/fachinfo/reg/57900/chapter/composition")],
            &[T("Macrogol 3350, Natriumsulfat, Ascorbat")],
            &[T("Aspartam (E951), 0,233 g je Beutel, Acesulfam-Kalium, Zitronenaroma – dazu literweise zu trinken")],
        ],
        &[
            &[L("Picoprep, CitraFleet", "https://ch.oddb.org/de/gcc/fachinfo/reg/62754/chapter/composition")],
            &[T("Natriumpicosulfat, Magnesiumoxid, Citronensäure")],
            &[T("Saccharin-Natrium (E954), Orangenaroma – wirkt zusätzlich stimulierend auf die Darmbewegung")],
        ],
        &[
            &[T("Plenvu, Clensia, "), L("Cololyt", "https://ch.oddb.org/de/gcc/fachinfo/reg/48205/chapter/composition")],
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


static T_FAELLE: Tabelle = Tabelle {
    kopf: &["Alter", "Bild", "Verlauf"],
    gewichte: &[14, 48, 38],
    zeilen: &[
        &[
            &[N("80 J")],
            &[T("Basel, nach Pneumonie: Fieber, Gelenk- und Bauchschmerz, Petechien, Makrohämaturie. Hautbiopsie mit IgA.")],
            &[T("Supportiv, nach Infektsanierung rückläufig. "), L("Rüdiger 2002", "https://doi.org/10.1055/s-2002-32350")],
        ],
        &[
            &[N("80 J")],
            &[T("Japan: Purpura, Darm, milde Nephritis, 10 Prozent Halbmonde.")],
            &[T("Prednisolon 0,7 mg/kg, nach 6 Monaten Proteinurie weg. "), L("Ueda 2019", "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6367074/")],
        ],
        &[
            &[N("85 J")],
            &[T("Japan, dieselbe Serie: nephrotisches Syndrom, 100 Prozent Halbmonde.")],
            &[T("Steroide, dann CMV, Pneumonie, Sepsis, Tod. "), L("Ueda 2019", "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6367074/")],
        ],
        &[
            &[N("86 J")],
            &[T("Brasilien, nach Atemwegsinfekt: Purpura, Kreatinin 3,1, Proteinurie 4,5 g/d.")],
            &[T("Ohne Immunsuppression gebessert. "), L("Sande Miguel 2017", "https://doi.org/10.4236/jbm.2017.56004")],
        ],
        &[
            &[N("90 J")],
            &[T("Japan: Purpura, Aszites, Dünndarmödem, milde Niere.")],
            &[T("Rezidiv unter 3 mg Prednisolon, dann Azathioprin, stabil. "), L("Nishikura 2022", "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC9549259/")],
        ],
        &[
            &[N("93 J")],
            &[T("USA, nach Bronchitis und Levofloxacin: rasches Nierenversagen.")],
            &[T("Hochdosis-Steroide, Dialyse. "), L("Kukrety 2016", "https://pmc.ncbi.nlm.nih.gov/articles/PMC5031831/")],
        ],
        &[
            &[N("97 J")],
            &[T("Japan: Bauch, Purpura, Gelenke, Blut und Eiweiss im Urin.")],
            &[T("Prednisolon 0,5 mg/kg, kein Rezidiv. "), L("Kamiya 2015", "https://doi.org/10.1111/1346-8138.12747")],
        ],
    ],
    linien: true,
    chronik: false,
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
                B("Das Tempo."), T(" Der Abfall um "), N("25 g/l"), T(" war der eigentliche Befund – und er ist nicht stehengeblieben. Eine Blutarmut durch Entzündung oder Nierenschwäche entwickelt sich über Monate. Ein Abfall innert Wochen spricht für Blutverlust; ein Abfall, der weitergeht, spricht für einen Blutverlust, der weitergeht. Bei dieser Krankheit in erster Linie aus dem Darm. Das gehört rasch abgeklärt und nicht in eine Sprechstunde in sechs Wochen."),
            ]),
            P(&[
                B("Der Bauch."), T(" Seit Wochen kein Appetit, starke Bauchschmerzen, wenig Schlaf, immer wieder Erbrechen – jedes Mal, nachdem eine Kleinigkeit gegessen wurde –, kaum Flüssigkeit, und eine Woche lang gar kein Stuhlgang. Diese Kombination ist keine gewöhnliche Verstopfung; sie ist das Bild einer Passagestörung. Siehe den Abschnitt «Was seit Wochen läuft»."),
            ]),
            P(&[
                B("Und ein Vorlauf."), T(" Es gab schon einmal einen massiven Ausschlag an den Beinen, nach einer Phase starker Belastung. Damit ist das heutige Bild wahrscheinlich kein Erstereignis – siehe den Abschnitt «Ein früherer Schub»."),
            ]),
            P(&[
                B("Und der Stand heute."), T(" Der Stuhlgang ist wieder in Gang und der Ausschlag an den Beinen geht zurück. Zwei echte Besserungen – der Hämoglobinabfall ist damit aber nicht erklärt. Siehe den Abschnitt «Was sich gebessert hat»."),
            ]),
            P(&[
                B("Und was seither dazugekommen ist."), T(" Der Stuhl ist "), B("schwarz und flüssig"), T(", mit weissem Schleim dazwischen. Und schwarz erbrochen: Kaffeesatz. Die Darmspiegelung hat nichts gefunden – ausser, dass der Darm nicht richtig transportiert. Das Hämoglobin ist nochmals gefallen. Blut im Urin, mit blossem Auge sichtbar. Ein Harnleiter, der an mehreren Stellen gestaut ist. Ein Magen, der weiter gebläht ist und Luft aufstossen lässt, bis sie würgt; der Gastroenterologe spricht von fehlender Peristaltik. Ödeme an den Füssen, die es beim früheren Schub schon gab. Und deshalb eine laufende Bluttransfusion. Damit ist der Hämoglobinabfall nicht erklärt, sondern nur ersetzt – die Frage nach der Quelle steht unverändert. Siehe die Abschnitte «Blut im Urin», «Der gestaute Harnleiter» und «Die Transfusion»."),
            ]),
        ],
    },
    H2("Was seit Wochen läuft"),
    P(&[T("Was die Patientin in den vergangenen Wochen erlebt hat, gehört zusammen auf ein Blatt. Einzeln klingt jeder Punkt nach einer Unannehmlichkeit; zusammen ergeben sie etwas anderes. Diese Liste ist die Vorgeschichte, mit der man zum Termin geht; was sich davon inzwischen gebessert hat, steht im Abschnitt danach.")]),
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

    H2("Was sich gebessert hat"),
    P(&[T("Zwei Dinge haben sich geändert, und beide sind echte Besserung: Der Stuhlgang ist wieder in Gang, und der Ausschlag an den Beinen geht zurück. Was jede der beiden Meldungen beantwortet – und was sie ausdrücklich nicht beantwortet – steht hier. Die dritte Meldung, der unauffällige Urin, stand bis vor kurzem ebenfalls an dieser Stelle; sie ist überholt und hat einen eigenen Abschnitt bekommen. Weitere Änderungen ebenso, weil mehr an ihnen hängt: Sie isst und trinkt wieder, der Bauchbefund hat einen Namen bekommen, und sie bekommt Blut.")]),

    H3("Der Stuhlgang ist wieder in Gang"),
    P(&[T("Das ist die wichtigste der drei Meldungen. Der Verdacht, der über allem stand – Passagestörung, im schlimmsten Fall Darmverschluss –, ist damit entschärft: Was durchgeht, ist nicht verschlossen. Der Notfallgrund fällt weg.")]),
    P(&[T("Was nicht wegfällt, ist die Frage, warum die Passage eine Woche lang stand. Die beiden Erklärungen, die dafür auf dem Tisch lagen, liegen weiter dort – das Darmwandödem der Vaskulitis und ein Hindernis, das mit 84 Jahren ohnehin gesucht gehört. Beide beantwortet dieselbe Untersuchung, und sie ist jetzt leichter durchzuführen als vorher: Solange die Passage stand, verbot sich die Darmspiegelung; jetzt verbietet sie sich nicht mehr. Der Zeitpunkt dafür ist nicht «wenn es wieder schlechter wird», sondern jetzt, solange es gut geht.")]),

    H3("Der Ausschlag geht zurück"),
    P(&[T("Gut für die Haut – und die Haut ist der harmloseste der vier Bereiche. Daraus folgen zwei Dinge, und beide zeigen in dieselbe Richtung: Was jetzt nicht getan wird, lässt sich später nicht nachholen.")]),
    P(&[B("Das Fenster für die Hautbiopsie schliesst sich."), T(" Der Beweis der Diagnose ist der IgA-Nachweis in der Gefässwand, und er gelingt nur an einer frischen Läsion, jünger als etwa 48 Stunden; an abgeheilten Flecken findet sich nichts mehr. Wer die Diagnose sichern will, hat dafür so lange Zeit, wie noch frische Flecken nachkommen – danach nicht mehr. Dasselbe gilt für Fotos mit Datum, und die kosten nichts.")]),
    P(&[B("Die Urinkontrolle fängt jetzt erst an."), T(" Der häufigste Fehler bei dieser Krankheit ist, die Kontrollen zu beenden, sobald die Haut wieder sauber aussieht. Die Nierenbeteiligung "), L("entwickelt sich", "https://www.aafp.org/pubs/afp/issues/2020/0815/p229.html"), T(" typischerweise ein bis drei Monate nach dem Ausschlag und kann sich bis zu sechs Monate verzögern; 91 Prozent treten innerhalb von sechs Wochen auf, 97 Prozent innerhalb von sechs Monaten. Der abheilende Ausschlag ist deshalb kein Grund, mit dem Urin aufzuhören – er markiert den Beginn der Zeit, in der hingeschaut werden muss.")]),

    H3("Der Urin – diese Meldung ist überholt"),
    P(&[T("Hier stand die beste Nachricht des Blattes: Das Organ, das über den Verlauf entscheidet, zeige im Moment nichts. Das gilt nicht mehr. Inzwischen ist Blut im Urin, sichtbar, und der Harnleiter ist an mehreren Stellen gestaut. Beides hat weiter unten einen eigenen Abschnitt.")]),
    P(&[T("Zwei Sätze aus der alten Fassung bleiben trotzdem stehen, weil sie sich rückblickend als die richtigen erwiesen haben. Der erste: "), B("Ein unauffälliger Streifentest ist nicht dasselbe wie ein unauffälliger Urin."), T(" Der Streifen misst eine Konzentration, und deshalb hängt sein Ergebnis daran, wie verdünnt der Urin ist – unter laufender Infusion ist er verdünnt. In einer Untersuchung an 2932 Urinproben wurde eine erhebliche Eiweissausscheidung in den verdünnten Proben vom Streifentest "), L("nicht erkannt", "https://pmc.ncbi.nlm.nih.gov/articles/PMC5063823/"), T(". Der Protein-Kreatinin-Quotient hat dieses Problem nicht: Er bezieht das Eiweiss auf das Kreatinin und ist von der Verdünnung unabhängig.")]),
    P(&[T("Der zweite: "), B("Ein normaler Befund ist eine Momentaufnahme."), T(" Urin und Blutdruck gehören über mindestens sechs Monate kontrolliert, ausdrücklich auch dann, wenn der erste Befund normal war. Genau so ist es gekommen. Der Wert des damaligen Ergebnisses lag nie darin, dass es die Frage schloss, sondern darin, dass es der erste Punkt einer Kurve war – und die Kurve hat sich inzwischen bewegt.")]),

    H3("Was offen bleibt: die 25 g/l"),
    P(&[T("Keine der drei Meldungen erklärt den Hämoglobinabfall. Der unauffällige Urin sagt etwas über die Niere und nichts über den Blutverlust – die langsame Blutarmut aus nachlassender Nierenfunktion passt zu diesem Tempo ohnehin nicht. Und die Auskunft, der dunkle Stuhl sei Galle gewesen, nimmt, falls sie zutrifft, die naheliegendste Quelle aus der Rechnung. Damit wird die Frage nicht kleiner, sondern dringlicher: Es fehlen "), N("25 g/l"), T(", und die wahrscheinlichste Erklärung wäre gerade ausgeschieden. Wie sich das ohne grossen Aufwand weiterverfolgen lässt, steht im Abschnitt «Dunkler Stuhl: Galle oder Blut?».")]),
    P(&[T("Der einfachste Schritt ist ein neuer Hämoglobinwert zusammen mit den Retikulozyten, jetzt, wo wieder getrunken wird und die Austrocknung den Wert nicht mehr schönt. Er sagt zweierlei: ob der Verlust steht oder weiterläuft, und ob das Knochenmark antwortet.")]),

    H2("Die neue Diagnose: starke Magenentzündung"),
    P(&[T("Damit steht ein Befund im Raum, der zwei Dinge auf einmal erklärt, die bisher unverbunden nebeneinanderlagen. Eine entzündete Magenschleimhaut erklärt das Erbrechen, das jedes Mal kam, sobald eine Kleinigkeit gegessen war. Und sie erklärt einen Teil der fehlenden "), N("25 g/l"), T(": Eine wunde Schleimhaut blutet nicht in einem Schwall, sie sickert – über Wochen, ohne dass es am Stuhl auffallen muss.")]),
    P(&[B("Nur ist «Magenentzündung» ein Befund und keine Ursache."), T(" Das Wort beschreibt, wie die Schleimhaut aussieht, nicht, was sie so aussehen lässt. Genau daran hängt aber die Behandlung. Drei Erklärungen stehen hier nebeneinander, und dieselbe Untersuchung trennt sie:")]),
    Liste(&[
        &[B("Die Vaskulitis selbst."), T(" Naheliegend – nur zeigt der Blick in die Zahlen etwas Unerwartetes. In einer "), L("Untersuchung an 108 Erwachsenen", "https://pmc.ncbi.nlm.nih.gov/articles/PMC11429444/"), T(" mit Bauchbeteiligung war bei den 61 Magenspiegelungen der Magen selbst nur zweimal betroffen, in 3,3 Prozent der Fälle – der Zwölffingerdarm dagegen 34-mal, in 55,7 Prozent, und dort vor allem der absteigende Teil und der Bulbus. Zu sehen waren Rötung mit Erosionen (48 Prozent) sowie Erosionen mit Geschwüren (31 Prozent). "), B("Daraus folgt etwas Praktisches:"), T(" Ist diese Entzündung die Vaskulitis, liegt ihr Beweis meist hinter dem Magenausgang. Das Gerät muss bis in den absteigenden Zwölffingerdarm, und die Gewebeproben gehören dorthin – nicht nur in den Magen.")],
        &[B("Die Medikamente."), T(" Kortison und niedrig dosiertes Aspirin schädigen die Magenschleimhaut additiv; genau diese Kombination meldet der Interaktionscheck als Klasse C, mit ausdrücklich steigendem Risiko im höheren Lebensalter. Dazu Novalgin, dessen Fachinformation "), L("gastrointestinale Blutungen, Ulzerationen und Perforationen", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/unwanted_effects"), T(" unter den unerwünschten Wirkungen führt. Drei Mittel, die am selben Ort angreifen – und alle drei liefen, bevor der Befund erhoben wurde.")],
        &[B("Helicobacter pylori."), T(" Aus denselben Gewebeproben in einem Schritt beantwortet. Ist das Bakterium da, ist die Behandlung eine andere: Die Fachinformation sieht dafür "), L("Pantoprazol zusammen mit zwei Antibiotika über sieben Tage", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/usage"), T(" vor. Ein Säureblocker allein heilt diese Form nicht aus.")],
    ]),
    P(&[B("Der wichtigste Satz zu dieser Diagnose steht in der Fachinformation des neuen Medikaments."), T(" Sie nennt "), L("Warnsymptome", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/restrictions"), T(", bei denen eine bösartige Erkrankung ausgeschlossen werden "), B("muss"), T(": unerklärlicher Gewichtsverlust, wiederholtes Erbrechen, Schluckstörung, Bluterbrechen, Blutarmut, Teerstuhl. Drei davon treffen hier sicher zu – Gewichtsverlust, wiederholtes Erbrechen, Blutarmut –, und der vierte ist genau die offene Frage aus dem Abschnitt «Dunkler Stuhl: Galle oder Blut?». Die Begründung steht gleich dahinter: Die Behandlung mit Pantoprazol kann die Symptome kaschieren und die Diagnosestellung dadurch verzögern.")]),
    P(&[T("Das kehrt eine naheliegende Erwartung um. Der Magenschutz macht die Spiegelung nicht weniger dringend, sondern dringender – er nimmt die Beschwerden weg, ohne die Frage zu beantworten, woher sie kamen. Wird es unter dem Mittel besser, ist das eine Erleichterung und kein Befund.")]),
    P(&[B("Inzwischen hat sich diese Frage beantwortet, und zwar in die unangenehme Richtung."), T(" Der Magen ist weiter gebläht, das Aufstossen hält an. Unter Pantoprazol ist es also nicht besser geworden. Damit entfällt die Erleichterung und der Befund bleibt stehen: Beschwerden, die unter voller Säurehemmung fortbestehen, sind mit der Säure nicht erklärt. Die Warnsymptome aus der Fachinformation gelten unverändert – und die Begründung, mit der sie dort stehen, greift jetzt doppelt. Ein Mittel, dem man zutraut, die Symptome zu kaschieren, hat sie nicht einmal kaschiert.")]),

    H2("Blut im Urin: warum das nicht aus dem Magen kommen kann"),
    P(&[B("Der Stand in zwei Sätzen."), T(" Das Blut im Stuhl ist geklärt: Kaffeesatz oben und Teerstuhl unten beweisen zusammen eine Blutung im oberen Verdauungstrakt, also aus Magen oder Zwölffingerdarm. Das Blut im Urin ist "), B("nicht"), T(" geklärt – es kann aus der Niere stammen, dann ist es die Nierenbeteiligung dieser Vaskulitis, oder aus Harnleiter und Blase, wofür der Stau an mehreren Stellen spricht. Eine einzige Untersuchung entscheidet das; sie steht weiter unten in diesem Abschnitt. Beides sind aber zwei getrennte Blutungen an zwei Orten, und keine erklärt die andere.")]),
    P(&[T("Die Frage liegt nahe, und ihre Antwort ist eindeutig: Blut aus dem Magen kann nicht in den Urin gelangen. Verdauungstrakt und Harnwege sind zwei getrennte Rohrsysteme; sie berühren einander nirgends. Was im Magen blutet, nimmt den Weg durch den Darm – und wird dort verdaut wie Nahrung. Genau davon handelt der Abschnitt «Dunkler Stuhl: Galle oder Blut?»: Das Bluteiweiss wird aufgenommen, und der Harnstoff im Blut steigt. Das ist die einzige Spur, die eine Blutung im Magen jenseits des Stuhls hinterlässt – Harnstoff im Blut, nicht Blut im Urin.")]),
    P(&[B("Es sind also zwei Blutungsorte und nicht einer."), T(" Das ist bei dieser Krankheit kein Zufall, sondern ihr Bauplan. Die IgA-Vaskulitis ist eine Entzündung der kleinen Gefässe, und die IgA-Ablagerungen sitzen "), L("in Magen-Darm-Trakt, Gelenken, Haut und Nieren", "https://www.aafp.org/pubs/afp/issues/2020/0815/p229.html"), T(" zugleich. Der Ausschlag an den Beinen, der Bauch und der Urin sind nicht drei Krankheiten, sondern derselbe Vorgang an drei Orten. Wer jetzt Blut im Urin sieht, sieht nichts Neues – er sieht das dritte Organ.")]),
    P(&[B("Deshalb spricht das Blut im Urin nicht gegen die Darmblutung, sondern für sie."), T(" Es zeigt, dass die Vaskulitis in diesem Moment aktiv ist, und eine aktive Vaskulitis ist genau die Erklärung, die eine Blutung im Zwölffingerdarm braucht. Dass beides zusammen die ungünstige Kombination ist, zeigt eine "), L("Untersuchung an 30 Erwachsenen", "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6942937/"), T(" mit IgA-Vaskulitis und Nierenbeteiligung: Von den zehn mit einer Magen-Darm-Blutung wurden 50 Prozent dialysepflichtig, von den zwanzig übrigen 5 Prozent. Die Zahl gehört mit Vorsicht gelesen – 30 Patienten sind wenig, und in der um Bluthochdruck und Eiweissausscheidung bereinigten Rechnung war der Zusammenhang nicht mehr statistisch gesichert. Als Richtungsangabe taugt sie trotzdem, und sie zeigt dorthin, wohin dieses Blatt an mehreren Stellen zeigt: zur Spiegelung.")]),
    P(&[B("Was sichtbares Blut bedeutet – und was nicht."), T(" Die Patientin berichtet eine Makrohämaturie, also mit blossem Auge sichtbares Blut. Das ist ein anderer Befund als die mikroskopische Hämaturie. Die Nierenbeteiligung dieser Krankheit zeigt sich typischerweise unter dem Mikroskop: "), L("mikroskopische Hämaturie, Erythrozytenzylinder, Eiweiss im Urin", "https://www.aafp.org/pubs/afp/issues/2020/0815/p229.html"), T(", bis hin zum offenen Nierenversagen. Sichtbares Blut kommt vor, ist aber zugleich der Befund, mit dem in den Fallberichten eine seltene Komplikation begonnen hat, die durch den gestauten Harnleiter neu im Raum steht – siehe den nächsten Abschnitt.")]),
    P(&[B("Eine einzige Untersuchung entscheidet, woher das Blut kommt."), T(" Ein Streifentest genügt dafür nicht; er wird auch dann positiv, wenn nur freier Blutfarbstoff im Urin ist, ohne rote Blutkörperchen. Das Sediment sagt mehr. Rote Blutkörperchen, die den Nierenfilter passiert haben, sehen anders aus als solche, die aus Harnleiter oder Blase stammen: "), B("Akanthozyten"), T(" heissen die ringförmigen Zellen mit den bläschenartigen Ausstülpungen. In der Arbeit, die sie beschrieben hat, fanden sich Akanthozyten von mindestens 5 Prozent bei 75 von 143 Patienten mit gesicherter Glomerulonephritis, also mit einer Empfindlichkeit von 52 Prozent – aber nur bei 4 von 187 mit nicht-glomerulärer Ursache, einer "), L("Spezifität von 98 Prozent", "https://pubmed.ncbi.nlm.nih.gov/1921146/"), T(". Übersetzt: Findet man sie, ist die Frage entschieden; findet man sie nicht, ist nichts ausgeschlossen. Erythrozytenzylinder beweisen den Ursprung in der Niere ebenso, weil sie nur im Nierenkanälchen entstehen können.")]),
    P(&[T("Dazu gehört, aus derselben Probe, der Protein-Kreatinin-Quotient. Er beantwortet die zweite Hälfte der Frage. Blut allein ist das eine, Blut zusammen mit Eiweissverlust das andere – am Eiweissverlust hängt die Prognose, und an ihm entscheidet sich, ob eine Nierenbiopsie angezeigt ist.")]),
    P(&[B("Und was das Alter dazu sagt."), T(" Eine Auswertung des japanischen Nierenbiopsie-Registers vergleicht Erwachsene zwischen 19 und 64 Jahren mit über 65-Jährigen, alle mit gesicherter IgA-Vaskulitis-Nephritis. Die Älteren starteten nicht schlechter, aber ihr Verlauf war steiler: Ein Anstieg des Kreatinins um die Hälfte trat bei "), L("21,7 gegenüber 4,7 Prozent", "https://pmc.ncbi.nlm.nih.gov/articles/PMC5940189/"), T(" ein, beobachtet über im Mittel 3,9 Jahre. Das Nierenüberleben nach neun Jahren war in beiden Gruppen gut, 91,4 gegenüber 93,6 Prozent – erreicht allerdings unter konsequenter Behandlung mit Kortison und einem Blutdruckmittel aus der RAS-Gruppe. Der Schluss der Autoren ist genau der, der hier zählt: Jenseits von 65 gehört der Verlauf der Nierenfunktion sorgfältig überwacht.")]),

    H2("Der gestaute Harnleiter"),
    P(&[T("Diese Auskunft ist neu und sie wiegt schwer. Ein gestauter Harnleiter heisst, dass der Urin nicht abfliesst. Hält das an, leidet die Niere an einem Hindernis unterhalb ihrer selbst – ein Nierenversagen, das nicht in der Niere entsteht und sich, anders als die übrigen Formen, allein durch die Wiederherstellung des Abflusses beheben lässt. Verfolgen lässt es sich am Kreatinin; das Bild dazu liefert der Ultraschall, und der kostet weder Kontrastmittel noch Strahlung und lässt sich beliebig oft wiederholen.")]),
    P(&[B("«An mehreren Stellen» ist dabei die eigentliche Information."), T(" Ein Stein sitzt an einer Stelle, ein Tumor ebenso. Mehrere Engstellen sprechen entweder für etwas, das dem Harnleiter der Länge nach zusetzt, oder für etwas, das wandert. Drei Erklärungen stehen nebeneinander, und sie sind nicht gleich wahrscheinlich:")]),
    Liste(&[
        &[B("Gerinnsel."), T(" Bei sichtbarem Blut im Urin die naheliegendste Erklärung und die einzige, die von selbst mehrere Stellen betrifft: Blut gerinnt auch in den Harnwegen. Sie erklärt zudem, warum der Stau erst jetzt auffällt, zusammen mit der Makrohämaturie und nicht vorher.")],
        &[B("Die Vaskulitis selbst."), T(" Es gibt sie, die stenosierende Ureteritis bei dieser Krankheit – dieselbe entzündete, geschwollene Wand wie im Darm, nur im Harnleiter. Sie ist selten; die Übersichten zählen für dreissig Jahre rund vierzehn beschriebene Fälle, und "), B("beschrieben sind sie fast nur bei Kindern"), T(". Die Berichte lesen sich allerdings wie diese Woche: Ein 14-Jähriger bekam 15 Tage nach Beginn der Purpura eine "), L("beidseitige Harnleiterenge", "https://pubmed.ncbi.nlm.nih.gov/9091100/"), T(", aufgefallen durch Flankenschmerz, Makrohämaturie und Nierenversagen – unter Kortison wurden beide Harnleiter wieder durchgängig und die Nierenfunktion normalisierte sich. In einem anderen Fall zeigte die Gewebeprobe aus dem Harnleiter eine "), L("schwere blutige Ureteritis mit Vaskulitis", "https://pubmed.ncbi.nlm.nih.gov/6854750/"), T(". Und ein dritter Bericht zeigt, wohin es führt, wenn man es übersieht: eine funktionslose Niere, die entfernt werden musste – der "), L("Ultraschall war in der ersten Woche normal", "https://pubmed.ncbi.nlm.nih.gov/18219497/"), T(", die Diagnose fiel erst acht Monate später. Dass sie ohnehin Kortison bekommt, ist vor diesem Hintergrund kein Nebenumstand.")],
        &[B("Druck von aussen – der geblähte Darm."), T(" Die Frage liegt nahe, und die Antwort lautet: Ja, das gibt es. Die Harnleiter laufen hinter dem Bauchfell, der Darm liegt davor; ein massiv gefüllter, gedehnter Darm kann den Harnleiter zudrücken. Beschrieben ist es unter anderem bei einer "), L("83-jährigen Frau", "https://doi.org/10.4235/agmr.20.0052"), T(" mit langjähriger Verstopfung: Die Computertomografie zeigte einen riesigen Kotstein, der den Übergang des Harnleiters in die Blase zusammendrückte und rechts eine Harnstauung machte – nach Ausräumung und Einlauf war die Stauung weg und das Kreatinin wieder normal. "), B("Hier passt der Vergleich allerdings nur halb."), T(" Jener Fall betraf eine über Jahre bestehende Verstopfung, hier war es eine Woche, und der Stuhl geht inzwischen wieder. Zu «an mehreren Stellen» passt Druck von aussen ohnehin schlecht, denn er sitzt meist an einer. Ganz vom Tisch ist der Gedanke damit nicht, weil der Darm weiterhin gebläht ist und schlecht transportiert – und geprüft wird er im selben Bild, das ohnehin ansteht.")],
        &[B("Die urologische Ursache, die das Alter vorgibt."), T(" Für sichtbares Blut im Urin gilt, was die amerikanische Urologenvereinigung für die Abklärung festlegt: Ein "), L("Alter ab 60 Jahren", "https://www.auanet.org/guidelines-and-quality/guidelines/microhematuria"), T(" ist für sich allein schon ein Hochrisikomerkmal, und dann gehören Blasenspiegelung und eine Bildgebung der oberen Harnwege dazu, im Regelfall eine CT-Urografie. Diese Regel steht hier nicht, weil ein Tumor wahrscheinlich wäre, sondern weil das Alter sie auslöst.")],
    ]),
    P(&[B("Daraus folgt etwas sehr Praktisches."), T(" Die Computertomografie des Bauches steht in diesem Blatt seit der ersten Fassung, wegen der Passagestörung. Als CT-Urografie gefahren, beantwortet dieselbe Untersuchung im selben Durchgang auch, wo der Harnleiter gestaut ist und wodurch. Ein Termin, ein Kontrastmittel, zwei Fragen. Ob das Kontrastmittel bei der aktuellen Nierenfunktion vertretbar ist, entscheidet die Ärztin – die Frage gehört gestellt, und der Ultraschall bleibt daneben das Mittel, das sich ohne Bedenken wiederholen lässt.")]),
    P(&[T("Und eine Warnung, die aus den Fallberichten kommt: Der Stau muss nicht von Anfang an da sein und nicht dauerhaft bleiben. In zwei der drei genannten Fälle war die erste Bildgebung unauffällig. Wer einmal gestaut war, gehört deshalb nachkontrolliert, auch wenn es zwischendurch besser aussieht.")]),

    H2("Ödeme an den Füssen"),
    P(&[T("Wasser im Gewebe ist kein Befund für sich, sondern eine Bilanz: Was die Gefässe nicht halten oder die Niere nicht ausscheidet, sammelt sich unten. Fünf Erklärungen kommen infrage, und vier davon treffen gleichzeitig zu. Die Frage lautet deshalb nicht, welche es ist, sondern wie viel von welcher.")]),
    Liste(&[
        &[B("Eiweissverlust über die Niere."), T(" Daran hängt am meisten. In einer prospektiven Untersuchung an 49 Erwachsenen mit IgA-Vaskulitis-Nephritis hatten 69,3 Prozent eine Eiweissausscheidung, 16,3 Prozent eine im nephrotischen Bereich – und diese war der einzige "), L("unabhängige Risikofaktor für einen ungünstigen Verlauf", "https://pmc.ncbi.nlm.nih.gov/articles/PMC12025811/"), T(". Eine Remission erreichten mit ihr 9,7 Prozent, ohne sie 60. Damit ist der Protein-Kreatinin-Quotient nicht mehr eine Untersuchung unter vielen, sondern die, an der die Prognose hängt.")],
        &[B("Zu wenig Eiweiss von vorn."), T(" Wochen ohne Nahrung senken das Albumin ebenso, ganz ohne Niere. Die japanische Registerarbeit nennt neben dem Alter über 65 ausdrücklich die "), L("Hypoalbuminämie", "https://pmc.ncbi.nlm.nih.gov/articles/PMC5940189/"), T(" als das, worauf zu achten ist. Beide Wege führen zum selben tiefen Albuminwert; auseinanderhalten lassen sie sich nur, wenn man den Urin dazu misst.")],
        &[B("Das Kortison."), T(" Die Fachinformation von Spiricort führt unter den Störungen des Wasser- und Elektrolythaushalts "), L("Natriumretention, Wasserretention und Kaliumverlust", "https://ch.oddb.org/de/gcc/fachinfo/reg/38840/chapter/unwanted_effects"), T(" auf. Ein Teil des Wassers kann also schlicht vom Medikament kommen – und der Kaliumverlust gehört zugleich in den Abschnitt zum gestörten Darmtransport.")],
        &[B("Die Infusion und die Transfusion."), T(" Wer NaCl und Blut bekommt, bekommt Volumen. Das ist genau die Konstellation, in der die Kreislaufüberlastung droht – siehe den Abschnitt «Die Transfusion». Geschwollene Füsse sind hier also nicht nur ein Befund, sondern auch eine Rückmeldung zur Bilanz.")],
        &[B("Das Herz."), T(" Bei 84 Jahren, unter Blutarmut und Volumenzufuhr, gehört die Frage gestellt. Sie steht hier nicht als Vermutung, sondern als offene Frage an die Ärztin.")],
    ]),
    P(&[B("Und jetzt der wichtigste Satz dieses Abschnitts: Ödeme gab es schon beim letzten Mal."), T(" Das ist keine Nebenbeobachtung, sondern ein rückwirkender Befund. Der Abschnitt «Ein früherer Schub» stellt die Frage, ob damals der Urin kontrolliert wurde, und nennt sie die Frage, auf die es ankommt. Ödeme beim damaligen Schub verschieben die wahrscheinliche Antwort: Sie sind das, was man sieht, wenn Eiweiss über die Niere verloren geht, und sie machen es wahrscheinlicher, dass die Niere schon damals beteiligt war. Dann wäre das heutige Bild nicht der Beginn einer Nierenbeteiligung, sondern ihre zweite Runde – und das ändert, wie dringlich eine Nierenbiopsie zu beurteilen ist.")]),
    P(&[T("Beweisen lässt sich das rückwirkend nicht, erfragen schon: Gibt es aus der damaligen Zeit Urinbefunde, Blutdruckwerte, einen Albuminwert? Und wenn nicht, ist das selbst die Antwort auf die Frage, warum heute niemand weiss, wie lange die Niere schon leidet.")]),
    P(&[B("Zwei Vorbehalte gehören dazu."), T(" Geschwollene Füsse sind bei 84-Jährigen häufig und oft harmlos; langes Sitzen und Bewegungsmangel genügen. Und die Zuordnung «damals nach dem Stress» ist genau die, vor der der Abschnitt «Zum Stress als Auslöser» warnt – erinnert wird der Stress, ausgelöst hat nach der Datenlage eher ein Infekt. Der "), B("Zeitpunkt"), T(" der Ödeme bleibt trotzdem verwertbar, auch wenn die Ursachenzuschreibung es nicht ist. Es geht nicht darum, was den Schub ausgelöst hat, sondern darum, dass damals schon Wasser in den Beinen stand.")]),

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

    H2("Die Transfusion"),
    P(&[T("Das Hämoglobin ist nochmals gefallen, und jetzt läuft ein Erythrozytenkonzentrat. Damit ist zuerst einmal das Richtige geschehen: Wenn der Wert weiter fällt und die Patientin darunter leidet, ersetzt man, was fehlt. Nur ersetzt eine Transfusion genau das und nichts weiter. Sie erklärt den Verlust nicht und sie stellt ihn nicht ab. Der Satz, der in diesem Blatt an mehreren Stellen steht, gilt danach unverändert: Die Blutungsquelle ist nicht gefunden.")]),
    P(&[B("Der erneute Abfall ist dabei selbst ein Befund, und zwar der wichtigste."), T(" Weiter oben steht, dass ein sinkendes Hämoglobin unter laufender Infusion zuerst Verdünnung ist und nicht automatisch eine neue Blutung. Dieses Argument trägt nicht mehr. Die Infusion läuft seit Tagen, die Austrocknung ist ausgeglichen, und der Wert fällt trotzdem weiter – so weit, dass transfundiert werden muss. Das ist kein Rechenfehler der Verdünnung mehr, sondern ein Verlust, der anhält. Und ein Verlust, der anhält, hat eine Quelle, die noch offen ist.")]),
    P(&[B("Die nützlichste Zahl kommt erst nach der Konserve."), T(" Als Faustregel hebt eine Einheit das Hämoglobin um rund "), N("10 g/l"), T(" – genauer: "), L("4 ml Spendererythrozyten je Kilogramm", "https://www.nss.nhs.scot/media/6134/single_unit_transfusion_guidance-draft_14final-pdf.pdf"), T(" Körpergewicht, und die Näherung gilt für einen nicht blutenden Erwachsenen von 70 bis 80 kg. Daraus wird der wichtigste Messwert der nächsten Tage: das Hämoglobin am Tag nach der Transfusion. Steigt es um deutlich weniger als erwartet oder fällt es wieder, dann blutet es weiter – und das ist ein härterer Beweis als jede Stuhlfarbe und jede Vermutung.")]),
    P(&[B("Warum Einheit für Einheit."), T(" Die Empfehlung lautet, eine Einheit zu geben und danach neu zu beurteilen: Sind die Beschwerden besser? Gibt es Zeichen einer Reaktion? Wie steht der neue Wert? Das "), L("vermeidet unnötige Transfusionen", "https://hospital.blood.co.uk/patient-services/patient-blood-management/appropriate-use-of-blood-components/single-unit-blood-transfusions/"), T(" und senkt das Risiko der Kreislaufüberlastung. Ein Vorbehalt gehört dazu, und er trifft womöglich genau hier zu: Die Regel gilt ausdrücklich nicht für Patienten mit einer klinisch bedeutsamen aktiven Blutung. Ob das auf sie zutrifft, ist die offene Frage dieses ganzen Blattes.")]),
    P(&[B("Die Kreislaufüberlastung ist bei ihr keine Formalie."), T(" TACO heisst diese Komplikation, und sie ist "), L("die häufigste Todesursache", "https://www.lifeblood.com.au/health-professionals/clinical-practice/adverse-events/TACO"), T(" unter den transfusionsbedingten Zwischenfällen, die der amerikanischen Arzneimittelbehörde gemeldet werden – 62 der gemeldeten Todesfälle zwischen 2016 und 2020, also 34 Prozent; im britischen Meldesystem waren es zwischen 2010 und 2024 157 Todesfälle oder 41,4 Prozent. Besonders anfällig sind Menschen über 60, und eine förmliche Risikoeinschätzung vor der Transfusion wird namentlich für über 70-Jährige verlangt. Dazu kommen als Risikofaktoren eine Herz- oder Nierenerkrankung und eine ausgeprägte Blutarmut. Praktisch heisst das: langsam transfundieren, an ein Entwässerungsmittel denken – und die Flüssigkeitsbilanz mitrechnen, denn die NaCl-Infusion läuft ja daneben weiter.")]),
    P(&[B("Und warum zurückhaltend nicht sparsam heisst."), T(" Bei akuter Blutung im oberen Verdauungstrakt ist weniger Blut das bessere Ergebnis. In einer Studie an 921 Patienten wurde die Hälfte erst ab "), N("70 g/l"), T(" transfundiert, die andere schon ab "), N("90 g/l"), T(". Die "), L("Überlebenswahrscheinlichkeit nach sechs Wochen", "https://pubmed.ncbi.nlm.nih.gov/23281973/"), T(" lag in der zurückhaltenden Gruppe bei 95 gegenüber 91 Prozent, Nachblutungen traten bei 10 statt 16 Prozent auf, unerwünschte Ereignisse bei 40 statt 48 Prozent. Die internationalen Empfehlungen von 2023 ziehen daraus die Linie, die auch am Anfang dieses Blattes steht: "), L("70 g/l bei stabilen Erwachsenen", "https://pubmed.ncbi.nlm.nih.gov/37824153/"), T(", 80 bei vorbestehender Herz-Kreislauf-Erkrankung. Wer bei 83 transfundiert, tut das also nicht wegen der Zahl, sondern wegen der Patientin – und das ist zulässig, sofern es so begründet wird.")]),

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

    H2("Belegte Fälle in diesem Alter"),
    P(&[T("Einen veröffentlichten Fall genau einer 84-jährigen Frau mit IgA-Vaskulitis gibt es in der durchsuchten Literatur nicht. Was es gibt, sind Frauen knapp darunter und knapp darüber – und die Bilder, die zu dieser Patientin passen: Purpura an den Beinen, Bauch, oft Niere. Die nächste deutschsprachige Beschreibung ist eine 80-Jährige aus "), L("Basel", "https://doi.org/10.1055/s-2002-32350"), T(", drei Wochen nach einer Pneumonie. Die nächste mit Nierenhistologie sind 80 und 85 Jahre, "), L("Japan 2019", "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6367074/"), T(".")]),
    Tab(&T_FAELLE),
    P(&[T("Zur Einordnung der Tabelle gehört, wie dünn dieser Rand der Literatur ist. Die Autoren des 93-jährigen Falls bezeichnen ihre Patientin ausdrücklich als die "), L("älteste bekannte Patientin mit dieser Krankheit", "https://pmc.ncbi.nlm.nih.gov/articles/PMC5031831/"), T(" – obwohl die 97-Jährige aus Japan ein Jahr zuvor erschienen war. Solche Superlative in Einzelfallberichten sind also mit Vorsicht zu lesen. Was bleibt, ist der Befund der Tabelle selbst: Jenseits der neunzig ist die Literatur eine Handvoll Einzelfälle, und für eine 84-Jährige gibt es keinen passgenauen Bericht.")]),
    P(&[T("Weiter entfernt, aber dokumentiert: 89 Jahre mit kompletter Remission unter niedrig dosiertem Prednisolon plus Mizoribin ("), L("Sugimoto 2021", "https://doi.org/10.1007/s13730-020-00513-6"), T("); 75 und 76 Jahre in derselben "), L("Ueda-Serie", "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6367074/"), T(" mit Steroidpuls, teils steroidinduziertem Diabetes; 71 Jahre mit Darmblutung und gutem Ansprechen auf parenterale Kortikoide ("), L("Cureus 2022", "https://doi.org/10.7759/cureus.34422"), T("). Ein 84-jähriger "), B("Mann"), T(" nach COVID ist beschrieben – das ist nicht diese Patientin.")]),
    P(&[
        T("Was die grösseren Untersuchungen für sie bedeuten: Erwachsene haben häufiger und schwerer Nierenbefall als Kinder. In der Kohorte von "), L("Pillebout", "https://pubmed.ncbi.nlm.nih.gov/11961015/"), T(" (250 Erwachsene, Alter 15 bis 86) erreichten nur 20 Prozent eine Remission, 11 Prozent ein endgültiges Nierenversagen, und die häufigste Todesursache war ein Karzinom, nicht die Vaskulitis. Ab etwa 63 Jahren mehr nekrotische Purpura und Nierenversagen ("), L("IGAVAS", "https://pubmed.ncbi.nlm.nih.gov/33410479/"), T("). In der japanischen Registerarbeit von "), L("Komatsu", "https://doi.org/10.1371/journal.pone.0196955"), T(" waren 46 Patientinnen und Patienten 65 bis 84 Jahre alt: schlechtere eGFR, mehr Halbmonde, weniger Remission. Männer überwiegen in fast allen westlichen Serien – eine 84-jährige Frau ist die seltenere Kombination."),
    ]),
    P(&[
        T("Zur Therapie gibt es "), B("keine Studie an Oktogenarierinnen"), T(". Die "), L("KDIGO-Leitlinie 2025", "https://doi.org/10.1016/j.kint.2025.04.004"), T(" rät davon ab, Steroide nur zu geben, um einer Nephritis vorzubeugen, wenn Haut, Gelenke oder Darm allein betroffen sind (Empfehlung 1B). Bei bedrohter Niere oder schwerem Darmbefall sind Steroide Praxis. Zusätzliches Cyclophosphamid brachte in der einzigen Erwachsenen-Studie ("), L("CESAR", "https://pubmed.ncbi.nlm.nih.gov/20505654/"), T(") keinen Nutzen. Was in diesem Alter zählt, steht in der Ueda-Serie: drei von dreizehn über 60 starben an Infekten unter Steroiden. Das ist Literatur für den Termin, kein Behandlungsplan."),
    ]),
    Klein(&[T("Recherchestand 28. August 2026. Einzelfälle, Evidenzstufe IV bis V. Keine Behandlungsempfehlung.")]),

    H2("Ein früherer Schub"),
    P(&[T("Tastbare Purpura an den Unterschenkeln ist die Signatur dieser Krankheit. Ist das schon einmal aufgetreten, ist das heutige Bild wahrscheinlich ein "), B("Rezidiv"), T(" – und das ändert die Einordnung.")]),
    P(&[T("Bei Erwachsenen kehrt die IgA-Vaskulitis in etwa "), B("43 Prozent"), T(" der Fälle wieder, gut ein Viertel davon mehrfach; nur rund die Hälfte erreicht eine vollständige Remission. Wiederkehren ist bei Erwachsenen die Regel und nicht die Ausnahme, und höheres Alter beim Erstauftreten ist selbst ein Vorhersagefaktor für Rückfälle. Die Tumorfrage bleibt dabei auf dem Tisch: Die Suche wird ausdrücklich bei "), B("neu aufgetretener oder wiederkehrender"), T(" IgA-Vaskulitis im Alter empfohlen – ein Rezidiv entlastet nicht.")]),
    H3("Die Frage, auf die es ankommt"),
    P(&[B("Wurde damals der Urin kontrolliert?"), T(" Daran entscheidet sich, wie lange die Niere schon unter Beschuss steht. Eine Nierenbeteiligung tritt oft erst Wochen nach dem Ausschlag auf und macht keine Beschwerden – sie zeigt sich nur im Urin. Wurde der erste Schub als Hautsache abgetan und hat niemand Urinstatus und Protein-Kreatinin-Quotient bestimmt, kann die Niere seither still gelitten haben. Das würde erklären, warum das Bild heute schwerer ist.")]),
    P(&[T("Und falls damals eine Diagnose gestellt wurde: Gab es eine Hautbiopsie? Ein IgA-Nachweis in der Gefässwand von damals wäre heute Gold wert – er erspart die Frage, ob es dieselbe Krankheit ist.")]),
    H3("Zum Stress als Auslöser"),
    P(&[T("Für die IgA-Vaskulitis speziell ist psychischer Stress "), B("kein belegter Auslöser"), T(". Belegt sind Infekte, Medikamente und Tumoren.")]),
    P(&[T("Woher die Überzeugung kommt, lässt sich zeigen. In einer "), L("Befragung", "https://pmc.ncbi.nlm.nih.gov/articles/PMC4008683/"), T(" von Vaskulitis-Patienten nannte rund "), B("die Hälfte"), T(" «Stress oder Sorgen» als Ursache ihrer Erkrankung. Dazu gehören zwei Einschränkungen. Erstens war die Befragung von der Granulomatose mit Polyangiitis und ihrer eosinophilen Form beherrscht; die IgA-Vaskulitis machte zwei Prozent der Befragten aus – über diese Krankheit sagt sie also fast nichts. Zweitens hing die Überzeugung, ein Schub sei psychisch ausgelöst, zwar mit mehr Erschöpfung und mit Einbussen im körperlichen, beruflichen und sozialen Alltag zusammen, aber nur schwach: Die Zusammenhänge lagen bei Korrelationen von 0,12 bis 0,19.")]),
    P(&[T("Und dort, wo man nicht nach der Überzeugung, sondern nach dem Verlauf gefragt hat, kam der Stress nicht heraus. Eine "), L("Verlaufsuntersuchung", "https://pmc.ncbi.nlm.nih.gov/articles/PMC11249541/"), T(" an 112 Patienten mit ANCA-assoziierter Vaskulitis erhob von 2011 bis 2022 alle drei bis sechs Monate Stress, belastende Lebensereignisse, Insektenstiche und Infekte und verglich dann die fünfzehn Monate vor einem Schub mit einer schubfreien Zeit. Beim Stress fand sie "), B("keinen Unterschied"), T(" – im Median zwei belastende Ereignisse in beiden Gruppen. Bei den Infekten fand sie einen, und zwar bei Atemwegsinfekten neun bis fünfzehn Monate vor dem Schub. Frühere Arbeiten hatten nur rückblickend und meist nur nach dem Krankheitsbeginn gefragt; hier wurde laufend erhoben. Das Ergebnis zeigt in dieselbe Richtung wie alles andere in diesem Abschnitt: Der Infekt ist der Auslöser, der Stress ist die Erinnerung daran.")]),
    P(&[T("Plausibel bleibt die indirekte Kette: Stress schwächt die Infektabwehr, und Infekte sind der belegte Auslöser. Eine belastende Phase mit einer verschleppten Erkältung darin ist naheliegend – nur erinnert man hinterher den Stress und nicht das Halsweh. Die praktische Folge: Die Stress-Erklärung darf die Suche nach dem tatsächlichen Auslöser nicht ersetzen. Hier heisst das vor allem: Welche Medikamente sind in den letzten Wochen neu dazugekommen?")]),
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
        &[B("Blutung im Zwölffingerdarm."), T(" Seit der Stuhl schwarz und flüssig ist und schwarz erbrochen wird, steht diese Erklärung nicht mehr zur Auswahl, sondern zur Bestätigung: Kaffeesatz oben und Teerstuhl unten heisst Blut aus dem oberen Verdauungstrakt. Die Vaskulitis schädigt die Schleimhaut am häufigsten dort, gleich hinter dem Magen. Die Blutung muss nicht sichtbar sein – sie kann über Wochen sickern, ohne dass der Stuhl auffällt. Solange gar kein Stuhl kam, fiel dieser Hinweis ganz weg; jetzt, wo die Passage wieder offen ist, steht er wieder zur Verfügung. Seit Blut im Urin ist, steht diese Erklärung wieder zuoberst: Der Urin zeigt, dass die Vaskulitis aktiv ist, und eine aktive Vaskulitis ist es, die im Darm blutet – siehe den Abschnitt «Blut im Urin».")],
        &[B("Die Magenentzündung."), T(" Eine entzündete, erodierte Schleimhaut ist eine Blutungsquelle; das Ob steht nicht mehr in Frage, nur das Wie viel – und woher die Entzündung kommt. Dass es unter Pantoprazol nicht besser geworden ist, spricht dafür, dass sie nicht die ganze Erklärung ist. Siehe den Abschnitt «Die neue Diagnose: starke Magenentzündung».")],
        &[B("Die Harnwege."), T(" Neu auf dieser Liste, und sie gehört hierher, seit das Blut im Urin sichtbar ist. Blut, das den Nierenfilter passiert, ist bei dieser Krankheit typischerweise nur unter dem Mikroskop zu sehen; was den Urin mit blossem Auge färbt und Gerinnsel bildet, kann eine andere Grössenordnung haben. Welche der beiden es ist, entscheidet das Sediment – siehe den Abschnitt «Blut im Urin».")],
        &[B("Medikamente."), T(" Kortison zusammen mit Schmerzmitteln vom NSAR-Typ (Ibuprofen, Diclofenac, Naproxen) erhöht das Geschwürrisiko erheblich; NSAR belasten zusätzlich die Niere. Blutverdünner verstärken jede vorhandene Blutungsquelle. Die vollständige Medikamentenliste gehört auf den Tisch, rezeptfreie Mittel eingeschlossen. "), B("Ein Satz aus der Fachinformation von Spiricort gehört eigens hervorgehoben:"), T(" Unter Kortison können "), L("intestinale Blutungen symptomarm verlaufen", "https://ch.oddb.org/de/gcc/fachinfo/reg/38840/chapter/unwanted_effects"), T(", und ein Magengeschwür mit Blutung und möglicher Perforation tritt dort «häufig ohne die typische Symptomatik» auf. Das erklärt, wie eine Blutung so weit kommen konnte, ohne die Schmerzen zu machen, die man von einem Geschwür erwartet – und es ist ein Grund mehr, nicht auf Beschwerden zu warten.")],
        &[B("Eine zweite, unabhängige Quelle."), T(" Mit 84 Jahren ist ein Dickdarmtumor oder ein Magengeschwür statistisch häufiger als die Vaskulitis selbst. Der Dickdarmtumor ist inzwischen erledigt: Die Darmspiegelung war unauffällig. Das Magengeschwür bleibt, und mit ihm die Frage nach dem, was die Spiegelung im Magen tatsächlich gesehen und biopsiert hat.")],
        &[B("Der Dünndarm dazwischen."), T(" Die Strecke, die weder die Magen- noch die Darmspiegelung erreicht. Sobald beide unauffällig sind und es weiter blutet, ist sie die verbleibende – und für sie gibt es einen eigenen Weg. Siehe den Abschnitt «Die Darmspiegelung: nichts gefunden, ausser beim Transport».")],
        &[B("Die Niere."), T(" Bei nachlassender Nierenfunktion bildet der Körper weniger Erythropoetin, das Hormon für die Blutbildung. Das erklärt eine langsam sinkende Kurve, keinen Sturz um "), N("25 g/l"), T(" in kurzer Zeit.")],
        &[B("Novalgin."), T(" Metamizol kann das Knochenmark treffen: Leukopenie selten, Agranulozytose sehr selten, aplastische Anämie und Panzytopenie ohne Häufigkeitsangabe. Als alleinige Erklärung unwahrscheinlich, aber mit einem Differentialblutbild in einem Schritt geprüft – siehe den Abschnitt zu Novalgin.")],
        &[B("Mangelernährung."), T(" Wochen ohne Nahrung liefern zu wenig Eisen, Folsäure und Vitamin B12. Als alleinige Erklärung für dieses Tempo zu langsam, als verstärkender Faktor real – und mit drei Laborwerten geprüft.")],
        &[B("Entzündungsanämie."), T(" Ebenso: als Grundrauschen möglich, für diesen Verlauf zu langsam.")],
        &[B("Verdünnung durch die Infusion."), T(" Sie senkt den gemessenen Wert, ohne dass Blut verloren geht – siehe oben. Zusammen mit häufigen Blutentnahmen erklärt das einen Teil des Abfalls und ist vor der grossen Abklärung in einer Minute geprüft.")],
    ]),

    H2("Dunkler Stuhl: Galle oder Blut?"),
    P(&[T("Die Auskunft, der dunkle Stuhl komme von der Galle, ist nicht abwegig – die Farbe des Stuhls stammt tatsächlich aus der Galle. Das Bilirubin der Galle wird im Dickdarm von den Darmbakterien "), L("abgebaut", "https://www.ncbi.nlm.nih.gov/books/NBK470290/"), T("; die dabei entstehenden Stoffe sind farblos und geben dem Stuhl erst durch Oxidation an der Luft seine Farbe. Nach Wochen ohne Nahrung und einer Woche ohne Stuhlgang ist ein dunkler erster Stuhl also keine Überraschung.")]),
    P(&[T("Nur beantwortet die Farbe die Frage nicht, um die es geht. "), B("Teerstuhl"), T(" – der Stuhl bei einer Blutung im oberen Verdauungstrakt – ist nicht bloss dunkel, sondern schwarz, glänzend, klebrig, schwer abzuspülen und von durchdringendem Geruch. Drei Angaben dazu sind in dieser Lage nützlich: Es braucht rund "), N("50 ml"), T(" Blut im Magen, um den Stuhl "), L("schwarz zu färben", "https://www.ncbi.nlm.nih.gov/books/NBK411/"), T("; nach einer Blutung kann er noch tagelang so aussehen, die Farbe sagt also nichts darüber, ob gerade jetzt geblutet wird; und Eisenpräparate und Wismut färben ihn ebenfalls schwarz. Die Medikamentenliste beantwortet hier unter Umständen mehr als das Auge.")]),
    P(&[T("Wie viel an dieser Unterscheidung hängt, zeigt die Auswertung der "), L("JAMA-Reihe zur klinischen Untersuchung", "https://pubmed.ncbi.nlm.nih.gov/22416103/"), T(": Ein Teerstuhl, den die Ärztin selbst gesehen hat, vervielfacht die Chance auf eine Blutung im oberen Verdauungstrakt um das 25-fache; ein von der Patientin oder den Angehörigen berichteter schwarzer Stuhl nur um das Fünf- bis Sechsfache. Der Unterschied zwischen «sah dunkel aus» und «war Teerstuhl» ist also kein Wortstreit, sondern der Unterschied zwischen einem starken und einem mittelmässigen Hinweis. Wo es darauf ankommt, schaut jemand hin, der es beurteilen kann.")]),
    P(&[B("Der billigste Hinweis steckt im Blut, das ohnehin abgenommen wird."), T(" Bei einer Blutung im oberen Verdauungstrakt wird das Bluteiweiss im Darm verdaut und aufgenommen; der Harnstoff im Blut steigt daraufhin, das Kreatinin nicht. Ein Verhältnis von Harnstoff zu Kreatinin über 30 vervielfacht die Chance auf eine obere Blutungsquelle um das Siebenfache; eine "), L("Metaanalyse", "https://onlinelibrary.wiley.com/doi/10.1111/jgh.70224"), T(" fand den besten Trennwert bei 22, mit 66 Prozent Sensitivität und 71 Prozent Spezifität. Ein Beweis ist das nicht, aber ein Hinweis, der weder eine zusätzliche Entnahme noch einen weiteren Termin kostet.")]),
    Klein(&[T("Ein Fallstrick beim Nachrechnen: Diese Grenzwerte gelten für die amerikanischen Einheiten, Harnstoffstickstoff und Kreatinin je in mg/dl; dort liegt der übliche Bereich bei etwa 8 bis 15. Schweizer Labors geben Harnstoff in mmol/l und Kreatinin in µmol/l an, und in dieser Rechnung sehen dieselben Verhältnisse ganz anders aus – der übliche Bereich liegt dann "), L("bei etwa 40 bis 100", "https://litfl.com/urea-creatinine-ratio/"), T(". Wer die Zahlen des Laborberichts direkt gegen die 30 hält, bekommt Unsinn heraus. Die Rechnung gehört der Ärztin; hier steht sie, damit die Frage überhaupt gestellt wird.")]),
    P(&[T("Und wie die Frage auch ausgeht: Der Hämoglobinabfall braucht eine Erklärung. Fällt der Stuhl als Quelle aus, rücken die übrigen Einträge der Liste oben nach vorn – und der wirksamste nächste Schritt bleibt derselbe, den dieses Blatt an mehreren Stellen nennt: die Spiegelung, die Blutungsquelle und Tumorfrage in einem Durchgang beantwortet.")]),

    H3("Die Beschreibung ist jetzt eine andere"),
    P(&[T("Inzwischen liegt eine genauere Auskunft vor: "), B("schwarz und flüssig"), T(", mit weissem Schleim dazwischen. Das ist keine Farbangabe mehr, sondern die Beschreibung von Teerstuhl. Dasselbe Kapitel, das oben schon zitiert ist, definiert die Meläna als "), L("schwarzen, teerartigen Stuhl", "https://www.ncbi.nlm.nih.gov/books/NBK411/"), T(" – und hebt hervor, dass Betroffene sich vor allem an die klebrige Beschaffenheit erinnern. Zusammen mit dem nochmals gefallenen Hämoglobin und der Transfusion ist die Frage dieses Abschnitts damit praktisch beantwortet. Die Galle erklärt einen dunklen ersten Stuhl nach Wochen ohne Stuhlgang. Sie erklärt keinen schwarzen, flüssigen Stuhl bei gleichzeitig weiter fallendem Hämoglobin.")]),
    P(&[T("Drei Vorbehalte bleiben, und sie kosten alle nichts:")]),
    Liste(&[
        &[B("Erst die Medikamentenliste, dann das Auge."), T(" Eisenpräparate und Wismut färben den Stuhl ebenso schwarz. Ob eines davon läuft, ist in einer Minute geklärt und entscheidet mehr als jede Beschreibung.")],
        &[B("Läuft ein Abführmittel?"), T(" Dann gehört ihm die flüssige Beschaffenheit zugerechnet – die Farbe aber nicht. Beides sauber auseinanderzuhalten spart eine Fehldeutung in beide Richtungen.")],
        &[B("Die Farbe sagt nichts über jetzt."), T(" Ein bis zwei Liter Blut lassen den Stuhl "), L("bis zu fünf Tage lang", "https://www.ncbi.nlm.nih.gov/books/NBK411/"), T(" teerartig aussehen; der erste solche Stuhl erscheint 4 bis 20 Stunden nach der Blutung. Der Stuhl beweist also, "), B("dass"), T(" geblutet wurde, nicht dass gerade jetzt geblutet wird. Was jetzt blutet, verrät der Hämoglobinverlauf nach der Transfusion.")],
    ]),
    P(&[B("Und daraus folgt eine Frist, keine Empfehlung."), T(" Teerstuhl ist das Leitzeichen der akuten Blutung im oberen Verdauungstrakt. Für diese Lage empfiehlt die europäische Fachgesellschaft nach der Kreislaufstabilisierung die Magenspiegelung "), L("innerhalb von 24 Stunden", "https://pubmed.ncbi.nlm.nih.gov/33567467/"), T(" – eine starke Empfehlung. Die amerikanische Fachgesellschaft schlägt dieselben "), L("24 Stunden ab Vorstellung", "https://doi.org/10.14309/ajg.0000000000001245"), T(" vor, und zwar ausdrücklich unabhängig davon, ob das Risiko als hoch oder niedrig eingeschätzt wird. An allen anderen Stellen sagt dieses Blatt, die Spiegelung sei der ergiebigste Schritt. Hier sagt es etwas Schärferes: Sie hat eine Uhr.")]),

    H3("Der weisse Schleim"),
    P(&[T("Er gehört nicht zur selben Geschichte. Die schwarze Farbe kommt von oben, aus Magen oder Zwölffingerdarm; Schleim kommt von unten. Er ist ein Erzeugnis der Darmschleimhaut, und eine "), L("gereizte Schleimhaut", "https://my.clevelandclinic.org/health/symptoms/mucus-in-stool"), T(" gibt mehr davon ab. Zwei Erklärungen liegen nahe: ein Dickdarm, der nach vier bis fünf Wochen Stillstand wieder in Gang kommt – und der Dickdarmbefall der Vaskulitis selbst.")]),
    P(&[T("Dazu liefert dieselbe Untersuchung an 108 Erwachsenen, die weiter oben die Magenspiegelungen ausgewertet hat, auch die Zahlen für unten. Bei den 31 "), L("Darmspiegelungen", "https://pmc.ncbi.nlm.nih.gov/articles/PMC11429444/"), T(" war der Krummdarm in 61,3 Prozent befallen, der Dickdarm in 38,7 und der Enddarm in 22,6 Prozent. "), B("Daraus folgt dasselbe wie oben, nur am anderen Ende:"), T(" Das Gerät muss bis in den Krummdarm, das letzte Stück Dünndarm vor dem Übergang in den Dickdarm. Wer dort nicht hinschaut, sieht die Stelle nicht, an der die Vaskulitis bei der Darmspiegelung am häufigsten sitzt.")]),

    H2("Schwarz erbrochen: die Blutung liegt oben"),
    P(&[T("Damit ist die zweite Hälfte des Beweises da, und die Frage «Galle oder Blut» ist endgültig erledigt. Schwarzes Erbrochenes heisst Kaffeesatz, und Kaffeesatz ist Blut, das in der Magensäure gelegen hat: Die Säure "), L("oxidiert den roten Blutfarbstoff zu braunem Hämatin", "https://www.merckmanuals.com/professional/gastrointestinal-disorders/gastrointestinal-bleeding/overview-of-gastrointestinal-bleeding"), T(". Oben Kaffeesatz, unten Teerstuhl – dasselbe Blut, an beiden Enden derselben Strecke. Die Quelle liegt damit im oberen Verdauungstrakt, also im Bereich, den die Magenspiegelung erreicht.")]),
    P(&[B("Die JAMA-Auswertung von oben hat dazu eine zweite Liste, die in diesem Blatt bisher fehlte."), T(" Für die Herkunft: Blut oder Kaffeesatz im Magensaft vervielfacht die Chance auf eine obere Blutungsquelle um das 9,6-fache. Für die "), B("Schwere"), T(" – und darum geht es jetzt – nennt dieselbe Arbeit ein "), L("Hämoglobin unter 8 g/dl", "https://pubmed.ncbi.nlm.nih.gov/22416103/"), T(", das die Chance auf eine schwere, dringend behandlungsbedürftige Blutung um das 4,5- bis 6,2-fache erhöht, und einen beschleunigten Puls, der sie um das 4,9-fache erhöht. "), N("83 g/l"), T(" sind "), N("8,3 g/dl"), T(", und der Wert ist seither gefallen. Diese Patientin steht also nicht am Rand dieser Liste, sondern mitten darin.")]),
    P(&[B("Ein Trost steckt trotzdem darin."), T(" Kaffeesatz ist nicht frisches Blut. Dass es überhaupt braun geworden ist, heisst, dass es Zeit in der Säure hatte – das "), L("spricht für eine Blutung, die langsamer geworden oder zum Stehen gekommen ist", "https://www.merckmanuals.com/professional/gastrointestinal-disorders/gastrointestinal-bleeding/overview-of-gastrointestinal-bleeding"), T(", nicht für einen Schwall. Kein Grund zu warten, aber ein Grund, nicht in Panik zu geraten.")]),
    P(&[B("Was daraus für die Suche folgt."), T(" Die Frist von 24 Stunden steht schon im Abschnitt zum Stuhl. Neu ist die Richtung: nicht in die Breite, sondern noch einmal genau dorthin, wo die Magenentzündung gefunden wurde. Damit werden die beiden Fragen dieses Blattes an die erste Spiegelung dringlich – wurde bis in den absteigenden Zwölffingerdarm geschaut, und wurde dort Gewebe entnommen? Dort sitzt die Vaskulitis bei Erwachsenen in 55,7 Prozent der Fälle, im Magen selbst nur in 3,3.")]),
    P(&[B("Zwei praktische Punkte zum Schluss."), T(" Der erste: Sie erbricht – ob eine Tablette unter diesen Umständen ankommt, ist eine berechtigte Frage, und der Weg über die Vene stellt sich von selbst. Ehrlich dazu gehört aber, was ein Säureblocker vor der Spiegelung leistet und was nicht: Die "), L("Cochrane-Übersicht", "https://doi.org/10.1002/14651858.CD005415.pub4"), T(" findet keinen Einfluss auf Sterblichkeit, Nachblutung, Operationsbedarf oder Transfusionsbedarf – nur weniger frische Blutungszeichen bei der Spiegelung. Er ist Vorbereitung, nicht Behandlung, und ersetzt die Spiegelung nicht. Der zweite: Die amerikanische Fachgesellschaft schlägt vor der Spiegelung eine "), L("Infusion von Erythromycin", "https://doi.org/10.14309/ajg.0000000000001245"), T(" vor; sie treibt den Mageninhalt weiter und macht die Sicht frei. Bei einem Magen, der ohnehin schlecht transportiert und in dem Blut liegt, ist das der passende Vorschlag.")]),

    H2("Die Darmspiegelung: nichts gefunden, ausser beim Transport"),
    P(&[T("Zwei Nachrichten in einer, und sie ziehen in verschiedene Richtungen. Die erste ist gut: Der Dickdarm ist unauffällig. Damit fällt der Dickdarmtumor weg, der bei 84 Jahren ganz oben auf der Liste stand, und der Dickdarm als Blutungsquelle gleich mit. Das ist echte Entlastung und keine Vertröstung.")]),
    P(&[B("Nur beantwortet dieser Befund den Teerstuhl nicht."), T(" Ein schwarzer Stuhl entsteht "), B("oberhalb"), T(" – dort, wo Blut durch den Darm wandert und dabei verdaut wird. Genau dorthin schaut die Darmspiegelung nicht. Ein unauffälliger Dickdarm ist mit einem Teerstuhl vollständig vereinbar; er war nie der Test dafür. Wer die beiden Befunde gegeneinander hält, hält Äpfel gegen Birnen.")]),
    P(&[B("Zwei Rückfragen gehören trotzdem gestellt."), T(" Erstens: Wie weit kam das Gerät? Der Krummdarm ist bei dieser Krankheit der häufigste Sitz, und er wird nicht bei jeder Darmspiegelung eingesehen – die "), L("Qualitätsmasse der Fachgesellschaften", "https://pmc.ncbi.nlm.nih.gov/articles/PMC10005623/"), T(" verlangen als Nachweis der Vollständigkeit den Blinddarm, nicht den Krummdarm. Zweitens: Wie sauber war der Darm? Nach einer Woche Stillstand und bei einem Darm, der ohnehin schlecht transportiert, ist die Frage nach der Vorbereitung keine Förmlichkeit. Beides steht im Befundbericht, und beides entscheidet, wie viel «nichts gefunden» hier wiegt.")]),
    P(&[B("Und jetzt wird eine Lücke sichtbar."), T(" Die Magenspiegelung reicht bis in den absteigenden Zwölffingerdarm, die Darmspiegelung im besten Fall bis in den Krummdarm. Dazwischen liegen mehrere Meter Dünndarm, die kein Gerät von beiden sieht. Genau dafür gibt es einen Namen und einen Weg: Eine "), L("Blutungsquelle im Dünndarm", "https://pubmed.ncbi.nlm.nih.gov/26303132/"), T(" gehört erwogen, sobald Magen- und Darmspiegelung unauffällig sind; erste Wahl ist dann die Kapselendoskopie, und bleibt auch sie ohne Befund, folgt die Computertomografie als Enterografie. "), B("Nur steht dieser Weg jetzt hinten an."), T(" Dieselbe Leitlinie hält fest, dass vor der Dünndarmabklärung eine zweite Magenspiegelung stehen kann – und seit schwarz erbrochen wird, ist die Quelle oben lokalisiert. Der Dünndarm ist damit nicht der nächste Schritt, sondern der übernächste: der Weg für den Fall, dass oben nichts gefunden wird und es weiterblutet.")]),
    P(&[B("Der Transportbefund ist der zweite Teil der Nachricht."), T(" Der Gastroenterologe hat es deutlich gesagt: keine rechte Peristaltik. Das bestätigt, was dieses Blatt seit der ersten Fassung als Passagestörung beschreibt – aber "), B("ohne mechanisches Hindernis"), T(". Kein Tumor, keine Enge, und trotzdem eine echte Störung. Vier Erklärungen kommen infrage, und drei davon lassen sich mit Blutwerten prüfen, die ohnehin abgenommen werden:")]),
    Liste(&[
        &[B("Die Vaskulitis selbst."), T(" Die entzündete, geschwollene Darmwand bewegt sich schlechter. Das ist die Erklärung, die zum Rest des Bildes passt.")],
        &[B("Kalium und Magnesium."), T(" Beide stehen in diesem Blatt schon zweimal auf der Liste – wegen des Kostaufbaus nach Wochen ohne Nahrung und wegen Pantoprazol. Jetzt ein drittes Mal: Ein Mangel an beiden kann die Beweglichkeit des Dickdarms beeinträchtigen, und es gibt Fallberichte, in denen sich eine "), L("Pseudoobstruktion nach Ausgleich des Kaliums löste", "https://pmc.ncbi.nlm.nih.gov/articles/PMC12701538/"), T(". Ehrlicherweise gehört dazu: Neuere Arbeiten "), L("bestreiten", "https://journals.lww.com/ijam/fulltext/2015/01010/scrutinizing_the_evidence_linking_hypokalemia_and.4.aspx"), T(", dass ein Kaliummangel für sich allein einen Darmstillstand auslöst. Als mitwirkender Faktor bleibt er plausibel, und der Ausgleich ist ohnehin geboten. Woher der Mangel kommt, ist hier nicht schwer zu sagen: Wochen ohne Nahrung – und das Kortison, dessen Fachinformation den "), L("Kaliumverlust", "https://ch.oddb.org/de/gcc/fachinfo/reg/38840/chapter/unwanted_effects"), T(" ausdrücklich aufführt.")],
        &[B("Die Schilddrüse."), T(" Eine Unterfunktion kann einen Darmstillstand so überzeugend nachahmen, dass er für einen "), L("mechanischen Dünndarmverschluss", "https://pmc.ncbi.nlm.nih.gov/articles/PMC10796157/"), T(" gehalten wird. Ein TSH-Wert kostet nichts und steht bisher nicht auf der Abklärungsliste dieses Blattes. Er gehört darauf.")],
        &[B("Medikamente."), T(" Die vollständige Liste gehört auch aus diesem Grund auf den Tisch, rezeptfreie Mittel eingeschlossen.")],
    ]),
    P(&[B("Und ja, es besteht ein Zusammenhang mit der Vaskulitis."), T(" Das ist nicht bloss plausibel, sondern in der Übersicht zur Magen-Darm-Beteiligung von Vaskulitiden ausdrücklich aufgeführt: Zu dem, was eine Vaskulitis im Verdauungstrakt anrichtet, zählen neben Geschwür, Schleimhautschwellung und Blutung auch der "), L("paralytische Ileus", "https://pmc.ncbi.nlm.nih.gov/articles/PMC3309893/"), T(" und der Darmverschluss. Dieselbe Arbeit nennt für die IgA-Vaskulitis als häufig betroffene Abschnitte den absteigenden Zwölffingerdarm und den Krummdarm – genau die beiden Stellen, die dieses Blatt an anderer Stelle als Zielorte der beiden Spiegelungen nennt. Dort stammt die Angabe aus der Untersuchung an 108 Erwachsenen, hier aus einer unabhängigen Quelle; zwei Wege, ein Ergebnis.")]),
    P(&[T("Der Mechanismus ist derselbe wie überall in diesem Blatt: Die Krankheit greift die Wand von Röhren an. Eine entzündete, geschwollene, schlechter durchblutete Darmwand zieht sich nicht mehr richtig zusammen – dieselbe Wandschwellung, die vorher die Passage behindert hat. Dazu kommt ein zweiter Weg, der nicht spezifisch für diese Vaskulitis ist, aber zum Bild passt: Eine Entzündung des Nervengeflechts im Darm führt zu "), L("Magenlähmung und Pseudoobstruktion", "https://www.gastrojournal.org/article/S0016-5085(04)00223-9/fulltext"), T(" – die Nerven, die den Transport steuern, arbeiten dann nicht mehr richtig.")]),
    P(&[B("Nur beweist der Zusammenhang die Ursache nicht."), T(" Die drei anderen Erklärungen von oben – Kalium, Magnesium, Schilddrüse – bleiben genau deshalb auf der Liste, weil sie sich mit drei Blutwerten prüfen und, anders als die Vaskulitis, unmittelbar beheben lassen. Und «keine rechte Peristaltik» ist ein klinischer Eindruck, kein Messwert. Das macht ihn nicht weniger wert – nur beantwortet er nicht, woran es liegt.")]),
    P(&[B("Dass sie Luft aufstösst, bis sie würgt, gehört in dasselbe Bild."), T(" Wenn der Transport nach unten nicht funktioniert, nimmt die Luft den Weg zurück nach oben. Aufstossen und Würgen sind dann nicht zwei neue Beschwerden, sondern dieselbe Störung von oben gesehen – so wie der geblähte Magen und das frühere Erbrechen nach jedem Bissen. Für den Bericht an die Ärztin zählt vor allem, dass es unter Pantoprazol nicht besser geworden ist: Ein Säureblocker senkt die Säure und hat auf den Transport keinen Einfluss.")]),

    P(&[B("Und was der Transportbefund für die Kapsel bedeutet."), T(" Die naheliegende Sorge lautet: Bleibt sie stecken? Die europäische Leitlinie ist genauer als die Sorge. Eine "), L("Motilitätsstörung ohne zugrundeliegende Enge ist kein Hinderungsgrund", "https://pubmed.ncbi.nlm.nih.gov/36423618/"), T("; ein Hinderungsgrund ist eine bekannte oder vermutete Enge, solange die Durchgängigkeit nicht belegt ist. Zeichen einer Obstruktion gelten als Risiko für ein Steckenbleiben, und das liegt je nach Fragestellung zwischen 2,1 und 8,2 Prozent. Praktisch heisst das: Die Computertomografie, die in diesem Blatt ohnehin an mehreren Stellen steht, beantwortet genau die Frage, an der die Kapsel hängt – gibt es eine Enge oder nicht.")]),

    H2("Was abgeklärt gehört"),
    Tab(&T_ABKLAERUNG),

    H2("Zur Behandlung"),
    P(&[T("Die Therapie führen die Spezialisten; hier steht nur, was den Rahmen erklärt.")]),
    P(&[T("Sind allein Haut und Gelenke betroffen, wird oft beobachtet und nur gegen die Beschwerden behandelt. Sobald Darm oder Niere beteiligt sind, kommt Kortison zum Einsatz, üblicherweise Prednison um 1 mg pro Kilogramm Körpergewicht mit anschliessendem Ausschleichen. Eine frühe Kortisonbehandlung senkt die Wahrscheinlichkeit eines bleibenden Nierenschadens deutlich. Bei schwerer Nierenbeteiligung kommen zusätzliche Immunsuppressiva in Frage – das entscheidet die Nierenbiopsie.")]),
    P(&[T("Im hohen Alter gilt: die niedrigste wirksame Dosis, und die Nebenwirkungen von Anfang an mitbehandeln – Magenschutz, Blutzucker, Knochendichte, Infektrisiko. Der Magenschutz läuft inzwischen als "), B("Pantoprazol"), T("; er hat einen eigenen Abschnitt weiter unten.")]),
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
        &[B("Das Blutbild gehört dazu."), T(" Metamizol kann eine "), L("Agranulozytose", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/restrictions"), T(" auslösen – sehr selten, unter 1 von 10'000, aber sie kann tödlich verlaufen. Sie ist nicht dosisabhängig, kann jederzeit auftreten, auch nach früher problemloser Einnahme, und noch kurz nach dem Absetzen. Bei Fieber, Schüttelfrost, Halsschmerzen oder schmerzhaften Stellen der Schleimhaut in Mund, Nase oder Rachen ist die Behandlung zu unterbrechen und sofort ein "), B("vollständiges Blutbild mit Differentialblutbild"), T(" zu machen. Ohne Häufigkeitsangabe stehen ausserdem aplastische Anämie und Panzytopenie in der Liste, beide auch mit tödlichem Ausgang. In den deutschen Meldungen traten zwei Drittel der Fälle innert sechs Wochen auf, knapp ein Drittel innert sieben Tagen.")],
        &[B("Damit gehört Novalgin auf die Liste der Erklärungen für den Hämoglobin-Abfall."), T(" Nicht als wahrscheinlichste, aber als eine, die ein Differentialblutbild in einem Schritt mitprüft. Die "), L("Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/restrictions"), T(" nennt ausdrücklich "), B("Blässe"), T(" als Anzeichen, mit dem man zum Arzt soll – daneben Krankheitsgefühl, Infektionszeichen, andauerndes Fieber, Hämatome und Blutungen.")],
        &[B("Blutdruck."), T(" Gelegentlich – 1 von 1000 bis 1 von 100 – löst Metamizol einen isolierten Blutdruckabfall aus. Die "), L("Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/restrictions"), T(" verlangt vorher ausdrücklich die «Optimierung des hämodynamischen Status bei Patienten mit vorbestehender Hypotonie mit Volumenmangel, Dehydratation, instabilem Kreislauf». Das beschreibt genau diese Patientin und ist ein weiterer Grund für die Infusion. In die Vene darf höchstens 1 ml pro Minute laufen.")],
        &[B("Niere."), T(" Sehr selten ein "), L("akutes Nierenversagen", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/unwanted_effects"), T(", «vor allem wenn bereits eine Nierenerkrankung vorliegt», dazu eine akute interstitielle Nephritis ohne Häufigkeitsangabe. Bei einer Krankheit, über deren Verlauf die Niere entscheidet, ist das kein Nebensatz. "), L("Hohe Dosen", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/usage"), T(" sind bei eingeschränkter Nierenfunktion zu vermeiden, und im Alter ist die Nierenfunktion ohnehin mitzudenken.")],
        &[B("Der Kreatininwert kann falsch sein."), T(" Metamizol stört "), L("Labortests, die auf der Trinder-Reaktion beruhen", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/interactions"), T(" – ausdrücklich auch die Messung des "), B("Kreatinins"), T(". Wer die Nierenfunktion im Verlauf beurteilt, muss das wissen.")],
    ]),
    P(&[T("Zwei Dinge, die im Alltag Verwirrung stiften:")]),
    Liste(&[
        &[B("Roter Urin unter Novalgin muss kein Blut sein."), T(" Metamizol bildet "), L("Rubazonsäure", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/other_advice"), T(", ein harmloses Abbauprodukt, das den Harn rot färbt und nach dem Absetzen verschwindet. Weil dieses Blatt die Urinkontrolle in den Mittelpunkt stellt, zählt das doppelt – und ebenso, dass die Frage nicht mit blossem Auge beantwortet wird, sondern mit Streifen und Sediment.")],
        &[B("Magen-Darm-Blutungen stehen trotzdem in der Liste."), T(" Ohne Häufigkeitsangabe, aber die Fachinformation nennt "), L("gastrointestinale Blutungen, Ulzerationen und Perforationen", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/unwanted_effects"), T(" unter den unerwünschten Wirkungen. «Kein NSAR» heisst also nicht «für den Magen unbedenklich».")],
    ]),
    P(&[T("Zuerst zu klären sind zwei "), L("Gegenanzeigen", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/contra_indications"), T(": eine "), B("eingeschränkte Knochenmarksfunktion oder Blutbildungsstörung"), T(" und eine frühere Agranulozytose unter Pyrazolonen. Und falls sie niedrig dosiertes Aspirin zum Herzschutz nimmt: Metamizol schwächt dessen Wirkung auf die Blutplättchen ab.")]),

    H2("Pantoprazol"),
    P(&[T("Pantoprazol ist ein "), B("Protonenpumpenhemmer"), T(". Es blockiert die Säurepumpe der Belegzellen in der Magenwand und senkt damit die Säureproduktion. Dieselbe Substanz heisst in der Schweiz unter anderem Pantozol, Pantoprazol Sandoz, Pantoprazol-Mepha, Pantoprazol Spirig HC oder Pantoprazol Zentiva; es gibt sie als magensaftresistente Tablette, als Granulat und für die Vene.")]),
    P(&[B("Dass es jetzt läuft, ist folgerichtig."), T(" Der Interaktionscheck weiter unten hatte für die Kombination aus Kortison und niedrig dosiertem Aspirin genau das als Massnahme genannt: die vorbeugende Gabe eines Protonenpumpenhemmers. Und der Abschnitt zur Behandlung führt den Magenschutz seit jeher unter dem, was bei Kortison im Alter von Anfang an mitbehandelt gehört. Beides ist damit eingelöst.")]),
    P(&[T("Fünf Punkte aus der Fachinformation, die in dieser Lage zählen:")]),
    Liste(&[
        &[B("Eine Stunde vor dem Essen, ganz, mit Wasser."), T(" Die "), L("Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/usage"), T(" verlangt, die magensaftresistente Tablette unzerkaut und unzerbrochen "), B("1 Stunde vor einer Mahlzeit"), T(" mit etwas Wasser einzunehmen. Das ist keine Formalie: Die Säurepumpen müssen arbeiten, damit das Mittel sie treffen kann. Bei einer Patientin, die jetzt wieder frühstückt, heisst das konkret: zuerst die Tablette mit Wasser, das Frühstück eine Stunde später – nicht mit dem Haferbrei zusammen und nicht mit dem Tee.")],
        &[B("Im Alter höchstens 40 mg am Tag."), T(" Für ältere Patientinnen und bei eingeschränkter Nierenfunktion setzt die "), L("Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/usage"), T(" dieselbe Obergrenze: 40 mg Pantoprazol täglich, nicht mehr.")],
        &[B("Die Magenentzündung steht nicht in der Indikationsliste."), T(" Aufgeführt sind "), L("Refluxkrankheit, Magen- und Zwölffingerdarmgeschwür, die Helicobacter-Behandlung, die Vorbeugung von NSAR-Geschwüren bei erhöhtem Risiko und das Zollinger-Ellison-Syndrom", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/indications"), T(". Eine Gastritis als solche ist nicht darunter. Das ist kein Einwand gegen die Verordnung – bei erodierter Schleimhaut und unter Kortison mit Aspirin ist der Einsatz gängig und hier gut begründet. Es erklärt aber, warum die Fachinformation zur Dauer nichts Passendes sagt. Zwei Fragen bleiben deshalb ausdrücklich zu stellen: wie lange, und woran wird der Erfolg gemessen?")],
        &[B("Eisen kommt schlechter an."), T(" Das Eisen aus pflanzlicher Nahrung – aus Haferflocken etwa – braucht die Magensäure, um in die aufnehmbare Form zu kommen; ein Protonenpumpenhemmer nimmt ihm diese Voraussetzung. In einer "), L("Untersuchung an 43 Patientinnen und Patienten", "https://pmc.ncbi.nlm.nih.gov/articles/PMC9175665/"), T(" mit Eisenmangel unter einem Protonenpumpenhemmer, bei denen keine andere Ursache zu finden war, hatte Eisen zum Schlucken bei 41 nicht angeschlagen; auf Eisen in die Vene stieg der Hämoglobinwert bei 95 Prozent um mindestens "), N("20 g/l"), T(". Bei "), N("83 g/l"), T(" ist das die praktisch wichtigste Zeile dieses Abschnitts: Wird Eisen verordnet, ist der Weg über den Mund unter laufendem Pantoprazol der unsichere.")],
        &[B("Magnesium und Vitamin B12 bei Dauergabe."), T(" Die "), L("Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/restrictions"), T(" nennt schwere Magnesiumerniedrigungen nach mindestens drei Monaten, meist erst nach einem Jahr – Müdigkeit, Krämpfe, Schwindel, Herzrhythmusstörungen, schleichend beginnend und leicht zu übersehen. Für die Aufnahme von Vitamin B12 gilt dasselbe bei langer Anwendung, und die Fachinformation nennt ältere Menschen dabei ausdrücklich. Beides betrifft die Dauer, nicht die ersten Wochen – aber es ist der Grund, warum die Frage nach dem Ende der Behandlung dazugehört.")],
    ]),
    P(&[T("Zwei Dinge, die im Alltag Verwirrung stiften:")]),
    Liste(&[
        &[B("Neue Hautflecken müssen nicht die Vaskulitis sein."), T(" Protonenpumpenhemmer sind in sehr seltenen Fällen mit einem "), L("subakuten kutanen Lupus erythematodes", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/restrictions"), T(" verbunden: Hautveränderungen an den der Sonne ausgesetzten Stellen, begleitet von Gelenkschmerzen. Das sieht dem, was hier ohnehin läuft, zum Verwechseln ähnlich. In einem Blatt, das den Ausschlag als Verlaufszeichen benutzt, gehört das gewusst – neue Flecken an Gesicht, Hals oder Unterarmen sind etwas anderes als die tastbare Purpura an den Unterschenkeln.")],
        &[B("Ein neuer Durchfall ist nicht automatisch die Krankheit."), T(" Weniger Säure heisst mehr Bakterien im oberen Verdauungstrakt; die "), L("Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/restrictions"), T(" nennt ein leicht erhöhtes Risiko für Salmonellen, Campylobacter und Clostridium difficile. Bei einem Darm, dessen Passage gerade erst wieder in Gang gekommen ist, ist das eine zusätzliche Erklärung, die man kennen sollte.")],
    ]),
    P(&[T("Und ein Punkt, der eine Frage aus dem Novalgin-Abschnitt wiederholt: Auch die "), L("Interaktionsangaben von Pantoprazol", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/interactions"), T(" nennen Methotrexat – hochdosiert kann Pantoprazol dessen Spiegel erhöhen und verlängern. Zusammen mit Novalgin ist Methotrexat kontraindiziert. Ob die Patientin Methotrexat nimmt, ist damit eine Frage, an der gleich zwei ihrer Medikamente hängen.")]),

    H2("Was sie jetzt isst und trinkt"),
    P(&[T("Sie isst wieder Haferflocken und trinkt Schwarztee mit Milch, Zimt und Zucker. Das ist zuerst einmal die beste Nachricht seit Wochen: Es geht wieder etwas hinein, und es bleibt drin. Was dazugehört, sind keine Vorschriften zum Speiseplan, sondern zwei Fragen der Uhrzeit.")]),
    P(&[B("Der Tee und das Eisen."), T(" Schwarzer Tee ist einer der stärksten bekannten Hemmer der Eisenaufnahme aus pflanzlicher Nahrung. In einer "), L("Untersuchung mit markiertem Eisen", "https://pubmed.ncbi.nlm.nih.gov/10999016/"), T(" senkte er die Aufnahme aus einer Brotmahlzeit um 79 bis 94 Prozent; die Gerbstoffe binden das Eisen zu einem unlöslichen Komplex. Die Milch ändert daran wenig – dieselbe Arbeit fand für den Zusatz von Milch zu Tee und Kaffee kaum einen Einfluss. Es ist der Tee und nicht der Schluck Milch darin.")]),
    P(&[B("Eine Stunde Abstand halbiert den Effekt."), T(" Eine "), L("kontrollierte Studie", "https://pubmed.ncbi.nlm.nih.gov/29046302/"), T(" hat genau diese Mahlzeit untersucht – Haferbrei mit markiertem Eisen – und den Tee einmal dazu und einmal eine Stunde danach gegeben. Zum Brei getrunken hemmte er die Eisenaufnahme um 37,2 Prozent, eine Stunde danach nur noch um 18,1 Prozent. Bei einem Hämoglobin von "), N("83 g/l"), T(" ist das keine Feinheit. Zusammen mit der Einnahmevorschrift für Pantoprazol ergibt das eine einfache Reihenfolge für den Morgen: zuerst die Tablette mit Wasser, eine Stunde später die Haferflocken, den Tee zuletzt.")]),
    P(&[B("Der Zucker ist hier kein Einwand."), T(" Nach Wochen ohne Nahrung sind Kalorien, die freiwillig hineingehen, ein Gewinn. Was in diesen Tagen zählt, ist nicht die Menge Zucker, sondern das, was schon im Abschnitt zum Kostaufbau steht: Phosphat, Kalium und Magnesium gehören kontrolliert.")]),
    P(&[B("Zum Zimt eine Zahl, damit die Frage vom Tisch ist."), T(" Cassia-Zimt – der übliche Haushaltszimt – enthält Cumarin, im Mittel rund 3 Gramm je Kilogramm Zimt, im Höchstfall 10. Die duldbare tägliche Aufnahme liegt "), L("nach Einschätzung des deutschen Bundesinstituts für Risikobewertung", "https://www.bfr.bund.de/de/service/haeufig-gestellte-fragen/thema/faq-zu-cumarin-in-zimt-und-anderen-lebensmitteln/"), T(" bei 0,1 mg Cumarin je Kilogramm Körpergewicht und Tag; für 60 kg Körpergewicht sind das rund 2 Gramm Cassia-Zimt am Tag, also etwa ein gestrichener Teelöffel. Eine Prise im Tee bleibt weit darunter. Dieselbe Stelle hält ausdrücklich fest, dass bislang kein Fall von Leberschaden nach dem Verzehr von Zimt beschrieben ist. Ceylon-Zimt enthält ohnehin nur wenig Cumarin.")]),

    H2("Interaktionscheck"),
    P(&[T("Geprüft mit "), L("SDIF", "https://sdif.oddb.org"), T(", dem Swiss Drug Interaction Finder: Er wertet die Interaktionsangaben aus den Schweizer Fachinformationen aus und gleicht sie mit der EPha-Datenbank ab, die jede Kombination von A bis X einstuft – A keine Massnahmen, C regelmässige Überwachung, D Kombination vermeiden, X kontraindiziert. "), L("Der Lauf vom 28. August 2026", "https://sdif.oddb.org/?tab=check&drugs=N02BB02-A02BC02-H02AB06-A06AD65-A06AD11-B01AC06"), T(" mit Novalgin, Pantoprazol, Kortison, Macrogol, Lactulose und niedrig dosiertem Aspirin ergab Folgendes. Das Werkzeug und der Lauf selbst, mit fertig gefülltem Warenkorb, stehen hier – der zweite Link setzt den Korb über die ATC-Codes zusammen, weshalb für die Macrogol-Klasse ein anderes Präparat derselben Klasse angezeigt werden kann.")]),
    P(&[
        B("Der Vorbehalt zuerst:"), T(" Geprüft ist nur, was auf diesem Blatt steht. Die vollständige Medikamentenliste kennt nur die Patientin selbst, und sie gehört zum Termin mitgebracht – rezeptfreie Mittel eingeschlossen. Ein maschineller Check kann nur vergleichen, was man ihm gibt."),
    ]),
    Tab(&T_INTERAKTION),
    H3("Was bei Kortison dazukommt"),
    Liste(&[
        &[B("Aspirin und Kortison: Klasse C."), T(" Erhöhtes Risiko einer Blutung im Magen-Darm-Trakt durch additive Schädigung der Magenschleimhaut. Das Risiko steigt ausdrücklich mit höherem Lebensalter, mit einer Vorgeschichte von Geschwüren und unter gleichzeitiger Blutverdünnung – alle drei gehören hier geprüft. Empfohlene Massnahme: klinische Überwachung auf Geschwüre und die "), B("vorbeugende Gabe eines Protonenpumpenhemmers"), T(". Genau das ist inzwischen geschehen – siehe den Abschnitt zu Pantoprazol. Die Empfehlung des Checks und die Verordnung decken sich hier also.")],
        &[B("Lactulose und Kortison."), T(" Kortikosteroide senken das Kalium, und die "), L("Duphalac-Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/32894/chapter/interactions"), T(" nennt Lactulose als Mittel, das diesen Kaliumverlust theoretisch verstärkt. Bei einer Patientin, deren Kalium nach Wochen ohne Nahrung ohnehin überwacht gehört, ist das ein weiteres Argument für Macrogol.")],
    ]),
    H3("Ohne Treffer"),
    P(&[T("Novalgin mit Macrogol, mit Lactulose, mit Paracetamol und mit einem ACE-Hemmer: kein Treffer. Und Pantoprazol, jetzt selbst im Warenkorb, hat mit keinem der übrigen Mittel einen – weder mit Novalgin noch mit Kortison, Macrogol oder Lactulose meldet die Fachinformation oder EPha etwas. Macrogol hat in der EPha-Datenbank überhaupt keinen Eintrag – es wird nicht aufgenommen und interagiert praktisch nicht. Auch das spricht für Movicol neutral.")]),
    H3("Ein Fund am Rande, der hier zählt"),
    P(&[T("Beim Auflösen des Namens «Paracetamol» griff der Check auf Kombinationspräparate zu, die zusätzlich Tramadol oder Codein enthalten; die dortigen Warnungen betrafen den Opioid-Anteil und nicht das Paracetamol. Als Interaktion ist das ein Fehlalarm – als Hinweis ist es der praktisch wichtigste des ganzen Laufs: "), B("Opioide verstopfen."), T(" Die "), L("Duphalac-Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/32894/chapter/interactions"), T(" führt Opiate ausdrücklich unter den Substanzen, die die Wirkung von Lactulose abschwächen, weil sie selbst obstipierend wirken. Bei einer Patientin, die seit Wochen keinen Stuhlgang hat, ist ein opioidhaltiges Schmerzmittel – Codein, Tramadol – das Falsche. Novalgin enthält keines, und das ist ein Punkt zu seinen Gunsten.")]),
    P(&[T("Drei weitere Treffer des Laufs sind ebenfalls keine. Der Check meldete «kontraindiziert» zwischen Aspirin und Movicol, weil er im Namen «Macrogol, Kombinationen» das Wort «Kombinationen» als Wirkstoff las. Er ordnete Novalgin einer Regel für Aspirin und NSAR zu, obwohl Metamizol keines von beiden ist. Und er stellte Pantoprazol gegen Aspirin, weil die "), L("Fachinformation von Pantoprazol", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/interactions"), T(" dort Antikoagulantien nennt – gemeint sind Cumarine wie Phenprocoumon und Warfarin und deren INR, nicht niedrig dosiertes Aspirin. Wer maschinell prüft, muss die Treffer nachlesen; die Fachinformation entscheidet, nicht die Trefferliste.")]),

    H2("Abführmittel: welche es gibt und warum sie süss sind"),
    P(&[T("Vorbemerkung, weil sie alles andere überwiegt: Solange nicht abgeklärt ist, warum die Passage eine Woche lang stand, ist die Frage nach dem richtigen Abführmittel die zweite Frage. Die erste steht im Abschnitt oben. Dass wieder Stuhl abgeht, beantwortet sie nicht – es macht die Untersuchung nur möglich. Was hier folgt, gilt für die Zeit danach – und für den Fall, dass ein Mittel bereits verordnet ist und schlecht vertragen wird.")]),
    P(&[T("Dass ein Abführmittel zum Trinken süss ist, ist kein Zufall des Herstellers. Bei der einen Gruppe ist der Wirkstoff selbst ein Zucker; bei der anderen ist die Süsse ein Zusatz – und den gibt es auch ohne.")]),
    P(&[
        B("Lactulose ist der Zucker."), T(" "), L("Duphalac", "https://ch.oddb.org/de/gcc/fachinfo/reg/32894/chapter/composition"), T(", "), L("Gatinar", "https://ch.oddb.org/de/gcc/fachinfo/reg/37585/chapter/composition"), T(" und "), L("Rudolac", "https://ch.oddb.org/de/gcc/fachinfo/reg/51067/chapter/composition"), T(" sind Lactulose-Sirup. Die Fachinformation nennt unter Hilfsstoffen: keine. Süss ist hier nicht ein Zusatz, sondern der Wirkstoff, und daran lässt sich nichts ändern. Lactulose wird nicht aufgenommen, sondern im Dickdarm von Bakterien "), L("vergoren", "https://ch.oddb.org/de/gcc/fachinfo/reg/32894/chapter/effects"), T(" – dabei entstehen Gase. Blähungen sind deshalb kein Nebeneffekt, sondern das Stoffwechselprodukt. In den Zulassungsstudien war "), L("Durchfall sehr häufig (13,1 Prozent)", "https://ch.oddb.org/de/gcc/fachinfo/reg/32894/chapter/unwanted_effects"), T(", Flatulenz, Bauchschmerzen, Übelkeit und Erbrechen häufig. "), L("Importal", "https://ch.oddb.org/de/gcc/fachinfo/reg/52785/chapter/composition"), T(" (Lactitol) ist ein Zuckeralkohol und funktioniert nach demselben Prinzip."),
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
        &[B("Bei Darmverschluss verboten."), T(" Alle diese Mittel führen "), L("intestinale Obstruktion, Ileus und Perforation", "https://ch.oddb.org/de/gcc/fachinfo/reg/58420/chapter/contra_indications"), T(" als "), L("Gegenanzeige", "https://ch.oddb.org/de/gcc/fachinfo/reg/32894/chapter/contra_indications"), T(". Duphalac verlangt darüber hinaus "), L("ausdrücklich", "https://ch.oddb.org/de/gcc/fachinfo/reg/32894/chapter/restrictions"), T(", dass schmerzhafte Bauchsymptome unklarer Ursache "), B("vor"), T(" Behandlungsbeginn abgeklärt werden, um eine nicht diagnostizierte Obstruktion auszuschliessen. Nach wochenlang fehlendem Stuhlgang ist das keine Formalie, auch jetzt nicht, wo die Passage wieder offen ist.")],
        &[B("Ohne Flüssigkeit keine Wirkung."), T(" Duphalac empfiehlt während einer Abführbehandlung "), L("1,5 bis 2 Liter am Tag", "https://ch.oddb.org/de/gcc/fachinfo/reg/32894/chapter/usage"), T(". Für Movicol "), L("steht ausdrücklich", "https://ch.oddb.org/de/gcc/fachinfo/reg/58420/chapter/restrictions"), T(", dass die zubereitete Lösung die reguläre Flüssigkeitszufuhr "), B("nicht ersetzt"), T("; ein Beutel wird in 125 ml Wasser gelöst. Wer kaum trinkt, dem hilft das Mittel wenig – und die Austrocknung trifft ausgerechnet die Niere.")],
        &[B("Im Alter die kleinere Dosis."), T(" Für Menschen über 65 genügt bei Movicol laut "), L("Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/58420/chapter/usage"), T(" normalerweise ein Beutel täglich statt ein bis zwei.")],
        &[B("Die neutrale Variante lässt sich vorbereiten."), T(" Die zubereitete Lösung von Movicol neutral ist "), L("im Kühlschrank 24 Stunden haltbar", "https://ch.oddb.org/de/gcc/fachinfo/reg/58420/chapter/other_advice"), T(", die der aromatisierten Varianten nur 6 – die neutrale kann also am Vorabend angesetzt und kalt getrunken werden.")],
    ]),
    Alarm {
        titel: "Erbrechen von Galle ist kein Geschmacksproblem",
        blocks: &[
            P(&[T("Grün-gelbe Galle zu erbrechen heisst, dass Darminhalt rückwärts läuft. Bei Bauchbeteiligung einer Vaskulitis und gleichzeitig fallendem Hämoglobin ist das ein Warnzeichen für einen "), B("Subileus"), T(" – die entzündete, geschwollene Darmwand behindert die Passage. Ein Darmwandödem mit Passagestörung ist eine bekannte Komplikation genau dieser Krankheit.")]),
            P(&[T("Trifft das zu, ist ein Abführmittel durch den Mund nicht nur wirkungslos, sondern falsch: Man drückt Flüssigkeit gegen einen Engpass. Dann hilft auch der Wechsel auf Macrogol nichts. Dazu kommen die Aspirationsgefahr beim Erbrechen und der Flüssigkeitsverlust – der trifft ausgerechnet die Niere, das Organ, das hier ohnehin gefährdet ist.")]),
            P(&[B("Zwei Fragen gehören heute geklärt, nicht nächste Woche.")]),
            P(&[B("Wofür ist das Abführmittel gedacht?"), T(" Ist es die Vorbereitung auf die Darmspiegelung, ist Erbrechen unter der Spülung zwar häufig – aber mit Galle und Aufstossen gehört die Vorbereitung abgebrochen und die Ärztin informiert, nicht durchgezogen. Ist es gegen gewöhnliche Verstopfung, gilt dieselbe Abklärungspflicht.")]),
            P(&[B("Gehen Winde und Stuhl noch ab?"), T(" Diese Frage ist im Moment beantwortet – es geht wieder Stuhl ab. Sie bleibt der Prüfstein: Hört es wieder auf, ist das der Notfall und nicht die Sprechstunde.")]),
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
        &[T("Fotos des Ausschlags mit Datum. Purpura heilt oft schneller ab, als ein Termin zustande kommt – der Ausschlag geht bereits zurück, was jetzt nicht fotografiert wird, ist weg.")],
        &[T("Angaben zum früheren Schub: wann, wie lange, was wurde gemacht, wurde der Urin kontrolliert, gab es eine Biopsie")],
        &[T("Bisherige Urin- und Nierenwerte, den letzten Befund im Wortlaut – mit der Angabe, welche Untersuchungen darin enthalten waren")],
        &[T("Wann der Stuhlgang wieder eingesetzt hat, wie der erste aussah, wie er sich seither verändert – und ob ihn jemand vom Fach gesehen hat")],
        &[T("Den Befund zur Magenentzündung im Wortlaut: wie und wo sie festgestellt wurde, ob eine Spiegelung stattgefunden hat, ob Gewebeproben entnommen wurden und aus welchem Abschnitt")],
        &[T("Wann Pantoprazol begonnen hat, in welcher Dosis, und zu welcher Tageszeit es im Verhältnis zum Frühstück eingenommen wird")],
        &[T("Den Urinbefund im Wortlaut: seit wann Blut sichtbar ist, ob ein Sediment untersucht wurde und was darin stand – Akanthozyten, Erythrozytenzylinder, Protein-Kreatinin-Quotient")],
        &[T("Den Befund zum gestauten Harnleiter: womit er erhoben wurde, an welchen Stellen und auf welcher Seite, und ob es eine frühere Aufnahme zum Vergleich gibt")],
        &[T("Die Transfusion mit Datum: wie viele Einheiten, und der Hämoglobinwert davor und am Tag danach")],
        &[T("Den Bericht der Darmspiegelung im Wortlaut – wie weit das Gerät kam, wie die Vorbereitung beurteilt wurde und was zum Transport festgehalten ist")],
    ]),

    H2("Fragen, die sich lohnen"),
    Liste(&[
        &[T("Nach einer Woche ohne Stuhlgang geht die Passage wieder – ist damit geklärt, warum sie stand, oder steht die Untersuchung des Bauches weiter aus?")],
        &[T("Der Stuhl ist schwarz und flüssig: Hat jemand vom Fach ihn gesehen und als Teerstuhl beurteilt – und ist er dokumentiert?")],
        &[T("Läuft ein Eisenpräparat oder ein Wismutpräparat? Beide färben den Stuhl ebenfalls schwarz – das Erbrochene aber nicht.")],
        &[T("Kaffeesatz oben und Teerstuhl unten: Ist damit die Magenspiegelung erneut angesetzt, und diesmal mit Biopsien aus dem absteigenden Zwölffingerdarm?")],
        &[T("Sie erbricht – kommt Pantoprazol als Tablette überhaupt an, oder gehört es in die Vene?")],
        &[T("Teerstuhl ist das Leitzeichen der oberen Blutung, und beide Fachgesellschaften nennen dafür 24 Stunden bis zur Magenspiegelung – wann ist sie angesetzt?")],
        &[T("Woher kommt der weisse Schleim? Wurde bei der Darmspiegelung bis in den Krummdarm geschaut, wo der Befall am häufigsten ist – und wie sauber war der Darm?")],
        &[T("Die Darmspiegelung war unauffällig, der Stuhl ist schwarz: Damit ist der Dickdarm als Quelle erledigt, der Teerstuhl aber nicht erklärt. Was ist für den Dünndarm dazwischen vorgesehen – Kapselendoskopie, CT-Enterografie?")],
        &[T("Der Darm transportiert nicht richtig, ohne mechanisches Hindernis: Sind Kalium und Magnesium ausgeglichen, und ist der TSH-Wert bestimmt?")],
        &[T("Wird die fehlende Peristaltik der Vaskulitis zugerechnet – und wenn ja, ändert das etwas an der Behandlung?")],
        &[T("Sie stösst Luft auf bis zum Würgen: Gibt es etwas, das den Transport unterstützt, und verträgt es sich mit den übrigen Mitteln?")],
        &[T("Wie stehen Harnstoff und Kreatinin aus derselben Blutentnahme zueinander?")],
        &[T("Es ist Blut im Urin: Wurde ein Sediment untersucht, und kommt das Blut aus der Niere oder aus den ableitenden Harnwegen – Akanthozyten, Erythrozytenzylinder?")],
        &[T("Der Harnleiter ist an mehreren Stellen gestaut – wodurch? Gerinnsel, die Vaskulitis selbst, Druck von aussen durch den geblähten Darm, oder etwas Urologisches?")],
        &[T("Die Füsse sind geschwollen, und das war beim früheren Schub schon so: Wie hoch ist das Albumin im Blut, und wie hoch der Eiweissverlust im Urin?")],
        &[T("Gibt es aus der Zeit des früheren Schubs Urinbefunde, Blutdruckwerte oder einen Albuminwert?")],
        &[T("Staut es einseitig oder beidseitig, und wie hat sich das Kreatinin seither entwickelt?")],
        &[T("Lässt sich die Computertomografie des Bauches als CT-Urografie fahren, damit Darm und Harnwege in einem Durchgang beantwortet sind?")],
        &[T("Kommt eine stenosierende Ureteritis infrage – die Harnleiterbeteiligung dieser Vaskulitis? Sie ist selten und fast nur bei Kindern beschrieben, aber sie spricht auf Kortison an.")],
        &[T("Wird der Ultraschall der Harnwege wiederholt, auch wenn er zwischendurch normal ausfällt?")],
        &[T("Wie hoch war das Hämoglobin vor der Transfusion, und wie hoch ist es am Tag danach? Bleibt der erwartete Anstieg von rund 10 g/l je Einheit aus, blutet es weiter.")],
        &[T("Wird Einheit für Einheit transfundiert und dazwischen neu beurteilt – und ist bei 84 Jahren an die Kreislaufüberlastung gedacht, samt Bilanz mit der laufenden Infusion?")],
        &[T("Ist eine Computertomografie des Bauches geplant, und wann?")],
        &[T("Wie wird die Flüssigkeitszufuhr sichergestellt, wenn kaum getrunken wird und Erbrechen dazukommt?")],
        &[T("Nach Wochen ohne Nahrung: Wie wird der Kostaufbau begleitet, und werden Phosphat, Kalium und Magnesium dabei kontrolliert?")],
        &[T("Sie bekommt Novalgin: Wird darunter das Differentialblutbild kontrolliert, und sind Knochenmarkserkrankung und frühere Agranulozytose ausgeschlossen?")],
        &[T("Nimmt sie Methotrexat, Clozapin oder Carbamazepin? Die ersten beiden sind zusammen mit Novalgin kontraindiziert, beim dritten steigt das Agranulozytoserisiko.")],
        &[T("Falls Aspirin zum Herzschutz läuft: Wird es 30 bis 60 Minuten vor Novalgin gegeben, und ist unter Kortison ein Magenschutz verordnet?")],
        &[T("Kann der Kreatininwert durch Metamizol verfälscht sein – und wie wird die Nierenfunktion dann beurteilt?")],
        &[T("Wofür ist das Abführmittel verordnet – gegen Verstopfung oder als Vorbereitung der Darmspiegelung? Und ist es in dieser Lage überhaupt zulässig?")],
        &[T("Kann statt eines aromatisierten Präparats Movicol neutral oder Laxipeg aromafrei verschrieben werden – ohne Aroma und ohne Süssstoff?")],
        &[T("Jetzt, wo die Passage wieder offen ist: Ist die Magen- und Darmspiegelung angesetzt, und wann?")],
        &[T("Die Magenentzündung ist festgestellt – woher kommt sie? Von der Vaskulitis, von der Kombination aus Kortison, Aspirin und Novalgin, oder von Helicobacter pylori?")],
        &[T("Wurde bei der Spiegelung bis in den absteigenden Zwölffingerdarm geschaut und dort biopsiert? Dort sitzt die Vaskulitis, im Magen selbst nur selten.")],
        &[T("Ist auf Helicobacter pylori getestet worden, und mit welchem Ergebnis?")],
        &[T("Die Fachinformation von Pantoprazol verlangt bei Gewichtsverlust, wiederholtem Erbrechen und Blutarmut den Ausschluss einer bösartigen Erkrankung, weil das Mittel die Symptome kaschieren kann – ist das vorgesehen?")],
        &[T("Wird Pantoprazol eine Stunde vor dem Frühstück eingenommen, mit Wasser und nicht mit dem Tee?")],
        &[T("Wie lange soll Pantoprazol laufen, und woran wird entschieden, wann es aufhört?")],
        &[T("Falls Eisen verordnet wird: Über den Mund oder in die Vene? Unter einem Protonenpumpenhemmer ist die Aufnahme über den Mund unsicher.")],
        &[T("Wie hoch ist der Eiweissverlust im Urin, gemessen als Protein-Kreatinin-Quotient?")],
        &[T("Wurde ANCA bestimmt, also die im Alter häufigere Vaskulitisform ausgeschlossen?")],
        &[T("Der Ausschlag geht zurück: Wird die Hautbiopsie jetzt gemacht, solange noch frische Flecken da sind?")],
        &[T("Ist bei diesem Verlauf eine Nierenbiopsie angezeigt?")],
        &[T("Welche der aktuellen Medikamente belasten Magen oder Niere?")],
        &[T("Wie oft und über welchen Zeitraum werden Urin und Blutdruck kontrolliert – mindestens sechs Monate?")],
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
//
// Hier steht je ein Eintrag pro Praeparat als Register. Das einzelne
// Kapitel ist im Text verlinkt, direkt bei der Aussage, die es belegt:
//   .../fachinfo/reg/<Swissmedic-Nr>/chapter/<kapitel>
// Die Kapitel heissen composition, galenic_form, indications, usage,
// contra_indications, restrictions (Warnhinweise), interactions, pregnancy,
// driving_ability, unwanted_effects, overdose, effects, kinetic, preclinic,
// other_advice, iksnrs, packages, registration_owner, date.

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
    ("Gong Y-Q et al.: Abdominal imaging and endoscopic characteristics of adult abdominal IgA vasculitis. Ann Med 2024. PMC11429444",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC11429444/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC11429444/" }),
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
    ("Zum Stress als Schubauslöser: Examining the role of patient-reported external factors and risk of relapse in ANCA vasculitis, 2011–2022. PMC11249541",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC11249541/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC11249541/" }),
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
    ("Hurrell RF, Reddy M, Cook JD: Inhibition of non-haem iron absorption in man by polyphenolic-containing beverages. Br J Nutr 1999; 81: 289-295",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/10999016/", url: "https://pubmed.ncbi.nlm.nih.gov/10999016/" }),
    ("Ahmad Fuzi SF et al.: A 1-h time interval between a meal containing iron and consumption of tea attenuates the inhibitory effects on iron absorption. Am J Clin Nutr 2017; 106: 1413-1421",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/29046302/", url: "https://pubmed.ncbi.nlm.nih.gov/29046302/" }),
    ("Boxer M.: Iron deficiency anemia from iron malabsorption caused by proton pump inhibitors. eJHaem 2020; 1: 548-551. PMC9175665",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9175665/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9175665/" }),
    ("Zum Cumarin im Zimt: FAQ zu Cumarin in Zimt und anderen Lebensmitteln. Bundesinstitut für Risikobewertung",
     Verweis { text: "bfr.bund.de – FAQ zu Cumarin in Zimt und anderen Lebensmitteln", url: "https://www.bfr.bund.de/de/service/haeufig-gestellte-fragen/thema/faq-zu-cumarin-in-zimt-und-anderen-lebensmitteln/" }),
    ("Zum nachlassenden Durstgefühl im Alter: Adult Dehydration. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK555956/", url: "https://www.ncbi.nlm.nih.gov/books/NBK555956/" }),
    ("Zum Zeitpunkt der Nierenbeteiligung und zur Dauer der Nachkontrolle: Henoch-Schönlein Purpura (IgA Vasculitis) – Rapid Evidence Review. Am Fam Physician 2020",
     Verweis { text: "https://www.aafp.org/pubs/afp/issues/2020/0815/p229.html", url: "https://www.aafp.org/pubs/afp/issues/2020/0815/p229.html" }),
    ("Zum Streifentest bei verdünntem Urin: The efficacy of semi-quantitative urine protein-to-creatinine ratio for the detection of significant proteinuria. PMC5063823",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC5063823/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC5063823/" }),
    ("Zur Farbe des Stuhls: Physiology, Bilirubin. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK470290/", url: "https://www.ncbi.nlm.nih.gov/books/NBK470290/" }),
    ("Zum Teerstuhl: Hematemesis, Melena, and Hematochezia. Clinical Methods, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK411/", url: "https://www.ncbi.nlm.nih.gov/books/NBK411/" }),
    ("Srygley FD et al.: Does this patient have a severe upper gastrointestinal bleed? JAMA 2012; 307: 1072–1079",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/22416103/", url: "https://pubmed.ncbi.nlm.nih.gov/22416103/" }),
    ("Simadibrata DM et al.: Blood Urea Nitrogen-to-Creatinine Ratio to Differentiate Upper From Lower Gastrointestinal Bleeding. J Gastroenterol Hepatol 2026",
     Verweis { text: "https://onlinelibrary.wiley.com/doi/10.1111/jgh.70224", url: "https://onlinelibrary.wiley.com/doi/10.1111/jgh.70224" }),
    ("Zu den Einheiten des Harnstoff-Kreatinin-Verhältnisses: Urea-Creatinine Ratio. Life in the Fast Lane",
     Verweis { text: "https://litfl.com/urea-creatinine-ratio/", url: "https://litfl.com/urea-creatinine-ratio/" }),
    ("Lee-Robichaud H et al.: Lactulose versus Polyethylene Glycol for Chronic Constipation. Cochrane Database Syst Rev 2010; CD007570",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/20614462/", url: "https://pubmed.ncbi.nlm.nih.gov/20614462/" }),
    ("SDIF – Swiss Drug Interaction Finder: der Lauf dieses Blattes, mit gefülltem Warenkorb",
     Verweis { text: "https://sdif.oddb.org/?tab=check&drugs=N02BB02-A02BC02-H02AB06-A06AD65-A06AD11-B01AC06", url: "https://sdif.oddb.org/?tab=check&drugs=N02BB02-A02BC02-H02AB06-A06AD65-A06AD11-B01AC06" }),
    ("SDIF – Interaktionscheck auf Basis der Schweizer Fachinformationen und EPha",
     Verweis { text: "https://sdif.oddb.org", url: "https://sdif.oddb.org" }),
    ("EPha.ch – die abgestuften Interaktionsdaten hinter dem Check",
     Verweis { text: "https://epha.ch", url: "https://epha.ch" }),
    ("Novalgin Tropfen (Metamizol), vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/16952", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/16952" }),
    ("Novalgin-F Tabletten (Metamizol), vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/16951", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/16951" }),
    ("Pantoprazol Sandoz (Pantoprazol), vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/58350", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/58350" }),
    ("Pantozol (Pantoprazol), vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/52710", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/52710" }),
    ("Movicol, Movicol neutral, Movicol Chocolat (Macrogol 3350)",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/58420", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/58420" }),
    ("Laxipeg banane, Laxipeg aromafrei (Macrogol 4000)",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/62765", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/62765" }),
    ("Transipeg, Transipeg forte (Macrogol 3350)",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/53282", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/53282" }),
    ("Duphalac (Lactulose)",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/32894", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/32894" }),
    ("Gatinar (Lactulose)",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/37585", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/37585" }),
    ("Rudolac (Lactulose)",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/51067", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/51067" }),
    ("Importal (Lactitol)",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/52785", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/52785" }),
    ("Moviprep (Darmspiegelung)",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/57900", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/57900" }),
    ("Picoprep (Darmspiegelung)",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/62754", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/62754" }),
    ("Cololyt (Darmspiegelung)",
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
    ("Rüdiger JJ, Eriksson U, Schiller P, Leuppi JD: Purpura Schönlein-Henoch bei einer 80-jährigen Frau. Dtsch Med Wochenschr 2002; 127: 1719-22",
     Verweis { text: "https://doi.org/10.1055/s-2002-32350", url: "https://doi.org/10.1055/s-2002-32350" }),
    ("Ueda H et al.: Clinical and histopathologic features of adult-onset IgA vasculitis in the elderly. Intern Med 2019; 58: 31-38. PMC6367074",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6367074/", url: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6367074/" }),
    ("Sande Miguel T et al.: Henoch-Schönlein Purpura in an Elderly Patient. J Biosci Med 2017; 5: 37-45",
     Verweis { text: "https://doi.org/10.4236/jbm.2017.56004", url: "https://doi.org/10.4236/jbm.2017.56004" }),
    ("Nishikura N et al.: IgA Vasculitis in a 90-Year-Old Woman Treated with Azathioprine. Cureus 2022; 14: e28996. PMC9549259",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC9549259/", url: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC9549259/" }),
    ("Kukrety S et al.: Henoch Schonlein Purpura as a Cause of Renal Failure in an Adult. Case Rep Med 2016; 2016: 7890379. PMC5031831",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC5031831/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC5031831/" }),
    ("Kamiya K et al.: IgA vasculitis in a 97-year-old woman. J Dermatol 2015",
     Verweis { text: "https://doi.org/10.1111/1346-8138.12747", url: "https://doi.org/10.1111/1346-8138.12747" }),
    ("Sugimoto H et al.: Successful treatment of IgA vasculitis with mizoribine in an elderly woman. CEN Case Rep 2021; 10: 46-52",
     Verweis { text: "https://doi.org/10.1007/s13730-020-00513-6", url: "https://doi.org/10.1007/s13730-020-00513-6" }),
    ("IgA vasculitis with gastrointestinal and neurologic involvement in an adult. Cureus. DOI 10.7759/cureus.34422",
     Verweis { text: "https://doi.org/10.7759/cureus.34422", url: "https://doi.org/10.7759/cureus.34422" }),
    ("Pillebout E et al.: Henoch-Schönlein Purpura in adults: outcome and prognostic factors. JASN 2002; 13: 1271-78. PMID 11961015",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/11961015/", url: "https://pubmed.ncbi.nlm.nih.gov/11961015/" }),
    ("Audemard-Verger A et al.: Characteristics and management of IgA vasculitis (Henoch-Schönlein) in adults: data from the 260-patient IGAVAS study. Arthritis Rheumatol 2017. PMID 28605168",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/28605168/", url: "https://pubmed.ncbi.nlm.nih.gov/28605168/" }),
    ("Audemard-Verger A et al.: Impact of aging on phenotype and prognosis in IgA vasculitis. Rheumatology 2021; 60: 4245-51. PMID 33410479",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/33410479/", url: "https://pubmed.ncbi.nlm.nih.gov/33410479/" }),
    ("Komatsu H et al.: Distinct characteristics and outcomes in elderly-onset IgA vasculitis with nephritis. Japan Renal Biopsy Registry. PLoS One 2018; 13: e0196955. PMID 29738576",
     Verweis { text: "https://doi.org/10.1371/journal.pone.0196955", url: "https://doi.org/10.1371/journal.pone.0196955" }),
    ("Pillebout E et al.: Addition of cyclophosphamide to corticosteroids for adult HSP (CESAR). Kidney Int 2010; 78: 495-502. PMID 20505654",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/20505654/", url: "https://pubmed.ncbi.nlm.nih.gov/20505654/" }),
    ("Köhler H, Wandel E, Brunck B: Acanthocyturia – a characteristic marker for glomerular bleeding. Kidney Int 1991; 40: 115-20. PMID 1921146",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/1921146/", url: "https://pubmed.ncbi.nlm.nih.gov/1921146/" }),
    ("Maherzi A et al.: Urétérite sténosante bilatérale au décours du purpura rhumatoïde. Arch Pédiatr 1997; 4: 36-9. PMID 9091100",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/9091100/", url: "https://pubmed.ncbi.nlm.nih.gov/9091100/" }),
    ("Kher KK, Sheth KJ, Makker SP: Stenosing ureteritis in Henoch-Schönlein purpura. J Urol 1983; 129: 1040-2. PMID 6854750",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/6854750/", url: "https://pubmed.ncbi.nlm.nih.gov/6854750/" }),
    ("Siomou E et al.: Masked severe stenosing ureteritis – a rare complication of Henoch-Schönlein purpura. Pediatr Nephrol 2008; 23: 821-5. PMID 18219497",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/18219497/", url: "https://pubmed.ncbi.nlm.nih.gov/18219497/" }),
    ("Microhematuria: AUA/SUFU Guideline. American Urological Association",
     Verweis { text: "auanet.org – Microhematuria: AUA/SUFU Guideline", url: "https://www.auanet.org/guidelines-and-quality/guidelines/microhematuria" }),
    ("Carson JL et al.: Red Blood Cell Transfusion – 2023 AABB International Guidelines. JAMA 2023; 330: 1892-1902. PMID 37824153",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/37824153/", url: "https://pubmed.ncbi.nlm.nih.gov/37824153/" }),
    ("Villanueva C et al.: Transfusion strategies for acute upper gastrointestinal bleeding. N Engl J Med 2013; 368: 11-21. PMID 23281973",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/23281973/", url: "https://pubmed.ncbi.nlm.nih.gov/23281973/" }),
    ("Transfusion-associated circulatory overload (TACO). Australian Red Cross Lifeblood",
     Verweis { text: "lifeblood.com.au – TACO", url: "https://www.lifeblood.com.au/health-professionals/clinical-practice/adverse-events/TACO" }),
    ("Single unit blood transfusions. NHS Blood and Transplant",
     Verweis { text: "hospital.blood.co.uk – Single unit blood transfusions", url: "https://hospital.blood.co.uk/patient-services/patient-blood-management/appropriate-use-of-blood-components/single-unit-blood-transfusions/" }),
    ("Single Unit Transfusion of Red Cells – Guidance Resource. NHS National Services Scotland",
     Verweis { text: "nss.nhs.scot – Single unit transfusion guidance", url: "https://www.nss.nhs.scot/media/6134/single_unit_transfusion_guidance-draft_14final-pdf.pdf" }),
    ("Gralnek IM et al.: Endoscopic diagnosis and management of nonvariceal upper gastrointestinal hemorrhage. ESGE Guideline Update 2021. Endoscopy 2021; 53: 300-332. PMID 33567467",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/33567467/", url: "https://pubmed.ncbi.nlm.nih.gov/33567467/" }),
    ("Laine L et al.: ACG Clinical Guideline – Upper Gastrointestinal and Ulcer Bleeding. Am J Gastroenterol 2021; 116: 899-917",
     Verweis { text: "https://doi.org/10.14309/ajg.0000000000001245", url: "https://doi.org/10.14309/ajg.0000000000001245" }),
    ("Zum Schleim im Stuhl: Mucus in Stool. Cleveland Clinic",
     Verweis { text: "my.clevelandclinic.org – Mucus in Stool", url: "https://my.clevelandclinic.org/health/symptoms/mucus-in-stool" }),
    ("Hokama A et al.: Endoscopic and radiographic features of gastrointestinal involvement in vasculitis. World J Gastrointest Endosc 2012; 4: 50-6. PMC3309893",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC3309893/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC3309893/" }),
    ("De Giorgio R, Camilleri M: Inflammatory neuropathies of the enteric nervous system. Gastroenterology 2004",
     Verweis { text: "gastrojournal.org – Inflammatory neuropathies of the enteric nervous system", url: "https://www.gastrojournal.org/article/S0016-5085(04)00223-9/fulltext" }),
    ("Yıldırım F et al.: Disease Course and Long-Term Outcomes in Adult IgA Vasculitis Nephritis. Diagnostics 2025; 15: 957. PMC12025811",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12025811/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12025811/" }),
    ("Joo N, Lee HS: Acute hydronephrosis owing to a giant fecaloma in an older patient. Ann Geriatr Med Res 2020; 24: 223-6",
     Verweis { text: "https://doi.org/10.4235/agmr.20.0052", url: "https://doi.org/10.4235/agmr.20.0052" }),
    ("Spiricort (Prednisolon), vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/38840", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/38840" }),
    ("Overview of Gastrointestinal Bleeding. Merck Manual Professional Edition",
     Verweis { text: "merckmanuals.com – Overview of Gastrointestinal Bleeding", url: "https://www.merckmanuals.com/professional/gastrointestinal-disorders/gastrointestinal-bleeding/overview-of-gastrointestinal-bleeding" }),
    ("Kanno T et al.: Proton pump inhibitor treatment initiated prior to endoscopic diagnosis in upper gastrointestinal bleeding. Cochrane Database Syst Rev 2022; CD005415",
     Verweis { text: "https://doi.org/10.1002/14651858.CD005415.pub4", url: "https://doi.org/10.1002/14651858.CD005415.pub4" }),
    ("Gerson LB et al.: ACG Clinical Guideline – Diagnosis and Management of Small Bowel Bleeding. Am J Gastroenterol 2015; 110: 1265-87. PMID 26303132",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/26303132/", url: "https://pubmed.ncbi.nlm.nih.gov/26303132/" }),
    ("Pennazio M et al.: Small-bowel capsule endoscopy and device-assisted enteroscopy. ESGE Guideline Update 2022. Endoscopy 2023; 55: 58-95. PMID 36423618",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/36423618/", url: "https://pubmed.ncbi.nlm.nih.gov/36423618/" }),
    ("Key quality indicators in colonoscopy. Gastroenterol Rep 2023. PMC10005623",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10005623/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10005623/" }),
    ("Colonic Pseudo-Obstruction in an Elderly Patient: Resolution Following Correction of Hypokalemia. Cureus 2025. PMC12701538",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12701538/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12701538/" }),
    ("Scrutinizing the evidence linking hypokalemia and ileus. Int J Acad Med 2015",
     Verweis { text: "journals.lww.com – Hypokalemia and ileus", url: "https://journals.lww.com/ijam/fulltext/2015/01010/scrutinizing_the_evidence_linking_hypokalemia_and.4.aspx" }),
    ("Hypothyroidism Presenting as Adynamic Ileus Mimicking a Mechanical Small Bowel Obstruction. Cureus 2023. PMC10796157",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10796157/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10796157/" }),
    ("KDIGO 2025 Clinical Practice Guideline for the Management of IgA Nephropathy and IgA Vasculitis. Kidney Int 2025",
     Verweis { text: "https://doi.org/10.1016/j.kint.2025.04.004", url: "https://doi.org/10.1016/j.kint.2025.04.004" }),
];
