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

/// Ein vollstaendiges Blatt: Titel, Text und Quellen. `src/html.rs` und
/// `src/pdf.rs` setzen jedes Dokument, das hier hereingereicht wird – so
/// entsteht aus derselben Pipeline neben dem Vaskulitis-Blatt auch das
/// Blatt zum Kostaufbau in `src/hunger.rs`.
pub struct Dokument {
    pub titel: &'static str,
    pub titel2: &'static str,
    pub untertitel: &'static str,
    pub stand: &'static str,
    /// Lebender Kolumnentitel. Im HTML ersetzt `html::render` damit den
    /// Vorgabetext der `@page`-Regel in `blatt.css`.
    pub kopfzeile: &'static str,
    pub blocks: &'static [Block],
    pub quellen: &'static [(&'static str, Verweis)],
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
    "Informationsblatt für die Patientin und ihre Angehörigen · Stand 5. September 2026 · zum Mitnehmen zum Arzttermin";
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
            &[B("Erbrechen, jedes Mal nachdem eine Kleinigkeit gegessen wurde – Haferbrei blieb drin, das Birchermüesli nicht")],
            &[T(
                "Das ist der wichtigste Punkt der Liste. Erbrechen kurz nach dem Essen heisst, \
                 dass der Mageninhalt nicht weiterkommt. Erbrechen gehört zu den Leitzeichen \
                 einer Passagestörung; bei weiter unten liegendem Hindernis wird es gallig. Dass gekochter Haferbrei wieder drinblieb, war die zweite grosse Entlastung nach dem Stuhlgang; dass das rohe Birchermüesli am 2. September wieder herauskam, ordnet sich derselben Regel unter – siehe den Abschnitt «Poulet ging nicht». Und die Magenentzündung erklärt rückblickend, warum es vorher jedes Mal kam.",
            )],
        ],
        &[
            &[B("Sehr wenig getrunken, keine Lust zu trinken – inzwischen trinkt sie zuverlässig")],
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
            &[B("Urinmenge")],
            &[T("Die Bilanz, seit der Katheter liegt: wie viel hinein, wie viel heraus, dazu das Gewicht täglich. Nach der KDIGO-Leitlinie genügt eine Menge unter "), N("0,5 ml"), T(" je Kilogramm und Stunde über sechs Stunden für sich allein, um ein akutes Nierenversagen zu benennen – auch bei unauffälligem Kreatinin. Und der Katheter gehört heraus, sobald diese Messung nicht mehr gebraucht wird.")],
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
            &[T("Gewichtsverlauf, Albumin, "), B("Phosphat"), T(", Kalium, Magnesium – nach Wochen ohne Nahrung die Voraussetzung dafür, dass der Kostaufbau sicher beginnen kann, und bei Gefährdeten in den ersten drei Tagen alle zwölf Stunden zu kontrollieren. Magnesium steht unter Pantoprazol ein zweites Mal auf dieser Liste. Dazu "), B("Thiamin (Vitamin B1) 100 mg vor der ersten Kohlenhydratgabe"), T(", dann zweimal täglich über sieben bis zehn Tage – und dazu die Frage nach dem "), B("Weg"), T(": Bei einem Magen, der sich nicht entleert, ist die Tablette unsicher. Siehe den Abschnitt zum Kostaufbau.")],
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
            &[T("TSH – das Steuerhormon der Schilddrüse; ein hoher Wert spricht für eine Unterfunktion. Die kann einen Darmstillstand nachahmen und ist behandelbar. Der Vorbehalt steht im Abschnitt zum Darmtransport: Bei akut Schwerkranken sind Schilddrüsenwerte oft verschoben, ein auffälliger Wert braucht deshalb eine Nachkontrolle in ruhigeren Zeiten.")],
        ],
        &[
            &[B("Haut")],
            &[T("Biopsie einer frischen Läsion mit direkter Immunfluoreszenz auf IgA. Nur solange frische Flecken da sind.")],
        ],
        &[
            &[B("Nierengewebe")],
            &[T("Nierenbiopsie. Nach der KDIGO-Leitlinie 2025 ist sie die einzige Möglichkeit, eine IgA-Vaskulitis-Nephritis zu diagnostizieren, und bei Erwachsenen angezeigt bei erheblicher Organschädigung, bei einer Eiweissausscheidung ab "), N("0,5 g/Tag"), T(" über mehr als vier Wochen oder bei eingeschränkter Nierenfunktion. Sie trennt ausserdem die zwei Mechanismen, die von aussen gleich aussehen – siehe den Abschnitt «Die Glomerulonephritis».")],
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

static T_BERICHT: Tabelle = Tabelle {
    kopf: &["Bereich", "Was der Bericht nennt"],
    gewichte: &[24, 76],
    linien: true,
    chronik: false,
    zeilen: &[
        &[
            &[B("Aufenthalt")],
            &[T("23. bis 30. August 2026, Eintritt notfallmässig wegen Appetitlosigkeit, zu wenig Trinken und Bauchschmerzen links unten, seit vier Wochen zunehmend; distale Purpura seit rund zwei Wochen.")],
        ],
        &[
            &[B("Hämoglobin")],
            &[T("Bei Eintritt "), N("98 g/l"), T(", im Verlauf "), N("75 g/l"), T(" bei Makrohämaturie. Zwei Erythrozytenkonzentrate am 29. August. Die Blutarmut ist "), B("mikrozytär"), T(".")],
        ],
        &[
            &[B("Blutzerfall")],
            &[T("LDH "), N("361 U/l"), T(", Haptoglobin und CK normal. Das spricht gegen einen Zerfall der roten Blutkörperchen – die Frage, die dieses Blatt in der Abklärungstabelle stellt, ist damit beantwortet.")],
        ],
        &[
            &[B("Niere")],
            &[T("Kreatinin "), N("171 µmol/l"), T(" bei Eintritt, GFR "), N("30 ml/min"), T(". Am 30. August Oligurie trotz Rehydrierung. Die Niereninsuffizienz wird als postrenal und möglicherweise zusätzlich renal beurteilt.")],
        ],
        &[
            &[B("Harnwege")],
            &[T("Mehrsegmentige Engstellen "), B("beider"), T(" Harnleiter und eine Abgangsenge des linken Nierenbeckens. Am 27. August beidseits eine Pigtail-Ableitung eingelegt.")],
        ],
        &[
            &[B("Urin")],
            &[T("Bei Eintritt unauffällig. Im Verlauf brauner Urin bei Makrohämaturie, mit der Frage nach Methämoglobinurie. Glomeruläre Erythrozyten sind angefordert und stehen aus.")],
        ],
        &[
            &[B("Entzündung")],
            &[T("CRP "), N("60,4 mg/l"), T(" bei Eintritt, im Verlauf steigend. Zwei Tage vor Eintritt hatte auswärts jemand Co-Amoxicillin verordnet, bei Verdacht auf eine Entzündung des Sigmas.")],
        ],
        &[
            &[B("Säure-Basen")],
            &[T("Metabolische Azidose, Bikarbonat "), N("17 mmol/l"), T(".")],
        ],
        &[
            &[B("Bauch im Bild")],
            &[T("Ungleichmässig verdickte Gallenblasenwand, erweiterte Gallenwege, gestaute Nierenbeckenkelchsysteme, "), B("fokale Verdickung des Bauchfells"), T(", freie Flüssigkeit im Bauch und Erguss in beiden Brustfellhöhlen.")],
        ],
        &[
            &[B("Bauchwasser")],
            &[T("Punktiert: Albumin unter "), N("10 g/l"), T(", Zellzahl "), N("119/µl"), T(". Die Zytologie stand im Bericht noch aus; am 2. September lautet die Meldung, es seien «ein paar pathologische Zellen» gefunden worden.")],
        ],
        &[
            &[B("Brustfellhöhlen")],
            &[T("Erguss auf beiden Seiten. Am 2. September wurde links punktiert, "), B("knapp ein Liter"), T(" abgezogen; rechts sei es in Ordnung. Danach besseres Atmen und mehr Appetit.")],
        ],
        &[
            &[B("Eierstock")],
            &[T("Am 3. September beim Gynäkologen: ein Tumor, bezeichnet als "), B("Eierstockkarzinom"), T(". Ob nach Bild, Zellen oder Gewebe, ist noch nicht bekannt. Siehe den Abschnitt «Die Diagnose vom 3. September».")],
        ],
        &[
            &[B("Spiegelungen")],
            &[T("Magen und Dickdarm am 28. August: sichtbare Entzündung am Übergang von Speiseröhre zu Magen; der Dickdarm soweit unauffällig – "), B("bei schlechter Vorbereitung"), T(". Bericht und Gewebeproben stehen aus.")],
        ],
        &[
            &[B("Immunologie")],
            &[T("pANCA negativ. ANCA und ANA stehen aus.")],
        ],
        &[
            &[B("Körpermasse")],
            &[N("165 cm"), T(", "), N("60 kg"), T(" bei Eintritt. Seither ist das Gewicht auf "), N("66 kg"), T(" gestiegen, während ihr gewohntes bei "), N("58 kg"), T(" liegt – die Zahlen dieses Blattes rechnen deshalb auf "), N("58 kg"), T(": "), N("580 bis 1160 kcal"), T(" für den ersten Tag des Kostaufbaus, und "), N("29 ml"), T(" Urin je Stunde als die Grenze, unter der ein akutes Nierenversagen beginnt. Siehe den Abschnitt «Das Gewicht».")],
        ],
        &[
            &[B("Nebendiagnosen")],
            &[T("Prädiabetes mit HbA1c "), N("6,3 %"), T(". Zustand nach Hepatitis A und nach Blinddarmentfernung. 2016 dermatologische Beurteilung: Verdacht auf Vaskulitis bei distaler Purpura.")],
        ],
    ],
};

static T_MEDI: Tabelle = Tabelle {
    kopf: &["Mittel", "Verordnung, und was daran hier zählt"],
    gewichte: &[27, 73],
    linien: true,
    chronik: false,
    zeilen: &[
        &[
            &[B("Novalgin i.v.")],
            &[B("Läuft, als Infusion."), T(" Metamizol gegen Schmerzen. Kein Opioid – und damit das einzige Schmerzmittel auf dieser Liste, das den Darm nicht lähmt. Dafür hat es eigene Auflagen; siehe den Abschnitt «Novalgin».")],
        ],
        &[
            &[B("Pantoprazol 40 mg")],
            &[B("Fest, 1-0-1."), T(" Säureblocker. Dazu steht in diesem Blatt ein eigener Abschnitt – samt der Warnung aus der Fachinformation, dass er Beschwerden verdecken kann, ohne die Frage zu beantworten.")],
        ],
        &[
            &[B("Ondansetron 4 mg")],
            &[B("Fest, viermal täglich."), T(" Gegen Übelkeit. Das einzige fest verordnete Mittel neben dem Säureblocker – und eines, dessen Fachinformation Verstopfung durch verlängerte Dickdarm-Transitzeit aufführt.")],
        ],
        &[
            &[B("Oxycodon 5 mg")],
            &[B("In Reserve."), T(" Opioid gegen Schmerzen. Obstipation bei 30,5 Prozent, Gegenanzeige paralytischer Ileus.")],
        ],
        &[
            &[B("Morphin 5 mg s.c.")],
            &[B("In Reserve."), T(" Zweites Opioid. Die Gegenanzeigen nennen paralytischen Ileus, akutes Abdomen unbekannter Genese und schwere Niereninsuffizienz.")],
        ],
        &[
            &[B("Metoclopramid")],
            &[B("In Reserve."), T(" Das einzige Mittel auf der Liste, das den Transport fördert. Höchstens fünf Tage, und bei dieser Nierenfunktion in halber Dosis.")],
        ],
        &[
            &[B("Haloperidol 1 mg")],
            &[B("In Reserve."), T(" Gegen Übelkeit und Unruhe. Mit Ondansetron zusammen in der Einstufung «Kombination vermeiden» wegen der QT-Zeit.")],
        ],
        &[
            &[B("Dexamethason 4 mg")],
            &[B("In Reserve."), T(" Ein Kortikoid. Dieses Blatt sagt an mehreren Stellen, sie habe nie eines bekommen – ob davon etwas verabreicht wurde, ist die Frage dazu.")],
        ],
        &[
            &[B("Bioflorin")],
            &[B("Viermal täglich, auf Wunsch der Patientin."), T(" Milchsäurebakterien.")],
        ],
        &[
            &[B("Kochsalzlösung")],
            &[N("1500 ml"), T(" am Tag. Die Infusion, von der in diesem Blatt mehrfach die Rede ist – jetzt mit einer Zahl.")],
        ],
    ],
};

pub static DOKUMENT: &[Block] = &[
    H2("Wie die Krankheit entdeckt wurde"),
    P(&[T("Der Doppelname führt in die Irre. Keiner der beiden hat die Krankheit als Erster gesehen, und was sie wirklich ist, wusste bis 1973 niemand.")]),
    Tab(&T_CHRONIK),
    Klein(&[T("Von Heberdens Buben bis zum Fluoreszenzmikroskop vergingen 172 Jahre. Das Kortison, das weiter unten im Abschnitt zur Behandlung steht, war schon zwei Jahrzehnte in Gebrauch, bevor man wusste, wogegen es eigentlich wirkt.")]),

    Lead {
        werte: "Ausgangslage: Patientin, 84 Jahre · Hämoglobin von 98 auf 75 g/l gefallen · 3. September: Eierstockkarzinom",
        blocks: &[
            P(&[T("Drei Dinge daran sind wichtig, und sie sind nicht dasselbe.")]),
            P(&[
                B("Der Wert."), T(" "), N("75 g/l"), T(" ist eine mittelschwere Blutarmut; der Normbereich für Frauen liegt bei etwa "), N("117–157 g/l"), T(". Die üblichen Transfusionsgrenzen liegen bei "), N("70 g/l"), T(", bei bekannter Herzkrankheit bei "), N("80 g/l"), T(". 83 liegt knapp darüber – also nicht automatisch transfusionsbedürftig, aber in dem Bereich, in dem der Zustand der Patientin entscheidet und nicht die Zahl."),
            ]),
            P(&[
                B("Das Tempo."), T(" Der Abfall um "), N("23 g/l"), T(" war der eigentliche Befund – und er ist nicht stehengeblieben. Eine Blutarmut durch Entzündung oder Nierenschwäche entwickelt sich über Monate. Ein Abfall innert Wochen spricht für Blutverlust; ein Abfall, der weitergeht, spricht für einen Blutverlust, der weitergeht. Bei dieser Krankheit in erster Linie aus dem Darm. Das gehört rasch abgeklärt und nicht in eine Sprechstunde in sechs Wochen."),
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
                B("Und was seither dazugekommen ist."), T(" Der Stuhl ist "), B("schwarz und flüssig"), T(", mit weissem Schleim dazwischen. Und schwarz erbrochen: Kaffeesatz. Die Darmspiegelung hat nichts gefunden – ausser, dass der Darm nicht richtig transportiert. Das Hämoglobin ist nochmals gefallen. Blut im Urin, mit blossem Auge sichtbar. Ein Harnleiter, der an mehreren Stellen gestaut ist. Ein geblähter Darm, der ihr am meisten zu schaffen macht; Luft, die sie bis zum Würgen aufstossen lässt und nur schwer nach unten abgeht; der Gastroenterologe spricht von fehlender Peristaltik. Ödeme an den Füssen, die es beim früheren Schub schon gab. Und deshalb eine laufende Bluttransfusion. Damit ist der Hämoglobinabfall nicht erklärt, sondern nur ersetzt – die Frage nach der Quelle steht unverändert. Siehe die Abschnitte «Blut im Urin», «Der gestaute Harnleiter» und «Die Transfusion»."),
            ]),
            P(&[
                B("Und der Morgen des 30. August."), T(" Das Frühstück ist drin geblieben – Schwarztee mit Milch und Haferbrei, und diesmal kein Erbrechen. Der Wind geht wieder ab. Die Transfusion ist beendet, die Kochsalzinfusion läuft weiter. Geschlafen hat sie leidlich. Blut im Urin ist weiterhin da, und der Urin wird jetzt über einen Katheter abgeleitet. Das sind mehrere kleine Besserungen an einem Tag – und keine davon beantwortet, woher das Blut kommt. Siehe die Abschnitte «Der Katheter» und «Poulet ging nicht»."),
            ]),
            P(&[
                B("Und dann die Verlegung."), T(" Sie kommt ins Universitätsspital Zürich – in das Haus, dessen Vaskulitis-Sprechstunde und Nephrologie in diesem Blatt seit der ersten Fassung als Adressen stehen. Damit ist dieses Blatt nicht mehr die Vorbereitung auf einen Termin in einigen Wochen, sondern die Unterlage für eine Übergabe von heute. Siehe den Abschnitt «Die Verlegung ins Universitätsspital»."),
            ]),
            P(&[
                B("Und der Stand vom 1. September, mit dem Guten zuerst."), T(" Der Wind geht wieder ab, der Appetit kommt ein wenig zurück, und der Stuhl bewegt sich – nach Wochen des Stillstands drei Zeichen in dieselbe Richtung. Nur ist er "), B("grün und schwarz und flüssig"), T(". Schwarz kommt von oben und lässt die Frist für die Magenspiegelung unverändert stehen; grün kommt von unten und wirft eine Frage auf, die auf diesem Blatt bisher fehlte – ein Antibiotikum kurz vor dem Eintritt, Spitalaufenthalt, 84 Jahre und Säureblocker sind zusammen die Liste, auf der Clostridioides difficile steht, und die Stuhlprobe dafür ist heute verfügbar. Siehe den Abschnitt «Dunkler Stuhl». Und Novalgin läuft jetzt als Infusion – das einzige Schmerzmittel auf ihrer Liste, das den Darm nicht bremst."),
            ]),
            P(&[
                B("Weiter zum 1. September:"), T(" "), B("Kein sichtbares Blut mehr im Urin"), T(" – die beste Nachricht seit Tagen, und doch kein Befund über die Niere; warum, steht im Abschnitt «Blut im Urin». Beine und Bauch sind weiter geschwollen, der Bauch ist unruhig. Und sie bekommt Sauerstoff. Dazu gehört ein Satz, der leicht untergeht: Sauerstoff ist kein Schmerzmittel – er läuft, weil ein Messwert zu tief war, und für diesen Messwert stehen in diesem Blatt vier Gründe. Siehe den Abschnitt «Der Sauerstoff»."),
            ]),
            P(&[
                B("Und jetzt liegt der ärztliche Bericht vor."), T(" Er verändert die Grundlage dieses Blattes an mehreren Stellen und stellt seine Überschrift in Frage: Die Beurteilung lautet nicht «IgA-Vaskulitis», sondern "), B("unklares Krankheitsbild"), T(" mit drei Möglichkeiten – autoimmun, paraneoplastisch, Peritonealkarzinose. Dazu Befunde, die hier bisher fehlten: freie Flüssigkeit im Bauch, Erguss in beiden Brustfellhöhlen, eine verdickte Stelle am Bauchfell, eine Nierenfunktion bei einer GFR von "), N("30 ml/min"), T(", und die vollständige Medikamentenliste. Der Abschnitt «Der Zwischenbericht» geht das durch."),
            ]),
            P(&[
                B("Und am 2. September die Meldung, auf die dieses Blatt gewartet hat."), T(" Es seien «ein paar pathologische Zellen» gefunden worden, und sie geht heute zum "), B("Gynäkologen"), T(". Wenn das zweite die Folge des ersten ist, dann ist der Befund eingetroffen, den dieses Blatt seit dem Zwischenbericht an die erste Stelle setzt – und die beiden Zweige, die neben der Vaskulitis offen standen, werden jetzt verfolgt statt bloss genannt. Was daran hängt und welche eine Rückfrage darüber entscheidet, steht im Abschnitt «Die Zellen im Bauchwasser». "), B("Dazu vom selben Tag:"), T(" Das Birchermüesli kam wieder heraus, das Gewicht liegt bei "), N("66 kg"), T(" gegenüber sonst "), N("58 kg"), T(", und ein Blutdrucksenker ist dazugekommen, der auf keiner Liste steht. Und das Erbrochene ist jetzt "), B("gelb statt schwarz"), T(" – damit ist das Zeichen weg, das die 24-Stunden-Frist für die Magenspiegelung ausgelöst hat; siehe den Abschnitt «Schwarz erbrochen»."),
            ]),
            P(&[
                B("Und am Abend des 2. September die Punktion."), T(" Aus der linken Brustfellhöhle ist "), B("knapp ein Liter"), T(" Flüssigkeit abgezogen worden; rechts sei es in Ordnung. Seither schnauft sie besser und hat mehr Appetit. Damit ist die Frage beantwortet, die dieses Blatt im Abschnitt «Der Sauerstoff» stellt – und von den vier Gründen für ihre Atemnot ist der einzige behandelt, der sich behandeln liess. Zugleich liegt jetzt zum zweiten Mal an einem Tag Flüssigkeit aus einer Körperhöhle im Labor, und für sie gilt dasselbe fünfstufige Meldesystem wie für das Bauchwasser. Was daraus folgt und was mit dem Liter geschehen sein muss, steht im Abschnitt «Ein Liter von der linken Lunge»."),
            ]),
            P(&[
                B("Und am 3. September die Diagnose."), T(" Der Gynäkologe hat einen Tumor gefunden: ein "), B("Eierstockkarzinom"), T(". Von den drei Möglichkeiten des Zwischenberichts – autoimmun, paraneoplastisch, Bauchfellkarzinose – hat sich damit nicht die erste bestätigt, unter deren Namen dieses Blatt steht, sondern die dritte, und die zweite wird zu ihrer wahrscheinlichsten Erklärung. Das Blatt wird deshalb nicht umbenannt und nicht stillschweigend weitergeschrieben; der Abschnitt gleich nach diesem Kasten sagt, was die Diagnose zusammenführt, was an ihr noch offen ist und welche Behandlungen es gibt. Vom selben Tag: Die Beine sind weiter geschwollen, die Kochsalzlösung läuft, den neuen Blutdrucksenker nimmt sie – und sie hat "), B("Fleisch, Gemüsesuppe und ein wenig Polenta"), T(" gegessen und behalten. Empfindlich auf Gerüche ist sie nach wie vor."),
            ]),
            P(&[
                B("Und am 5. September: Die Diagnose ist bestätigt."), T(" Das Eierstockkarzinom steht damit fest; worauf die Bestätigung beruht – Zellen oder Gewebe –, ist die Frage, die bleibt, weil nur das Gewebe Typ und Grad liefert. Vom selben Tag, vom Dispenser und vom Nachttisch abgelesen: "), B("Kalium"), T(" als Retardtablette und "), B("Magnesium"), T(" als Granulat sind neu dazugekommen, Pantoprazol und Movicol laufen, die Haut wird mit einer Harnstoff-Emulsion gepflegt – und am Infusionsständer hängt nicht mehr Kochsalz, sondern "), B("Ringer-Acetat"), T(". Kalium und Magnesium passen zum Kostaufbau, den dieses Blatt verlangt; bei einer GFR von 30 tragen beide aber eine Gegenanzeige an der Grenze, und die Infusionslösung nennt die Ödeme als ihre. Was daraus folgt, steht im Abschnitt «Der Stand vom 5. September» bei der Medikamentenliste."),
            ]),
        ],
    },

    H2("Die Diagnose vom 3. September: ein Eierstockkarzinom"),
    P(&[B("Dieses Blatt trägt seit heute eine Diagnose, die es nicht im Titel hat."), T(" Der Gynäkologe hat einen Tumor am Eierstock gefunden und ihn als Karzinom bezeichnet. Das ist der Befund, auf den die Abschnitte «Die Zellen im Bauchwasser» und «Warum der Gynäkologe» zugelaufen sind – und er ist der wichtigste Satz, der in diesem Blatt steht. Er bleibt hier oben stehen, während der Rest des Blattes die Ausarbeitung einer anderen Möglichkeit ist. Wer weiterliest, liest ab jetzt mit dieser Diagnose im Kopf. "), B("Am 5. September ist die Diagnose bestätigt worden."), T(" Die Frage des folgenden Abschnitts – auf welcher Stufe – bleibt trotzdem stehen, weil «bestätigt» aus zweiter Hand dieselben drei Stufen deckt wie «gefunden».")]),

    H3("Was gesagt worden ist – und was noch nicht"),
    P(&[T("«Ovarialkarzinom» ist ein Wort aus zweiter Hand, und hinter ihm liegen drei verschiedene Stufen. Ein Gynäkologe kann es nach dem "), B("Ultraschall"), T(" sagen, wenn er eine Raumforderung am Eierstock sieht, die zusammen mit Bauchwasser und bösartigen Zellen praktisch nur eines bedeuten kann. Er kann es nach der "), B("Zytologie"), T(" sagen, wenn die Zellen aus dem Bauchwasser mit der Färbung PAX8 auf das Müller-System zeigen. Und er kann es nach einer "), B("Gewebeprobe"), T(" sagen, mit Typ und Grad. Die drei Stufen führen zur selben Behandlung, aber nicht zur selben Sicherheit, und die dritte ist die einzige, aus der sich Typ und Grad ergeben – und damit alles, was über die Behandlung entscheidet. Die erste Frage an die Ärzte ist deshalb keine medizinische, sondern eine nach dem Papier: "), B("Steht die Diagnose auf einem Bild, auf Zellen oder auf Gewebe?"), T(" Und: Welcher Typ, welches Stadium, und ist ein Tumorboard angesetzt?")]),

    H3("Was die Diagnose zusammenführt"),
    P(&[T("Fast alles, was dieses Blatt seit Wochen als offene Frage mitgeführt hat, bekommt mit dieser einen Diagnose eine gemeinsame Erklärung. Das ist der Grund, warum sie so gut passt, und zugleich der Grund zur Vorsicht: Eine Erklärung, die alles erklärt, gehört besonders sorgfältig geprüft.")]),
    Liste(&[
        &[B("Das Bauchwasser und die verdickte Stelle am Bauchfell."), T(" Das ist die Bauchfellkarzinose, der dritte Zweig des Zwischenberichts – der Weg, auf dem ein Eierstockkarzinom sich ausbreitet, ist die Bauchhöhle selbst.")],
        &[B("Der Erguss in beiden Brustfellhöhlen und der Liter links."), T(" Der Brustraum ist beim Eierstockkarzinom der "), L("häufigste Ort einer Ausbreitung ausserhalb des Bauches", "https://pubmed.ncbi.nlm.nih.gov/22458298/"), T(". Damit gewinnt die Zytologie des Liters vom 2. September noch einmal an Gewicht: Bösartige Zellen darin bedeuten Stadium IV, und rund 30 Prozent der bösartigen Ergüsse fallen in der Zytologie falsch negativ aus.")],
        &[B("Der gestaute Harnleiter und das Nierenversagen."), T(" Ein Tumor im kleinen Becken drückt auf den Harnleiter. In einer Auswertung von 2580 Frauen mit Eierstockkarzinom entwickelten "), L("7,4 Prozent einen Harnstau", "https://pubmed.ncbi.nlm.nih.gov/38350551/"), T("; er löste sich nur bei 27 Prozent wieder, am ehesten nach einer Tumoroperation. Der Stau vom 27. August, die Ableitung und die Frage nach dem Kreatinin bekommen damit eine Ursache, die nicht in der Niere liegt.")],
        &[B("Der Darmstillstand, das Erbrechen, das Poulet, das nicht ging."), T(" Eine Bauchfellkarzinose legt sich auf den Darm und bremst ihn – von aussen, ohne dass ein Verschluss im Bild zu sehen sein muss. Der Abschnitt «Die Darmspiegelung» hat genau diese Frage gestellt: Warum steht ein Darm, in dem nichts gefunden wurde? Hier ist die Antwort, die ausserhalb der Darmwand liegt.")],
        &[B("Das Gewicht, der Appetit, die Ödeme."), T(" Wochen ohne Appetit, das Albumin, das fällt, das Wasser in den Beinen, das steigt – das ist der Weg einer Krebserkrankung im Bauch, und es war nie gut zur Vaskulitis gepasst.")],
        &[B("Und die Purpura an den Beinen."), T(" Sie ist damit nicht widerlegt, sondern eingeordnet: Bei Erwachsenen kann die IgA-Vaskulitis die Begleiterscheinung einer noch unentdeckten Krebserkrankung sein – dieses Blatt hat das im Abschnitt «Was daraus nicht folgt» vorweggenommen. Die Flecken an den Beinen waren dann das erste Zeichen, Wochen vor allem anderen. Einen eigenen Beleg für die Verbindung gerade zum Eierstockkarzinom hat die Suche nicht ergeben; die Literatur nennt solide Tumoren allgemein. Das gehört so gesagt.")],
    ]),

    H3("Das Stadium, und was die Zahlen dazu bedeuten"),
    P(&[T("Bauchwasser mit bösartigen Zellen und eine Karzinose am Bauchfell sind mindestens Stadium III. Bösartige Zellen im Brustfellerguss machen daraus "), L("Stadium IV", "https://pubmed.ncbi.nlm.nih.gov/27335253/"), T(" – 12 bis 33 Prozent der Frauen sind bei der Diagnose in diesem Stadium, das mittlere Überleben liegt in den Auswertungen zwischen 15 und 29 Monaten, nach fünf Jahren leben etwa 20 Prozent. Bei Ausbreitung in den Brustraum nennt die Übersicht dazu ein mittleres Überleben von "), L("rund zwei Jahren", "https://pubmed.ncbi.nlm.nih.gov/22458298/"), T(". "), B("Diese Zahlen gehören hier hin, und sie gehören sofort eingeschränkt."), T(" Sie stammen aus Gruppen, die im Mittel deutlich jünger waren, operiert und behandelt wurden, und in keiner steht eine 84-Jährige mit einer GFR von 30 und einem Hämoglobin von 75. Sie sagen etwas über die Krankheit und nichts über ihren Verlauf – derselbe Satz, der in diesem Blatt schon zu den achtzehn Frauen mit Bauchfellkarzinose steht.")]),

    H3("Welche Behandlungen es gibt"),
    P(&[B("Die Grundlage sind zwei Dinge, Operation und Chemotherapie, und die Frage ist ihre Reihenfolge."), T(" Der internationale Standard ist, den Tumor operativ so weit wie möglich zu entfernen und danach mit einer platinhaltigen Chemotherapie zu behandeln. Zwei grosse randomisierte Studien haben geprüft, ob es umgekehrt geht – erst drei Zyklen Chemotherapie, dann die Operation, dann drei weitere. Die europäische Studie an 670 Frauen fand die Reihenfolge Chemotherapie-zuerst "), L("nicht unterlegen", "https://pubmed.ncbi.nlm.nih.gov/20818904/"), T(", bei weniger Komplikationen nach der Operation. Die britische Studie an 550 Frauen ebenso: mittleres Überleben "), L("22,6 gegenüber 24,1 Monaten", "https://pubmed.ncbi.nlm.nih.gov/26002111/"), T(", und Todesfälle innerhalb von 28 Tagen nach der Operation bei 6 Prozent, wenn zuerst operiert wurde, gegenüber unter 1 Prozent, wenn zuerst behandelt wurde. Für eine 84-Jährige mit acht Litern Wasser, einem Hämoglobin von 75 und einer GFR von 30 ist eine grosse Erstoperation kaum der Weg; wenn behandelt wird, dann nach diesen Studien mit einiger Wahrscheinlichkeit zuerst mit Medikamenten.")]),
    P(&[B("Die Chemotherapie: Carboplatin und Paclitaxel, und warum man sie im Alter nicht halbieren darf."), T(" Die Kombination der beiden ist der Standard. Bei gebrechlichen älteren Frauen lag es nahe, Carboplatin allein zu geben, um sie zu schonen. Genau das hat eine randomisierte Studie an 120 Frauen über 70 – mittleres Alter 80, ein Drittel im Stadium IV – geprüft und "), L("vorzeitig abgebrochen", "https://pubmed.ncbi.nlm.nih.gov/33885718/"), T(": Carboplatin allein war mit einem deutlich schlechteren Überleben verbunden. Die Kombination alle drei Wochen hatte zugleich die wenigsten behandlungsbedingten Nebenwirkungen. Wer also im Alter «etwas Milderes» vorschlägt, schlägt nach dieser Studie das Schlechtere vor.")]),
    P(&[B("Und hier greift die Nierenfunktion direkt in die Dosis."), T(" Carboplatin wird über die Niere ausgeschieden; die Dosis wird nach der "), L("Calvert-Formel aus der Nierenfunktion berechnet", "https://ch.oddb.org/de/gcc/fachinfo/reg/47671/chapter/usage"), T(" – Dosis gleich Zielwert mal (GFR plus 25). Die Fachinformation setzt dieser Formel unter einer Kreatinin-Clearance von 60 ml/min eine Grenze und nennt für eine Clearance von 16 bis 40 ml/min eine schwere Knochenmarkschädigung bei etwa 25 Prozent; unter den "), L("Gegenanzeigen", "https://ch.oddb.org/de/gcc/fachinfo/reg/47671/chapter/contra_indications"), T(" steht die schwere Niereninsuffizienz. Ob eine GFR von 30 darunter fällt, entscheidet die Onkologie – aber die Nierenfunktion ist damit nicht ein Nebenbefund der Behandlung, sondern ihre erste Bedingung. Und mit ihr das Kreatinin nach der Harnleiterentlastung, das dieses Blatt seit dem 27. August erfragt.")]),
    P(&[B("Zwei Medikamente, die dazukommen können, und was bei ihr dagegen spricht."), T(" Bevacizumab hemmt die Gefässneubildung des Tumors und verlängerte in der grossen Studie an 1873 Frauen die Zeit ohne Fortschreiten um "), L("rund vier Monate", "https://pubmed.ncbi.nlm.nih.gov/22204724/"), T(", ohne das Überleben zu verlängern; Durchbrüche der Darmwand traten bei 2,6 bis 2,8 Prozent auf gegenüber 1,2 unter Chemotherapie allein, und die Fachinformation warnt ausdrücklich vor "), L("gastrointestinalen Perforationen", "https://ch.oddb.org/de/gcc/fachinfo/reg/56922/chapter/restrictions"), T(". Bei einer Frau, deren Darm wochenlang stand und auf dessen Bauchfell der Tumor sitzt, ist das kein Randhinweis. Die zweite Gruppe sind die PARP-Hemmer, als Erhaltung nach der Chemotherapie: Niraparib verlängerte die Zeit ohne Fortschreiten von "), L("8,2 auf 13,8 Monate", "https://pubmed.ncbi.nlm.nih.gov/31562799/"), T(", bei Tumoren mit einem bestimmten Reparaturdefekt von 10,4 auf 21,9. Ob dieser Defekt vorliegt, wird am Gewebe bestimmt – ein Grund mehr für die Gewebeprobe. Die häufigste schwere Nebenwirkung war eine Blutarmut bei 31 Prozent. Ihr Hämoglobin liegt bei 75.")]),
    P(&[B("Bestrahlung spielt beim Eierstockkarzinom heute fast keine Rolle."), T(" Früher gehörte die Bestrahlung des ganzen Bauches zur Behandlung; sie ist wegen ihrer Schäden und wegen der Wirksamkeit der platinhaltigen Chemotherapie "), L("aufgegeben worden", "https://pubmed.ncbi.nlm.nih.gov/35256425/"), T(". Was bleibt, ist die gezielte Bestrahlung einzelner Stellen, die Schmerzen oder Blutungen machen. Wer nach «Bestrahlung» fragt, fragt also nach einer Ausnahme, nicht nach dem Standard.")]),
    P(&[B("Und der Massstab, an dem all das gemessen wird, heisst bei ihr nicht Alter, sondern Gebrechlichkeit."), T(" Die Hälfte aller Frauen mit Eierstockkarzinom ist bei der Diagnose über 65; ältere Frauen bekommen "), L("weniger Operationen und weniger Chemotherapie, erleiden mehr Nebenwirkungen und haben schlechtere Verläufe", "https://pubmed.ncbi.nlm.nih.gov/27499341/"), T(" – und der Allgemeinzustand nach Augenmass sagt die Verträglichkeit einer Behandlung schlecht voraus. Empfohlen wird ein formales geriatrisches Assessment, und die Studie an den 120 Frauen über 70 hat genau ein solches als Einschlusskriterium benutzt. Die Frage an das Tumorboard lautet deshalb nicht «Ist sie zu alt?», sondern: "), B("Ist ein geriatrisches Assessment gemacht worden, und was sagt es?"), T(" Dazu gehört, dass es einen dritten Weg gibt, der keiner Rechtfertigung bedarf: die Behandlung der Beschwerden ohne Behandlung des Tumors – Punktionen, wenn Wasser drückt, Schmerzmittel, Ernährung. Welcher der Wege der ihre ist, entscheidet nicht das Blatt und nicht das Tumorboard allein. Sie entscheidet es, und dafür muss sie wissen, was sie hat.")]),

    H3("Zwei Dinge, die mit der Diagnose dringlicher werden"),
    P(&[B("Die Beine."), T(" Sie sind weiter geschwollen, und bisher stehen dafür in diesem Blatt sieben Erklärungen – alle vom Wasser her. Mit der Diagnose kommt eine achte dazu, die anders behandelt wird: Das Eierstockkarzinom gehört zusammen mit dem Bauchspeicheldrüsen- und dem Hirntumor zu den Krebsarten mit dem "), L("höchsten Thromboserisiko", "https://pubmed.ncbi.nlm.nih.gov/38395064/"), T(". Geschwollene Beine bei einer bettlägerigen Frau mit dieser Diagnose sind deshalb eine Frage an den Ultraschall der Beinvenen, bevor sie eine Frage an die Waage sind. Und die Antwort ist nicht einfach: Eine Blutverdünnung bei einer Frau, die vor Tagen schwarz erbrochen hat und Blut bekommen musste, ist eine Abwägung, die nur die Station treffen kann. Aber sie gehört getroffen, nicht übersehen.")]),
    P(&[B("Der Geruch."), T(" Sie ist nach wie vor sehr empfindlich auf Gerüche. Das ist kein Charakterzug und keine Folge der Medikamente, sondern ein bekanntes Zeichen der Krankheit selbst: In einer Untersuchung an vierzig Frauen und Männern mit soliden Tumoren "), L("vor jeder Behandlung", "https://pubmed.ncbi.nlm.nih.gov/26945569/"), T(" berichteten 48 Prozent über veränderten Geschmack oder Geruch, darunter ein verstärkter Geruchssinn – und diese Veränderungen hingen mit frühem Sättigungsgefühl und Ernährungsrisiko zusammen. In einer zweiten Untersuchung mit Riech- und Schmecktests hatten "), L("74 Prozent", "https://pubmed.ncbi.nlm.nih.gov/31486983/"), T(" mindestens eine solche Veränderung, und die Geruchsveränderungen belasteten die Lebensqualität am stärksten. Für den Kostaufbau heisst das: Was sie nicht riechen mag, ist keine Frage des Willens, und es gehört auf das Verlaufsblatt und zur Ernährungsberatung – als Symptom, nicht als Vorliebe.")]),


    H2("Der Zwischenbericht vom 30. August"),
    P(&[B("Dieses Blatt steht ab hier auf einer anderen Grundlage."), T(" Bis gestern war es aus dem zusammengesetzt, was am Krankenbett berichtet wurde – tröpfchenweise, aus zweiter Hand, mit den Ungenauigkeiten, die dabei unvermeidlich sind. Jetzt liegt der Bericht des behandelnden Arztes zur Verlegung vor: Zahlen, Befunde, eine Beurteilung und eine Liste dessen, was noch aussteht. Vieles darin bestätigt, was hier stand. Einiges korrigiert es. Und ein Satz darin stellt die Überschrift dieses Blattes in Frage.")]),
    P(&[B("Die Beurteilung lautet nicht «IgA-Vaskulitis»."), T(" Sie lautet: "), B("unklares Krankheitsbild"), T(", mit drei Möglichkeiten – autoimmun, "), B("paraneoplastisch"), T(" oder "), B("Peritonealkarzinose"), T(", also eine Absiedlung im Bauchfell. Die Vaskulitis ist damit nicht widerlegt; sie ist der erste dieser drei Zweige, und dieses Blatt hat ihn gründlich ausgearbeitet. Aber sie steht nicht mehr allein. Das gehört an dieser Stelle gesagt und nicht weggeschrieben: "), B("Wer die folgenden Seiten liest, liest die Ausarbeitung eines Zweiges, während zwei andere offen danebenstehen."), T(" Beide neuen Zweige laufen auf denselben ausstehenden Befund zu, und das ist der wichtigste von allen – die Zelluntersuchung des Bauchwassers. "), B("Nachtrag vom 3. September:"), T(" Der dritte Zweig hat sich bestätigt – der Gynäkologe hat ein Eierstockkarzinom gefunden. Der Abschnitt davor sagt, was daraus folgt. Der Text dieses Abschnitts bleibt stehen, wie er am 30. August richtig war.")]),
    Tab(&T_BERICHT),

    H3("Was der Bericht korrigiert"),
    P(&[T("Fünf Angaben in diesem Blatt waren ungenau. Sie stehen hier, statt stillschweigend ausgebessert zu werden, weil sonst nicht nachvollziehbar bleibt, warum frühere Fassungen etwas anderes sagten.")]),
    Liste(&[
        &[B("Die Hämoglobinzahlen."), T(" In diesem Blatt stand «von 108 auf 83 g/l». Dokumentiert ist: "), N("98 g/l"), T(" bei Eintritt, "), N("75 g/l"), T(" im Verlauf, zwei Konserven am 29. August. Der Abfall ist damit "), N("23 g/l"), T(" statt 25 – an der Sache ändert das nichts, an der Genauigkeit schon. Alle Rechnungen dieses Blattes sind nachgeführt.")],
        &[B("Der Harnleiter ist beidseits betroffen,"), T(" nicht einer. Dazu kommt eine Abgangsenge des linken Nierenbeckens.")],
        &[B("Die obere Harnableitung ist längst entlastet."), T(" Der Abschnitt «Der Katheter» stellte gestern die Frage, ob eine Schiene oder eine Nierenfistel nötig sei. Sie war zu diesem Zeitpunkt schon gelegt: beidseits Pigtail, am 27. August. Der Abschnitt ist entsprechend berichtigt.")],
        &[B("Der Urin war bei Eintritt unauffällig."), T(" Die Makrohämaturie kam im Verlauf. Damit steht eine Frage im Raum, die der Bericht nicht beantwortet: Wann genau, und in welchem zeitlichen Verhältnis zur Einlage der Ableitungen am 27. August?")],
        &[B("Der frühere Schub war 2016,"), T(" also vor rund zehn Jahren, und es war eine dermatologische Beurteilung mit dem Verdacht auf eine Vaskulitis bei distaler Purpura. Der Abschnitt «Ein früherer Schub» hat damit ein Datum.")],
    ]),

    H3("Was der Bericht bestätigt"),
    Liste(&[
        &[B("Die Passagestörung, und in schärferer Form."), T(" Der Bericht nennt als Grund für die Spiegelungen ausdrücklich «Miserere» – das Erbrechen von Darminhalt. Das ist kein Nebenwort. Es ist eines der Zeichen, die bei einer Darmverlegung erst spät auftreten. Dazu gehört eine Ehrlichkeit: Miserere beweist keine mechanische Verlegung. Es kommt "), L("auch bei der reinen Darmlähmung vor", "https://www.ncbi.nlm.nih.gov/books/NBK448079/"), T(", gerade bei alten Menschen – und die Erkennung dieser Tatsache erspart mitunter eine unnötige Operation. Es passt damit zu genau dem, was der Gastroenterologe gesagt hat.")],
        &[B("Kein Zerfall der roten Blutkörperchen."), T(" LDH leicht erhöht, Haptoglobin und CK normal. Die Abklärungstabelle dieses Blattes verlangt genau diese drei Werte, um einen Blutzerfall vom Blutverlust nach aussen zu trennen. Die Antwort lautet: Verlust, kein Zerfall.")],
        &[B("Die Ödeme."), T(" Schon bei Eintritt beschrieben, als leichte periphere Ödeme – zusammen mit bräunlichen Hautveränderungen an den Unterschenkeln. «Bräunlich» heisst: Die Purpura war beim Eintritt bereits am Abheilen. Das Zeitfenster für die Hautbiopsie, von dem dieses Blatt spricht, war damit schon am 23. August klein.")],
        &[B("Die Nierenbeteiligung als das, woran es hängt."), T(" Der Bericht führt die Niereninsuffizienz als erste Diagnose auf und beurteilt sie als postrenal – durch das Abflusshindernis – und "), B("möglicherweise zusätzlich renal"), T(". Genau diese Unterscheidung ist der Gegenstand des Abschnitts «Die Glomerulonephritis», und genau sie beantwortet der ausstehende Befund der glomerulären Erythrozyten.")],
    ]),

    H3("Was noch aussteht – und in welcher Reihenfolge es zählt"),
    P(&[T("Der Bericht führt die offenen Befunde selbst auf. Sie sind nicht gleich wichtig; diese Reihenfolge ergibt sich aus dem, was daran hängt.")]),
    Liste(&[
        &[B("1. Die Zytologie des Bauchwassers."), T(" Sie entscheidet über zwei der drei Verdachtsdiagnosen – und sie ist als einzige inzwischen zurück. Siehe den Abschnitt «Die Zellen im Bauchwasser».")],
        &[B("2. Glomeruläre Erythrozyten im Urin."), T(" Sie entscheiden, ob die Niere selbst blutet oder nur der abflussbehinderte Harnweg – und damit, ob eine Nierenbiopsie zur Frage wird.")],
        &[B("3. Die Gewebeproben aus Magen und Darm."), T(" Sie sind das Einzige, was die Vaskulitis am Darm beweisen könnte, und zugleich das, was eine Bösartigkeit fände.")],
        &[B("4. ANCA und ANA."), T(" pANCA ist negativ; das ist ein erster Schritt gegen die im Alter häufigere ANCA-assoziierte Vaskulitis, aber nur ein Teil davon.")],
        &[B("5. Methämoglobin."), T(" Die Frage steht im Bericht, weil der Urin braun war und nicht rot. Sie ändert die Deutung der Blutmenge im Urin.")],
    ]),

    H2("Die Zellen im Bauchwasser"),
    P(&[T("Am 2. September kommen zwei Meldungen, und sie gehören zusammen: Es seien «ein paar pathologische Zellen» gefunden worden – und sie geht heute zum Gynäkologen. Die zweite ist mit einiger Wahrscheinlichkeit die Folge der ersten. Dieser Abschnitt sagt, was daraus folgt, was daraus ausdrücklich nicht folgt und welche eine Rückfrage über alles Weitere entscheidet. "), B("Am 3. September ist die Antwort gekommen:"), T(" ein Eierstockkarzinom. Der Abschnitt bleibt stehen, weil er zeigt, wie dieses Blatt dahin gekommen ist – und weil die Frage nach der Kategorie im schriftlichen Befund weiter offen ist.")]),

    H3("Warum die Formulierung im Befund alles entscheidet"),
    P(&[T("«Ein paar pathologische Zellen» ist eine Auskunft aus zweiter Hand; die Zytologie hat für genau diesen Bereich eine abgestufte Sprache. Das internationale Meldesystem für Ergussflüssigkeiten kennt fünf Stufen: nicht beurteilbar, kein Anhalt für Bösartigkeit, "), B("Atypie unklarer Bedeutung"), T(", "), B("verdächtig auf Bösartigkeit"), T(" und "), B("bösartig"), T(". Zu jeder Stufe ist ausgerechnet, wie oft sich am Ende doch eine Bösartigkeit findet. In einer Übersicht über zwanzig Studien lag dieses Risiko bei "), L("50,8 Prozent für die Atypie, 91,3 Prozent für «verdächtig» und 98,2 Prozent für «bösartig»", "https://pubmed.ncbi.nlm.nih.gov/39025059/"), T(".")]),
    P(&[B("Das ist der Grund, warum die genaue Formulierung nachgefragt gehört."), T(" Zwischen der ersten und der zweiten dieser drei Stufen liegt der Unterschied zwischen einer offenen Frage und einer beinahe entschiedenen – und beides kann sich hinter «ein paar pathologische Zellen» verbergen. Die Frage an die Station lautet deshalb wörtlich: "), B("Welche der fünf Kategorien steht im zytologischen Befund?")]),
    P(&[B("Von der anderen Seite gelesen."), T(" Eine Metaanalyse von sechzehn Studien mit 19 128 Proben hat die Treffsicherheit dieses Systems geprüft. Nimmt man «verdächtig» und darüber als positiv, liegt die "), L("Spezifität bei 100 Prozent", "https://pubmed.ncbi.nlm.nih.gov/38613789/"), T(", die Empfindlichkeit bei 57. Übersetzt: Ab dieser Stufe gibt es praktisch keine falschen Alarme – wer dort landet, hat etwas. Umgekehrt bleibt die Empfindlichkeit begrenzt; ein unauffälliger Befund beruhigt also weniger, als ein auffälliger belastet.")]),

    H3("Warum der Gynäkologe"),
    P(&[T("Wenn im Bauchwasser einer Frau bösartige Zellen liegen und kein Ursprung bekannt ist, steht ein Bereich voran: das Müller-System aus Eierstock, Eileiter und Gebärmutter. Diese Lage ist so häufig und so eigen, dass sie einen eigenen Namen und einen eigenen Behandlungsweg hat – und das ist der Grund für den Termin heute.")]),
    P(&[B("Und hier steht eine Zahl, die für Angehörige zählt."), T(" Unter den Krebserkrankungen ohne auffindbaren Ursprung ist gerade diese die Gruppe, die ausdrücklich "), B("aus der ungünstigen Kategorie herausgenommen"), T(" wird. Die Arbeit, die das begründet hat, verfolgte achtzehn Frauen mit Bauchfellkarzinose ohne auffindbaren Ursprung, die nach den Regeln für das fortgeschrittene Eierstockkarzinom behandelt wurden: mittleres Überleben 23 Monate, bei geringem Resttumor 31 gegenüber 11 Monaten, und drei Frauen waren nach 41, 59 und 77 Monaten krankheitsfrei. Der Schluss der Arbeit lautet, diese Frauen seien von den übrigen Krebserkrankungen unbekannten Ursprungs zu "), L("unterscheiden", "https://pubmed.ncbi.nlm.nih.gov/2502058/"), T(" – wegen des trägeren Verlaufs, der besseren Ansprechrate und der Aussicht auf ein langes krankheitsfreies Intervall. Eine systematische Übersicht über 579 Fälle kommt zum selben Ergebnis: Diese Kranken gehören "), L("nicht in die Gruppe mit schlechter Prognose", "https://pubmed.ncbi.nlm.nih.gov/19897383/"), T("; ihr Überleben liegt zwei bis sechs Monate unter dem beim Eierstockkrebs.")]),
    P(&[B("Die Einschränkung gehört unmittelbar dazu."), T(" Die achtzehn Frauen wurden zwischen 1978 und 1984 behandelt, die Übersicht reicht bis 2008; die Behandlung hat sich seither geändert, und in keiner der beiden Arbeiten steht eine 84-Jährige mit einer GFR von 30. Diese Zahlen sagen etwas über die Art der Erkrankung und nichts über ihren Verlauf. Sie stehen hier, weil «unbekannter Ursprung» in der Umgangssprache das Schlimmste bedeutet und in diesem einen Fall nicht das Schlimmste ist.")]),

    H3("Eine Färbung kann die Herkunftsfrage beantworten, ohne neuen Eingriff"),
    P(&[T("Die Flüssigkeit vom 30. August liegt im Labor. Für die Frage nach dem Ursprung gibt es einen Marker, der genau dafür gemacht ist: PAX8 zeigt eine Herkunft aus dem Müller-System an. In einer Untersuchung an Ergussflüssigkeiten war er bei "), L("70 Prozent der Eierstockkarzinome", "https://pubmed.ncbi.nlm.nih.gov/20607683/"), T(" – 22 von 31 – und bei 68,8 Prozent der Gebärmutterkarzinome positiv; unter 71 nicht-gynäkologischen Karzinomen und Nichtkarzinomen dagegen nur in zwei Fällen. Die serös-papillären Karzinome des Bauchfells waren durchweg positiv. Die Autoren empfehlen ausdrücklich, PAX8 bei bösartigen Ergüssen von Frauen mitzubestimmen.")]),
    P(&[B("Praktisch heisst das:"), T(" Die Frage «woher kommen diese Zellen» braucht keinen neuen Eingriff, sondern eine Auskunft darüber, was mit der bereits gewonnenen Flüssigkeit geschehen ist. Zwei Fragen genügen: Wurde ein "), B("Zellblock"), T(" angelegt – die Zellen also zu einem Block verarbeitet, der sich färben lässt wie Gewebe –, und welche Immunfärbungen sind darauf gelaufen? Fehlt PAX8, lässt es sich nachfordern, solange Material da ist. Das ist die billigste Frage auf diesem ganzen Blatt.")]),

    H3("Zum CA-125, bevor die Zahl kommt"),
    P(&[T("Beim Gynäkologen wird mit einiger Sicherheit CA-125 bestimmt. Dieser Wert wird bei ihr hoch sein, und er beweist für sich genommen nichts. In einer Vergleichsstudie an 133 Kranken war CA-125 im Blut erhöht bei "), L("52 Prozent der Lebererkrankungen, 87 Prozent der Rippenfellergüsse und 100 Prozent der nicht-gynäkologischen Bauchfellkarzinosen", "https://pubmed.ncbi.nlm.nih.gov/11925128/"), T("; der Wert folgt der Flüssigkeit an den Häuten, gleich welchen Ursprungs. Sie hat Bauchwasser und Erguss auf beiden Seiten. Ein hoher Wert ist deshalb zu erwarten und kein Beweis, ein mässiger keine Entwarnung. Was entscheidet, sind die Zellen und die Färbung – nicht der Marker.")]),

    H3("Was daraus nicht folgt: dass die Vaskulitis vom Tisch ist"),
    P(&[T("Der naheliegende Schluss wäre, dieses Blatt sei damit überholt. Er ist falsch, und zwar aus einem Grund, der weiter unten längst steht. Bei älteren Menschen kann die IgA-Vaskulitis die "), B("Begleiterscheinung"), T(" einer noch unentdeckten Krebserkrankung sein – rund 16 Prozent der erwachsenen Betroffenen entwickeln einen Tumor, zu gut zwei Dritteln solide. Das ist der zweite der drei Zweige des Berichts, der paraneoplastische, und er ist kein Gegenteil des ersten, sondern seine Erklärung. Bestätigt sich eine Bösartigkeit, dann war die Purpura an den Beinen möglicherweise ihr erstes Zeichen – Wochen, bevor irgendetwas anderes auffiel.")]),
    P(&[B("Damit ändert sich die Reihenfolge, nicht der Inhalt."), T(" Was in diesem Blatt zur Vaskulitis steht, bleibt richtig; es steht nur nicht mehr an erster Stelle. Und zwei Dinge daraus werden dringlicher statt weniger dringlich: die Hautbiopsie, solange überhaupt noch frische Flecken da sind, weil sie den Zusammenhang beweisen würde – und die Urinkontrolle, weil die Niere unabhängig von der Ursache das Organ bleibt, an dem der Verlauf hängt.")]),
    P(&[B("Und die Behandlungsfrage kippt."), T(" Die letzte der zehn Fragen weiter unten lautete: Wird die Behandlung der Vaskulitis neu beurteilt, sobald die Zytologie da ist? Sie ist damit keine Vorsichtsfrage mehr, sondern die Frage des Tages. Ein Kortikoid oder eine stärkere Immunsuppression sind bei einer Vaskulitis das eine und bei einer Bösartigkeit im Bauchfell etwas ganz anderes.")]),
    Klein(&[T("Der Vorbehalt gehört an diese Stelle und nicht ans Ende. «Ein paar pathologische Zellen» ist eine mündliche Auskunft und kein Befund. Offen ist erstens, aus welchem Material sie stammt: Am 30. August standen zwei Untersuchungen aus – die Zytologie des Bauchwassers und die Gewebeproben aus Magen und Dickdarm. Dieser Abschnitt nimmt das Bauchwasser an, weil der Gang zum Gynäkologen dazu passt; stammen die Zellen aus den Gewebeproben, zeigt die Suche in eine andere Richtung. Offen ist zweitens die Kategorie. Beides klärt ein Blick in den schriftlichen Befund, und beides gehört geklärt, bevor aus diesem Abschnitt eine Gewissheit gemacht wird.")]),

    H2("Die Medikamentenliste, zum ersten Mal vollständig"),
    P(&[T("Dieses Blatt verlangt seit der ersten Fassung die vollständige Medikamentenliste – ausdrücklich auch deshalb, weil Medikamente eine der vier Erklärungen für den fehlenden Darmtransport sind. Jetzt liegt sie vor, und sie sieht anders aus als angenommen: Novalgin, niedrig dosiertes Aspirin und die Abführmittel, auf denen der bisherige Interaktionscheck beruhte, stehen nicht darauf.")]),
    Tab(&T_MEDI),
    P(&[B("Und sie ist bereits überholt."), T(" Novalgin läuft inzwischen als Infusion, steht aber auf der Liste vom 30. August nicht; Co-Amoxicillin lief vor dem Eintritt und steht ebenfalls nicht darauf. Eine Verordnungsliste ist eine Momentaufnahme, und diese hier ist eine Woche alt. Was in der Tabelle steht, ist der dokumentierte Stand; was tatsächlich läuft, sagt die Station.")]),

    H3("Der Stand vom 5. September, vom Dispenser abgelesen"),
    P(&[T("Am 5. September liegen im Dispenser "), B("Pantoprazol 40 mg"), T(" und "), B("Kaliumchlorid retard 10 mmol"), T("; auf dem Nachttisch ein Beutel "), B("Magnesiocard 10"), T(" (10 mmol Magnesium), ein Beutel "), B("Movicol"), T(" trinkfertig und eine Tube "), B("Excipial U Lipolotio"), T(" (Harnstoff 4 Prozent, Hautpflege). Am Ständer hängt "), B("Ringer-Acetat 500 ml"), T(". Was sonst läuft – Novalgin, der Blutdrucksenker, Sauerstoff –, ist an diesem Tag nicht abgelesen worden; die Liste ist wieder eine Momentaufnahme, nur eine frischere.")]),
    P(&[B("Kalium und Magnesium sind die richtige Nachricht."), T(" Sie sind genau die zwei Werte, die dieses Blatt seit dem Kostaufbau erfragt, und dass sie ersetzt werden, heisst mit einiger Wahrscheinlichkeit, dass sie gemessen wurden und tief waren – oder dass vorgebeugt wird. Beides ist richtig. Nur tragen beide Präparate bei ihr eine Gegenanzeige, die an der Grenze liegt, und beide brauchen deshalb die Zahl, nicht das Rezept.")]),
    Liste(&[
        &[B("Kaliumchlorid retard."), T(" Die Fachinformation eines Schweizer Retard-Präparats nennt als Gegenanzeigen "), L("«schwere Niereninsuffizienz mit Oligo- und Anurie» und «verzögerte oder verhinderte Magen-Darm-Passage»", "https://ch.oddb.org/de/gcc/fachinfo/reg/29138/chapter/contra_indications"), T(". Sie hatte am 30. August eine Oligurie, und ihr Darm stand wochenlang – eine Retardtablette, die im Darm liegen bleibt, ist genau das, wovor der zweite Satz warnt. Dazu die Nierenfunktion: Kalium wird über die Niere ausgeschieden, und bei einer GFR von 30 steigt es leichter, als es fällt. Der Interaktionscheck mit SDIF ergibt für die Kombination mit einem "), L("ACE-Hemmer die EPha-Klasse C", "https://sdif.oddb.org/?tab=check&drugs=A12BA01-A12CC05-A02BC02-C09AA03-C03CA04-N02BB02"), T(" – erhöhtes Risiko einer Hyperkaliämie, engmaschige Überwachung von Kalium und Nierenfunktion. Ob das zutrifft, hängt an der einen Frage, die seit dem 2. September offen ist: "), B("Welcher Blutdrucksenker ist es?"), T(" Lisinopril ist ein ACE-Hemmer.")],
        &[B("Magnesium."), T(" Die Fachinformation nennt als Gegenanzeige "), L("«schwere Nierenfunktionsstörungen (glomeruläre Filtrationsrate unter 30 ml/min)»", "https://ch.oddb.org/de/gcc/fachinfo/reg/51458/chapter/contra_indications"), T(" – ihre GFR liegt bei 30, also genau auf der Linie – und verlangt bei jeder Einschränkung der Nierenfunktion "), L("die Überwachung des Serum-Magnesiums als «unerlässlich»", "https://ch.oddb.org/de/gcc/fachinfo/reg/51458/chapter/restrictions"), T(", weil Magnesium sich anreichert. Der Abschnitt zum Spinat hat das bereits gesagt: Ein Mangel gehört ausgeglichen, aber bei dieser Niere nicht ohne Wert.")],
        &[B("Ringer-Acetat statt Kochsalz."), T(" Das ist eine Änderung zum Besseren und eine offene Frage zugleich. Die Lösung enthält je Liter 137 mmol Natrium und 110 mmol Chlorid statt 154 und 154 – weniger Salz, weniger Chlorid, dazu Acetat als Puffer, was zur Übersäuerung passt. Sie enthält aber auch 4 mmol Kalium und 1,25 mmol Magnesium je Liter, die zu den Tabletten und Beuteln dazukommen. Und ihre Fachinformation nennt als erste Gegenanzeige "), L("«Ödeme»", "https://ch.oddb.org/de/gcc/fachinfo/reg/58605/chapter/contra_indications"), T(", als weitere Hyperkaliämie und schwere Niereninsuffizienz, und verlangt, dass "), L("Elektrolyte, Säure-Basen-Haushalt und Wasserbilanz überwacht werden", "https://ch.oddb.org/de/gcc/fachinfo/reg/58605/chapter/restrictions"), T(". Die Frage nach der Menge, die dieses Blatt seit dem 3. September stellt, bleibt damit dieselbe – nur mit einer Lösung, die sie selbst stellt.")],
    ]),
    P(&[B("Daraus wird eine einzige Frage, und sie ist einfach:"), T(" "), B("Wie hoch sind Kalium, Magnesium und Kreatinin heute – und wer schaut morgen wieder?"), T(" Drei Zahlen, die alle drei Präparate rechtfertigen oder nicht. Sie stehen im Labor; sie müssen nur genannt werden.")]),
    P(&[B("Eine Unterscheidung entscheidet, wie diese Liste zu lesen ist."), T(" Fest verordnet sind nur zwei Mittel: der Säureblocker und das Mittel gegen Übelkeit. Alles Übrige – beide Opioide, Metoclopramid, Haloperidol, das Kortikoid – steht "), B("in Reserve"), T(", also für den Bedarfsfall. Was verordnet ist, ist deshalb nicht dasselbe wie das, was gegeben wurde. "), B("Die Frage lautet also nicht, was auf dem Blatt steht, sondern was das Verabreichungsprotokoll zeigt:"), T(" welches Reservemittel wie oft und in welcher Dosis tatsächlich verabreicht wurde. Das ist eine einzige Auskunft, und ohne sie lässt sich der folgende Abschnitt nicht abschliessen.")]),
    P(&[B("Denn dies springt aus der Liste heraus."), T(" Die Hauptbeschwerde ist ein Darm, der nicht transportiert. Auf der Liste stehen "), B("drei Mittel, die genau das verursachen können"), T(" – zwei Opioide und das Mittel gegen Übelkeit. Das ist die einzige der vier Erklärungen für den Stillstand, die sich durch Weglassen prüfen lässt.")]),
    Liste(&[
        &[B("Oxycodon."), T(" Die Fachinformation führt Obstipation als sehr häufige "), L("unerwünschte Wirkung mit 30,5 Prozent", "https://ch.oddb.org/de/gcc/fachinfo/reg/55352/chapter/unwanted_effects"), T(" auf, dazu Übelkeit mit 35,4 und Erbrechen mit 16,0 Prozent – also genau die drei Beschwerden, gegen die die übrigen Mittel dieser Liste gegeben werden. Ileus steht als gelegentliche unerwünschte Wirkung daneben, und periphere Ödeme ebenfalls. Dieselbe Stelle verlangt, dass "), B("ab Behandlungsbeginn"), T(" vorbeugend abgeführt wird. Und unter den Gegenanzeigen steht der "), L("paralytische Ileus", "https://ch.oddb.org/de/gcc/fachinfo/reg/55352/chapter/contra_indications"), T(".")],
        &[B("Morphin."), T(" Dieselbe Richtung, und die Gegenanzeigen wiegen schwerer. Die Fachinformation nennt "), L("paralytischen Ileus, akutes Abdomen unbekannter Genese und schwere Nieren- oder Leberinsuffizienz", "https://ch.oddb.org/de/gcc/fachinfo/reg/56400/chapter/contra_indications"), T(". Alle drei berühren diesen Fall: Der Gastroenterologe spricht von fehlender Peristaltik, die Beurteilung lautet «unklares Krankheitsbild» mit Bauchschmerz, und die Nierenfunktion liegt bei einer GFR von 30. Ob eine davon im ärztlichen Urteil zutrifft, entscheidet die Station – die Frage gehört gestellt, weil sie in der Fachinformation steht.")],
        &[B("Ondansetron, viermal täglich, fest."), T(" Die Fachinformation führt "), L("Verstopfung infolge Erhöhung der Dickdarm-Transitzeit", "https://ch.oddb.org/de/gcc/fachinfo/reg/67214/chapter/unwanted_effects"), T(" als unerwünschte Wirkung. Der Wortlaut ist bemerkenswert genau: erhöhte Transitzeit im Dickdarm. Das ist der Befund des Gastroenterologen, als unerwünschte Wirkung formuliert – und dieses Mittel läuft nicht in Reserve, sondern viermal am Tag.")],
    ]),
    P(&[B("Daraus folgt keine Anklage, sondern eine Frage mit klarer Form."), T(" Alle drei Mittel haben einen guten Grund: Schmerz und Erbrechen sind real und gehören behandelt, und niemand gibt einer Frau von 84 Jahren leichtfertig ein Opioid. Nur ist die Hauptbeschwerde inzwischen eine andere geworden. Die Frage lautet deshalb nicht «warum gibt man das», sondern: "), B("Was davon läuft noch, in welcher Dosis – und ist der Darmstillstand unter Fortführung neu beurteilt worden?"), T(" Und falls die Opioide bleiben: Läuft die vorbeugende Abführbehandlung, die die Fachinformation ab dem ersten Tag verlangt? Die Antwort darauf könnte die Beschwerde lindern, die die Patientin am meisten belastet – und keine der anderen drei Erklärungen lässt sich so schnell prüfen.")]),
    P(&[B("Zwei weitere Punkte aus derselben Liste."), T(" Metoclopramid ist das einzige Mittel darauf, das den Transport "), B("fördert"), T(", und es steht in doppeltem Widerspruch zum Rest. Erstens heben es und die Opioide einander laut Fachinformation gegenseitig auf – wechselseitiger Antagonismus an der Magen-Darm-Beweglichkeit. Zweitens gilt für Metoclopramid eine "), L("maximale Therapiedauer von fünf Tagen", "https://ch.oddb.org/de/gcc/fachinfo/reg/32733/chapter/usage"), T(" wegen möglicherweise nicht rückbildungsfähiger Spätdyskinesien, und bei einer Kreatinin-Clearance zwischen 15 und 60 ml/min soll die Dosis "), B("um die Hälfte reduziert"), T(" werden. Ihre GFR liegt bei 30, und der Eintritt war am 23. August – beides sind Angaben, die in dieser Rechnung vorkommen.")]),
    P(&[B("Und die Kombination, die der Interaktionscheck an die Spitze setzt."), T(" Ondansetron und Haloperidol zusammen sind in der EPha-Einstufung "), B("Risikoklasse D – Kombination vermeiden"), T(": additive Verlängerung der QT-Zeit, erhöhtes Risiko für Kammerrhythmusstörungen, mit der ausdrücklichen Auflage regelmässiger EKG-Kontrollen. Dazu Metoclopramid mit Haloperidol, wo die Fachinformation verstärkte extrapyramidale Störungen nennt – bei einer 84-Jährigen kein theoretisches Risiko. Und zwei Opioide nebeneinander führen als schwerwiegende Interaktion additive Dämpfung bis zu Atemdepression und Koma. Alle drei Paare sind Reservemittel; ob sie je zusammentrafen, sagt wieder nur das Verabreichungsprotokoll.")]),
    P(&[B("Der Punkt, der eine Aussage dieses Blattes berührt."), T(" Auf der Liste steht "), B("Dexamethason 4 mg in Reserve"), T(". Dieses Blatt sagt an mehreren Stellen, sie habe nie ein Kortikoid bekommen, und das war nach dem Stand der Angaben richtig. «In Reserve» heisst: verordnet für den Bedarfsfall, nicht zwingend gegeben. Die Frage ist damit offen – und es hängt einiges daran. Ein Kortikoid würde die Beurteilung der Ödeme ändern, weil die Fachinformation "), L("Natrium- und Wasserretention sowie Ödeme", "https://ch.oddb.org/de/gcc/fachinfo/reg/41074/chapter/unwanted_effects"), T(" aufführt; es würde bei der Magenblutung mitreden, weil dieselbe Stelle peptische Ulzera mit möglicher Blutung oder Perforation nennt; und es würde eine spätere Gewebeprobe beeinflussen. Bis das geklärt ist, steht die Aussage «nie Kortison» in diesem Blatt unter Vorbehalt.")]),
    Klein(&[T("Ein Fehlalarm aus demselben Lauf gehört benannt, weil er zeigt, wie solche Werkzeuge irren. Der Check meldete «Ondansetron gegen Morphin: kontraindiziert». Nachgelesen betrifft dieser Text "), B("Apomorphin"), T(", ein Mittel gegen die Parkinson-Krankheit – der Wortabgleich hatte «morphin» innerhalb von «Apomorphin» gefunden. Diese Kombination ist nicht kontraindiziert. Jeder Treffer eines Interaktionsprogramms gehört gegen die Fachinformation nachgelesen, bevor er irgendwo hineingeschrieben wird.")]),

    H3("Neu und auf keiner Liste: ein Blutdrucksenker"),
    P(&[T("Am 2. September kommt die Angabe dazu, sie bekomme ein Mittel gegen den Blutdruck. Auf der Verordnungsliste vom 30. August steht keines, und unter den Nebendiagnosen des Berichts steht kein Bluthochdruck. Entweder ist es seit der Verlegung dazugekommen, oder es lief zu Hause und fehlt auf der Liste. Beides ist wissenswert, und das zweite wäre der Beleg dafür, dass diese Liste nicht vollständig war.")]),
    P(&[B("Welche Klasse es ist, entscheidet drei verschiedene Dinge – und jedes davon berührt eine offene Frage dieses Blattes.")]),
    Liste(&[
        &[B("Ein Kalziumantagonist, etwa Amlodipin."), T(" Dann wäre ein Teil des Wassers in den Beinen die Tablette. Die Fachinformation führt "), L("Ödeme mit 11,1 Prozent", "https://ch.oddb.org/de/gcc/fachinfo/reg/59261/chapter/unwanted_effects"), T(" auf – als einzige unerwünschte Wirkung dieses Mittels, die dort als «sehr häufig» geführt wird; «Knöchelschwellungen» stehen in derselben Rubrik noch einmal gesondert als häufig. Der Abschnitt «Ödeme an den Füssen» bekäme damit einen weiteren Eintrag, und zwar einen der wenigen, die sich durch Weglassen prüfen lassen.")],
        &[B("Ein ACE-Hemmer oder ein Sartan."), T(" Dann zählt die Dosis. Für Lisinopril nennt die Fachinformation bei einer Kreatinin-Clearance zwischen 10 und 30 ml/min eine "), L("Anfangsdosis von 2,5 bis 5 mg", "https://ch.oddb.org/de/gcc/fachinfo/reg/56777/chapter/usage"), T("; ihre GFR liegt bei 30. Dieselbe Fachinformation verlangt eine niedrigere Anfangsdosis bei Volumen- oder Salzmangel jeder Ursache und nennt als Risikofaktoren für eine Kaliumerhöhung ausdrücklich Niereninsuffizienz und Diabetes – bei ihr eine GFR von 30 und ein Prädiabetes mit HbA1c "), N("6,3 %"), T(". Kalium steht in diesem Blatt ohnehin auf der Liste, dort allerdings aus dem entgegengesetzten Grund.")],
        &[B("Ein Entwässerungsmittel."), T(" Dann ist es kein Blutdruckmittel im eigentlichen Sinn, sondern die Antwort auf die sechs Kilo – und die Zuordnung der Angehörigen wäre genau die Art von Zuordnung, vor der dieses Blatt beim Sauerstoff warnt. Die Gegenanzeigen von Torasemid lesen sich allerdings wie eine Aufzählung ihrer Befunde: "), L("Hypovolämie, Hyponatriämie, Hypokaliämie, Nierenversagen mit Anurie und erhebliche Miktionsstörungen", "https://ch.oddb.org/de/gcc/fachinfo/reg/67971/chapter/contra_indications"), T(". Das spricht nicht dagegen, es zu geben; es spricht dafür, die Elektrolyte dabei zu messen.")],
    ]),
    P(&[B("Die Rückfrage ist kurz und hat vier Teile:"), T(" Welches Präparat, in welcher Dosis, seit wann – und über welchen Weg? Der letzte Teil ist keine Förmlichkeit: Sie hat am 2. September wieder erbrochen. Eine Tablette, die wieder herauskommt, ist keine Dosis; das gilt für den Blutdrucksenker wie für das Pantoprazol, bei dem dieses Blatt dieselbe Frage schon stellt.")]),

    H2("Die Verlegung ins Universitätsspital"),
    P(&[T("Zwei Dinge ändern sich damit, und sie ziehen in entgegengesetzte Richtungen.")]),
    P(&[B("Das Gute zuerst."), T(" Dieses Blatt nennt seit der ersten Fassung zwei Fächer, die gemeinsam zuständig sind – die Rheumatologie für die Vaskulitis, die Nephrologie für die Niere –, und führt am Schluss ihre Adressen auf. Beide sind im Haus, in das sie heute verlegt wird. Der Weg dorthin führte bisher über eine Zuweisung und eine Warteliste; für eine stationäre Patientin ist es ein Konsil, das die behandelnde Abteilung anfordert. Was in diesem Blatt an eine Sprechstunde adressiert war, lässt sich damit im Haus stellen – und die Gastroenterologie steht daneben.")]),
    P(&[B("Und das, was an einem Verlegungstag regelmässig schiefgeht."), T(" Eine Verlegung ist die Stelle, an der Information verlorengeht. In einer Auswertung von 335 Verlegungen an ein Universitätsspital war die Übergabedokumentation im Mittel zu "), N("58,3 Prozent"), T(" vollständig, und bei 42 Prozent der Patienten trat innert 24 Stunden nach der Ankunft ein unerwünschtes Ereignis ein. Eine "), L("vollständigere Dokumentation", "https://pmc.ncbi.nlm.nih.gov/articles/PMC5096986/"), T(" ging mit geringerer Sterblichkeit, weniger unerwünschten Ereignissen und weniger doppelt gemachter Arbeit einher, auch nachdem die Schwere der Erkrankung herausgerechnet war. Die Einschränkung gehört dazu: Untersucht wurden rückblickend Verlegungen auf Intensivstationen in den Vereinigten Staaten. Was sich überträgt, ist nicht die Zahl, sondern die Stelle, an der es klemmt.")]),
    P(&[B("Daraus folgt, wofür dieses Blatt ab heute da ist."), T(" Die Liste «Was zum Termin mitgehört» ist eine Übergabeliste geworden. Drei Fäden dürfen dabei nicht abreissen, weil an jedem eine Frist hängt:")]),
    Liste(&[
        &[B("Die Blutungsquelle."), T(" Kaffeesatz oben und Teerstuhl unten heissen Blutung im oberen Verdauungstrakt, und dafür nennen die Fachgesellschaften 24 Stunden nach der Kreislaufstabilisierung. Diese Uhr läuft während der Verlegung weiter.")],
        &[B("Der Hämoglobinwert nach der Transfusion."), T(" Er ist die nützlichste Zahl der ganzen Woche – und er wird genau dann fällig, wenn die Patientin das Haus wechselt. Siehe den Abschnitt «Die Transfusion».")],
        &[B("Der Kostaufbau."), T(" Er hat gerade begonnen, und die gefährlichen Tage sind die ersten vier. Thiamin, Phosphat, Kalium, Magnesium – siehe den Abschnitt «Der Kostaufbau nach Wochen ohne Nahrung» und das Begleitblatt «Kostaufbau nach langem Hungern».")],
    ]),
    P(&[T("Keiner dieser drei Punkte ist neu; alle drei stehen weiter unten ausführlich. Sie stehen hier noch einmal, weil eine Verlegung genau die Art von Ereignis ist, nach der laufende Fragen als beantwortet gelten, ohne beantwortet worden zu sein.")]),
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

    H3("Was offen bleibt: die 23 g/l"),
    P(&[T("Keine der drei Meldungen erklärt den Hämoglobinabfall. Der unauffällige Urin sagt etwas über die Niere und nichts über den Blutverlust – die langsame Blutarmut aus nachlassender Nierenfunktion passt zu diesem Tempo ohnehin nicht. Und die Auskunft, der dunkle Stuhl sei Galle gewesen, nimmt, falls sie zutrifft, die naheliegendste Quelle aus der Rechnung. Damit wird die Frage nicht kleiner, sondern dringlicher: Es fehlen "), N("23 g/l"), T(", und die wahrscheinlichste Erklärung wäre gerade ausgeschieden. Wie sich das ohne grossen Aufwand weiterverfolgen lässt, steht im Abschnitt «Dunkler Stuhl: Galle oder Blut?».")]),
    P(&[T("Der einfachste Schritt ist ein neuer Hämoglobinwert zusammen mit den Retikulozyten, jetzt, wo wieder getrunken wird und die Austrocknung den Wert nicht mehr schönt. Er sagt zweierlei: ob der Verlust steht oder weiterläuft, und ob das Knochenmark antwortet.")]),

    H2("Die neue Diagnose: starke Magenentzündung"),
    P(&[T("Damit steht ein Befund im Raum, der zwei Dinge auf einmal erklärt, die bisher unverbunden nebeneinanderlagen. Eine entzündete Magenschleimhaut erklärt das Erbrechen, das jedes Mal kam, sobald eine Kleinigkeit gegessen war. Und sie erklärt einen Teil der fehlenden "), N("23 g/l"), T(": Eine wunde Schleimhaut blutet nicht in einem Schwall, sie sickert – über Wochen, ohne dass es am Stuhl auffallen muss.")]),
    P(&[B("Nur ist «Magenentzündung» ein Befund und keine Ursache."), T(" Das Wort beschreibt, wie die Schleimhaut aussieht, nicht, was sie so aussehen lässt. Genau daran hängt aber die Behandlung. Drei Erklärungen stehen hier nebeneinander, und dieselbe Untersuchung trennt sie:")]),
    Liste(&[
        &[B("Die Vaskulitis selbst."), T(" Naheliegend – nur zeigt der Blick in die Zahlen etwas Unerwartetes. In einer "), L("Untersuchung an 108 Erwachsenen", "https://pmc.ncbi.nlm.nih.gov/articles/PMC11429444/"), T(" mit Bauchbeteiligung war bei den 61 Magenspiegelungen der Magen selbst nur zweimal betroffen, in 3,3 Prozent der Fälle – der Zwölffingerdarm dagegen 34-mal, in 55,7 Prozent, und dort vor allem der absteigende Teil und der Bulbus. Zu sehen waren Rötung mit Erosionen (48 Prozent) sowie Erosionen mit Geschwüren (31 Prozent). "), B("Daraus folgt etwas Praktisches:"), T(" Ist diese Entzündung die Vaskulitis, liegt ihr Beweis meist hinter dem Magenausgang. Das Gerät muss bis in den absteigenden Zwölffingerdarm, und die Gewebeproben gehören dorthin – nicht nur in den Magen.")],
        &[B("Die Medikamente."), T(" Niedrig dosiertes Aspirin schädigt die Magenschleimhaut, und dazu Novalgin, dessen Fachinformation "), L("gastrointestinale Blutungen, Ulzerationen und Perforationen", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/unwanted_effects"), T(" unter den unerwünschten Wirkungen führt. Zwei Mittel, die am selben Ort angreifen, und beide liefen, bevor der Befund erhoben wurde. Kortison, das hier sonst als dritter Faktor stünde, entfällt: Sie hat nie eines bekommen – siehe den Abschnitt «Zur Behandlung».")],
        &[B("Helicobacter pylori."), T(" Aus denselben Gewebeproben in einem Schritt beantwortet. Ist das Bakterium da, ist die Behandlung eine andere: Die Fachinformation sieht dafür "), L("Pantoprazol zusammen mit zwei Antibiotika über sieben Tage", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/usage"), T(" vor. Ein Säureblocker allein heilt diese Form nicht aus.")],
    ]),
    P(&[B("Der wichtigste Satz zu dieser Diagnose steht in der Fachinformation des neuen Medikaments."), T(" Sie nennt "), L("Warnsymptome", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/restrictions"), T(", bei denen eine bösartige Erkrankung ausgeschlossen werden "), B("muss"), T(": unerklärlicher Gewichtsverlust, wiederholtes Erbrechen, Schluckstörung, Bluterbrechen, Blutarmut, Teerstuhl. Drei davon treffen hier sicher zu – Gewichtsverlust, wiederholtes Erbrechen, Blutarmut –, und der vierte ist genau die offene Frage aus dem Abschnitt «Dunkler Stuhl: Galle oder Blut?». Die Begründung steht gleich dahinter: Die Behandlung mit Pantoprazol kann die Symptome kaschieren und die Diagnosestellung dadurch verzögern.")]),
    P(&[T("Das kehrt eine naheliegende Erwartung um. Der Magenschutz macht die Spiegelung nicht weniger dringend, sondern dringender – er nimmt die Beschwerden weg, ohne die Frage zu beantworten, woher sie kamen. Wird es unter dem Mittel besser, ist das eine Erleichterung und kein Befund.")]),
    P(&[B("Inzwischen hat sich diese Frage beantwortet, und zwar in die unangenehme Richtung."), T(" Der Magen ist weiter gebläht, das Aufstossen hält an. Unter Pantoprazol ist es also nicht besser geworden. Damit entfällt die Erleichterung und der Befund bleibt stehen: Beschwerden, die unter voller Säurehemmung fortbestehen, sind mit der Säure nicht erklärt. Die Warnsymptome aus der Fachinformation gelten unverändert – und die Begründung, mit der sie dort stehen, greift jetzt doppelt. Ein Mittel, dem man zutraut, die Symptome zu kaschieren, hat sie nicht einmal kaschiert.")]),

    H2("Blut im Urin: warum das nicht aus dem Magen kommen kann"),
    P(&[B("Der Stand in zwei Sätzen."), T(" Das Blut im Stuhl ist geklärt: Kaffeesatz oben und Teerstuhl unten beweisen zusammen eine Blutung im oberen Verdauungstrakt, also aus Magen oder Zwölffingerdarm. Das Blut im Urin ist "), B("nicht"), T(" geklärt – es kann aus der Niere stammen, dann ist es die Nierenbeteiligung dieser Vaskulitis, oder aus Harnleiter und Blase, wofür der Stau an mehreren Stellen spricht. Eine einzige Untersuchung entscheidet das; sie steht weiter unten in diesem Abschnitt. Beides sind aber zwei getrennte Blutungen an zwei Orten, und keine erklärt die andere.")]),
    P(&[B("Das sichtbare Blut ist inzwischen weg."), T(" Das ist eine echte Besserung und ein Datum, das in den Verlauf gehört. Es ist aber "), B("kein Befund über die Niere"), T(", und die Untersuchung, die dieser Abschnitt verlangt, wird dadurch dringender statt überflüssig. Zwei Gründe.")]),
    P(&[B("Erstens sind solche Episoden von Natur aus selbstlimitierend."), T(" Bei einer IgA-Erkrankung der Niere dauert eine Episode sichtbaren Bluts im Urin typischerweise "), L("weniger als drei Tage", "https://www.ncbi.nlm.nih.gov/books/NBK538214/"), T(", nach anderen Angaben ein bis sieben. Entscheidend ist der Satz, der in denselben Darstellungen unmittelbar danach steht: "), B("Zwischen den Episoden bestehen die mikroskopische Blutbeimengung und die Eiweissausscheidung fort."), T(" Das Verschwinden der Farbe sagt also nichts darüber, ob der Filter noch entzündet ist – es sagt nur, dass gerade weniger Blut kommt. Wer jetzt aufhört zu kontrollieren, macht genau den Fehler, vor dem der Abschnitt «Der Ausschlag geht zurück» warnt.")]),
    P(&[B("Zweitens passt es ebenso gut zur anderen Erklärung."), T(" Die Ableitungen liegen seit dem 27. August. Kam das Blut aus dem gestauten, dann instrumentierten Harnweg, ist sein Verschwinden die erwartete Folge der Entlastung. Beide Deutungen erklären dieselbe Beobachtung – und sie unterscheiden sich in allem, was danach kommt. Getrennt werden sie nur durch das Sediment. "), B("Das Zeitfenster dafür schliesst sich gerade:"), T(" Solange sichtbar Blut kam, war die Probe reich an Material. Die angeforderten glomerulären Erythrozyten stammen hoffentlich aus dieser Zeit; falls nicht, gehört jetzt eine Probe untersucht und nicht in zwei Wochen.")]),
    P(&[B("Und eine gute Folge hat es sofort."), T(" Der Bericht führt den Hämoglobinabfall ausdrücklich auf die Makrohämaturie zurück. Hört diese auf, muss der Wert halten. Tut er es nicht, war sie nicht die Quelle – und dann rückt die Blutung im Verdauungstrakt zurück in den Vordergrund. Der Hämoglobinwert der nächsten Tage beantwortet das ohne jede weitere Untersuchung.")]),
    P(&[T("Die Frage liegt nahe, und ihre Antwort ist eindeutig: Blut aus dem Magen kann nicht in den Urin gelangen. Verdauungstrakt und Harnwege sind zwei getrennte Rohrsysteme; sie berühren einander nirgends. Was im Magen blutet, nimmt den Weg durch den Darm – und wird dort verdaut wie Nahrung. Genau davon handelt der Abschnitt «Dunkler Stuhl: Galle oder Blut?»: Das Bluteiweiss wird aufgenommen, und der Harnstoff im Blut steigt. Das ist die einzige Spur, die eine Blutung im Magen jenseits des Stuhls hinterlässt – Harnstoff im Blut, nicht Blut im Urin.")]),
    P(&[B("Es sind also zwei Blutungsorte und nicht einer."), T(" Das ist bei dieser Krankheit kein Zufall, sondern ihr Bauplan. Die IgA-Vaskulitis ist eine Entzündung der kleinen Gefässe, und die IgA-Ablagerungen sitzen "), L("in Magen-Darm-Trakt, Gelenken, Haut und Nieren", "https://www.aafp.org/pubs/afp/issues/2020/0815/p229.html"), T(" zugleich. Der Ausschlag an den Beinen, der Bauch und der Urin sind nicht drei Krankheiten, sondern derselbe Vorgang an drei Orten. Wer jetzt Blut im Urin sieht, sieht nichts Neues – er sieht das dritte Organ.")]),
    P(&[B("Deshalb spricht das Blut im Urin nicht gegen die Darmblutung, sondern für sie."), T(" Es zeigt, dass die Vaskulitis in diesem Moment aktiv ist, und eine aktive Vaskulitis ist genau die Erklärung, die eine Blutung im Zwölffingerdarm braucht. Dass beides zusammen die ungünstige Kombination ist, zeigt eine "), L("Untersuchung an 30 Erwachsenen", "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6942937/"), T(" mit IgA-Vaskulitis und Nierenbeteiligung: Von den zehn mit einer Magen-Darm-Blutung wurden 50 Prozent dialysepflichtig, von den zwanzig übrigen 5 Prozent. Die Zahl gehört mit Vorsicht gelesen – 30 Patienten sind wenig, und in der um Bluthochdruck und Eiweissausscheidung bereinigten Rechnung war der Zusammenhang nicht mehr statistisch gesichert. Als Richtungsangabe taugt sie trotzdem, und sie zeigt dorthin, wohin dieses Blatt an mehreren Stellen zeigt: zur Spiegelung.")]),
    P(&[B("Was sichtbares Blut bedeutet – und was nicht."), T(" Die Patientin berichtet eine Makrohämaturie, also mit blossem Auge sichtbares Blut. Das ist ein anderer Befund als die mikroskopische Hämaturie. Die Nierenbeteiligung dieser Krankheit zeigt sich typischerweise unter dem Mikroskop: "), L("mikroskopische Hämaturie, Erythrozytenzylinder, Eiweiss im Urin", "https://www.aafp.org/pubs/afp/issues/2020/0815/p229.html"), T(", bis hin zum offenen Nierenversagen. Sichtbares Blut kommt vor, ist aber zugleich der Befund, mit dem in den Fallberichten eine seltene Komplikation begonnen hat, die durch den gestauten Harnleiter neu im Raum steht – siehe den nächsten Abschnitt.")]),
    P(&[B("Eine einzige Untersuchung entscheidet, woher das Blut kommt."), T(" Ein Streifentest genügt dafür nicht; er wird auch dann positiv, wenn nur freier Blutfarbstoff im Urin ist, ohne rote Blutkörperchen. Das Sediment sagt mehr. Rote Blutkörperchen, die den Nierenfilter passiert haben, sehen anders aus als solche, die aus Harnleiter oder Blase stammen: "), B("Akanthozyten"), T(" heissen die ringförmigen Zellen mit den bläschenartigen Ausstülpungen. In der Arbeit, die sie beschrieben hat, fanden sich Akanthozyten von mindestens 5 Prozent bei 75 von 143 Patienten mit gesicherter Glomerulonephritis, also mit einer Empfindlichkeit von 52 Prozent – aber nur bei 4 von 187 mit nicht-glomerulärer Ursache, einer "), L("Spezifität von 98 Prozent", "https://pubmed.ncbi.nlm.nih.gov/1921146/"), T(". Übersetzt: Findet man sie, ist die Frage entschieden; findet man sie nicht, ist nichts ausgeschlossen. Erythrozytenzylinder beweisen den Ursprung in der Niere ebenso, weil sie nur im Nierenkanälchen entstehen können.")]),
    P(&[T("Dazu gehört, aus derselben Probe, der Protein-Kreatinin-Quotient. Er beantwortet die zweite Hälfte der Frage. Blut allein ist das eine, Blut zusammen mit Eiweissverlust das andere – am Eiweissverlust hängt die Prognose, und an ihm entscheidet sich, ob eine Nierenbiopsie angezeigt ist.")]),
    P(&[B("Und was das Alter dazu sagt."), T(" Eine Auswertung des japanischen Nierenbiopsie-Registers vergleicht Erwachsene zwischen 19 und 64 Jahren mit über 65-Jährigen, alle mit gesicherter IgA-Vaskulitis-Nephritis. Die Älteren starteten nicht schlechter, aber ihr Verlauf war steiler: Ein Anstieg des Kreatinins um die Hälfte trat bei "), L("21,7 gegenüber 4,7 Prozent", "https://pmc.ncbi.nlm.nih.gov/articles/PMC5940189/"), T(" ein, beobachtet über im Mittel 3,9 Jahre. Das Nierenüberleben nach neun Jahren war in beiden Gruppen gut, 91,4 gegenüber 93,6 Prozent – erreicht allerdings unter konsequenter Behandlung mit Kortison und einem Blutdruckmittel aus der RAS-Gruppe. Der Schluss der Autoren ist genau der, der hier zählt: Jenseits von 65 gehört der Verlauf der Nierenfunktion sorgfältig überwacht.")]),

    H2("Die Glomerulonephritis: was in der Niere geschieht"),
    P(&[T("Glomeruli sind die Filterkörperchen der Niere, rund eine Million je Niere. «Glomerulonephritis» heisst nichts anderes, als dass diese Filter entzündet sind. Bei dieser Krankheit geschieht das auf demselben Weg wie in Haut und Darm: IgA-Immunkomplexe lagern sich ab, hier im Mesangium, dem Stützgewebe zwischen den Kapillarschlingen des Filters. Das ist keine Analogie, sondern derselbe Vorgang – genau das war der Beweis von 1973, der in der Zeittafel zuoberst steht. Ein entzündeter Filter lässt durch, was er zurückhalten sollte: rote Blutkörperchen und Eiweiss. Deshalb sind Sediment und Protein-Kreatinin-Quotient die zwei Messgrössen, die in diesem Blatt immer wieder vorkommen.")]),
    P(&[B("Die Diagnose stellt nur die Nierenbiopsie."), T(" Die KDIGO-Leitlinie von 2025 ist an dieser Stelle unmissverständlich: Eine IgA-Vaskulitis-Nephritis lässt sich "), L("nur mit einer Nierenbiopsie diagnostizieren", "https://kdigo.org/wp-content/uploads/2024/08/KDIGO-2025-IgAV-Guideline-Key-Takeaways_IgAV.pdf"), T(", und diese soll bei Erwachsenen mit vermuteter IgA-Vaskulitis durchgeführt werden, wenn Zeichen einer erheblichen Organschädigung bestehen, wenn die Eiweissausscheidung über mehr als vier Wochen bei mindestens 0,5 g pro Tag liegt oder wenn die Nierenfunktion eingeschränkt ist. Damit ist die Nierenbiopsie in diesem Blatt keine offene Möglichkeit mehr, sondern an benannte Bedingungen geknüpft – und ob eine davon zutrifft, entscheidet sich an zwei Werten, die man kennen muss: Eiweiss im Urin und Kreatinin.")]),
    P(&[B("Was den Verlauf vorhersagt – und was nicht."), T(" Nach den Registerdaten, die die Leitlinie zusammenfasst, sind es unkontrollierter Bluthochdruck und die Menge der Eiweissausscheidung, sowohl bei der ersten Vorstellung als auch im Mittel über den Verlauf. Eine Schwelle zieht sie ausdrücklich: Eine Eiweissausscheidung von mindestens "), N("0,5 g/Tag"), T(" kennzeichnet eine Patientin mit erhöhtem Risiko für einen fortschreitenden Verlust der Nierenfunktion. Bemerkenswert ist, was die Leitlinie "), B("nicht"), T(" gelten lässt: Die Datenlage reiche nicht aus, um Behandlungsentscheidungen auf die Zahl der Halbmonde oder auf den MEST-C-Score der Oxford-Klassifikation zu stützen. Beides steht in Befundberichten prominent – als Grundlage einer Therapieentscheidung taugt es laut Leitlinie nicht.")]),
    P(&[B("Was unabhängig von jeder Immunbehandlung zu tun ist."), T(" Auch das steht dort, und es ist der Teil, der oft untergeht: Blutdruck auf ein Ziel von "), N("≤120/70 mmHg"), T(", mit einem RAS-Hemmer – ACE-Hemmer oder Sartan – als erster Wahl, dazu Massnahmen gegen die Überlastung des Filters. Das ist die Linie für die Zeit nach der akuten Phase, nicht für den heutigen Tag mit laufender Transfusion; aber sie gehört auf den Plan, weil sie den Verlauf über Jahre bestimmt.")]),
    P(&[B("Der Sonderfall, der Eile macht."), T(" Steigt das Kreatinin rasch, spricht man von einer rapid progressiven Glomerulonephritis. Für diesen Fall verweist die Leitlinie auf das Vorgehen bei der ANCA-assoziierten Vaskulitis – Glukokortikoide zusammen mit Cyclophosphamid. Das ist die Lage, in der sich die Kortisonfrage aus dem Abschnitt «Zur Behandlung» von selbst beantwortet. Ob sie vorliegt, sagt der Verlauf des Kreatinins und sonst nichts.")]),
    P(&[B("Und ein zweiter Weg, auf dem sichtbares Blut der Niere schadet."), T(" Er ist in ihrem Alter der wahrscheinlichere und in diesem Blatt bisher nicht vorgekommen. Bei älteren Menschen ist ein Schub von sichtbarem Blut im Urin zusammen mit einem Nierenversagen eine der häufigsten Erscheinungsformen der verwandten IgA-Nephropathie. Der Schaden entsteht dabei nicht am entzündeten Filter, sondern durch das Blut selbst: In einer "), L("Untersuchung an 91 Patientinnen und Patienten ab 50 Jahren", "https://pubmed.ncbi.nlm.nih.gov/37547537/"), T(" – Durchschnittsalter 65 – fanden sich in "), B("allen"), T(" Nierenbiopsien Erythrozytenzylinder in den Nierenkanälchen und eine Tubulusnekrose, während Halbmonde und Zellvermehrung im Filter selten waren. Die roten Blutkörperchen verstopfen die Kanälchen und schädigen sie.")]),
    P(&[T("Was diese Arbeit für die Behandlungsfrage bedeutet, ist unbequem und gehört trotzdem hierher. 52 der 91 wurden immunsuppressiv behandelt, 39 nicht. Nach einem Jahr hatte sich die Nierenfunktion vollständig erholt bei 29 gegenüber 36 Prozent, teilweise bei 30,8 gegenüber 20,5, gar nicht bei 40,4 gegenüber 43,6 – kein bedeutsamer Unterschied, auch nicht nach statistischem Ausgleich der Ausgangsunterschiede. Das Nierenüberleben nach einem, drei und fünf Jahren war ebenfalls gleich. "), B("Dafür hatten 27 Prozent der Behandelten schwere unerwünschte Ereignisse."), T(" Der Schluss der Autoren: Immunsuppression ändert die ungünstige Prognose in dieser Lage nicht und geht häufig mit schweren Komplikationen einher.")]),
    P(&[B("Zwei Einschränkungen dazu, und beide zählen."), T(" Erstens betrifft diese Arbeit die IgA-Nephropathie, nicht die IgA-Vaskulitis-Nephritis. Die beiden teilen den Krankheitsmechanismus – dieselbe fehlerhaft verzuckerte IgA1 –, sind aber nicht dasselbe. Zweitens beschreibt sie genau eine Konstellation: ältere Patientin, sichtbares Blut im Urin, Nierenversagen. Ob das ihre ist, weiss man erst, wenn man weiss, wie das Kreatinin steht.")]),
    P(&[B("Damit stehen zwei Mechanismen nebeneinander, die von aussen gleich aussehen und verschieden behandelt werden."), T(" Der eine ist der entzündete Filter mit Halbmonden und rasch steigendem Kreatinin – dort greift die Immunbehandlung. Der andere sind Erythrozytenzylinder, die die Kanälchen verstopfen – dort hilft sie nach dieser Datenlage nicht und schadet in einem Viertel der Fälle. Beide zeigen sich als Blut im Urin und ein steigendes Kreatinin. Getrennt werden sie durch eine einzige Untersuchung, und es ist dieselbe, die die Leitlinie ohnehin verlangt: die Nierenbiopsie.")]),

    H2("Der gestaute Harnleiter"),
    P(&[T("Diese Auskunft ist neu und sie wiegt schwer. Ein gestauter Harnleiter heisst, dass der Urin nicht abfliesst. Hält das an, leidet die Niere an einem Hindernis unterhalb ihrer selbst – ein Nierenversagen, das nicht in der Niere entsteht und sich, anders als die übrigen Formen, allein durch die Wiederherstellung des Abflusses beheben lässt. Verfolgen lässt es sich am Kreatinin; das Bild dazu liefert der Ultraschall, und der kostet weder Kontrastmittel noch Strahlung und lässt sich beliebig oft wiederholen.")]),
    P(&[B("«An mehreren Stellen» ist dabei die eigentliche Information."), T(" Ein Stein sitzt an einer Stelle, ein Tumor ebenso. Mehrere Engstellen sprechen entweder für etwas, das dem Harnleiter der Länge nach zusetzt, oder für etwas, das wandert. Vier Erklärungen stehen nebeneinander, und sie sind nicht gleich wahrscheinlich:")]),
    Liste(&[
        &[B("Gerinnsel."), T(" Bei sichtbarem Blut im Urin die naheliegendste Erklärung und die einzige, die von selbst mehrere Stellen betrifft: Blut gerinnt auch in den Harnwegen. Sie erklärt zudem, warum der Stau erst jetzt auffällt, zusammen mit der Makrohämaturie und nicht vorher.")],
        &[B("Die Vaskulitis selbst."), T(" Es gibt sie, die stenosierende Ureteritis bei dieser Krankheit – dieselbe entzündete, geschwollene Wand wie im Darm, nur im Harnleiter. Sie ist selten; die Übersichten zählen für dreissig Jahre rund vierzehn beschriebene Fälle, und "), B("beschrieben sind sie fast nur bei Kindern"), T(". Die Berichte lesen sich allerdings wie diese Woche: Ein 14-Jähriger bekam 15 Tage nach Beginn der Purpura eine "), L("beidseitige Harnleiterenge", "https://pubmed.ncbi.nlm.nih.gov/9091100/"), T(", aufgefallen durch Flankenschmerz, Makrohämaturie und Nierenversagen – unter Kortison wurden beide Harnleiter wieder durchgängig und die Nierenfunktion normalisierte sich. In einem anderen Fall zeigte die Gewebeprobe aus dem Harnleiter eine "), L("schwere blutige Ureteritis mit Vaskulitis", "https://pubmed.ncbi.nlm.nih.gov/6854750/"), T(". Und ein dritter Bericht zeigt, wohin es führt, wenn man es übersieht: eine funktionslose Niere, die entfernt werden musste – der "), L("Ultraschall war in der ersten Woche normal", "https://pubmed.ncbi.nlm.nih.gov/18219497/"), T(", die Diagnose fiel erst acht Monate später. Dass sie nie Kortison bekommen hat, ist vor diesem Hintergrund kein Nebenumstand: In den Fallberichten war genau das das Mittel, unter dem die Harnleiter wieder durchgängig wurden.")],
        &[B("Druck von aussen – der geblähte Darm."), T(" Die Frage liegt nahe, und die Antwort lautet: Ja, das gibt es. Die Harnleiter laufen hinter dem Bauchfell, der Darm liegt davor; ein massiv gefüllter, gedehnter Darm kann den Harnleiter zudrücken. Beschrieben ist es unter anderem bei einer "), L("83-jährigen Frau", "https://doi.org/10.4235/agmr.20.0052"), T(" mit langjähriger Verstopfung: Die Computertomografie zeigte einen riesigen Kotstein, der den Übergang des Harnleiters in die Blase zusammendrückte und rechts eine Harnstauung machte – nach Ausräumung und Einlauf war die Stauung weg und das Kreatinin wieder normal. "), B("Hier passt der Vergleich allerdings nur halb."), T(" Jener Fall betraf eine über Jahre bestehende Verstopfung, hier war es eine Woche, und der Stuhl geht inzwischen wieder. Zu «an mehreren Stellen» passt Druck von aussen ohnehin schlecht, denn er sitzt meist an einer. Ganz vom Tisch ist der Gedanke damit nicht, weil der Darm weiterhin gebläht ist und schlecht transportiert – und geprüft wird er im selben Bild, das ohnehin ansteht.")],
        &[B("Die urologische Ursache, die das Alter vorgibt."), T(" Für sichtbares Blut im Urin gilt, was die amerikanische Urologenvereinigung für die Abklärung festlegt: Ein "), L("Alter ab 60 Jahren", "https://www.auanet.org/guidelines-and-quality/guidelines/microhematuria"), T(" ist für sich allein schon ein Hochrisikomerkmal, und dann gehören Blasenspiegelung und eine Bildgebung der oberen Harnwege dazu, im Regelfall eine CT-Urografie. Diese Regel steht hier nicht, weil ein Tumor wahrscheinlich wäre, sondern weil das Alter sie auslöst.")],
    ]),
    P(&[B("Daraus folgt etwas sehr Praktisches."), T(" Die Computertomografie des Bauches steht in diesem Blatt seit der ersten Fassung, wegen der Passagestörung. Als CT-Urografie gefahren, beantwortet dieselbe Untersuchung im selben Durchgang auch, wo der Harnleiter gestaut ist und wodurch. Ein Termin, ein Kontrastmittel, zwei Fragen. Ob das Kontrastmittel bei der aktuellen Nierenfunktion vertretbar ist, entscheidet die Ärztin – die Frage gehört gestellt, und der Ultraschall bleibt daneben das Mittel, das sich ohne Bedenken wiederholen lässt.")]),
    P(&[T("Und eine Warnung, die aus den Fallberichten kommt: Der Stau muss nicht von Anfang an da sein und nicht dauerhaft bleiben. In zwei der drei genannten Fälle war die erste Bildgebung unauffällig. Wer einmal gestaut war, gehört deshalb nachkontrolliert, auch wenn es zwischendurch besser aussieht.")]),

    H2("Der Katheter"),
    P(&[T("Der Urin wird ihr abgeleitet. Das ist eine kleine Meldung mit mehreren Folgen, und die erste ist die, die am leichtesten missverstanden wird.")]),
    P(&[B("Der Bericht beantwortet inzwischen die Kernfrage dieses Abschnitts."), T(" Er nennt eine "), B("Pigtail-Ableitung beidseits, eingelegt am 27. August"), T(" – also eine Ableitung, die oberhalb des Hindernisses ansetzt und nicht in der Blase. Die Frage des nächsten Absatzes, ob die obere Harnableitung eigens entlastet werden müsse, war zum Zeitpunkt der Meldung längst mit Ja beantwortet und erledigt. Der Absatz bleibt trotzdem stehen, weil die Unterscheidung stimmt und weil sie erklärt, warum diese Ableitung nötig war: Ein Blasenkatheter hätte hier nichts genützt.")]),
    P(&[B("Ein Blasenkatheter entleert die Blase – und sonst nichts."), T(" Das Hindernis, von dem der Abschnitt davor handelt, sitzt weiter oben, im Harnleiter. Dort ändert ein Katheter nichts. Die Fachdarstellung zur Harnstauung trennt das ausdrücklich: Der "), L("Blasenkatheter", "https://www.ncbi.nlm.nih.gov/books/NBK563217/"), T(" ist das Mittel, wenn die Abflussstörung auf Höhe der Blase vermutet wird; sitzt sie im Harnleiter, braucht es eine Harnleiterschiene über die Blasenspiegelung oder eine perkutane Nierenfistel durch die Haut. "), B("Praktisch heisst das:"), T(" Zeigen Kreatinin oder Ultraschall den Stau weiterhin, ist der Katheter nicht die Antwort darauf – dann steht die Frage im Raum, ob die obere Harnableitung eigens entlastet werden muss.")]),
    P(&[B("Wofür er hier wirklich gut ist: die Menge."), T(" Genau dafür steht er auf der Liste der begründeten Anwendungen – die "), L("genaue Messung der Urinmenge", "https://pmc.ncbi.nlm.nih.gov/articles/PMC9580547/"), T(" ist eine davon. Und diese Messung hat bisher gefehlt. Nach der KDIGO-Leitlinie zum akuten Nierenversagen genügt eine "), L("Urinmenge unter 0,5 ml je Kilogramm und Stunde über sechs Stunden", "https://kdigo.org/wp-content/uploads/2016/10/KDIGO-2012-AKI-Guideline-English.pdf"), T(" für sich allein, um ein akutes Nierenversagen zu benennen – ohne einen einzigen Kreatininwert. Bei einer Patientin mit laufender Kochsalzinfusion, Ödemen an den Füssen und einer Niere, die bei dieser Krankheit das gefährdete Organ ist, ist die Bilanz deshalb kein Nebenprotokoll, sondern der Messwert, an dem sich zeigt, ob die Zufuhr passt.")]),
    P(&[B("Und ein Nebeneffekt, der hier zufällt."), T(" Der Urin für Sediment und Protein-Kreatinin-Quotient liegt jetzt griffbereit. Der Abschnitt «Blut im Urin» verlangt beides seit Tagen; die Sammlung ist als Hinderungsgrund damit weg.")]),
    P(&[B("Was er kostet."), T(" Die Dauer der Katheterisierung ist der "), L("beherrschende Risikofaktor", "https://pmc.ncbi.nlm.nih.gov/articles/PMC9580547/"), T(" für eine katheterassoziierte Harnwegsinfektion; das tägliche Risiko einer Bakteriurie wird mit "), L("3 bis 7 Prozent je Kathetertag", "https://pmc.ncbi.nlm.nih.gov/articles/PMC8992741/"), T(" angegeben. Daraus folgt keine Ablehnung, sondern ein Datum: Der Katheter gehört heraus, sobald die Bilanz nicht mehr gebraucht wird – und wer ihn liegen lässt, sollte sagen können, wofür.")]),
    P(&[B("Ein Vorbehalt zum Blut im Urin gehört dazu."), T(" Ein Katheter kann selbst bluten. Eine Übersichtsarbeit über die nicht-infektiösen Katheterkomplikationen nennt für die "), L("kurzzeitige Katheterisierung sichtbares Blut im Urin bei 4,7 Prozent", "https://www.ncbi.nlm.nih.gov/books/NBK159201/"), T("; Ursachen sind Reibung an der Harnröhre, ein falscher Weg beim Einführen oder ein im Harnröhrenbereich aufgeblasener Ballon. Damit steht neben Niere und Harnleiter eine dritte Erklärung im Raum. Sie entkräftet die beiden anderen nicht – der Bericht hält fest, dass der Urin bei Eintritt unauffällig war und die Makrohämaturie erst im Verlauf kam –, aber sie verschiebt die Frage auf ein Datum: Die Ableitungen wurden am 27. August eingelegt. Kam der braune Urin davor oder danach? Der Bericht sagt es nicht, und es steht einiges daran. Und sie macht die Untersuchung, die dieses Blatt ohnehin verlangt, noch wichtiger: Akanthozyten und Erythrozytenzylinder im Sediment trennen die Blutung aus dem Nierenfilter von jeder Blutung weiter unten, den Katheter eingeschlossen.")]),

    H2("Die Nierenfunktion – und was daran hängt"),
    P(&[B("Die Patientin sagt, die Nieren funktionierten nicht richtig. Das deckt sich mit dem Bericht,"), T(" und zwar genauer, als eine Selbstauskunft es meist tut. Der Bericht führt die Niereninsuffizienz als "), B("erste"), T(" Diagnose auf, mit einer GFR von "), N("30 ml/min"), T(" und einem Kreatinin von "), N("171 µmol/l"), T(" bei Eintritt, und beurteilt sie als postrenal – durch das Abflusshindernis – und möglicherweise zusätzlich renal. Am 30. August bestand trotz Rehydrierung eine Oligurie.")]),
    P(&[B("Und an dieser Stelle bricht der Bericht ab – wörtlich."), T(" Der Satz lautet: «Trotz Rehydrierung am 30.8. Oligurie, Kreatinin…» und endet dort. "), B("Der wichtigste Wert des ganzen Berichts fehlt also"), T(" – ausgerechnet der, an dem sich alles entscheidet. Das ist keine Spitzfindigkeit: Die Ableitungen liegen seit dem 27. August. War der Stau die Ursache, "), B("muss"), T(" das Kreatinin seither fallen. Fällt es nicht, gibt es eine zweite Ursache in der Niere selbst – und dann ist man beim Abschnitt «Die Glomerulonephritis» und bei der Frage nach der Nierenbiopsie. Eine Zahl, zwei Wege.")]),
    P(&[B("Dazu die zweite Messgrösse, die seit dem Katheter verfügbar ist."), T(" Bei "), N("60 kg"), T(" liegt die Schwelle bei "), N("30 ml"), T(" Urin je Stunde; darunter über sechs Stunden ist es für sich allein ein akutes Nierenversagen. Zusammen sagen Kreatininverlauf und Urinmenge, ob die Entlastung gewirkt hat – ohne eine einzige zusätzliche Untersuchung.")]),
    P(&[B("Was daran hängt, ist aber mehr als eine Diagnose."), T(" Eine Nierenfunktion auf diesem Niveau ändert, welche Medikamente in welcher Dosis vertretbar sind – und in ihrem Fall betrifft das ausgerechnet die Mittel, bei denen dieses Blatt ohnehin nachfragt. Der nächste Absatz ist deshalb der wichtigste dieses Abschnitts.")]),

    H3("Morphin bei dieser Nierenfunktion"),
    P(&[T("Morphin wird in der Leber umgebaut, und einer der dabei entstehenden Stoffe – Morphin-6-Glukuronid – ist nach der Fachinformation "), B("stärker und länger wirksam als Morphin selbst"), T(". Ausgeschieden wird er über die Nieren. Dieselbe Fachinformation zieht daraus den Schluss und schreibt ihn aus: Bei Patienten mit Niereninsuffizienz wurde über "), L("hohe Konzentrationen von Morphin-6-Glukuronid berichtet, welche komatöse Zustände verursachen können", "https://ch.oddb.org/de/gcc/fachinfo/reg/56400/chapter/kinetic"), T(". Und im selben Kapitel steht der zweite Satz, der hier zutrifft: Bei älteren Patienten kann der Morphin-Abbau verlangsamt sein, mit höheren Spitzenkonzentrationen und längeren Halbwertszeiten.")]),
    P(&[B("Die Fachliteratur ist an diesem Punkt ungewöhnlich klar und, was seltener ist, auch praktisch."), T(" Eine Übersicht zu "), L("Opioiden bei Nierenversagen", "https://pubmed.ncbi.nlm.nih.gov/15504625/"), T(" empfiehlt, Morphin und Codein "), B("zu meiden"), T("; Hydromorphon und Oxycodon "), B("mit Vorsicht und engmaschiger Überwachung"), T(" einzusetzen; Methadon und Fentanyl gelten als sicher. "), B("Daraus folgt etwas Konstruktives statt einer blossen Warnung:"), T(" Von den beiden Opioiden auf ihrer Reserveliste ist Morphin dasjenige, das hier am wenigsten passt – und Oxycodon dasjenige, das bleiben kann, wenn es überwacht wird. Das ist keine Anordnung, sondern eine Frage, die sich mit einem Blick in die Verordnung beantworten lässt.")]),
    P(&[B("Und jetzt schliesst sich eine Kette, deren Glieder in diesem Blatt einzeln schon dastehen."), T(" Niereninsuffizienz lässt den wirksamen Morphin-Metaboliten anfluten. Höheres Alter verlangsamt den Abbau zusätzlich. Der laufende Sauerstoff verschlechtert die opioidbedingte Atemdepression, statt sie abzufangen – und lässt sie am Fingergerät nicht sichtbar werden. Und die metabolische Azidose mit einem Bikarbonat von "), N("17 mmol/l"), T(" macht sie auf eine gesteigerte Atemarbeit angewiesen, also auf genau das, was Opioide dämpfen. "), B("Jedes einzelne Glied ist belegt und für sich genommen beherrschbar."), T(" Zusammen ergeben sie die Frage, die dieses Blatt an die Station richtet: Welches Opioid wurde tatsächlich gegeben, wie oft, und wird die "), B("Atemfrequenz"), T(" gezählt?")]),

    H2("Aszites, Erguss und das Bauchfell"),
    P(&[T("Freie Flüssigkeit im Bauch, Erguss in beiden Brustfellhöhlen, eine fokale Verdickung des Bauchfells: Aus diesem Befund kommen die beiden neuen Verdachtsdiagnosen. Und er liefert zugleich die Erklärung, die dieses Blatt bei den Ödemen bisher nicht hatte – Wasser sammelt sich hier nicht nur in den Füssen, sondern in drei Räumen gleichzeitig. Das ist keine Bilanzfrage der Beine mehr, sondern eine des ganzen Körpers.")]),
    P(&[B("Eine einzige Zahl entscheidet die Richtung – und sie fehlt im Bericht."), T(" Aus der Bauchwasserpunktion ist das Albumin bekannt: unter "), N("10 g/l"), T(". Für sich allein sagt dieser Wert nichts. Aussagekräftig ist die "), B("Differenz"), T(" zum Albumin im Blut, der Serum-Aszites-Albumin-Gradient. Er sagt mit einer "), L("Treffsicherheit von 97 Prozent", "https://www.ncbi.nlm.nih.gov/books/NBK470482/"), T(" voraus, ob ein Pfortaderhochdruck vorliegt. Ein Gradient ab "), N("1,1 g/dl"), T(" spricht dafür – Leberzirrhose, Herzschwäche, Stauung. Ein Gradient darunter spricht dagegen, und in dieser Gruppe stehen "), B("Peritonealkarzinose"), T(", Bauchspeicheldrüsenentzündung, Serositis, nephrotisches Syndrom und Bauchfelltuberkulose.")]),
    P(&[B("Praktisch heisst das:"), T(" Ohne das Albumin im Blut vom selben Tag lässt sich aus «unter 10 g/l» nicht ableiten, ob die neuen Verdachtsdiagnosen gestützt oder entkräftet werden. Dieser Wert steht in diesem Blatt ohnehin schon zweimal auf der Liste – wegen der Ödeme und wegen des Kostaufbaus. Ein Wert, drei Fragen. Er dürfte längst gemessen sein; er steht bloss nicht im Bericht.")]),
    P(&[B("Die Zytologie war der wichtigste ausstehende Befund überhaupt – und sie ist inzwischen zurück; der Abschnitt «Die Zellen im Bauchwasser» geht sie durch. Warum so viel an ihr hing, steht hier."), T(" Bei einer Peritonealkarzinose ist die Zelluntersuchung des Bauchwassers ausserordentlich treffsicher: Sie ist die "), L("beste Einzeluntersuchung bei diesem Verdacht, und ihre Empfindlichkeit nähert sich 100 Prozent", "https://pubmed.ncbi.nlm.nih.gov/8189030/"), T(". "), B("Die Kehrseite gehört dazu, und sie ist wichtig:"), T(" Die Peritonealkarzinose ist nur einer von mehreren Wegen, auf denen ein Tumor Bauchwasser macht. Ein negativer Befund schliesst deshalb eine Bösartigkeit nicht aus – er schliesst die Absiedlung im Bauchfell aus. Dieselbe Arbeit warnt ausdrücklich vor den «humoralen Malignitätstests» wie Fibronektin oder Cholesterin im Bauchwasser: Sie fallen auch bei Bauchfelltuberkulose, Herzstauung und Bauchspeicheldrüsenentzündung positiv aus und lösen dann die Suche nach einem Tumor aus, den es nicht gibt.")]),
    P(&[B("Zur Zellzahl."), T(" 119 Zellen je Mikroliter – die Schwelle, ab der von einer spontanen bakteriellen Bauchfellentzündung ausgegangen wird, liegt bei "), N("250"), T(" neutrophilen Granulozyten je Mikroliter. Die genannte Zahl ist eine Gesamtzellzahl und liegt darunter. Für den Ausschluss zählt aber die Zahl der Neutrophilen, und die gehört im Befund genannt – zumal das CRP steigt und zwei Tage vor Eintritt auswärts ein Antibiotikum begonnen wurde, was einen solchen Befund verwischen kann.")]),
    P(&[B("Und was das für die drei Verdachtsdiagnosen heisst."), T(" Eine Serositis – eine Entzündung der Häute, die Bauch- und Brustraum auskleiden – steht in derselben Gruppe wie die Peritonealkarzinose und wäre der Weg, auf dem eine Autoimmunerkrankung dasselbe Bild macht. Der Erguss auf beiden Seiten passt dazu. Das ist keine Entwarnung, sondern die Feststellung, dass ein und derselbe Befund alle drei Zweige bedienen kann und deshalb allein nichts entscheidet. Entschieden wird es von der Zytologie, vom Albumin im Blut und von den Gewebeproben.")]),

    H2("Ein Liter von der linken Lunge"),
    P(&[B("Am 2. September ist links punktiert worden."), T(" Knapp ein Liter Flüssigkeit ist abgezogen worden, rechts sei es in Ordnung; seither schnauft sie besser und hat mehr Appetit. Dieser Abschnitt geht durch, was die Punktion bringt, woher so viel Wasser links kommt und was mit dem Liter geschehen sein muss, solange er im Labor liegt.")]),

    H3("Warum sie besser schnauft, und was das nicht heisst"),
    P(&[T("Dass eine Punktion die Atemnot lindert, ist häufig, aber nicht selbstverständlich – Flüssigkeit abzulassen hilft nicht jedem. In einer prospektiven Untersuchung an 103 punktierten Ergüssen erreichten "), L("83 Prozent eine deutliche Besserung", "https://pubmed.ncbi.nlm.nih.gov/40720164/"), T(" innerhalb einer Viertelstunde. Wer davon profitierte, liess sich vorher am Ultraschall ablesen: Ein flach stehendes oder umgestülptes Zwerchfell sagte die Besserung mit einer Odds Ratio von 7,9 voraus, und wer vor der Punktion stärker unter Atemnot litt, hatte danach mehr davon. Beides trifft auf sie zu – ein Bauch voll Wasser drückt das Zwerchfell von unten flach, und atemlos war sie.")]),
    P(&[B("Und jetzt der Satz, der dazugehört."), T(" Dieses Blatt nennt im nächsten Abschnitt vier Gründe für ihre Atemnot: den Erguss, die Blutarmut mit "), N("75 g/l"), T(", die Übersäuerung mit einem Bikarbonat von "), N("17 mmol/l"), T(" und die Opioide. "), B("Einer davon ist jetzt weg, drei sind unverändert."), T(" Dass sie besser schnauft, ist deshalb kein Grund, die Sättigung ohne Sauerstoff, die Atemfrequenz und die Blutgasanalyse als erledigt zu betrachten. Im Gegenteil: Zum ersten Mal lässt sich messen, was ohne den Erguss übrig bleibt – und das ist eine bessere Auskunft über die drei anderen Gründe, als sie vorher zu haben war.")]),
    P(&[B("Der Appetit ist die vierte Meldung in dieselbe Richtung."), T(" Der Wind, der Stuhlgang, gelb statt schwarz – und jetzt Luft und Appetit. Für den Kostaufbau heisst das nicht Entwarnung, sondern das Gegenteil: Die Regeln zum Wiederernähren greifen in dem Moment, in dem sie tatsächlich isst. Thiamin vor der ersten Kohlenhydratgabe, dazu Phosphat, Kalium und Magnesium in den ersten Tagen alle zwölf Stunden. Ein nach Wochen ohne Nahrung zurückkehrender Appetit ist genau die Lage, für die diese Regeln geschrieben sind – siehe den Abschnitt zum Kostaufbau und das Begleitblatt.")]),

    H3("Woher kommt so viel Wasser links?"),
    P(&[T("Die Frage hat zwei Hälften – warum überhaupt Flüssigkeit, und warum links. Die Antwort auf beide steckt in dem Liter, der eben abgezogen wurde, und braucht keinen weiteren Eingriff.")]),
    P(&[B("Die erste Hälfte ist die einfachere."), T(" Die führenden Ursachen eines Pleuraergusses bei Erwachsenen sind "), L("Herzschwäche, Infektion, Krebs und Lungenembolie", "https://pubmed.ncbi.nlm.nih.gov/37983698/"), T(". Dazu kommt bei ihr, was auf diesem Blatt ohnehin steht: zu wenig Eiweiss im Blut – im Bauchwasser lag das Albumin unter "), N("10 g/l"), T(" –, eine mögliche Entzündung oder Absiedlung an den Häuten, und acht Liter zusätzliches Wasser im Körper. Jeder dieser Wege füllt den Brustraum, und mehrere davon sind gleichzeitig offen.")]),
    P(&[B("Die Trennung, die alles entscheidet, heisst Transsudat oder Exsudat."), T(" Ein Transsudat entsteht durch Druck und Eiweissmangel: Der Körper presst Wasser heraus, die Häute selbst sind gesund. Ein Exsudat entsteht, weil die Häute selbst undicht sind – entzündet, infiziert oder von Tumorzellen besiedelt. Getrennt wird mit den Light-Kriterien aus Eiweiss und LDH in Flüssigkeit und Blut. Der Punkt ist, dass diese Werte aus der Punktion stammen, die schon gelaufen ist: Es braucht nur die Anforderung im Labor.")]),
    P(&[B("Eine Falle liegt dabei genau auf ihrem Weg."), T(" Unter einem Entwässerungsmittel wird ein Transsudat eingedickt und rutscht in den Exsudatbereich. In der Arbeit, die das gezeigt hat, ordneten die Light-Kriterien "), L("fünf Herzschwäche-Kranke fälschlich als Exsudat ein", "https://pubmed.ncbi.nlm.nih.gov/2152757/"), T(" – vier davon hatten zuvor entwässert bekommen. Der Serum-Erguss-Albumin-Gradient hat diesen Fehler nicht: über "), N("1,2 g/dl"), T(" Transsudat, darunter Exsudat. Das ist hier nicht theoretisch, denn eine der drei Möglichkeiten für den neuen Blutdrucksenker ist Torasemid, ein Entwässerungsmittel. Wer weiss, welches Mittel läuft, weiss, welches Kriterium gilt. Und wieder braucht es dafür nur das Albumin im Blut vom selben Tag – den Wert, der auf diesem Blatt inzwischen zum vierten Mal steht.")]),
    P(&[B("Die zweite Hälfte der Frage ist die aufschlussreichere: Warum links?"), T(" Denn die Seite sortiert die Ursachen.")]),
    Liste(&[
        &[B("Die systemischen Ursachen machen kleine Ergüsse auf beiden Seiten."), T(" Herzschwäche, Leberzirrhose, Nierenversagen und Eiweissmangel drücken das Wasser überall zugleich heraus. Dieselbe Übersichtsarbeit sagt es in einem Satz, der zugleich eine Handlungsanweisung ist: Kleine beidseitige Ergüsse bei entgleister Herzschwäche, Zirrhose oder Nierenversagen sind mit hoher Wahrscheinlichkeit Transsudate und brauchen "), L("keine diagnostische Punktion", "https://pubmed.ncbi.nlm.nih.gov/37983698/"), T(". Ein einseitiger Liter ist das Gegenteil davon – genau der Erguss, der punktiert gehört. Er wurde punktiert.")],
        &[B("Bauchwasser, das durch das Zwerchfell steigt, käme rechts."), T(" Diesen Weg gibt es: Aszites tritt durch feine Lücken im Zwerchfell in den Brustraum über. Nur ist dieser hepatische Hydrothorax "), L("ganz überwiegend rechtsseitig", "https://pubmed.ncbi.nlm.nih.gov/17645471/"), T(" und setzt eine Leberzirrhose mit Pfortaderhochdruck voraus, von der im Bericht nichts steht. Links passt nicht dazu.")],
        &[B("Eine Lungenembolie erklärt keinen Liter."), T(" Sie steht auf der Liste der vier führenden Ursachen, und bei einer 84-Jährigen, die seit Wochen liegt, gehört sie bedacht. Nur haben Embolien kleine Ergüsse: In einer Serie von 31 Ergüssen bei gesicherter Lungenembolie waren "), L("22 klein, acht mittelgross und einer gross", "https://pubmed.ncbi.nlm.nih.gov/23425873/"), T(", meist einseitig und stets Exsudate. Das schliesst eine Embolie nicht aus – es heisst, dass sie diese Menge nicht erklärt.")],
    ]),
    P(&[B("Übrig bleibt die Erklärung, die zum Rest des Falles passt:"), T(" ein Vorgang an den Häuten selbst – derselbe, der den Bauch füllt und die fokale Verdickung des Bauchfells gemacht hat. Serositis oder Absiedlung. Das ist keine neue Diagnose, sondern es sind dieselben drei Zweige, jetzt auch im Brustraum. Und es ist der Grund, warum die Untersuchung dieses Liters mehr wert ist als die Erleichterung, die er gebracht hat.")]),
    P(&[B("Ein Vorbehalt zu «rechts war in Ordnung»."), T(" Der Bericht vom 30. August nennt Erguss in "), B("beiden"), T(" Brustfellhöhlen. Drei Lesarten sind möglich: Der rechte hat sich zurückgebildet, er war zu klein zum Punktieren, oder er wurde nicht mit demselben Verfahren angesehen. Die drei bedeuten Verschiedenes – ein rückläufiger rechter Erguss neben einem wachsenden linken wäre ein deutliches Zeichen für einen örtlichen Vorgang links. Die Frage lautet deshalb: Was zeigte der Ultraschall rechts am selben Tag? Es ist die Regel, die auf diesem Blatt dreimal vorkommt: Ein unauffälliger Befund ist so viel wert wie die Bedingungen, unter denen er erhoben wurde.")]),

    H3("Was mit dem Liter geschehen sein muss"),
    P(&[T("Eine Pleurapunktion liefert Material, das sich nachträglich nicht beschaffen lässt. Die Standardanforderung lautet nach derselben Übersichtsarbeit: "), L("Gram-Färbung, Zellzahl mit Differenzierung, Kultur, Zytologie, Gesamteiweiss, LDH und pH", "https://pubmed.ncbi.nlm.nih.gov/37983698/"), T(". Dazu bei ihr das Albumin für den Gradienten. Der pH gehört ausdrücklich dazu: Ein Wert unter 7,2 spricht für einen komplizierten Erguss und hätte eigene Folgen für die Behandlung.")]),
    P(&[B("Der wichtigste Punkt ist die Zytologie – zum zweiten Mal an einem Tag."), T(" Aus dem Bauchwasser kam die Meldung «ein paar pathologische Zellen». Die Flüssigkeit aus der Brust wird nach "), B("demselben"), T(" fünfstufigen System beurteilt: Das internationale Meldesystem gilt für Ergussflüssigkeiten aller Höhlen, und die Risikozahlen im Abschnitt «Die Zellen im Bauchwasser» gelten hier ebenso. Damit liegen zum ersten Mal zwei unabhängige Proben aus zwei Körperhöhlen vor. Die Zelluntersuchung eines Exsudats erkannte in einer Serie von 600 Ergüssen "), L("70,1 Prozent", "https://pubmed.ncbi.nlm.nih.gov/30203633/"), T(" der Fälle richtig – eine einzelne Probe kann täuschen, zwei übereinstimmende deutlich weniger.")]),
    P(&[B("Und PAX8 lässt sich auch hier bestimmen."), T(" Der Abschnitt «Die Zellen im Bauchwasser» beschreibt die Färbung, die die Herkunftsfrage ohne neuen Eingriff beantworten kann. Sie braucht einen Zellblock, und ein Liter Flüssigkeit ist dafür reichlich Material. Das ist die zweite Gelegenheit für dieselbe Antwort, und sie besteht nur, solange die Flüssigkeit im Labor liegt – nicht mehr, wenn sie verworfen ist.")]),

    H3("Ein Liter ist die übliche Menge, nicht die Grenze"),
    P(&[T("Knapp ein Liter klingt nach viel und ist es nicht. In einer Auswertung von 1376 Punktionen lag die mittlere abgezogene Menge bei "), N("901 ml"), T("; ein Reexpansionsödem – die gefürchtete Komplikation, wenn eine zusammengedrückte Lunge zu rasch wieder aufgeht – trat bei "), L("sechs von 1376 Eingriffen auf, also in 0,7 Prozent", "https://pubmed.ncbi.nlm.nih.gov/38858252/"), T(", und zwischen Mengen über und unter "), N("1500 ml"), T(" bestand kein Unterschied. Praktisch heisst das: Läuft der Erguss nach, ist eine zweite Punktion kein aussergewöhnlicher Schritt, sondern der übliche.")]),
    P(&[B("Für die Waage dagegen ist der Liter ein Kilo von acht."), T(" Er nimmt Flüssigkeit weg, wo sie am meisten stört, aber er ändert nicht, warum sie sich bildet. Das tägliche Gewicht, die Ein- und Ausfuhrbilanz und das Albumin im Blut bleiben die drei Zahlen, an denen sich das entscheidet – siehe den Abschnitt «Das Gewicht: 66 statt 58 Kilo». Und wenn morgen ein Kilo weniger auf der Waage steht, ist das der abgezogene Liter und keine Besserung der Ödeme.")]),

    H2("Der Sauerstoff"),
    P(&[B("Die Meldung lautet, sie bekomme Sauerstoff wegen der Bauchschmerzen. Der erste Satz dazu ist der wichtigste:"), T(" "), B("Sauerstoff ist kein Schmerzmittel."), T(" Er lindert keinen Bauchschmerz und ist für Schmerzen auch nicht vorgesehen. Wenn er läuft, dann weil ein Messwert zu tief war – die Sauerstoffsättigung. Die Zuordnung zum Schmerz ist naheliegend, weil beides gleichzeitig da ist, aber sie führt in die Irre. "), B("Die Frage, die sich daraus ergibt, ist einfach und wird selten gestellt:"), T(" Wie hoch war die Sättigung, bevor der Sauerstoff angehängt wurde, und wie hoch ist sie ohne ihn?")]),
    P(&[T("Denn dafür stehen in diesem Blatt bereits vier Gründe, und sie schliessen einander nicht aus.")]),
    Liste(&[
        &[B("Der Erguss und das Bauchwasser."), T(" Flüssigkeit in beiden Brustfellhöhlen und ein Bauch, der von unten gegen das Zwerchfell drückt: Die Lunge kann sich nicht mehr voll entfalten. Bei einseitigem Erguss ist die "), L("eingeschränkte Lungenentfaltung der wichtigste Vorhersagewert für die Atemnot", "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6169850/"), T(", und grössere Ergussmengen gehen mit schlechterer Zwerchfellbewegung einher. Das ist zugleich der einzige der vier Gründe, der sich behandeln lässt statt anzuzeigen – durch eine Punktion, die hier ohnehin zur Diagnostik anstand. "), B("Am 2. September ist sie gemacht worden:"), T(" links knapp ein Liter, und sie schnauft besser. Die drei übrigen Gründe sind unverändert; siehe den Abschnitt «Ein Liter von der linken Lunge».")],
        &[B("Die Blutarmut."), T(" Bei "), N("75 g/l"), T(" trägt jeder Liter Blut weniger Sauerstoff. Zusätzlicher Sauerstoff in der Atemluft ändert daran wenig, weil das Problem nicht die Aufnahme ist, sondern das Transportmittel.")],
        &[B("Die Übersäuerung."), T(" Der Bericht nennt eine metabolische Azidose mit einem Bikarbonat von "), N("17 mmol/l"), T(". Dagegen wehrt sich der Körper, indem er "), B("mehr atmet"), T(" und Kohlendioxid abbläst. Wie viel mehr, lässt sich ausrechnen: Nach der "), L("Winter-Formel", "https://www.ncbi.nlm.nih.gov/books/NBK482146/"), T(" – erwarteter pCO₂ gleich Bikarbonat mal 1,5 plus 8 – müsste ihr Kohlendioxid bei rund "), N("33 mmHg"), T(" liegen, also deutlich unter dem Normalwert. Sie ist also auf eine erhöhte Atemarbeit angewiesen. "), B("Alles, was diese Atemarbeit dämpft, nimmt ihr die Kompensation."), T(" Das führt direkt zum vierten Punkt.")],
        &[B("Die Opioide."), T(" Auf der Reserveliste stehen zwei. Opioide senken den Atemantrieb – das ist ihre bekannteste gefährliche Wirkung und der Grund, warum Oxycodon Atemdepression unter den unerwünschten Wirkungen führt.")],
    ]),
    P(&[B("Und jetzt der Punkt, an dem Sauerstoff und Opioide zusammen etwas anderes sind als jedes für sich."), T(" Man gibt Sauerstoff, damit die Sättigung nicht abfällt. Nur "), B("verschlechtert zusätzlicher Sauerstoff die opioidbedingte Atemdepression, statt sie abzufangen"), T(" – weil hohe Sauerstoffwerte die Fühler dämpfen, die die Atmung antreiben. In einem Versuch an 20 gesunden Freiwilligen sank die Atemleistung unter demselben Opioid bei Raumluft von 7,4 auf 2,2 Liter je Minute, bei "), L("50 Prozent Sauerstoff dagegen auf 1,2", "https://pubmed.ncbi.nlm.nih.gov/23293275/"), T("; die Atemfrequenz fiel auf 3,6 statt 6,1 pro Minute, und ein Atemstillstand trat bei "), B("10 von 20 unter Sauerstoff auf gegenüber 2 von 20 bei Raumluft"), T(".")]),
    P(&[B("Der zweite Teil desselben Befundes ist der beunruhigendere."), T(" Während all dem blieb die gemessene Sättigung unter Sauerstoff bei 98,7 Prozent – sie fiel praktisch nicht. "), B("Das Gerät am Finger zeigte nichts an."), T(" Dasselbe an echten Patienten auf Station: In einer Auswertung von 202 überwachten Fällen war unter zusätzlichem Sauerstoff die Häufigkeit "), L("opioidbedingter Atemzwischenfälle 2,7-fach erhöht", "https://pubmed.ncbi.nlm.nih.gov/37794334/"), T(", die von Atemstillständen 2,8-fach, die von zu langsamer Atmung 3,0-fach – "), B("und die Entsättigungen unterschieden sich zwischen beiden Gruppen nicht"), T(". Genau darin liegt die Falle: Unter Sauerstoff ist die Sättigung am Finger kein Frühwarnzeichen mehr für eine zu flache Atmung, sondern ein spätes.")]),
    Klein(&[T("Die Einschränkung gehört dazu, und sie ist erheblich: Der Versuch lief an gesunden Freiwilligen mit einem sehr kurz wirksamen, starken Opioid als Einmalgabe und mit 50 Prozent Sauerstoff. Das ist nicht dasselbe wie eine Nasenbrille und ein Reservemittel bei einer 84-jährigen Patientin. Übertragbar ist nicht die Zahl, sondern der Mechanismus – und die Beobachtung an den 202 Stationspatienten stützt ihn unter Alltagsbedingungen.")]),
    P(&[B("Daraus folgen vier Fragen, und keine davon kostet etwas."), T(" Wie hoch ist die Sättigung ohne Sauerstoff? Ist eine Blutgasanalyse gemacht worden – sie beantwortet die Azidose und das Kohlendioxid in einem, und sie sagt, ob die Kompensation noch trägt? Wird die "), B("Atemfrequenz"), T(" gezählt und nicht nur die Sättigung abgelesen? Und: Ist der Pleuraerguss punktiert worden? "), B("Die vierte Frage hat sich am 2. September mit Ja beantwortet"), T(" – die drei ersten stehen unverändert, und sie sind jetzt sogar besser zu beurteilen als vorher: Was an Atemnot nach der Punktion übrig bleibt, kommt von der Blutarmut, der Übersäuerung oder den Opioiden, und nicht mehr vom Erguss.")]),

    H2("Das Gewicht: 66 statt 58 Kilo"),
    P(&[T("Sie wiegt zurzeit "), N("66 kg"), T(". Ihr gewohntes Gewicht liegt bei "), N("58 kg"), T(", der Bericht nennt bei Eintritt "), N("60 kg"), T(". Das ist die aufschlussreichste einzelne Zahl, die seit dem Bericht dazugekommen ist, und sie sagt drei verschiedene Dinge.")]),
    P(&[B("Erstens: Das sind acht Liter Wasser."), T(" Körpergewicht, das sich innerhalb von Tagen ändert, ist Wasser und nicht Gewebe – ein Kilogramm ist ein Liter. Gegenüber ihrem gewohnten Gewicht sind es acht, gegenüber dem Eintritt vor zehn Tagen sechs. Und dieses Wasser liegt nicht nur in den Beinen, sondern zugleich im Bauch, in beiden Brustfellhöhlen und im Zwischengewebe – genau die Räume, die der Bericht beschreibt. Die geschwollenen Beine sind damit nicht der Befund, sondern der sichtbare Teil eines grösseren.")]),
    P(&[B("Zweitens: Die Zahl verdeckt, was sie zeigen sollte."), T(" Sie hat wochenlang fast nichts gegessen und wiegt heute mehr als sonst. Wer nur die Waage liest, hält sie für gut ernährt. Aus "), N("66 kg"), T(" bei "), N("165 cm"), T(" rechnet sich ein Körpermassenindex von 24,2, aus "), N("58 kg"), T(" einer von 21,3 – und der Ernährungszustand hängt am zweiten. Für den Kostaufbau heisst das: Die Anfangsmenge rechnet auf das Trockengewicht, also auf "), N("580 bis 1160 kcal"), T(" und nicht auf die 600 bis 1200 aus dem Eintrittsgewicht. Der Unterschied in der Zahl ist klein, der Grundsatz ist es nicht – wer Dosierungen auf ein Ödemgewicht rechnet, rechnet auf Wasser.")]),
    P(&[B("Drittens: Die Waage beantwortet die Bilanzfrage, bevor irgendein Blatt gelesen wird."), T(" Dieses Blatt verlangt seit der Einlage des Katheters die Ein- und Ausfuhrbilanz. Eine Zunahme ist eine positive Bilanz, unabhängig davon, wie sorgfältig gebucht wurde. Genau das ist untersucht: In einer Beobachtungsstudie an 58 Kranken mit akutem Nierenversagen auf der Normalstation wurde die Volumenüberladung einmal aus der Bilanz und einmal aus dem Gewicht berechnet. Die beiden "), L("stimmten nicht überein", "https://pubmed.ncbi.nlm.nih.gov/32403114/"), T(" – und nur die aus dem Gewicht berechnete war mit einem schwereren Nierenversagen und mit der Dialysepflicht verbunden. Einschränkung: eine Pilotstudie mit 58 Kranken und drei Todesfällen; die Zahlen tragen nicht weit. Was trägt, ist der Schluss – täglich wiegen, und der Waage mehr glauben als der Bilanz.")]),

    H3("Und was das für die Infusion heisst"),
    P(&[T("Sie bekommt weiterhin "), N("1500 ml"), T(" Kochsalzlösung am Tag. Die Fachinformation nennt für die isotone Lösung "), L("9,0 g Natriumchlorid je Liter, Natrium und Chlorid je 154 mmol/l", "https://ch.oddb.org/de/gcc/fachinfo/reg/41181/chapter/galenic_form"), T(". Auf 1500 ml sind das 231 mmol Natrium und ebenso viel Chlorid am Tag, gut 13 Gramm Kochsalz – und beim Chlorid liegt die Lösung deutlich über dem, was im Blut steht.")]),
    P(&[B("Der naheliegende Schluss daraus ist geprüft worden, und er hält nicht."), T(" Man könnte meinen, bei einer metabolischen Azidose mit einem Bikarbonat von "), N("17 mmol/l"), T(" gehöre eine gepufferte Lösung statt Kochsalz. Genau diese Frage hat eine Nachauswertung an 2029 Kranken mit einem Bikarbonat unter 20 gestellt: Zwischen gepufferter Lösung und Kochsalz fand sich "), L("kein Unterschied", "https://pubmed.ncbi.nlm.nih.gov/33500146/"), T(" bei den schweren Nierenereignissen – 21,8 gegenüber 21,3 Prozent. Der Ausgangswert des Bikarbonats taugt also nicht zur Wahl der Lösung. Das steht hier, weil es der bequemere Einwand gewesen wäre.")]),
    P(&[B("Die Frage ist deshalb nicht die Art der Lösung, sondern die Menge."), T(" Als in diesem Blatt stand «Die Infusion ist deshalb richtig», war das richtig: Sie war ausgetrocknet, trank nicht und erbrach alles. Inzwischen trinkt sie – und zwar zuverlässig –, isst wieder etwas und hat sechs Kilo zugelegt. Aus derselben Massnahme ist damit eine andere Rechnung geworden. Ob und wie viel Volumen jetzt noch gebraucht wird, entscheidet die Station; die Zahlen, an denen sie es entscheidet, sind das tägliche Gewicht, die Bilanz und das Albumin im Blut. Alle drei stehen ohnehin schon auf der Liste dieses Blattes – die Waage macht aus ihnen eine Frage mit einer Zahl davor.")]),

    P(&[B("Nachtrag vom 5. September: Am Ständer hängt Ringer-Acetat, nicht mehr Kochsalz."), T(" Damit ist die Rechnung oben zum Chlorid überholt – 110 statt 154 mmol je Liter, und Acetat als Puffer. Was bleibt, ist die Menge, und dazu kommt ein Satz aus der Fachinformation der neuen Lösung, der hier schwerer wiegt als jede Salzrechnung: Unter den Gegenanzeigen steht an erster Stelle "), L("«Ödeme»", "https://ch.oddb.org/de/gcc/fachinfo/reg/58605/chapter/contra_indications"), T(". Der Abschnitt «Der Stand vom 5. September» bei der Medikamentenliste führt das aus.")]),

    H2("Ödeme an den Füssen"),
    P(&[T("Wasser im Gewebe ist kein Befund für sich, sondern eine Bilanz: Was die Gefässe nicht halten oder die Niere nicht ausscheidet, sammelt sich unten. Vier Erklärungen kommen infrage, und mehrere davon können gleichzeitig zutreffen. Eine fünfte, die hier zuerst stand, entfällt: Kortison hält Natrium und Wasser zurück – nur hat sie nie welches bekommen. Die Frage lautet deshalb nicht, welche es ist, sondern wie viel von welcher.")]),
    Liste(&[
        &[B("Eiweissverlust über die Niere."), T(" Daran hängt am meisten. In einer prospektiven Untersuchung an 49 Erwachsenen mit IgA-Vaskulitis-Nephritis hatten 69,3 Prozent eine Eiweissausscheidung, 16,3 Prozent eine im nephrotischen Bereich – und diese war der einzige "), L("unabhängige Risikofaktor für einen ungünstigen Verlauf", "https://pmc.ncbi.nlm.nih.gov/articles/PMC12025811/"), T(". Eine Remission erreichten mit ihr 9,7 Prozent, ohne sie 60. Damit ist der Protein-Kreatinin-Quotient nicht mehr eine Untersuchung unter vielen, sondern die, an der die Prognose hängt.")],
        &[B("Zu wenig Eiweiss von vorn."), T(" Wochen ohne Nahrung senken das Albumin ebenso, ganz ohne Niere. Die japanische Registerarbeit nennt neben dem Alter über 65 ausdrücklich die "), L("Hypoalbuminämie", "https://pmc.ncbi.nlm.nih.gov/articles/PMC5940189/"), T(" als das, worauf zu achten ist. Beide Wege führen zum selben tiefen Albuminwert; auseinanderhalten lassen sie sich nur, wenn man den Urin dazu misst.")],
        &[B("Die Infusion und die Transfusion."), T(" Wer NaCl und Blut bekommt, bekommt Volumen. Das ist genau die Konstellation, in der die Kreislaufüberlastung droht – siehe den Abschnitt «Die Transfusion». Geschwollene Füsse sind hier also nicht nur ein Befund, sondern auch eine Rückmeldung zur Bilanz.")],
        &[B("Das Herz."), T(" Bei 84 Jahren, unter Blutarmut und Volumenzufuhr, gehört die Frage gestellt. Sie steht hier nicht als Vermutung, sondern als offene Frage an die Ärztin.")],
        &[B("Die Bauchwassersucht selbst."), T(" Der neue Befund ordnet die Ödeme neu ein: Es sammelt sich nicht nur in den Füssen, sondern zugleich im Bauch und in beiden Brustfellhöhlen. Drei Räume auf einmal sprechen für eine gemeinsame Ursache – zu wenig Eiweiss im Blut, eine Entzündung der Häute oder eine Absiedlung im Bauchfell – und gegen die harmlosen Erklärungen weiter unten. Siehe den Abschnitt «Aszites, Erguss und das Bauchfell».")],
        &[B("Das Opioid."), T(" Die Fachinformation von Oxycodon führt "), L("Ödeme und periphere Ödeme", "https://ch.oddb.org/de/gcc/fachinfo/reg/55352/chapter/unwanted_effects"), T(" unter den gelegentlichen unerwünschten Wirkungen auf. Ob und wie oft das Reservemittel gegeben wurde, sagt das Verabreichungsprotokoll.")],
        &[B("Der Kostaufbau selbst."), T(" Diese Erklärung ist neu und die unauffälligste von allen. Kohlenhydrate senken die Natriumausscheidung der Niere – wer nach langem Hungern wieder zu essen beginnt, hält Wasser zurück. In der Berner Auswertung von 37 Hungerstreik-Aufnahmen war bei dem einen Fall mit einem mittelschweren Refeeding-Syndrom das "), B("beidseitige Knöchelödem"), T(" die "), L("klinische Manifestation", "https://pubmed.ncbi.nlm.nih.gov/25280415/"), T(" – es war nicht ein Nebenbefund, sondern das, woran man es sah. Sie hat vor wenigen Tagen wieder zu essen begonnen, mit gezuckertem Tee und Haferbrei. Das macht das Wasser in den Füssen nicht harmlos; es macht Phosphat, Kalium und Magnesium zu Werten, die man deswegen anschaut. Ausgeführt ist das im Begleitblatt «Kostaufbau nach langem Hungern».")],
    ]),
    P(&[B("Und eine Frage vom Küchentisch, die eine klare Antwort hat:"), T(" Gegen dieses Wasser helfen Flohsamenschalen nicht. Sie binden Wasser im "), B("Darminneren"), T(" – anatomisch also ausserhalb des Körpers –, während das Ödemwasser im Gewebe zwischen den Zellen liegt. Dazwischen stehen die Gefässe, und der einzige Weg von dort nach draussen führt über die Niere. Beides erreicht Flohsamen nie, und die Fachinformation sagt das so deutlich, wie es sich sagen lässt: Unter Pharmakokinetik stehen "), L("Absorption, Verteilung, Abbau und Ausscheidung allesamt als «nicht zutreffend»", "https://ch.oddb.org/de/gcc/fachinfo/reg/42933/chapter/kinetic"), T(" – der Stoff tritt gar nicht erst in den Körper ein. Auch die Richtung stimmt nicht: Das gebundene Wasser wandert in den Stuhl, und es stammt aus dem Kreislauf, nicht aus den Beinen; bei tiefem Albumin ist weniger Volumen im Gefäss eher das Gegenteil von hilfreich. Die Werkzeuge für dieses Wasser stehen in der Liste oben – die Zufuhr, das Albumin, die Niere und, falls ein Entwässerungsmittel dazukommt, Torasemid.")]),
    P(&[B("Und jetzt der wichtigste Satz dieses Abschnitts: Ödeme gab es schon beim letzten Mal."), T(" Das ist keine Nebenbeobachtung, sondern ein rückwirkender Befund. Der Abschnitt «Ein früherer Schub» stellt die Frage, ob damals der Urin kontrolliert wurde, und nennt sie die Frage, auf die es ankommt. Ödeme beim damaligen Schub verschieben die wahrscheinliche Antwort: Sie sind das, was man sieht, wenn Eiweiss über die Niere verloren geht, und sie machen es wahrscheinlicher, dass die Niere schon damals beteiligt war. Dann wäre das heutige Bild nicht der Beginn einer Nierenbeteiligung, sondern ihre zweite Runde – und das ändert, wie dringlich eine Nierenbiopsie zu beurteilen ist.")]),
    P(&[T("Beweisen lässt sich das rückwirkend nicht, erfragen schon: Gibt es aus der damaligen Zeit Urinbefunde, Blutdruckwerte, einen Albuminwert? Und wenn nicht, ist das selbst die Antwort auf die Frage, warum heute niemand weiss, wie lange die Niere schon leidet.")]),
    P(&[B("Zwei Vorbehalte gehören dazu."), T(" Geschwollene Füsse sind bei 84-Jährigen häufig und oft harmlos; langes Sitzen und Bewegungsmangel genügen. Und die Zuordnung «damals nach dem Stress» ist genau die, vor der der Abschnitt «Zum Stress als Auslöser» warnt – erinnert wird der Stress, ausgelöst hat nach der Datenlage eher ein Infekt. Der "), B("Zeitpunkt"), T(" der Ödeme bleibt trotzdem verwertbar, auch wenn die Ursachenzuschreibung es nicht ist. Es geht nicht darum, was den Schub ausgelöst hat, sondern darum, dass damals schon Wasser in den Beinen stand.")]),

    H2("Den Hämoglobinwert richtig lesen"),
    P(&[
        T("Ein verbreitetes Missverständnis, das hier viel ändert: "), B("Tief ist nicht besser, tief ist schlechter."), T(" Hämoglobin ist der rote Blutfarbstoff, der den Sauerstoff transportiert. Weniger Hämoglobin heisst weniger Sauerstoff in den Geweben. Der Normbereich für Frauen liegt bei rund "), N("117–157 g/l"), T("; 98 war bereits zu tief, 75 ist eine deutliche Blutarmut."),
    ]),
    P(&[
        T("Die Verwechslung dahinter ist naheliegend: "), B("HbA1c"), T(", das «Hämoglobin A1c», ist der Langzeit-Blutzuckerwert – und dort ist tief tatsächlich besser. Dieser Wert misst aber, wie stark der Zucker das Hämoglobin verzuckert hat, nicht wie viel Hämoglobin überhaupt vorhanden ist. Gleicher Wortstamm, gegenläufige Richtung."),
    ]),
    P(&[T("Die einzige Lage, in der ein Arzt Hämoglobin absichtlich senkt, ist die Polyzythämie: zu viel davon, das Blut wird zu dickflüssig, dann Aderlass. Das ist das Gegenteil dieser Situation.")]),
    P(&[
        B("Und was die Infusion mit dem Wert macht."), T(" Hämoglobin ist eine Konzentration, kein Vorrat: gemessen wird, wie viel Farbstoff in einem Liter Blut steckt. Wer seit Wochen kaum trinkt, hat weniger Flüssigkeit im Kreislauf – das dickt das Blut ein und lässt den Wert "), B("besser"), T(" aussehen, als er ist. Läuft dann die Infusion, verdünnt sich das Blut wieder, und das Hämoglobin kann weiter fallen, ohne dass ein einziger Tropfen zusätzlich verloren gegangen wäre. Dasselbe gilt umgekehrt für eine frische Blutung: Der Wert sinkt erst mit der Verdünnung, über 24 bis 72 Stunden. Zwei praktische Folgen: "), N("75 g/l"), T(", bei einer ausgetrockneten Patientin gemessen, ist eher zu günstig als zu schlecht – und ein Abfall unter laufender Infusion ist zuerst Verdünnung und nicht automatisch eine neue Blutung. Auseinanderhalten lässt sich das nur im Verlauf, zusammen mit Puls, Blutdruck und dem Aussehen des Stuhls."),
    ]),
    P(&[
        T("Was "), N("75 g/l"), T(" mit 84 Jahren praktisch bedeutet: Der Körper gleicht den Mangel mit höherem Puls und schnellerer Atmung aus. Daraus werden Müdigkeit, Schwindel beim Aufstehen, Kurzatmigkeit und Sturzgefahr – bei vorgeschädigtem Herz auch Angina pectoris. Genau darum liegt die Transfusionsgrenze bei bekannter Herzkrankheit bei 80 statt "), N("70 g/l"), T("."),
    ]),

    H2("Die Transfusion"),
    P(&[T("Das Hämoglobin ist nochmals gefallen, und jetzt läuft ein Erythrozytenkonzentrat. Damit ist zuerst einmal das Richtige geschehen: Wenn der Wert weiter fällt und die Patientin darunter leidet, ersetzt man, was fehlt. Nur ersetzt eine Transfusion genau das und nichts weiter. Sie erklärt den Verlust nicht und sie stellt ihn nicht ab. Der Satz, der in diesem Blatt an mehreren Stellen steht, gilt danach unverändert: Die Blutungsquelle ist nicht gefunden.")]),
    P(&[B("Der erneute Abfall ist dabei selbst ein Befund, und zwar der wichtigste."), T(" Weiter oben steht, dass ein sinkendes Hämoglobin unter laufender Infusion zuerst Verdünnung ist und nicht automatisch eine neue Blutung. Dieses Argument trägt nicht mehr. Die Infusion läuft seit Tagen, die Austrocknung ist ausgeglichen, und der Wert fällt trotzdem weiter – so weit, dass transfundiert werden muss. Das ist kein Rechenfehler der Verdünnung mehr, sondern ein Verlust, der anhält. Und ein Verlust, der anhält, hat eine Quelle, die noch offen ist.")]),
    P(&[B("Die nützlichste Zahl kommt erst nach der Konserve."), T(" Als Faustregel hebt eine Einheit das Hämoglobin um rund "), N("10 g/l"), T(" – genauer: "), L("4 ml Spendererythrozyten je Kilogramm", "https://www.nss.nhs.scot/media/6134/single_unit_transfusion_guidance-draft_14final-pdf.pdf"), T(" Körpergewicht, und die Näherung gilt für einen nicht blutenden Erwachsenen von 70 bis 80 kg. Daraus wird der wichtigste Messwert der nächsten Tage: das Hämoglobin am Tag nach der Transfusion. Steigt es um deutlich weniger als erwartet oder fällt es wieder, dann blutet es weiter – und das ist ein härterer Beweis als jede Stuhlfarbe und jede Vermutung.")]),
    P(&[B("Warum Einheit für Einheit."), T(" Die Empfehlung lautet, eine Einheit zu geben und danach neu zu beurteilen: Sind die Beschwerden besser? Gibt es Zeichen einer Reaktion? Wie steht der neue Wert? Das "), L("vermeidet unnötige Transfusionen", "https://hospital.blood.co.uk/patient-services/patient-blood-management/appropriate-use-of-blood-components/single-unit-blood-transfusions/"), T(" und senkt das Risiko der Kreislaufüberlastung. Ein Vorbehalt gehört dazu, und er trifft womöglich genau hier zu: Die Regel gilt ausdrücklich nicht für Patienten mit einer klinisch bedeutsamen aktiven Blutung. Ob das auf sie zutrifft, ist die offene Frage dieses ganzen Blattes.")]),
    P(&[B("Die Kreislaufüberlastung ist bei ihr keine Formalie."), T(" TACO heisst diese Komplikation, und sie ist "), L("die häufigste Todesursache", "https://www.lifeblood.com.au/health-professionals/clinical-practice/adverse-events/TACO"), T(" unter den transfusionsbedingten Zwischenfällen, die der amerikanischen Arzneimittelbehörde gemeldet werden – 62 der gemeldeten Todesfälle zwischen 2016 und 2020, also 34 Prozent; im britischen Meldesystem waren es zwischen 2010 und 2024 157 Todesfälle oder 41,4 Prozent. Besonders anfällig sind Menschen über 60, und eine förmliche Risikoeinschätzung vor der Transfusion wird namentlich für über 70-Jährige verlangt. Dazu kommen als Risikofaktoren eine Herz- oder Nierenerkrankung und eine ausgeprägte Blutarmut. Praktisch heisst das: langsam transfundieren, an ein Entwässerungsmittel denken – und die Flüssigkeitsbilanz mitrechnen, denn die NaCl-Infusion läuft ja daneben weiter.")]),
    P(&[B("Und warum zurückhaltend nicht sparsam heisst."), T(" Bei akuter Blutung im oberen Verdauungstrakt ist weniger Blut das bessere Ergebnis. In einer Studie an 921 Patienten wurde die Hälfte erst ab "), N("70 g/l"), T(" transfundiert, die andere schon ab "), N("90 g/l"), T(". Die "), L("Überlebenswahrscheinlichkeit nach sechs Wochen", "https://pubmed.ncbi.nlm.nih.gov/23281973/"), T(" lag in der zurückhaltenden Gruppe bei 95 gegenüber 91 Prozent, Nachblutungen traten bei 10 statt 16 Prozent auf, unerwünschte Ereignisse bei 40 statt 48 Prozent. Die internationalen Empfehlungen von 2023 ziehen daraus die Linie, die auch am Anfang dieses Blattes steht: "), L("70 g/l bei stabilen Erwachsenen", "https://pubmed.ncbi.nlm.nih.gov/37824153/"), T(", 80 bei vorbestehender Herz-Kreislauf-Erkrankung. Wer bei 75 transfundiert, tut das also nicht wegen der Zahl, sondern wegen der Patientin – und das ist zulässig, sofern es so begründet wird.")]),
    P(&[B("Der Bericht nennt dazu jetzt Zahlen."), T(" Zwei Erythrozytenkonzentrate am 29. August, bei einem Hämoglobin, das von "), N("98 g/l"), T(" bei Eintritt auf "), N("75 g/l"), T(" gefallen war – und zwar, wie der Bericht es formuliert, "), B("bei Makrohämaturie"), T(". Das ist eine Zuordnung, die dieses Blatt so nicht getroffen hat: Der behandelnde Arzt sieht den Blutverlust in erster Linie im Urin und nicht im Darm. Sichtbares Blut im Urin kann eine Blutarmut tatsächlich erklären, wenn es lange genug anhält. Nur schliesst das die zweite Quelle nicht aus – Teerstuhl und Kaffeesatz sind gesehen worden und stehen unverändert im Raum. Zwei Wege, an denen Blut verlorengeht, schliessen einander nicht aus; sie addieren sich.")]),
    P(&[B("Und der Stand von heute: Die Transfusion ist beendet, die Kochsalzinfusion läuft weiter."), T(" Damit wird der Messwert fällig, den dieser Abschnitt oben den nützlichsten der nächsten Tage nennt – das Hämoglobin nach der Konserve. Er beantwortet die Frage, die dieses ganze Blatt trägt, so direkt wie sonst nichts: Hält der Wert, ist die Blutung zum Stillstand gekommen; steigt er deutlich weniger als die rund "), N("10 g/l"), T(" je Einheit oder fällt er wieder, blutet es weiter. Dass nicht weiter transfundiert wird, ist dabei selbst eine Auskunft – jemand hat den erreichten Wert für ausreichend gehalten. Welcher Wert das war, ist die Frage dazu, und sie ist noch offen. Die Infusion bleibt derweil in der Bilanz: Sie bringt Volumen ohne Sauerstoffträger, und seit der Katheter liegt, lässt sich zum ersten Mal messen, was davon wieder herauskommt.")]),

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
    P(&[B("Er hat inzwischen ein Datum."), T(" Der Zwischenbericht nennt für die frühere Beurteilung das Jahr "), B("2016"), T(": eine dermatologische Konsultation mit dem Verdacht auf eine Vaskulitis bei distaler Purpura, also vor rund zehn Jahren. Damit ist aus «es gab schon einmal etwas» eine datierte Vorgeschichte geworden – und die Frage des nächsten Abschnitts, ob damals der Urin kontrolliert wurde, hat eine Adresse, an der sie sich beantworten lässt.")]),
    P(&[T("Tastbare Purpura an den Unterschenkeln ist die Signatur dieser Krankheit. Ist das schon einmal aufgetreten, ist das heutige Bild wahrscheinlich ein "), B("Rezidiv"), T(" – und das ändert die Einordnung.")]),
    P(&[T("Bei Erwachsenen kehrt die IgA-Vaskulitis in etwa "), B("43 Prozent"), T(" der Fälle wieder, gut ein Viertel davon mehrfach; nur rund die Hälfte erreicht eine vollständige Remission. Wiederkehren ist bei Erwachsenen die Regel und nicht die Ausnahme, und höheres Alter beim Erstauftreten ist selbst ein Vorhersagefaktor für Rückfälle. Die Tumorfrage bleibt dabei auf dem Tisch: Die Suche wird ausdrücklich bei "), B("neu aufgetretener oder wiederkehrender"), T(" IgA-Vaskulitis im Alter empfohlen – ein Rezidiv entlastet nicht.")]),
    H3("Die Frage, auf die es ankommt"),
    P(&[T("Und der damalige Schub blieb unbehandelt – Kortison hat sie nie bekommen. Was immer damals an der Niere geschah, geschah ohne Behandlung.")]),
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
        &[B("Medikamente."), T(" Schmerzmittel vom NSAR-Typ (Ibuprofen, Diclofenac, Naproxen) erhöhen das Geschwürrisiko erheblich und belasten zusätzlich die Niere. Blutverdünner verstärken jede vorhandene Blutungsquelle. Die vollständige Medikamentenliste gehört auf den Tisch, rezeptfreie Mittel eingeschlossen. Kortison steht hier nur als Möglichkeit, nicht als Tatsache: Sie hat nie eines bekommen.")],
        &[B("Eine zweite, unabhängige Quelle."), T(" Mit 84 Jahren ist ein Dickdarmtumor oder ein Magengeschwür statistisch häufiger als die Vaskulitis selbst. Der Dickdarmtumor ist inzwischen erledigt: Die Darmspiegelung war unauffällig. Das Magengeschwür bleibt, und mit ihm die Frage nach dem, was die Spiegelung im Magen tatsächlich gesehen und biopsiert hat.")],
        &[B("Der Dünndarm dazwischen."), T(" Die Strecke, die weder die Magen- noch die Darmspiegelung erreicht. Sobald beide unauffällig sind und es weiter blutet, ist sie die verbleibende – und für sie gibt es einen eigenen Weg. Siehe den Abschnitt «Die Darmspiegelung: nichts gefunden, ausser beim Transport».")],
        &[B("Die Niere."), T(" Bei nachlassender Nierenfunktion bildet der Körper weniger Erythropoetin, das Hormon für die Blutbildung. Das erklärt eine langsam sinkende Kurve, keinen Sturz um "), N("23 g/l"), T(" in kurzer Zeit.")],
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

    H3("Jetzt grün und flüssig – zwei Farben, zwei Herkünfte"),
    P(&[T("Die neue Beschreibung lautet "), B("grün und schwarz, flüssig"), T(". Das ist keine Korrektur der bisherigen, sondern eine Ergänzung, und sie trennt sich sauber: "), B("Die schwarze Farbe kommt weiterhin von oben"), T(" – alles, was der vorige Abschnitt zum Teerstuhl sagt, gilt unverändert, samt der Frist für die Magenspiegelung. "), B("Grün kommt von unten"), T(", und es ist neu.")]),
    P(&[B("Warum Stuhl grün wird."), T(" Die Galle ist bei ihrer Ausscheidung grün. Erst im Dickdarm bauen Bakterien das Bilirubin zu farblosen Stoffen ab, die an der Luft "), L("nachdunkeln", "https://www.ncbi.nlm.nih.gov/books/NBK470290/"), T(" und dem Stuhl seine gewohnte Farbe geben. Bleibt er grün, hat dieser Umbau nicht stattgefunden. Dafür gibt es genau zwei Gründe, und "), B("bei ihr treffen beide zu"), T(": Entweder war die Passage zu schnell, als dass die Bakterien Zeit gehabt hätten – oder die Bakterien, die es tun, fehlen.")]),
    P(&[B("Der erste Grund ist ausgerechnet die gute Nachricht."), T(" Der Wind geht wieder, der Stuhl ist flüssig und bewegt sich, der Appetit kommt zurück. Ein Darm, der wochenlang stand und jetzt wieder transportiert, schiebt den Inhalt schneller durch, als der Umbau dauert. Grüner Stuhl wäre dann kein neuer Befund, sondern das sichtbare Zeichen dafür, dass die Passage zurück ist – und passend dazu hielt schon der Bericht bei Eintritt fest, dass der Stuhl zwischen Durchfall, zum Teil grün, und Verstopfung schwankte.")]),
    P(&[B("Der zweite Grund führt auf eine Frage, die auf diesem Blatt bisher nicht stand."), T(" Zwei Tage vor dem Eintritt wurde auswärts Co-Amoxicillin verordnet. Ein Antibiotikum räumt genau die Bakterien ab, die den Farbumbau besorgen – und es öffnet die Tür für eines, das nicht abgeräumt wird. Der nächste Abschnitt geht dem nach.")]),

    H3("Die Frage, die jetzt dazugehört: Clostridioides difficile"),
    P(&[B("Eine Klarstellung zuerst, weil sonst ein falscher Eindruck entsteht:"), T(" Sie bekommt "), B("derzeit kein Antibiotikum"), T(". Co-Amoxicillin lief auswärts während zweier Tage "), B("vor"), T(" dem Eintritt vom 23. August, wegen des Verdachts auf eine Entzündung des Sigmas; auf der Medikamentenliste vom 30. August steht es nicht mehr. Wann genau es abgesetzt wurde, hält der Bericht nicht fest. "), B("Für diese Frage ändert das weniger, als es klingt"), T(" – aber es gehört richtig dargestellt.")]),
    P(&[T("Denn das Risiko hängt nicht daran, ob das Antibiotikum gerade läuft, sondern daran, dass es gelaufen ist. Als "), L("bedeutsamster Risikofaktor", "https://www.ncbi.nlm.nih.gov/books/NBK431054/"), T(" gilt ein breit wirksames Antibiotikum, ausdrücklich auch aus der Gruppe der Penicilline – Co-Amoxicillin gehört dazu. Dazu kommen höheres Lebensalter, ein Spitalaufenthalt, eine längere Aufenthaltsdauer, Begleiterkrankungen und Protonenpumpenhemmer. Alle diese Punkte treffen zu; beim Antibiotikum in der Vergangenheitsform, und genau so wird es in den Untersuchungen auch gezählt.")]),
    P(&[B("Wie gross der Beitrag der beiden Mittel ist, lässt sich beziffern."), T(" In einer schwedischen Untersuchung mit 43'152 Erkrankten und 355'172 Vergleichspersonen wurde ausdrücklich nach "), B("Einnahme in den letzten dreissig Tagen"), T(" gefragt – nicht nach laufender Einnahme. Dort lag das Risiko nach Antibiotikum "), B("und"), T(" Säureblocker bei einem "), L("Chancenverhältnis von 17,5", "https://pmc.ncbi.nlm.nih.gov/articles/PMC10904719/"), T(" – für das Antibiotikum allein bei 15,4, für den Säureblocker allein bei 2,7. Für eine länger zurückliegende Einnahme, 31 bis 180 Tage, sinken dieselben Zahlen auf 9,1, 5,4 und 2,1. "), B("Ihre Gabe liegt rund zehn Tage zurück und fällt damit voll in das erste Zeitfenster."), T(" "), B("Die Zahlen sagen deutlich, wo das Gewicht liegt:"), T(" beim Antibiotikum. Der Säureblocker verschiebt das Ganze noch etwas, aber er ist nicht der Grund.")]),
    P(&[B("Und hier gehört die Gegenprobe dazu, weil sonst ein falscher Schluss naheliegt."), T(" Beobachtungsstudien finden für Protonenpumpenhemmer durchgehend ein erhöhtes Risiko – eine Metaanalyse aus 23 Untersuchungen mit 186'033 Fällen kam auf ein "), L("Chancenverhältnis von 1,81", "https://pubmed.ncbi.nlm.nih.gov/27677811/"), T(". Nimmt man dagegen nur randomisierte Studien, verschwindet der Effekt: acht Studien mit 29'880 Teilnehmenden ergaben ein "), L("relatives Risiko von 1,19", "https://pmc.ncbi.nlm.nih.gov/articles/PMC12502825/"), T(" mit einem Vertrauensbereich von 0,75 bis 1,89 – also keinen belegbaren Unterschied. "), B("Daraus folgt ausdrücklich nicht, das Pantoprazol abzusetzen."), T(" Es hat hier eine eigene Begründung – die Magenentzündung und die Blutung –, und die wiegt schwerer als ein Risiko, das die besseren Studien nicht bestätigen. Was folgt, ist eine Untersuchung und keine Änderung der Verordnung.")]),
    P(&[B("Das Krankheitsbild passt zudem an zwei Stellen, an denen es unangenehm gut passt."), T(" Beschrieben wird wässriger Durchfall "), B("mit Schleim"), T(" oder verstecktem Blut – der weisse Schleim steht seit Tagen im Abschnitt darunter, und er war damals mit einem wieder anlaufenden Dickdarm erklärt. Und schwere Verläufe zeigen sich nicht als Durchfall, sondern als "), B("Bauchauftreibung, Darmlähmung und toxisches Megakolon"), T(". Der geblähte Bauch ohne mechanisches Hindernis ist die Hauptbeschwerde dieser Patientin, und dieses Blatt führt dafür bisher vier Erklärungen. Dies wäre die fünfte – und die einzige davon, die man behandeln kann, sobald man sie kennt.")]),
    P(&[B("Der praktische Punkt ist die Zeit, und er ist der Grund, warum das heute zählt."), T(" Auf Clostridioides difficile wird "), B("nur geflüssigter Stuhl untersucht"), T("; bei Patienten ohne Durchfall ist der Test nicht angezeigt, und geformter Stuhl wird gar nicht eingeschickt. Wochenlang gab es hier gar keinen Stuhl und danach zu wenig davon. "), B("Jetzt ist er flüssig – die Probe ist heute verfügbar und in ein paar Tagen vielleicht nicht mehr."), T(" Die Untersuchung läuft in zwei Schritten und kostet nichts als das Röhrchen. Ein negativer Befund nimmt eine Möglichkeit von der Liste; ein positiver ändert die Behandlung sofort.")]),

    H3("Und was Pantoprazol daran ändert: nichts"),
    P(&[T("Der Säureblocker läuft weiter, und der Stuhl ist trotzdem wieder schwarz. Das ist kein Versagen des Mittels, sondern die Erinnerung daran, wofür es nie zuständig war. Ein Säureblocker senkt die Säure; er verschliesst keine blutende Stelle. Die Cochrane-Übersicht zur Gabe "), L("vor der Spiegelung", "https://doi.org/10.1002/14651858.CD005415.pub4"), T(" ist an diesem Punkt eindeutig: Sie vermindert die Zeichen frischer Blutung, die man bei der Spiegelung sieht – aber nicht die Sterblichkeit, nicht das Nachbluten und nicht die Notwendigkeit einer Operation. "), B("Übersetzt heisst das:"), T(" Der Säureblocker macht den Befund milder, nicht die Blutung. Er ersetzt die Spiegelung nicht, sondern ist das, was man gibt, bis sie stattfindet – und dieselbe Fachinformation warnt an anderer Stelle, dass er die Beschwerden kaschieren und die Diagnose dadurch verzögern kann.")]),

    H3("Der weisse Schleim"),
    P(&[T("Er gehört nicht zur selben Geschichte. Die schwarze Farbe kommt von oben, aus Magen oder Zwölffingerdarm; Schleim kommt von unten. Er ist ein Erzeugnis der Darmschleimhaut, und eine "), L("gereizte Schleimhaut", "https://my.clevelandclinic.org/health/symptoms/mucus-in-stool"), T(" gibt mehr davon ab. Zwei Erklärungen liegen nahe: ein Dickdarm, der nach vier bis fünf Wochen Stillstand wieder in Gang kommt – und der Dickdarmbefall der Vaskulitis selbst.")]),
    P(&[T("Dazu liefert dieselbe Untersuchung an 108 Erwachsenen, die weiter oben die Magenspiegelungen ausgewertet hat, auch die Zahlen für unten. Bei den 31 "), L("Darmspiegelungen", "https://pmc.ncbi.nlm.nih.gov/articles/PMC11429444/"), T(" war der Krummdarm in 61,3 Prozent befallen, der Dickdarm in 38,7 und der Enddarm in 22,6 Prozent. "), B("Daraus folgt dasselbe wie oben, nur am anderen Ende:"), T(" Das Gerät muss bis in den Krummdarm, das letzte Stück Dünndarm vor dem Übergang in den Dickdarm. Wer dort nicht hinschaut, sieht die Stelle nicht, an der die Vaskulitis bei der Darmspiegelung am häufigsten sitzt.")]),

    H2("Schwarz erbrochen: die Blutung liegt oben"),
    P(&[T("Damit ist die zweite Hälfte des Beweises da, und die Frage «Galle oder Blut» ist endgültig erledigt. Schwarzes Erbrochenes heisst Kaffeesatz, und Kaffeesatz ist Blut, das in der Magensäure gelegen hat: Die Säure "), L("oxidiert den roten Blutfarbstoff zu braunem Hämatin", "https://www.merckmanuals.com/professional/gastrointestinal-disorders/gastrointestinal-bleeding/overview-of-gastrointestinal-bleeding"), T(". Oben Kaffeesatz, unten Teerstuhl – dasselbe Blut, an beiden Enden derselben Strecke. Die Quelle liegt damit im oberen Verdauungstrakt, also im Bereich, den die Magenspiegelung erreicht.")]),
    P(&[B("Die JAMA-Auswertung von oben hat dazu eine zweite Liste, die in diesem Blatt bisher fehlte."), T(" Für die Herkunft: Blut oder Kaffeesatz im Magensaft vervielfacht die Chance auf eine obere Blutungsquelle um das 9,6-fache. Für die "), B("Schwere"), T(" – und darum geht es jetzt – nennt dieselbe Arbeit ein "), L("Hämoglobin unter 8 g/dl", "https://pubmed.ncbi.nlm.nih.gov/22416103/"), T(", das die Chance auf eine schwere, dringend behandlungsbedürftige Blutung um das 4,5- bis 6,2-fache erhöht, und einen beschleunigten Puls, der sie um das 4,9-fache erhöht. "), N("75 g/l"), T(" sind "), N("8,3 g/dl"), T(", und der Wert ist seither gefallen. Diese Patientin steht also nicht am Rand dieser Liste, sondern mitten darin.")]),
    P(&[B("Ein Trost steckt trotzdem darin."), T(" Kaffeesatz ist nicht frisches Blut. Dass es überhaupt braun geworden ist, heisst, dass es Zeit in der Säure hatte – das "), L("spricht für eine Blutung, die langsamer geworden oder zum Stehen gekommen ist", "https://www.merckmanuals.com/professional/gastrointestinal-disorders/gastrointestinal-bleeding/overview-of-gastrointestinal-bleeding"), T(", nicht für einen Schwall. Kein Grund zu warten, aber ein Grund, nicht in Panik zu geraten.")]),
    P(&[B("Was daraus für die Suche folgt."), T(" Die Frist von 24 Stunden steht schon im Abschnitt zum Stuhl. Neu ist die Richtung: nicht in die Breite, sondern noch einmal genau dorthin, wo die Magenentzündung gefunden wurde. Damit werden die beiden Fragen dieses Blattes an die erste Spiegelung dringlich – wurde bis in den absteigenden Zwölffingerdarm geschaut, und wurde dort Gewebe entnommen? Dort sitzt die Vaskulitis bei Erwachsenen in 55,7 Prozent der Fälle, im Magen selbst nur in 3,3.")]),
    P(&[B("Zwei praktische Punkte zum Schluss."), T(" Der erste: Sie erbricht – ob eine Tablette unter diesen Umständen ankommt, ist eine berechtigte Frage, und der Weg über die Vene stellt sich von selbst. Ehrlich dazu gehört aber, was ein Säureblocker vor der Spiegelung leistet und was nicht: Die "), L("Cochrane-Übersicht", "https://doi.org/10.1002/14651858.CD005415.pub4"), T(" findet keinen Einfluss auf Sterblichkeit, Nachblutung, Operationsbedarf oder Transfusionsbedarf – nur weniger frische Blutungszeichen bei der Spiegelung. Er ist Vorbereitung, nicht Behandlung, und ersetzt die Spiegelung nicht. Der zweite: Die amerikanische Fachgesellschaft schlägt vor der Spiegelung eine "), L("Infusion von Erythromycin", "https://doi.org/10.14309/ajg.0000000000001245"), T(" vor; sie treibt den Mageninhalt weiter und macht die Sicht frei. Bei einem Magen, der ohnehin schlecht transportiert und in dem Blut liegt, ist das der passende Vorschlag.")]),

    H3("Jetzt gelb – und das ist die zweite gute Nachricht des Tages"),
    P(&[B("Am 2. September ist das Erbrochene gelb."), T(" Der ganze Abschnitt hier oben handelt davon, dass es schwarz war. Schwarz war Blut; gelb ist Galle. "), B("Damit ist das Zeichen weg, das die Frist von 24 Stunden ausgelöst hat."), T(" Das ist die erste Nachricht, und sie ist gut.")]),
    P(&[B("Was sie nicht heisst."), T(" Sie heisst nicht, dass die Blutung vorbei ist. Sie heisst, dass im Erbrochenen kein Blut mehr sichtbar ist – über den Stuhl sagt das nichts, und über den Hämoglobinwert erst recht nicht. Der eine Wert, der die Frage beantwortet, ob der Verlust steht oder weiterläuft, ist derselbe wie seit Tagen: ein neues Hämoglobin mit den Retikulozyten. Was sich geändert hat, ist die Dringlichkeit der Spiegelung, nicht die der Blutentnahme. Und käme das Schwarz zurück, liefe die Frist wieder.")]),
    P(&[B("Und was die Farbe zusätzlich verrät."), T(" Galle im Erbrochenen ist eine Ortsangabe. Erbrochenes besteht bei einem hohen Hindernis aus reinem Mageninhalt und "), L("wird gallig, je weiter unten das Hindernis liegt", "https://www.ncbi.nlm.nih.gov/books/NBK448079/"), T(" – Galle tritt erst unterhalb der Einmündung des Gallengangs in den Zwölffingerdarm hinzu. Gelb heisst deshalb: Es kommt Inhalt von unterhalb des Magenausgangs zurück. Ein reiner Verschluss des Magenausgangs ist damit unwahrscheinlich, und das passt zu dem, was der Gastroenterologe gesagt hat – keine rechte Peristaltik, aber kein mechanisches Hindernis.")]),
    P(&[B("Die Farbskala dieses Falles, in der Reihenfolge ihres Auftretens."), T(" Der Bericht nennt für die Zeit vor den Spiegelungen "), B("Miserere"), T(", also das Erbrechen von Darminhalt – das ferne Ende der Skala. Dann kam Schwarz, also Blut. Jetzt Gelb. "), B("Diese Reihenfolge geht in die richtige Richtung:"), T(" von Darminhalt über Blut zu Galle. Das beweist nichts, aber es ist die erste Beobachtung seit Wochen, die sich mehrfach in dieselbe Richtung bewegt – zusammen mit dem Wind, dem Stuhlgang und dem Appetit. Sie erbricht weiterhin; was sich geändert hat, ist, was dabei herauskommt.")]),

    H2("Die Darmspiegelung: nichts gefunden, ausser beim Transport"),
    P(&[T("Zwei Nachrichten in einer, und sie ziehen in verschiedene Richtungen. Die erste ist gut: Der Dickdarm ist unauffällig. Damit fällt der Dickdarmtumor weg, der bei 84 Jahren ganz oben auf der Liste stand, und der Dickdarm als Blutungsquelle gleich mit. Das ist echte Entlastung und keine Vertröstung.")]),
    P(&[B("Nur beantwortet dieser Befund den Teerstuhl nicht."), T(" Ein schwarzer Stuhl entsteht "), B("oberhalb"), T(" – dort, wo Blut durch den Darm wandert und dabei verdaut wird. Genau dorthin schaut die Darmspiegelung nicht. Ein unauffälliger Dickdarm ist mit einem Teerstuhl vollständig vereinbar; er war nie der Test dafür. Wer die beiden Befunde gegeneinander hält, hält Äpfel gegen Birnen.")]),
    P(&[B("Zwei Rückfragen gehören trotzdem gestellt."), T(" Erstens: Wie weit kam das Gerät? Der Krummdarm ist bei dieser Krankheit der häufigste Sitz, und er wird nicht bei jeder Darmspiegelung eingesehen – die "), L("Qualitätsmasse der Fachgesellschaften", "https://pmc.ncbi.nlm.nih.gov/articles/PMC10005623/"), T(" verlangen als Nachweis der Vollständigkeit den Blinddarm, nicht den Krummdarm. Zweitens: Wie sauber war der Darm? Nach einer Woche Stillstand und bei einem Darm, der ohnehin schlecht transportiert, ist die Frage nach der Vorbereitung keine Förmlichkeit. Beides steht im Befundbericht, und beides entscheidet, wie viel «nichts gefunden» hier wiegt.")]),
    P(&[B("Die zweite dieser Rückfragen ist inzwischen beantwortet, und zwar ungünstig."), T(" Der Zwischenbericht hält zur Spiegelung vom 28. August fest: der Dickdarm soweit unauffällig – "), B("bei schlechter Vorbereitung"), T(". Damit wiegt «nichts gefunden» deutlich weniger. Ein schlecht vorbereiteter Darm ist genau die Lage, in der flache Veränderungen übersehen werden, und die Qualitätsmasse der Fachgesellschaften knüpfen die Aussagekraft einer Spiegelung ausdrücklich an die Vorbereitung. Die erste Rückfrage – wie weit das Gerät kam – ist weiter offen; Bericht und Gewebeproben stehen aus. "), B("Praktisch heisst das:"), T(" Der Dickdarm ist nicht sauber ausgeschlossen, sondern unzureichend eingesehen. Ob wiederholt wird, hängt daran, was die Gewebeproben und die Zytologie zeigen.")]),
    H3("Warum die Vorbereitung schlecht war"),
    P(&[B("Die Frage beantwortet sich aus dem Fall selbst, und die Antwort ist unangenehm rund."), T(" Eine Darmvorbereitung besteht darin, literweise eine Spüllösung zu trinken und sie durch den Darm zu treiben. Beides setzt genau das voraus, was hier fehlt: Sie muss drin bleiben, und der Darm muss sie weiterbewegen. "), B("Die Vorbereitung ist also an derselben Störung gescheitert, deretwegen gespiegelt wurde."), T(" Das ist kein Vorwurf an irgendjemanden – es ist die Eigenart dieser Untersuchung, dass sie bei genau den Patienten am schlechtesten gelingt, bei denen sie am nötigsten ist.")]),
    P(&[T("Was daran belegt ist, lässt sich aufzählen. Für ältere Patienten hat eine "), L("Metaanalyse aus 2023", "https://pubmed.ncbi.nlm.nih.gov/37963423/"), T(" sechs unabhängige Risikofaktoren beziffert: Verstopfung erhöht das Risiko einer ungenügenden Vorbereitung um das 3,6-Fache, eine frühere Bauchoperation um das 2,7-Fache, Diabetes um das 2,5-Fache, unvollständiges Trinken der Lösung um das 2,4-Fache und zu wenig Bewegung während der Vorbereitung um das 3,1-Fache. Eine Übersicht über die "), L("schwer vorzubereitenden Patienten", "https://pmc.ncbi.nlm.nih.gov/articles/PMC10107216/"), T(" nennt zusätzlich: Ein Spitalaufenthalt verdoppelt das Risiko annähernd, und der Anteil stationärer Patienten mit sauberem Darm übersteigt 50 Prozent nicht; eine Magenlähmung erhöht es um das Vierfache, eine Verstopfung vom Transporttyp um das Doppelte; Opioide und höheres Alter stehen ebenfalls auf der Liste.")]),
    P(&[B("Und jetzt die Zusammenzählung, die den Befund erklärt."), T(" Bei ihr trifft praktisch jeder dieser Punkte zu: 84 Jahre, stationär, eine Woche ohne Stuhlgang, ein Darm ohne rechte Peristaltik, Erbrechen bis hin zu Darminhalt, zwei Opioide und ein Mittel gegen Übelkeit auf der Liste, Zustand nach Blinddarmentfernung, Prädiabetes, und bettlägerig. Dazu ein Punkt, den die Metaanalyse nicht kennt und der hier den Ausschlag geben dürfte: "), B("Wer erbricht, behält die Spüllösung nicht."), T(" Unvollständiges Trinken ist einer der belegten Risikofaktoren, und bei ihr war es nicht Unwilligkeit, sondern Unmöglichkeit. Die Nierenfunktion begrenzt zusätzlich, womit und wie forsch vorbereitet werden darf.")]),
    P(&[B("Daraus folgen zwei Dinge, und sie zeigen in verschiedene Richtungen."), T(" Erstens: Der unauffällige Dickdarm wiegt weniger, als er auf dem Papier aussieht. Zweitens: Eine blosse Wiederholung mit demselben Vorgehen würde vermutlich wieder scheitern – aus denselben Gründen. Die europäische Leitlinie verlangt bei ungenügender Vorbereitung eine "), L("frühe Wiederholung innerhalb eines Jahres", "https://www.thieme-connect.com/products/ejournals/html/10.1055/a-0959-0505"), T(", sofern nichts dagegen spricht, und rät, das nächste Vorgehen nach den vermuteten Gründen des Scheiterns auszurichten. Genau das ist hier die Aufgabe: nicht dieselbe Vorbereitung noch einmal, sondern eine, die mit einem Darm rechnet, der nicht transportiert – oder der Verzicht darauf, weil die Antwort schneller aus der Zytologie, den Gewebeproben und dem Schnittbild kommt.")]),

    P(&[B("Und zum Magen sagt derselbe Bericht etwas, das eine Annahme dieses Blattes verschiebt."), T(" Die sichtbare Entzündung sass am "), B("Übergang von der Speiseröhre zum Magen"), T(" – nicht im Magen selbst und nicht im absteigenden Zwölffingerdarm, wo dieses Blatt sie aufgrund der Untersuchung an 108 Erwachsenen vermutet hat. Das spricht eher für eine Refluxösophagitis als für die Vaskulitis, passt aber auch zum wochenlangen Erbrechen, das die Schleimhaut dort mechanisch und mit Säure belastet. Was es nicht beantwortet: ob im Zwölffingerdarm etwas war. Ob das Gerät bis dorthin kam und ob von dort Gewebe entnommen wurde, steht im ausstehenden Bericht – und genau das ist die Frage, an der die Diagnose hängt.")]),
    P(&[B("Und jetzt wird eine Lücke sichtbar."), T(" Die Magenspiegelung reicht bis in den absteigenden Zwölffingerdarm, die Darmspiegelung im besten Fall bis in den Krummdarm. Dazwischen liegen mehrere Meter Dünndarm, die kein Gerät von beiden sieht. Genau dafür gibt es einen Namen und einen Weg: Eine "), L("Blutungsquelle im Dünndarm", "https://pubmed.ncbi.nlm.nih.gov/26303132/"), T(" gehört erwogen, sobald Magen- und Darmspiegelung unauffällig sind; erste Wahl ist dann die Kapselendoskopie, und bleibt auch sie ohne Befund, folgt die Computertomografie als Enterografie. "), B("Nur steht dieser Weg jetzt hinten an."), T(" Dieselbe Leitlinie hält fest, dass vor der Dünndarmabklärung eine zweite Magenspiegelung stehen kann – und seit schwarz erbrochen wird, ist die Quelle oben lokalisiert. Der Dünndarm ist damit nicht der nächste Schritt, sondern der übernächste: der Weg für den Fall, dass oben nichts gefunden wird und es weiterblutet.")]),
    P(&[B("Der Transportbefund ist der zweite Teil der Nachricht."), T(" Der Gastroenterologe hat es deutlich gesagt: keine rechte Peristaltik. Das bestätigt, was dieses Blatt seit der ersten Fassung als Passagestörung beschreibt – aber "), B("ohne mechanisches Hindernis"), T(". Kein Tumor, keine Enge, und trotzdem eine echte Störung. Vier Erklärungen kommen infrage, und drei davon lassen sich mit Blutwerten prüfen, die ohnehin abgenommen werden:")]),
    Liste(&[
        &[B("Die Vaskulitis selbst."), T(" Die entzündete, geschwollene Darmwand bewegt sich schlechter. Das ist die Erklärung, die zum Rest des Bildes passt.")],
        &[B("Kalium und Magnesium."), T(" Beide stehen in diesem Blatt schon zweimal auf der Liste – wegen des Kostaufbaus nach Wochen ohne Nahrung und wegen Pantoprazol. Jetzt ein drittes Mal: Ein Mangel an beiden kann die Beweglichkeit des Dickdarms beeinträchtigen, und es gibt Fallberichte, in denen sich eine "), L("Pseudoobstruktion nach Ausgleich des Kaliums löste", "https://pmc.ncbi.nlm.nih.gov/articles/PMC12701538/"), T(". Ehrlicherweise gehört dazu: Neuere Arbeiten "), L("bestreiten", "https://journals.lww.com/ijam/fulltext/2015/01010/scrutinizing_the_evidence_linking_hypokalemia_and.4.aspx"), T(", dass ein Kaliummangel für sich allein einen Darmstillstand auslöst. Als mitwirkender Faktor bleibt er plausibel, und der Ausgleich ist ohnehin geboten. Woher ein Mangel käme, ist hier nicht schwer zu sagen: Wochen ohne Nahrung, Erbrechen, und Pantoprazol.")],
        &[B("Die Schilddrüse."), T(" Eine Unterfunktion kann einen Darmstillstand so überzeugend nachahmen, dass er für einen "), L("mechanischen Dünndarmverschluss", "https://pmc.ncbi.nlm.nih.gov/articles/PMC10796157/"), T(" gehalten wird. Geprüft wird das mit dem "), B("TSH"), T(" – dem Steuerhormon, mit dem die Hirnanhangsdrüse die Schilddrüse antreibt. Die Leserichtung ist dabei umgekehrt, ähnlich wie beim HbA1c weiter oben: Ein "), B("hoher"), T(" TSH-Wert spricht für eine "), B("Unter"), T("funktion, weil die Hirnanhangsdrüse lauter ruft, wenn zu wenig geliefert wird. Ein Bluttropfen aus der Entnahme, die ohnehin läuft.")],
        &[B("Medikamente."), T(" Die vollständige Liste gehört auch aus diesem Grund auf den Tisch, rezeptfreie Mittel eingeschlossen.")],
    ]),
    P(&[B("Und ja, es besteht ein Zusammenhang mit der Vaskulitis."), T(" Das ist nicht bloss plausibel, sondern in der Übersicht zur Magen-Darm-Beteiligung von Vaskulitiden ausdrücklich aufgeführt: Zu dem, was eine Vaskulitis im Verdauungstrakt anrichtet, zählen neben Geschwür, Schleimhautschwellung und Blutung auch der "), L("paralytische Ileus", "https://pmc.ncbi.nlm.nih.gov/articles/PMC3309893/"), T(" und der Darmverschluss. Dieselbe Arbeit nennt für die IgA-Vaskulitis als häufig betroffene Abschnitte den absteigenden Zwölffingerdarm und den Krummdarm – genau die beiden Stellen, die dieses Blatt an anderer Stelle als Zielorte der beiden Spiegelungen nennt. Dort stammt die Angabe aus der Untersuchung an 108 Erwachsenen, hier aus einer unabhängigen Quelle; zwei Wege, ein Ergebnis.")]),
    P(&[T("Der Mechanismus ist derselbe wie überall in diesem Blatt: Die Krankheit greift die Wand von Röhren an. Eine entzündete, geschwollene, schlechter durchblutete Darmwand zieht sich nicht mehr richtig zusammen – dieselbe Wandschwellung, die vorher die Passage behindert hat. Dazu kommt ein zweiter Weg, der nicht spezifisch für diese Vaskulitis ist, aber zum Bild passt: Eine Entzündung des Nervengeflechts im Darm führt zu "), L("Magenlähmung und Pseudoobstruktion", "https://www.gastrojournal.org/article/S0016-5085(04)00223-9/fulltext"), T(" – die Nerven, die den Transport steuern, arbeiten dann nicht mehr richtig.")]),
    P(&[B("Nur beweist der Zusammenhang die Ursache nicht."), T(" Die drei anderen Erklärungen von oben – Kalium, Magnesium, Schilddrüse – bleiben genau deshalb auf der Liste, weil sie sich mit drei Blutwerten prüfen und, anders als die Vaskulitis, unmittelbar beheben lassen. Und «keine rechte Peristaltik» ist ein klinischer Eindruck, kein Messwert. Das macht ihn nicht weniger wert – nur beantwortet er nicht, woran es liegt.")]),
    P(&[B("Dass sie Luft aufstösst, bis sie würgt, gehört in dasselbe Bild."), T(" Wenn der Transport nach unten nicht funktioniert, nimmt die Luft den Weg zurück nach oben. Aufstossen und Würgen sind dann nicht zwei neue Beschwerden, sondern dieselbe Störung von oben gesehen – so wie der geblähte Magen und das frühere Erbrechen nach jedem Bissen. Für den Bericht an die Ärztin zählt vor allem, dass es unter Pantoprazol nicht besser geworden ist: Ein Säureblocker senkt die Säure und hat auf den Transport keinen Einfluss.")]),

    Klein(&[T("Ein Vorbehalt zum TSH, der zur Redlichkeit gehört: Bei akut schwer kranken Menschen sind die Schilddrüsenwerte häufig verschoben, ohne dass die Schilddrüse krank wäre – das "), L("Syndrom der nicht-thyreoidalen Erkrankung", "https://www.ncbi.nlm.nih.gov/books/NBK482219/"), T(" betrifft Veränderungen der Schilddrüsenwerte bei einem grossen Teil hospitalisierter Patienten. Die Empfehlung lautet deshalb, den TSH-Wert nur bei begründetem Verdacht zu bestimmen und bei Schwerkranken nicht routinemässig. Der Verdacht ist hier begründet – es gibt einen ungeklärten Darmstillstand. Aber ein auffälliger Wert in dieser Lage ist ein Anlass zur Nachkontrolle und nicht schon eine Diagnose.")]),

    P(&[B("Und was sie im Moment am meisten belastet, ist genau das: der geblähte Darm."), T(" Das ist keine Nebenbeschwerde neben den Blutungen, sondern das Hauptgefühl – und es ist die unmittelbare Folge der fehlenden Peristaltik. Gas entsteht im Darm ständig; weitergetrieben wird es von der Peristaltik. Fällt sie aus, bleibt das Gas liegen, der Bauch spannt, und der Weg des geringsten Widerstands führt nach oben. Das Aufstossen bis zum Würgen ist dasselbe Gas, nur am anderen Ende.")]),
    P(&[T("Zwei Dinge folgen daraus, und sie zeigen in verschiedene Richtungen. Das eine ist beruhigend: "), B("Der Wind geht ab."), T(" Schwer, aber er geht – und er erleichtert sie jedes Mal. Die Verlaufstabelle weiter oben nennt ausbleibenden Stuhl "), B("zusammen mit"), T(" ausbleibendem Windabgang als das, was in den Notfall gehört und nicht in die Sprechstunde. Solange Wind abgeht, ist der Darm nicht verschlossen. Das andere ist die Kehrseite derselben Regel: Bleibt der Wind ganz aus und wird der Bauch dabei praller und schmerzhafter, ist das nichts, womit man bis zum nächsten Termin wartet.")]),
    P(&[B("Inzwischen geht er wieder besser."), T(" Zusammen mit dem Frühstück, das drin geblieben ist, sind das zwei Zeichen in dieselbe Richtung. Wie viel sie wiegen, ist untersucht – an der Darmlähmung nach Bauchoperationen, wo genau diese Frage seit Jahren gemessen wird. In einer Arbeit an 84 Patienten mit szintigrafisch bestimmtem Darmtransport war es "), B("nicht"), T(" die Zeit bis zum ersten Windabgang, die den wiederhergestellten Transport anzeigte, sondern die "), L("Kombination aus vertragener fester Kost und Stuhlgang", "https://pubmed.ncbi.nlm.nih.gov/23657087/"), T(" – mit einem positiven Vorhersagewert von 93 Prozent, bestätigt an weiteren 320 Patienten. "), B("Das ordnet die beiden Meldungen ein."), T(" Der Wind ist das schwächere der beiden Zeichen, und Haferbrei ist noch keine feste Kost. Beides ist eine echte Besserung – und noch nicht die Auskunft, dass der Transport wiederhergestellt ist. Die Einschränkung gehört dazu: Untersucht wurde die Darmlähmung nach einer Operation und nicht eine bei Vaskulitis.")]),
    P(&[B("Und es zeigt, woran die Erleichterung hängt."), T(" Ein Säureblocker senkt die Säure und bewegt nichts. Ein Abführmittel zielt auf den Stuhl und nicht auf das Gas. Was die Blähung löst, ist der Transport – und der hängt an den vier Erklärungen oben. Drei davon lassen sich mit Blutwerten prüfen und beheben, die vierte ist die Vaskulitis. Deshalb sind Kalium, Magnesium und der TSH-Wert hier nicht bloss eine Frage der Vollständigkeit: Sie sind das Einzige auf dieser Liste, das kurzfristig etwas an dem ändern könnte, was sie am meisten stört.")]),

    P(&[B("Und der «unruhige Bauch», von dem jetzt die Rede ist, lässt sich in zwei Richtungen lesen."), T(" Nach Tagen eines Darms, der nichts bewegt, kann Unruhe im Bauch das Zeichen sein, dass die Peristaltik zurückkommt – dann gehört sie zu Wind und Frühstück und ist die dritte gute Nachricht. Sie kann aber auch Kolik sein: Ein Darm, der gegen ein Hindernis anarbeitet, wird laut und krampft, und das ist das Gegenteil einer Entwarnung. "), B("Unterschieden wird das nicht am Gefühl, sondern an drei Beobachtungen:"), T(" Geht dabei Wind oder Stuhl ab? Wird der Bauch weicher oder praller? Und kommt der Schmerz in Wellen oder bleibt er stehen? Wellenförmiger Schmerz mit zunehmender Spannung und ohne Windabgang ist das Muster, das nicht bis zur nächsten Visite wartet.")]),
    P(&[B("Und was der Transportbefund für die Kapsel bedeutet."), T(" Die naheliegende Sorge lautet: Bleibt sie stecken? Die europäische Leitlinie ist genauer als die Sorge. Eine "), L("Motilitätsstörung ohne zugrundeliegende Enge ist kein Hinderungsgrund", "https://pubmed.ncbi.nlm.nih.gov/36423618/"), T("; ein Hinderungsgrund ist eine bekannte oder vermutete Enge, solange die Durchgängigkeit nicht belegt ist. Zeichen einer Obstruktion gelten als Risiko für ein Steckenbleiben, und das liegt je nach Fragestellung zwischen 2,1 und 8,2 Prozent. Praktisch heisst das: Die Computertomografie, die in diesem Blatt ohnehin an mehreren Stellen steht, beantwortet genau die Frage, an der die Kapsel hängt – gibt es eine Enge oder nicht.")]),

    H2("Was abgeklärt gehört"),
    Tab(&T_ABKLAERUNG),

    H2("Zur Behandlung"),
    P(&[T("Die Therapie führen die Spezialisten; hier steht nur, was den Rahmen erklärt.")]),
    P(&[T("Sind allein Haut und Gelenke betroffen, wird oft beobachtet und nur gegen die Beschwerden behandelt. Sobald Darm oder Niere beteiligt sind, kommt Kortison zum Einsatz, üblicherweise Prednison um 1 mg pro Kilogramm Körpergewicht mit anschliessendem Ausschleichen. Eine frühe Kortisonbehandlung senkt die Wahrscheinlichkeit eines bleibenden Nierenschadens deutlich. Bei schwerer Nierenbeteiligung kommen zusätzliche Immunsuppressiva in Frage – das entscheidet die Nierenbiopsie.")]),
    P(&[T("Käme Kortison zum Einsatz, gälte im hohen Alter: die niedrigste wirksame Dosis, und die Nebenwirkungen von Anfang an mitbehandeln – Magenschutz, Blutzucker, Knochendichte, Infektrisiko.")]),

    H3("Sie hat nie Kortison bekommen"),
    P(&[B("Mit einem Vorbehalt, der seit dem Zwischenbericht dazugehört."), T(" Auf der Medikamentenliste steht "), B("Dexamethason 4 mg in Reserve"), T(" – verordnet für den Bedarfsfall. Ob davon etwas verabreicht wurde, sagt nur das Verabreichungsprotokoll. Der folgende Abschnitt gilt unverändert für die Frage, ob behandelt werden soll; die Aussage «sie hat nie eines bekommen» steht bis zu dieser Auskunft unter Vorbehalt. Siehe den Abschnitt «Die Medikamentenliste».")]),
    P(&[T("Nicht jetzt und auch nicht beim früheren Schub. Das ist die Ausgangslage für diesen Abschnitt, und sie steht im Widerspruch zu dem, was der Absatz darüber als übliches Vorgehen beschreibt. Beides gehört nebeneinandergestellt, denn daraus wird die wichtigste Frage dieses Blattes an die Behandelnden. Und es bedeutet zweierlei: Der heutige Schub ist unbehandelt, und der damalige war es auch.")]),
    P(&[B("Was für Zurückhaltung spricht."), T(" Es blutet. Kortison schädigt die Magenschleimhaut, und die Fachinformation eines Prednisolon-Präparats führt "), L("Ulcus pepticum mit möglicher Perforation und Blutung", "https://ch.oddb.org/de/gcc/fachinfo/reg/38840/chapter/unwanted_effects"), T(" auf – ausdrücklich mit dem Zusatz, dass es «häufig ohne die typische Symptomatik» auftritt und dass intestinale Blutungen unter Kortikoiden symptomarm verlaufen können. In eine laufende Blutung hinein ist das ein ernsthaftes Argument. Dazu das Infektrisiko: In der Ueda-Serie starben drei von dreizehn über 60-Jährigen an Infekten unter Steroiden. Und die KDIGO-Leitlinie rät ausdrücklich davon ab, Steroide allein zur Vorbeugung einer Nephritis zu geben, wenn nur Haut, Gelenke oder Darm betroffen sind.")]),
    P(&[B("Was dagegen spricht, es dabei zu belassen."), T(" Die KDIGO-Zurückhaltung gilt der "), B("vorbeugenden"), T(" Gabe bei alleinigem Haut-, Gelenk- oder Darmbefall. Hier ist die Niere im Spiel: sichtbares Blut im Urin. Und der Darmbefall ist nicht mild, sondern blutet so stark, dass transfundiert werden muss. Für genau diese Lage – bedrohte Niere, schwerer Darmbefall – sind Steroide gängige Praxis, und der 71-jährige Fall mit Darmblutung sprach gut auf parenterale Kortikoide an. Dazu kommt der Harnleiter: Sollte der Stau von einer stenosierenden Ureteritis kommen, war Kortison in den Fallberichten genau das Mittel, unter dem die Harnleiter wieder durchgängig wurden.")]),
    P(&[B("Und wie es aussähe, wenn man es täte."), T(" Auch dazu ist die Leitlinie konkret: Wer nach einem Gespräch über Nutzen und Risiken systemische Glukokortikoide versuchen will, soll ein "), L("Schema mit reduzierter Dosis", "https://kdigo.org/wp-content/uploads/2024/08/KDIGO-2025-IgAV-Guideline-Key-Takeaways_IgAV.pdf"), T(" verwenden, zusammen mit einer vorbeugenden Behandlung gegen Infektionen. Das ist genau die Antwort auf das Argument aus der Ueda-Serie, in der drei von dreizehn über 60-Jährigen an Infekten starben. Und die Leitlinie hält ausdrücklich fest, dass das Risiko im Verlauf immer wieder neu einzuschätzen ist, weil sich die Entscheidung über eine Immunsuppression ändern kann.")]),
    P(&[B("Die Frage, die daraus folgt, gehört gestellt und nicht hier beantwortet."), T(" Wurde bewusst gegen Kortison entschieden, und wogegen wurde abgewogen – gegen die Blutung, gegen das Infektrisiko, gegen das Alter? Und wird die Entscheidung neu beurteilt, sobald die Blutungsquelle gefunden und versorgt ist? Denn dann fällt das stärkste Argument gegen die Behandlung weg. Was in diesem Blatt steht, ist Literatur für den Termin und kein Behandlungsplan; die Abwägung machen die Spezialisten, die die Patientin sehen.")]),
    P(&[T("Eine praktische Folge hat es auch. Alles, was in diesem Blatt an Kortison hing, fällt weg: der Magenschutz als Begleitung einer Kortisontherapie, der Interaktionstreffer zwischen Aspirin und Kortison, der Kaliumverlust und die Wasserretention als Erklärung für die Ödeme. Der Interaktionscheck weiter unten ist ohne Kortison neu gelaufen.")]),
    P(&[T("Drei Punkte, die sich anzusprechen lohnen:")]),
    Liste(&[
        &[B("NSAR meiden."), T(" Ibuprofen, Diclofenac, Naproxen belasten Niere und Magenschleimhaut gleichzeitig – beides genau dort, wo die Krankheit ohnehin angreift. Paracetamol ist die verträglichere Alternative. Sie hat "), B("Novalgin"), T(" bekommen; das ist kein NSAR und für kolikartige Schmerzen naheliegend, bringt aber eigene Auflagen mit – siehe den nächsten Abschnitt.")],
        &[B("Blutdruck einstellen."), T(" Bei Eiweiss im Urin schützt ein ACE-Hemmer oder ein Sartan die Niere zusätzlich.")],
        &[B("Was durch den Mund nicht geht, geht durch die Vene."), T(" Solange Erbrechen jede Mahlzeit beendet, ist auch die Aufnahme von Medikamenten unsicher. Das gehört bei der Verordnung mitbedacht.")],
    ]),
    P(&[T("Und die Nachkontrolle: Urin und Blutdruck regelmässig über mindestens sechs bis zwölf Monate, bei Erwachsenen eher länger. Eine Nierenbeteiligung kann auch dann noch auftreten, wenn der Ausschlag längst verschwunden ist. Genau das ist der häufigste Fehler – die Kontrollen enden, sobald die Haut wieder sauber aussieht.")]),

    H2("Novalgin"),
    P(&[T("Novalgin ist "), B("Metamizol"), T(" (Novaminsulfon, Dipyron), ein Pyrazolonderivat und "), B("kein NSAR"), T(". Es wirkt schmerzstillend, fiebersenkend und "), B("krampflösend"), T(" – gerade das Letzte passt zu kolikartigen Bauchschmerzen. Als Wahl ist es nachvollziehbar: Es belastet Magenschleimhaut und Niere nicht auf demselben Weg wie Ibuprofen oder Diclofenac. Dieselbe Substanz heisst in der Schweiz auch Minalgin, Metamizol-Mepha, Metamizol Spirig HC oder Novaminsulfon Sintetica; es gibt sie als Tabletten, Tropfen und Ampullen.")]),
    P(&[B("Und weil die Frage naheliegt, wenn in diesem Blatt so viel von Infektion, Fieber und Entzündungswerten die Rede ist:"), T(" "), B("Novalgin ist kein Antibiotikum."), T(" Es wirkt gegen keinen einzigen Erreger. Die Fachinformation führt als Wirkungen ausschliesslich "), L("analgetisch, spasmolytisch und antipyretisch", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/effects"), T(" auf – schmerzstillend, krampflösend, fiebersenkend –, und als zugelassene "), L("Anwendungsgebiete", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/indications"), T(" nur zweierlei: starke Schmerzen, bei denen andere Nicht-Opioid-Schmerzmittel nicht wirken oder nicht infrage kommen, und hohes Fieber, das auf andere Massnahmen nicht anspricht. Auch die Einordnung im ATC-System sagt es: "), B("N02BB02"), T(" – N für Nervensystem, N02 für Schmerzmittel. Antibiotika stehen in einer ganz anderen Gruppe.")]),
    P(&[B("Der Unterschied ist hier keine Begriffsklärung, sondern der Kern der Sache."), T(" Dieselbe Stelle beschreibt, "), B("wie"), T(" das Fieber gesenkt wird: durch Beeinflussung des Wärmezentrums im Hirn und vermehrte Wärmeabgabe. Novalgin verstellt also den Thermostaten – es tut nichts gegen das, was das Fieber auslöst. "), B("Daraus folgt für ihre Lage genau eines:"), T(" Das CRP steigt, ein Antibiotikum läuft nicht, und das einzige Mittel, das an der Temperatur etwas ändert, behandelt keine Ursache. Bleibt sie fieberfrei, ist das unter Novalgin kein Beweis dafür, dass nichts vorliegt. Und für die Agranulozytose, vor der dieselbe Fachinformation warnt, ist gerade das Fieber eines der Warnzeichen. Der Thermometerwert ist unter diesem Mittel weniger wert als sonst – das Differentialblutbild ist es nicht.")]),
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

    H3("Es läuft – und zwar intravenös"),
    P(&[B("Damit ist aus der Frage dieses Abschnitts eine Tatsache geworden."), T(" Novalgin steht nicht zur Auswahl, es läuft als Infusion. Das ändert zweierlei: Der wichtigste Vorzug des Mittels gilt hier tatsächlich, und mehrere Auflagen der Fachinformation, die für Tropfen nebensächlich sind, werden für die Infusion wichtig.")]),
    P(&[B("Der Vorzug zuerst, weil er in dieser Lage schwer wiegt."), T(" Metamizol ist kein Opioid. Es ist damit das einzige Schmerzmittel auf ihrer Liste, das den Darmtransport nicht bremst – während zwei Opioide in Reserve stehen, deren Fachinformationen Obstipation, Ileus und den paralytischen Ileus als Gegenanzeige führen. "), B("Daraus ergibt sich eine praktische Frage an die Station:"), T(" Reicht die Novalgin-Infusion für den Schmerz, oder wird zusätzlich Opioid abgerufen? Je mehr der Schmerz vom Nicht-Opioid abgedeckt ist, desto weniger Grund gibt es, den Darm weiter zu bremsen.")]),
    P(&[B("Und jetzt die Auflagen, die der intravenöse Weg mitbringt."), T(" Die Fachinformation ist an dieser Stelle ungewöhnlich genau, und drei Punkte betreffen sie unmittelbar:")]),
    Liste(&[
        &[B("Die Geschwindigkeit."), T(" Metamizol kann Blutdruckabfälle auslösen; sie sind dosisabhängig und treten "), L("eher nach parenteraler Verabreichung", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/restrictions"), T(" auf. Die Vorgabe dagegen ist eine Zahl: Die intravenöse Injektion muss "), B("sehr langsam erfolgen, höchstens 1 ml je Minute"), T(". Und ausdrücklich genannt sind die Patienten, bei denen der hämodynamische Zustand vorher zu optimieren ist – solche mit Volumenmangel, Austrocknung oder instabilem Kreislauf. Das beschreibt ihre letzten Wochen.")],
        &[B("Der Weg selbst."), T(" Dieselbe Stelle hält fest, dass bei der Wahl der Verabreichungsart zu berücksichtigen ist, dass die parenterale Gabe mit einem "), B("erhöhten Risiko anaphylaktischer Reaktionen"), T(" einhergeht. Das ist kein Argument gegen die Infusion – es ist der Grund, warum sie unter Aufsicht läuft.")],
        &[B("Die Niere."), T(" Von der Anwendung bei schwerer Nieren- oder Leberinsuffizienz wird abgeraten, weil dazu die Erfahrung fehlt; und unabhängig davon steht dort die Empfehlung, bei Niereninsuffizienz "), B("keine hohen Dosen"), T(" zu geben. Ihre GFR liegt bei "), N("30 ml/min"), T(". Die Frage nach der Tagesdosis gehört deshalb gestellt, auch wenn die Anwendung als solche vertretbar ist.")],
    ]),
    P(&[B("Der Punkt, der mir am wichtigsten erscheint, betrifft aber die Agranulozytose – und zwar auf eine Weise, die vorher nicht galt."), T(" Der Abschnitt oben nennt sie als die gefürchtete Nebenwirkung und verlangt das Differentialblutbild. Die Fachinformation fügt zwei Sätze hinzu, und einer davon trifft hier unmittelbar zu: Wenn Metamizol "), B("gegen Fieber"), T(" gegeben wird, können einige Symptome einer neu auftretenden Agranulozytose "), L("unbemerkt bleiben", "https://ch.oddb.org/de/gcc/fachinfo/reg/16952/chapter/restrictions"), T("; ebenso können sie bei Patienten unter einer Antibiotikabehandlung verschleiert werden. "), B("Der zweite Satz gilt hier nicht"), T(" – ein Antibiotikum läuft nicht, siehe die Klarstellung im Abschnitt zum Stuhl. Der erste gilt sehr wohl: Metamizol senkt das Fieber, ob man es dafür gibt oder nicht. Ein Warnzeichen, auf das man bei diesem Mittel ausdrücklich achten soll, wird von diesem Mittel selbst gedämpft.")]),
    P(&[B("Und hier ist eine Beobachtung, die dazu quer steht und die dem Bericht zu entnehmen ist."), T(" Das CRP war bei Eintritt mit "), N("60,4 mg/l"), T(" erhöht und ist im Verlauf gestiegen – "), B("ohne dass ein Antibiotikum läuft"), T(". Dafür gibt es zwei Lesarten, und beide sind wichtig: Entweder stammt die Entzündung vom Grundleiden, dann ist der steigende Wert ein Befund über die Krankheit; oder es besteht eine Infektion, die nicht behandelt wird. Was hier "), B("nicht"), T(" zutrifft, ist die dritte, beruhigende Lesart – dass ein laufendes Antibiotikum das Bild verwische. Zusammen mit dem fiebersenkenden Metamizol heisst das: Auf die klinischen Zeichen ist im Moment wenig Verlass, und das Differentialblutbild ist das, was zählt. Es steht in der Abklärungstabelle dieses Blattes seit der ersten Fassung.")]),
    P(&[B("Zwei weitere Punkte gehören zum Bild, weil andere Befunde dazu passen."), T(" Erstens führt die Fachinformation gastrointestinale Blutungen, Ulzerationen und Perforationen unter den unerwünschten Wirkungen – bei einer Patientin mit schwarzem Stuhl ist das kein Nebensatz, sondern gehört in dieselbe Rechnung wie die Blutungsquelle. Zweitens sind unter Metamizol Fälle akuter Leberschädigung beschrieben, die Tage bis Monate nach Beginn einsetzen. Der Zwischenbericht nennt eine ungleichmässig verdickte Gallenblasenwand, erweiterte Gallenwege und eine leicht erhöhte GGT. Das ist "), B("kein"), T(" Hinweis darauf, dass Novalgin daran schuld wäre – die Befunde lagen bei Eintritt vor. Es ist der Grund, die Leberwerte im Verlauf mitzuführen, weil hier zwei mögliche Ursachen nebeneinanderliegen.")]),
    Klein(&[T("Zum Interaktionscheck mit der wirklichen Liste: Novalgin erzeugte darin drei Treffer, und alle drei sind Fehlalarme derselben Art, vor der dieses Blatt schon einmal warnt. Das Programm ordnete Metamizol einmal der Acetylsalicylsäure zu, einmal dem Paracetamol und einmal der Gruppe der starken Analgetika. Metamizol ist keines davon. Echte neue Wechselwirkungen bringt die Aufnahme von Novalgin in den Warenkorb nicht – die schwerwiegenden Paare der Liste bleiben Ondansetron mit Haloperidol, Metoclopramid mit Haloperidol und die beiden Opioide untereinander.")]),

    H2("Pantoprazol"),
    P(&[T("Pantoprazol ist ein "), B("Protonenpumpenhemmer"), T(". Es blockiert die Säurepumpe der Belegzellen in der Magenwand und senkt damit die Säureproduktion. Dieselbe Substanz heisst in der Schweiz unter anderem Pantozol, Pantoprazol Sandoz, Pantoprazol-Mepha, Pantoprazol Spirig HC oder Pantoprazol Zentiva; es gibt sie als magensaftresistente Tablette, als Granulat und für die Vene.")]),
    P(&[B("Dass es läuft, ist folgerichtig – aber aus einem anderen Grund, als dieses Blatt zuerst annahm."), T(" Nicht als Begleitschutz einer Kortisontherapie, denn eine solche gibt es nicht. Sondern wegen des Befundes selbst: einer starken Magenentzündung bei einer Patientin, die niedrig dosiertes Aspirin und Novalgin bekommt und aus dem oberen Verdauungstrakt blutet. Das ist Grund genug.")]),
    P(&[T("Fünf Punkte aus der Fachinformation, die in dieser Lage zählen:")]),
    Liste(&[
        &[B("Eine Stunde vor dem Essen, ganz, mit Wasser."), T(" Die "), L("Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/usage"), T(" verlangt, die magensaftresistente Tablette unzerkaut und unzerbrochen "), B("1 Stunde vor einer Mahlzeit"), T(" mit etwas Wasser einzunehmen. Das ist keine Formalie: Die Säurepumpen müssen arbeiten, damit das Mittel sie treffen kann. Bei einer Patientin, die jetzt wieder frühstückt, heisst das konkret: zuerst die Tablette mit Wasser, das Frühstück eine Stunde später – nicht mit dem Haferbrei zusammen und nicht mit dem Tee.")],
        &[B("Im Alter höchstens 40 mg am Tag."), T(" Für ältere Patientinnen und bei eingeschränkter Nierenfunktion setzt die "), L("Fachinformation", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/usage"), T(" dieselbe Obergrenze: 40 mg Pantoprazol täglich, nicht mehr.")],
        &[B("Die Magenentzündung steht nicht in der Indikationsliste."), T(" Aufgeführt sind "), L("Refluxkrankheit, Magen- und Zwölffingerdarmgeschwür, die Helicobacter-Behandlung, die Vorbeugung von NSAR-Geschwüren bei erhöhtem Risiko und das Zollinger-Ellison-Syndrom", "https://ch.oddb.org/de/gcc/fachinfo/reg/58350/chapter/indications"), T(". Eine Gastritis als solche ist nicht darunter. Das ist kein Einwand gegen die Verordnung – bei erodierter Schleimhaut und unter Kortison mit Aspirin ist der Einsatz gängig und hier gut begründet. Es erklärt aber, warum die Fachinformation zur Dauer nichts Passendes sagt. Zwei Fragen bleiben deshalb ausdrücklich zu stellen: wie lange, und woran wird der Erfolg gemessen?")],
        &[B("Eisen kommt schlechter an."), T(" Das Eisen aus pflanzlicher Nahrung – aus Haferflocken etwa – braucht die Magensäure, um in die aufnehmbare Form zu kommen; ein Protonenpumpenhemmer nimmt ihm diese Voraussetzung. In einer "), L("Untersuchung an 43 Patientinnen und Patienten", "https://pmc.ncbi.nlm.nih.gov/articles/PMC9175665/"), T(" mit Eisenmangel unter einem Protonenpumpenhemmer, bei denen keine andere Ursache zu finden war, hatte Eisen zum Schlucken bei 41 nicht angeschlagen; auf Eisen in die Vene stieg der Hämoglobinwert bei 95 Prozent um mindestens "), N("20 g/l"), T(". Bei "), N("75 g/l"), T(" ist das die praktisch wichtigste Zeile dieses Abschnitts: Wird Eisen verordnet, ist der Weg über den Mund unter laufendem Pantoprazol der unsichere.")],
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
    P(&[B("Eine Stunde Abstand halbiert den Effekt."), T(" Eine "), L("kontrollierte Studie", "https://pubmed.ncbi.nlm.nih.gov/29046302/"), T(" hat genau diese Mahlzeit untersucht – Haferbrei mit markiertem Eisen – und den Tee einmal dazu und einmal eine Stunde danach gegeben. Zum Brei getrunken hemmte er die Eisenaufnahme um 37,2 Prozent, eine Stunde danach nur noch um 18,1 Prozent. Bei einem Hämoglobin von "), N("75 g/l"), T(" ist das keine Feinheit. Zusammen mit der Einnahmevorschrift für Pantoprazol ergibt das eine einfache Reihenfolge für den Morgen: zuerst die Tablette mit Wasser, eine Stunde später die Haferflocken, den Tee zuletzt.")]),
    P(&[B("Der Zucker ist hier kein Einwand."), T(" Nach Wochen ohne Nahrung sind Kalorien, die freiwillig hineingehen, ein Gewinn. Was in diesen Tagen zählt, ist nicht die Menge Zucker, sondern das, was schon im Abschnitt zum Kostaufbau steht: Phosphat, Kalium und Magnesium gehören kontrolliert.")]),
    P(&[B("Zum Zimt eine Zahl, damit die Frage vom Tisch ist."), T(" Cassia-Zimt – der übliche Haushaltszimt – enthält Cumarin, im Mittel rund 3 Gramm je Kilogramm Zimt, im Höchstfall 10. Die duldbare tägliche Aufnahme liegt "), L("nach Einschätzung des deutschen Bundesinstituts für Risikobewertung", "https://www.bfr.bund.de/de/service/haeufig-gestellte-fragen/thema/faq-zu-cumarin-in-zimt-und-anderen-lebensmitteln/"), T(" bei 0,1 mg Cumarin je Kilogramm Körpergewicht und Tag; für 60 kg Körpergewicht sind das rund 2 Gramm Cassia-Zimt am Tag, also etwa ein gestrichener Teelöffel. Eine Prise im Tee bleibt weit darunter. Dieselbe Stelle hält ausdrücklich fest, dass bislang kein Fall von Leberschaden nach dem Verzehr von Zimt beschrieben ist. Ceylon-Zimt enthält ohnehin nur wenig Cumarin.")]),

    H3("Der Kostaufbau nach Wochen ohne Nahrung"),
    P(&[T("Der Vergleich mit einem langen Hungerstreik trifft den Kern, und die Antwort dort lautet: Nach langem Fasten ist nicht das Essen das Risiko, sondern das Tempo, mit dem es zurückkommt. Der Fachbegriff dafür ist das "), B("Refeeding-Syndrom"), T(".")]),
    P(&[B("Was dabei geschieht."), T(" Im Fasten stellt der Körper um. Kommen wieder Kohlenhydrate, schüttet er Insulin aus, und Insulin treibt Phosphat, Kalium und Magnesium aus dem Blut in die Zellen. Die Blutwerte fallen binnen Stunden bis Tagen, obwohl doch gerade wieder gegessen wird. Das Kennzeichen ist der "), L("Abfall des Phosphats", "https://www.ncbi.nlm.nih.gov/books/NBK564513/"), T(".")]),
    P(&[B("Und warum das gerade bei ihr doppelt zählt."), T(" Phosphatmangel senkt die Pumpkraft des Herzens und macht Rhythmusstörungen. Vor allem aber senkt er das 2,3-DPG in den roten Blutkörperchen – und dadurch hält das Hämoglobin den Sauerstoff fester und gibt ihn im Gewebe schlechter ab. Bei einer Patientin mit "), N("75 g/l"), T(" und laufender Transfusion heisst das: Das wenige Hämoglobin, das vorhanden ist, würde auch noch schlechter liefern. Schwere Formen können bis zum Atemversagen führen. Der Kostaufbau ist damit keine Nebensache neben der Blutung, sondern hängt direkt an demselben Problem.")]),
    P(&[B("Wer als gefährdet gilt."), T(" Die britische Leitlinie nennt vier Merkmale: ein Körpermassenindex unter 16, ein unbeabsichtigter Gewichtsverlust von mehr als 15 Prozent in drei bis sechs Monaten, "), B("kaum oder keine Nahrung über mehr als zehn Tage"), T(", oder tiefe Werte von Kalium, Phosphat oder Magnesium schon vor Beginn der Ernährung. Das dritte Merkmal war hier über Wochen erfüllt, und das vierte ist genau das, was dieses Blatt an mehreren Stellen zu messen verlangt.")]),
    P(&[B("Was daraus als Vorgehen folgt."), T(" Langsam anfangen – "), L("10 bis 20 kcal je Kilogramm", "https://www.ncbi.nlm.nih.gov/books/NBK564513/"), T(" in den ersten 24 Stunden, danach alle ein bis zwei Tage um etwa ein Drittel des Ziels steigern. "), B("Thiamin, also Vitamin B1, 100 mg vor der ersten Kohlenhydratgabe"), T(", dann zweimal täglich über sieben bis zehn Tage; das schützt vor einer bleibenden neurologischen Schädigung. Und bei Gefährdeten Kalium, Magnesium und Phosphat in den ersten drei Tagen alle zwölf Stunden kontrollieren und ersetzen, danach dreimal in der Folgewoche.")]),
    P(&[T("Thiamin stand in diesem Blatt bisher nirgends – es steht jetzt in der Abklärungstabelle. Es ist billig, es wird vor dem Essen gegeben und nicht danach, und wenn es fehlt, ist der Schaden nicht mehr rückgängig zu machen. Das ist die Art von Punkt, die auf einer Station mit einer blutenden Patientin leicht untergeht.")]),
    P(&[B("Und zum Weg, weil er nicht gleichgültig ist."), T(" Die Erfahrung mit Hungerstreikenden zeigt, dass Thiamin durch den Mund versagen kann, und zwar auch in hoher Dosis. In Izmir wurden 41 Gefangene nachuntersucht, die zwischen 130 und 324 Tagen im Hungerstreik waren und dabei "), L("200 bis 600 mg Thiamin täglich als Tablette", "https://pubmed.ncbi.nlm.nih.gov/16987161/"), T(" bekommen hatten – ein Vielfaches der üblichen Dosis. Alle 41 entwickelten trotzdem ein Wernicke-Korsakow-Syndrom mit bleibender Schädigung. Ein Bericht von 2022 beschreibt dasselbe bei einer Frau nach 237 Tagen und zieht den Schluss, dass vorbeugendes Thiamin "), L("in die Vene oder in den Muskel", "https://pmc.ncbi.nlm.nih.gov/articles/PMC9359357/"), T(" als Standard zu prüfen ist. Der Grund ist einfach: Ein Darm, der lange nichts zu tun hatte, nimmt schlechter auf. "), B("Bei ihr kommt hinzu, dass der Magen sich nicht entleert und der Darm nicht transportiert."), T(" Damit ist der Weg durch den Mund für nichts Wichtiges der verlässliche – die Frage nach dem Weg gehört zu jeder Verordnung, an der etwas hängt. Ausführlich steht das im Begleitblatt «Kostaufbau nach langem Hungern», das dieselbe Frage anhand der Literatur zu Hungerstreikenden durchgeht.")]),
    P(&[B("Und die Reihenfolge, die sich daraus ergibt."), T(" Erst Menge und Tempo, dann die Konsistenz, dann erst die Speisekarte. Die beiden folgenden Abschnitte behandeln die zweite und die dritte Stufe.")]),

    H3("Poulet ging nicht"),
    P(&[T("Ein Versuch mit Poulet endete damit, dass alles wieder herauskam. Das ist keine Unverträglichkeit gegen Huhn, sondern die Regel bei einem Magen, der nicht entleert – und es erklärt zugleich, warum vorher etwas anderes gegangen ist.")]),
    P(&[B("Und am 3. September gingen Fleisch, Gemüsesuppe und ein wenig Polenta – und blieben drin."), T(" Das ist die vierte Prüfung der Regel dieses Abschnitts, und sie besteht sie nicht glatt: Fleisch ist genau das, was unten als das Schwierigste steht. Zwei Erklärungen sind möglich, und das Blatt lässt beide stehen, statt die Regel umzuschreiben. Entweder hat sich die Passage seit dem 2. September weiter gebessert – dafür sprechen das gelbe statt schwarze Erbrochene, der Stuhlgang und der Appetit nach der Punktion –, oder die Regel war zu streng gefasst. In beiden Fällen gilt: Was drinbleibt, gehört mit Uhrzeit und Menge auf das Verlaufsblatt, denn es ist die einzige Messung des Kostaufbaus, die es gibt. Und die Geruchsempfindlichkeit, die weiter besteht, steht im Abschnitt «Die Diagnose vom 3. September» an ihrem Platz: als Zeichen der Krankheit, nicht als Vorliebe.")]),
    P(&[B("Feste Nahrung ist das Schwierigste, was man einem stehenden Magen zumuten kann."), T(" Der Magen muss Festes erst zerkleinern, bevor es den Magenausgang passieren kann; dafür braucht er die Peristaltik, die hier fehlt. Haferbrei ist bereits fein, Tee ist flüssig – beide passieren, ohne zerkleinert zu werden. Ein Stück Fleisch ist weder das eine noch das andere. Was hier ging und was nicht ging, folgt also keiner Laune, sondern der Konsistenz.")]),
    P(&[T("Untersucht ist das an der Gastroparese, der Magenlähmung. In einer randomisierten Studie an 56 Patienten wurde eine Kost aus "), L("kleinen Partikeln", "https://pubmed.ncbi.nlm.nih.gov/24419482/"), T(" gegen gewöhnliche Kost mit unterschiedlicher Partikelgrösse gestellt: Übelkeit und Erbrechen, Völlegefühl und "), B("Blähung"), T(" besserten sich unter der feinen Kost deutlich stärker. Die "), L("Leitlinie der amerikanischen Gastroenterologen", "https://pubmed.ncbi.nlm.nih.gov/35926490/"), T(" empfiehlt für die Gastroparese ausserdem, unverdauliche Faserstoffe aus Gemüse und Obst zu meiden – was sich mit dem deckt, was oben zum Spinat steht. Die Einschränkung gehört dazu: Untersucht wurde eine Magenlähmung bei Diabetes, nicht ein Darmstillstand bei Vaskulitis. Das Prinzip hängt aber an der Mechanik und nicht an der Ursache.")]),
    P(&[B("Was daraus ausdrücklich nicht folgt: weniger Eiweiss."), T(" Sie braucht Eiweiss – die Ödeme, das Albumin und der Kostaufbau nach Wochen ohne Nahrung stehen weiter oben in diesem Blatt. Das Problem ist nicht das Eiweiss, sondern seine Form. Eiweiss in einer Form, die den Magen verlässt, ist eine Aufgabe für die Ernährungsberatung, und der Kostaufbau gehört ohnehin begleitet. Wer aus «Poulet ging nicht» ableitet, es solle weniger gegessen werden, zieht den falschen Schluss.")]),
    P(&[B("Und was das Erbrechen selbst anrichtet, schliesst einen Kreis."), T(" Wer alles wieder erbricht, verliert Flüssigkeit und Kalium. Kaliummangel steht in diesem Blatt schon als eine der Erklärungen für den fehlenden Transport. Damit hängt das eine am anderen: Der stehende Magen führt zum Erbrechen, das Erbrechen kostet Kalium, der Kaliummangel bremst den Transport weiter. Diesen Kreis durchbricht keine Speisekarte, sondern der Ersatz von Flüssigkeit und Elektrolyten – der über die Infusion bereits läuft – zusammen mit der Kontrolle der Werte. Und wiederholtes Erbrechen ist eines der Warnsymptome aus der Fachinformation von Pantoprazol: Es ist nicht neu, aber es ist zurück.")]),
    P(&[B("Und am nächsten Morgen ist das Frühstück drin geblieben."), T(" Schwarztee mit Milch und Haferbrei, kein Erbrechen. Das ist zuerst eine gute Nachricht und zweitens eine Bestätigung: Genau das sagt der Absatz oben voraus. Was fein oder flüssig ist, passiert den Magenausgang, ohne zerkleinert zu werden; ein Stück Fleisch nicht. Die Regel hat sich damit in beide Richtungen bewährt – einmal, indem sie erklärte, warum etwas misslang, und einmal, indem sie voraussagte, was gelingen würde. "), B("Der Schluss daraus ist nicht «wieder normal essen»,"), T(" sondern: bei der Konsistenz bleiben, die nachweislich geht, und die Speisekarte erst danach erweitern. Und das Erbrechen, das jetzt ausbleibt, hört damit auf, Kalium zu kosten – der Kreis aus dem Absatz davor dreht sich in die andere Richtung.")]),
    P(&[B("Und am 2. September kam das Birchermüesli wieder heraus."), T(" Damit hat die Regel dieses Abschnitts ihre dritte Prüfung, und sie besteht auch diese. Haferbrei ist gekocht, fein und warm; Birchermüesli ist rohe Haferflocke, Apfelstück und Nuss, kalt und mit Rahm oder Joghurt zusätzlich fett. Es ist fast das Gegenteil dessen, was oben steht – grobe Teilchen statt feiner, dazu die "), B("unverdaulichen Faserstoffe aus Obst"), T(", von denen die Leitlinie bei der Magenlähmung ausdrücklich abrät. «Haferflocken» ist eben nicht gleich «Haferflocken»; es kommt darauf an, ob sie gekocht sind.")]),
    P(&[B("Der ehrliche Einwand dagegen."), T(" Es kann ebenso gut sein, dass die Konsistenz damit gar nichts zu tun hat – sondern dass die Passage wieder schlechter ist. Diese beiden Erklärungen trennt eine einzige Beobachtung: Kommt jetzt "), B("alles"), T(" wieder heraus, auch Haferbrei und Tee, dann ist es die Passage, und dann gilt der Abschnitt zur Passagestörung und nicht die Speisekarte. Bleibt der Brei drin und nur das Müesli nicht, dann ist es die Konsistenz. Das ist keine Frage an die Ärztin, sondern eine Beobachtung am Bett, und sie kostet nichts.")]),
    P(&[B("Ein Satz aus dem Pantoprazol-Abschnitt bekommt damit ein anderes Gewicht."), T(" Wiederholtes Erbrechen gehört dort zu den Warnsymptomen, bei denen die Fachinformation den Ausschluss einer bösartigen Erkrankung verlangt. Solange das ein theoretischer Punkt war, stand er als Mahnung da. Seit dem 2. September steht er neben einem zytologischen Befund – siehe den Abschnitt «Die Zellen im Bauchwasser».")]),

    H3("Spinat, und was sonst noch den Stuhlgang fördert"),
    P(&[T("In Griechenland hat Spinat geholfen, den Stuhlgang wieder in Gang zu bringen. Das ist eine brauchbare Beobachtung und gehört ernst genommen. Nur ist die Lage heute eine andere, und dieselbe Massnahme kann jetzt in die falsche Richtung wirken.")]),
    P(&[B("Warum es damals wirkte – und warum das kein Versprechen für heute ist."), T(" Ballaststoffe wirken über das Volumen: Sie binden Wasser und vergrössern den Stuhl. Damit das hilft, muss etwas da sein, das das Volumen weiterschiebt – die Peristaltik. Genau die fehlt. Und ihr Hauptproblem ist im Moment nicht der fehlende Stuhl, sondern der geblähte Darm; mehr Volumen ist das, was bläht. Untersucht ist das: In einer Arbeit an 63 Patientinnen und Patienten mit Verstopfung ohne organische Ursache stieg die Stuhlfrequenz, nachdem die Ballaststoffe "), L("ganz weggelassen wurden", "https://pubmed.ncbi.nlm.nih.gov/22969234/"), T(" – von einem Stuhlgang alle 3,75 Tage auf einen täglich; wer bei viel Ballaststoff blieb, änderte nichts. Die Arbeit ist klein und einarmig, taugt also nicht als allgemeine Empfehlung. Als Warnung vor der reflexhaften Gleichung «Verstopfung, also mehr Ballaststoffe» taugt sie sehr wohl.")]),
    P(&[B("Zum Eisen im Spinat, weil die Blutarmut mitspielt."), T(" Der bekannteste Ernährungsirrtum überhaupt – nur stimmt auch seine Widerlegung nicht ganz. Eine Untersuchung der ETH Zürich hat mit markiertem Eisen geprüft, ob die "), L("Oxalsäure des Spinats die Eisenaufnahme hemmt", "https://pubmed.ncbi.nlm.nih.gov/17440529/"), T(": Sie tut es nicht. Kalium-Oxalat zu einer Grünkohlmahlzeit änderte an der Aufnahme nichts (11,5 gegenüber 10,7 Prozent), und die Aufnahme aus der Spinatmahlzeit lag zwar 24 Prozent unter der aus Grünkohl, aber nicht statistisch gesichert; der Unterschied hing eher an Kalzium und Polyphenolen. Spinat ist also nicht der Eisenblocker, für den er gilt. Er ist nur auch keine Antwort auf einen Blutverlust dieser Grössenordnung – was hier fehlt, holt kein Gemüse auf.")]),
    P(&[B("Und was sonst belegt ist."), T(" Die brauchbarste Vergleichsstudie stellte an 79 Menschen mit chronischer Verstopfung drei Hausmittel gegeneinander: zwei "), L("grüne Kiwis am Tag, 100 g Backpflaumen oder 12 g Flohsamen", "https://doi.org/10.14309/ajg.0000000000001149"), T(" über vier Wochen. In der Wirkung nahmen sich die drei nichts – alle drei erhöhten die Zahl der vollständigen spontanen Stuhlgänge. Der Unterschied lag bei den Nebenwirkungen: Die Kiwi verursachte weniger Schmerzen, Krämpfe und "), B("Blähungen"), T(" als die beiden anderen. Für eine Patientin, deren Hauptbeschwerde genau die Blähung ist, ist das der springende Punkt der ganzen Studie. Einschränkung: Die Teilnehmenden waren im Mittel 43 Jahre alt, hatten eine gewöhnliche Verstopfung und keine fehlende Peristaltik. Übertragbar ist die Rangfolge der Verträglichkeit, nicht die Zusage einer Wirkung.")]),
    P(&[B("Magnesium: hier Vorsicht statt Empfehlung."), T(" Magnesiumhaltige Mittel führen ab, und Magnesium steht in diesem Blatt ohnehin dreimal auf der Liste. Trotzdem ist es hier nicht das Naheliegende: Bei älteren Menschen und eingeschränkter Nierenfunktion droht eine "), L("Überladung mit Magnesium", "https://pmc.ncbi.nlm.nih.gov/articles/PMC6373027/"), T("; als Risikofaktoren gelten genau die eingeschränkte Nierenfunktion und die höhere Dosis, und eine der Wirksamkeitsstudien schloss über 75-Jährige und Nierenkranke von vornherein aus. Bei einer 84-Jährigen mit Blut im Urin und gestautem Harnleiter gehört das nicht ohne Nierenwerte und nicht ohne Rücksprache gegeben. Der Unterschied ist wichtig: Ein Magnesiummangel gehört ausgeglichen – eine abführende Magnesiumdosis ist etwas anderes.")]),
    P(&[B("Die Reihenfolge, die daraus folgt."), T(" Solange die Peristaltik fehlt, ist das Essen nicht das Werkzeug. Zuerst kommen die drei Blutwerte, die den Transport erklären könnten und die sich beheben lassen – Kalium, Magnesium, TSH – und die Frage, ob die Vaskulitis selbst behandelt wird. Danach ist die Frage nach dem Gemüse wieder eine gute Frage, und dann steht die Kiwi wegen ihrer Verträglichkeit vorn.")]),

    H3("Flohsamenschalen: gut belegt – und trotzdem nicht ihr Werkzeug"),
    P(&[B("Am 2. September empfahl ein Zeitungsartikel, "), L("Flohsamenschalen in den Joghurt zu rühren", "https://www.tagesanzeiger.ch/verdauung-flohsamenschalen-fuer-ein-gesundes-mikrobiom-211911241787"), T("; die Packung steht bei der Migros zwischen den Frühstücksflocken – "), L("Bio-Flohsamenschalen", "https://www.migros.ch/de/product/104236100000"), T(", "), N("110 g"), T(" für 3.50 Franken. "), B("Die Beweislage dafür ist gut, besser als die der meisten Hausmittel, und sie gehört zuerst genannt."), T(" Eine Meta-Analyse von "), N("16 Studien"), T(" mit "), N("1251 Menschen"), T(" fand bei chronischer Verstopfung "), L("66 Prozent Ansprechen gegenüber 41 Prozent", "https://pubmed.ncbi.nlm.nih.gov/35816465/"), T(" unter Scheinbehandlung – und Flohsamen war einer von nur zwei Ballaststoffen mit gesichertem Effekt. Beim Cholesterin senkten "), N("28 Studien"), T(" zusammengefasst das "), L("LDL um 0,33 mmol/l", "https://pubmed.ncbi.nlm.nih.gov/30239559/"), T(", beim Typ-2-Diabetes sanken Nüchternzucker und "), L("HbA1c um 0,97 Prozent", "https://pubmed.ncbi.nlm.nih.gov/26561625/"), T(". Das ist keine Modeempfehlung.")]),
    P(&[B("Nur sprechen drei Zahlen aus genau diesen Arbeiten gegen die Anwendung bei ihr."), T(" Es braucht dafür keine Warnung und keinen Einzelfallbericht – es steht in der Wirksamkeitsliteratur selbst:")]),
    Liste(&[
        &[B("Die Wirkung zeigte sich erst ab "), N("10 g"), B(" täglich und ab vier Wochen."), T(" Der Artikel empfiehlt ein bis zwei Teelöffel, rund "), N("5 g"), T(" – die halbe Dosis. Ein Esslöffel der Migros-Packung enthält "), N("3,5 g"), T(" Ballaststoff; wirksam wäre etwa das Dreifache dessen, was der Artikel vorschlägt. Und mit der Menge steigt die Pflicht zur Flüssigkeit.")],
        &[B("Der grösste Einzeleffekt derselben Meta-Analyse war nicht der Nutzen, sondern die Blähung."), T(" Sie fiel stärker aus als der Effekt auf die Stuhlfrequenz. Ihre Hauptbeschwerde ist genau die Blähung – das ist die entscheidungsrelevanteste Zahl von allen, und sie stammt aus der Arbeit, die für Flohsamen spricht.")],
        &[B("Sie bekommt bereits das besser Belegte."), T(" Die Übersicht über "), L("41 Studien zu rezeptfreien Mitteln", "https://pubmed.ncbi.nlm.nih.gov/33767108/"), T(" gibt Macrogol und Senna die Note A und Flohsamen die Note B; die systematische Übersicht zu "), L("Betagten über 65", "https://pubmed.ncbi.nlm.nih.gov/34642269/"), T(" kommt zum selben Ergebnis und nennt Macrogol als das Mittel für den längeren Gebrauch. Flohsamen dazuzugeben hiesse, das schwächer Belegte auf das besser Belegte zu setzen.")],
    ]),
    P(&[B("Dazu kommt das eine, was nicht aus Studien stammt, sondern aus ihr."), T(" Die Fachinformation beschreibt den Wirkweg als "), L("reflektorisch über einen Dehnungsreiz", "https://ch.oddb.org/de/gcc/fachinfo/reg/42933/chapter/effects"), T(": Die Schale bindet Wasser und quillt – das ist Physik und geschieht immer –, aber dass die Dehnung eine Vorwärtsbewegung auslöst, ist der einzige Schritt der Kette, der lebende Nerven und Muskeln braucht. Genau dieser Schritt hat bei ihr gestanden, und warum, ist nicht geklärt. "), B("Praktisch sind es zwei Sätze:"), T(" Die "), L("mindestens 150 ml Flüssigkeit je Einnahme", "https://ch.oddb.org/de/gcc/fachinfo/reg/42933/chapter/restrictions"), T(" sind erfüllbar, seit sie zuverlässig trinkt; nicht aber, solange sie erbricht. Damit steht nur noch eine Bedingung offen, und es ist die entscheidende: Sobald geklärt ist, warum die Passage gestanden hat, ist die Antwort ohne Vorbehalt ja – und unter den Hausmitteln steht dann immer noch die Kiwi vorn, aus dem Grund, der im Abschnitt zum Spinat steht.")]),

    H2("Interaktionscheck"),
    P(&[B("Dieser Abschnitt beruht auf dem falschen Warenkorb, und das gehört vorangestellt."), T(" Der folgende Lauf wurde mit den Mitteln gerechnet, von denen dieses Blatt annahm, dass sie laufen – Novalgin, Pantoprazol, zwei Abführmittel und niedrig dosiertes Aspirin. Die tatsächliche Liste aus dem Zwischenbericht enthält davon nur Pantoprazol. Der Lauf mit den wirklich verordneten Mitteln steht im Abschnitt «Die Medikamentenliste». "), B("Dieser hier bleibt trotzdem stehen,"), T(" weil seine Frage nicht erledigt ist: Welches Schmerzmittel und welches Abführmittel für sie taugen, wird spätestens beim Austritt wieder gebraucht – und weil sein wichtigster Fund sich inzwischen bestätigt hat.")]),
    P(&[B("Dieser Fund steht unten unter «Ein Fund am Rande»:"), T(" «Opioide verstopfen. Bei einer Patientin, die seit Wochen keinen Stuhlgang hat, ist ein opioidhaltiges Schmerzmittel das Falsche.» Er entstand aus einem Fehlalarm des Programms, als es «Paracetamol» einem Kombipräparat mit Codein zuordnete. Der Satz stand also da, bevor bekannt war, dass auf ihrer Liste "), B("zwei Opioide"), T(" stehen. Das ist kein Verdienst – nur der Beleg dafür, dass die Frage nach dem Schmerzmittel bei diesem Darm die richtige war.")]),
    P(&[T("Geprüft mit "), L("SDIF", "https://sdif.oddb.org"), T(", dem Swiss Drug Interaction Finder: Er wertet die Interaktionsangaben aus den Schweizer Fachinformationen aus und gleicht sie mit der EPha-Datenbank ab, die jede Kombination von A bis X einstuft – A keine Massnahmen, C regelmässige Überwachung, D Kombination vermeiden, X kontraindiziert. "), L("Der Lauf vom 29. August 2026", "https://sdif.oddb.org/?tab=check&drugs=N02BB02-A02BC02-A06AD65-A06AD11-B01AC06"), T(" mit Novalgin, Pantoprazol, Macrogol, Lactulose und niedrig dosiertem Aspirin ergab Folgendes. Kortison ist aus dem Warenkorb genommen, weil sie keines bekommt; damit entfallen die beiden Treffer, die daran hingen. Das Werkzeug und der Lauf selbst, mit fertig gefülltem Warenkorb, stehen hier – der zweite Link setzt den Korb über die ATC-Codes zusammen, weshalb für die Macrogol-Klasse ein anderes Präparat derselben Klasse angezeigt werden kann.")]),
    P(&[
        B("Der Vorbehalt zuerst:"), T(" Geprüft ist nur, was auf diesem Blatt steht. Die vollständige Medikamentenliste kennt nur die Patientin selbst, und sie gehört zum Termin mitgebracht – rezeptfreie Mittel eingeschlossen. Ein maschineller Check kann nur vergleichen, was man ihm gibt."),
    ]),
    Tab(&T_INTERAKTION),
    H3("Was mit dem Kortison weggefallen ist"),
    P(&[T("Der frühere Lauf, der Kortison enthielt, meldete zwei zusätzliche Treffer: Aspirin und Kortison als Klasse C – erhöhtes Blutungsrisiko im Magen-Darm-Trakt durch additive Schädigung der Schleimhaut, mit ausdrücklich steigendem Risiko im höheren Lebensalter, Massnahme «vorbeugende Gabe eines Protonenpumpenhemmers» – und Lactulose mit Kortison wegen des Kaliumverlusts. Beide sind gegenstandslos. Sie stehen hier trotzdem, weil sie wieder gelten, sobald Kortison verordnet würde: Dann wäre der Magenschutz nicht mehr nur wegen der Magenentzündung angezeigt, sondern zusätzlich als Begleitschutz.")]),
    H3("Ohne Treffer"),
    P(&[T("Novalgin mit Macrogol, mit Lactulose, mit Paracetamol und mit einem ACE-Hemmer: kein Treffer. Und Pantoprazol, jetzt selbst im Warenkorb, hat mit keinem der übrigen Mittel einen – weder mit Novalgin noch mit Macrogol oder Lactulose meldet die Fachinformation oder EPha etwas. Macrogol hat in der EPha-Datenbank überhaupt keinen Eintrag – es wird nicht aufgenommen und interagiert praktisch nicht. Auch das spricht für Movicol neutral.")]),
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
    P(&[T("Eine eigene Sprechstunde für die Purpura Schönlein-Henoch gibt es nicht. Zuständig waren bis zum 3. September zwei Fächer gemeinsam – Rheumatologie für die Vaskulitis, Nephrologie für die Niere. Mit der Diagnose vom 3. September steht ein drittes voran: die gynäkologische Onkologie. Bei einer Darmblutung oder einer Passagestörung kommt die Gastroenterologie dazu.")]),
    P(&[B("Seit der Verlegung sind diese Adressen im selben Haus."), T(" Was hier als Weg über eine Anmeldung beschrieben ist, gilt für den ambulanten Fall; für eine stationäre Patientin im Universitätsspital ist es ein Konsil, das die behandelnde Abteilung anfordert. Die Angaben bleiben trotzdem stehen – für die Zeit nach dem Austritt, in der die Kontrollen weiterlaufen müssen, und weil sie sagen, wen es im Haus überhaupt gibt.")]),
    H3("Gynäkologische Onkologie"),
    Adresse {
        name: "Gynäko-onkologische Sprechstunde, Klinik für Gynäkologie, Universitätsspital Zürich",
        rolle: &[T("Diagnose und Behandlung von Eierstock-, Gebärmutter-, Gebärmutterhals-, Vaginal- und Vulvakrebs; hier tagt das gynäkologische Tumorboard")],
        zeilen: &[
            &[T("Klinik für Gynäkologie, Frauenklinikstrasse 10, 8091 Zürich")],
        ],
        links: &[
            Verweis { text: "Ambulatorium Gynäkologie: +41 44 255 50 36", url: "tel:+41442555036" },
            Verweis { text: "gyn.ambulatorium@usz.ch", url: "mailto:gyn.ambulatorium@usz.ch" },
            Verweis { text: "usz.ch/fachbereich/gynaekologie", url: "https://www.usz.ch/fachbereich/gynaekologie/" },
        ],
    },
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
    P(&[T("Diese Liste ist mit der Verlegung zur Übergabeliste geworden. Sie ist nicht als Misstrauen gedacht: Vieles davon geht ohnehin mit. Aber der Abschnitt «Die Verlegung» nennt die Zahl, um die es geht – im Mittel war die Übergabedokumentation zu 58,3 Prozent vollständig –, und die Punkte, die hier fehlen, sind erfahrungsgemäss die, nach denen im neuen Haus zuerst niemand fragt.")]),
    Liste(&[
        &[T("Den Verlegungsbericht im Wortlaut, mit dem Namen der Ärztin oder des Arztes, die den Fall im abgebenden Haus geführt haben – für Rückfragen, die sich erst später stellen")],
        &[T("Seit wann der Blasenkatheter liegt, warum er gelegt wurde, und wie der Urin davor aussah")],
        &[T("Die Ein- und Ausfuhrbilanz seit Beginn der Infusion, mit dem täglichen Gewicht")],
        &[T("Wann der Kostaufbau begonnen hat und womit – und ob Thiamin gegeben wurde: wann, wie viel, auf welchem Weg")],
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

    H2("Die zehn Fragen, auf die es jetzt ankommt"),
    P(&[T("Diese Liste ist mit dem Zwischenbericht neu geordnet worden. Drei der bisherigen acht Fragen sind darin beantwortet – der Harnleiter ist entlastet, ein Blutzerfall ist ausgeschlossen, der frühere Schub ist datiert. Dafür sind andere dazugekommen, und die erste von ihnen war vorher gar nicht auf dem Blatt. Am 2. September ist die zweite hinzugetreten, und sie ist die einzige mit einer Frist von Stunden: Untersuchte Flüssigkeit lässt sich aufheben, ununtersuchte wird verworfen. Am 3. September ist die Diagnose gekommen, und sie ordnet die Liste ein drittes Mal: Die bisherige zehnte Frage – ob die Behandlung der Vaskulitis neu beurteilt wird – ist damit beantwortet und in der ersten aufgegangen; dafür ist die Frage nach den Beinvenen neu. Wenn im Zimmer nur fünf Minuten bleiben, sind es diese zehn.")]),
    Liste(&[
        &[B("1. Steht die Diagnose auf einem Bild, auf Zellen oder auf Gewebe – welcher Typ, welches Stadium, und wann tagt das Tumorboard?"), T(" Am 3. September heisst es: Eierstockkarzinom. Die Stufe der Sicherheit entscheidet über alles Weitere: Nach Ultraschall ist es eine begründete Annahme, nach Zytologie mit PAX8 eine Herkunftsbestimmung, nach Gewebeprobe eine Diagnose mit Typ und Grad – und nur am Gewebe lässt sich der Reparaturdefekt bestimmen, von dem die Erhaltungstherapie abhängt. Dazu gehören weiterhin die Kategorie im zytologischen Befund, der Zellblock und das Albumin im Blut vom selben Tag. Und die Frage, die mit der Diagnose an die Stelle der alten zehnten tritt: "), B("Ist ein geriatrisches Assessment gemacht worden"), T(" – denn nicht das Alter entscheidet über die Behandlung, sondern die Gebrechlichkeit, und der Allgemeinzustand nach Augenmass sagt sie schlecht voraus. Siehe den Abschnitt «Die Diagnose vom 3. September».")],
        &[B("2. Ist die Flüssigkeit aus der linken Lunge vollständig untersucht worden?"), T(" Am 2. September wurde links knapp ein Liter abgezogen. Dafür gibt es eine Standardanforderung – Zellzahl mit Differenzierung, Gesamteiweiss, LDH, pH, Gram-Färbung, Kultur und "), B("Zytologie"), T(" –, dazu bei ihr das Albumin für den Serum-Erguss-Gradienten und einen "), B("Zellblock"), T(" für die Immunfärbungen. Diese Frage hat als einzige eine Frist von Stunden: Was nicht angefordert ist, bevor die Flüssigkeit verworfen wird, kostet einen zweiten Eingriff. Siehe den Abschnitt «Ein Liter von der linken Lunge».")],
        &[B("3. Sind die Beinvenen mit Ultraschall untersucht worden?"), T(" Die Beine sind weiter geschwollen. Bisher standen dafür sieben Erklärungen im Blatt, alle vom Wasser her. Das Eierstockkarzinom gehört zu den Krebsarten mit dem höchsten Thromboserisiko, und sie liegt seit Tagen. Der Ultraschall ist schnell, ohne Eingriff und entscheidet, ob eine Blutverdünnung überhaupt zur Frage wird – eine Frage, die bei einer Frau, die vor Tagen schwarz erbrochen hat, schwer genug ist, um sie bewusst zu treffen statt zu übersehen.")],
        &[B("4. Welche Reservemedikamente wurden tatsächlich gegeben, wie oft und in welcher Dosis?"), T(" Es geht um die beiden Opioide, um Metoclopramid, Haloperidol und um Dexamethason. Daran hängen drei Dinge: die wahrscheinlichste behebbare Ursache des Darmstillstands, die Frage nach dem Kortikoid, die dieses Blatt an einem Dutzend Stellen berührt, und die Rhythmusfrage bei einer 84-Jährigen. Eine einzige Auskunft aus dem Verabreichungsprotokoll. Und dazu, seit dem 2. September: "), B("Welcher Blutdrucksenker läuft, seit wann, in welcher Dosis und über welchen Weg?"), T(" Er steht auf keiner Liste, und ob es ein Kalziumantagonist, ein ACE-Hemmer oder ein Entwässerungsmittel ist, führt zu drei verschiedenen Folgefragen.")],
        &[B("5. Ist der Darmstillstand unter fortlaufenden Opioiden neu beurteilt worden – und läuft die vorbeugende Abführbehandlung?"), T(" Die Fachinformation von Oxycodon nennt Obstipation bei 30,5 Prozent, führt Ileus als unerwünschte Wirkung, nennt den paralytischen Ileus als Gegenanzeige und verlangt Abführmittel ab dem ersten Behandlungstag. Der geblähte Darm ist das, was die Patientin am meisten belastet.")],
        &[B("6. Ist eine Stuhlprobe auf Clostridioides difficile eingeschickt?"), T(" Antibiotikum während zweier Tage vor dem Eintritt – es läuft nicht mehr, aber gezählt wird die Einnahme in den letzten dreissig Tagen –, dazu 84 Jahre, Spitalaufenthalt und Säureblocker: die Risikoliste ist erfüllt, und schwere Verläufe zeigen sich als Bauchauftreibung und Darmlähmung statt als Durchfall. Untersucht wird nur flüssiger Stuhl, und flüssig ist er jetzt.")],
        &[B("7. Kommt das Blut im Urin aus der Niere oder aus den Harnwegen?"), T(" Die glomerulären Erythrozyten sind angefordert und stehen aus; dazu gehören Akanthozyten, Erythrozytenzylinder und der Protein-Kreatinin-Quotient aus derselben Probe. Und die Zeitfrage dazu: Kam der braune Urin vor oder nach der Einlage der Ableitungen am 27. August?")],
        &[B("8. Wie hat sich das Kreatinin seit der Entlastung entwickelt, und wie viel Urin kommt?"), T(" Die Ableitungen liegen seit dem 27. August; wenn der Stau die Ursache war, muss das Kreatinin fallen. Seit dem 3. September hat der Stau eine wahrscheinliche Ursache – ein Tumor im kleinen Becken drückt auf den Harnleiter –, und damit hängt die Nierenfunktion direkt an der Tumorbehandlung: Die Carboplatin-Dosis wird aus ihr berechnet. Am 30. August bestand trotz Rehydrierung eine Oligurie. Bei einem Trockengewicht von "), N("58 kg"), T(" ist die Schwelle "), N("29 ml"), T(" Urin je Stunde – darunter ist es für sich allein ein akutes Nierenversagen. Und dazu die Zahl, die alles davon einordnet: "), B("die tägliche Gewichtskurve"), T(". Sie liegt bei "), N("66 kg"), T(" gegenüber sonst "), N("58 kg"), T("; das sind acht Liter, und es sind sechs seit dem Eintritt. "), B("Und daran hängt die Frage nach der Zufuhr:"), T(" Seit sie zuverlässig trinkt, ist die Begründung für die "), N("1500 ml"), T(" Kochsalzlösung entfallen – sie war für eine Patientin gedacht, die ausgetrocknet war, nicht trank und alles erbrach. Anderthalb Liter und gut "), N("13 g"), T(" Kochsalz am Tag gehen weiter in genau das Kompartiment, das schon acht Liter zu viel hat. Das Abstellen oder Reduzieren der Infusion ist der einzige Hebel, der keine Diagnose voraussetzt, sofort wirkt und sich jederzeit zurücknehmen lässt. Zwei Dinge entscheiden es, und beide kennt nur die Station: ob über denselben Zugang Medikamente laufen, und ob sie trotz der Ödeme im Gefäss zu wenig Volumen hat – bei tiefem Albumin ist beides gleichzeitig möglich. Deshalb ist es eine Frage und keine Forderung.")],
        &[B("9. Was steht in den ausstehenden Berichten der beiden Spiegelungen?"), T(" Wie weit kam das Gerät nach unten, wurde im absteigenden Zwölffingerdarm Gewebe entnommen, und was zeigen die Proben? Die Vaskulitis sitzt dort in 55,7 Prozent der Fälle und im Magen selbst in 3,3. Der Dickdarm war schlecht vorbereitet und ist damit nicht ausgeschlossen, sondern unzureichend eingesehen.")],
        &[B("10. Wie hoch sind Kalium, Magnesium und Kreatinin heute – und ist Thiamin gegeben worden, bevor der Kostaufbau begann?"), T(" Seit dem 5. September werden Kalium und Magnesium ersetzt; das ist richtig und verlangt zugleich die Zahl, denn beide Präparate tragen bei einer GFR von 30 eine Gegenanzeige an der Grenze, und die Infusionslösung liefert von beidem noch etwas dazu. Bei einem Trockengewicht von "), N("58 kg"), T(" heisst langsam beginnen "), N("580 bis 1160 kcal"), T(" am ersten Tag – gerechnet wird auf das Trockengewicht und nicht auf die "), N("66 kg"), T(" auf der Waage. Thiamin gehört vor die erste Kohlenhydratgabe, und bei einem Magen, der sich nicht entleert, nicht als Tablette – siehe das Begleitblatt zum Kostaufbau. Das Bikarbonat von 17 zeigt, dass der Säure-Basen-Haushalt ohnehin verschoben ist.")],
    ]),

    H2("Die vollständige Liste"),
    Liste(&[
        &[T("Kalium, Magnesium, Kreatinin vom 5. September – und wie oft werden sie unter der Substitution kontrolliert?")],
        &[T("Ist der Blutdrucksenker ein ACE-Hemmer (Lisinopril)? Zusammen mit Kaliumchlorid ist das EPha-Klasse C – Hyperkaliämie.")],
        &[T("Wie viel Ringer-Acetat läuft am Tag, und warum weiter eine Infusion, wo sie zuverlässig trinkt und die Lösung Ödeme als Gegenanzeige nennt?")],
        &[T("Die Bestätigung des Karzinoms vom 5. September: aus Zellen oder aus Gewebe?")],
        &[T("Wie ist die Diagnose Eierstockkarzinom gestellt worden – Ultraschall, Zytologie mit PAX8 oder Gewebeprobe? Welcher Typ, welcher Grad?")],
        &[T("Wie hoch ist das CA-125, und ist eine Bestimmung des Reparaturdefekts (BRCA, HRD) am Gewebe geplant?")],
        &[T("Wann tagt das Tumorboard, ist die Geriatrie beigezogen, und wird ein formales geriatrisches Assessment gemacht?")],
        &[T("Sind die Beinvenen mit Ultraschall auf eine Thrombose untersucht worden?")],
        &[T("Welchen Blutdrucksenker nimmt sie – Amlodipin, Lisinopril oder Torasemid? Einer davon macht selbst Beinödeme, einer entwässert.")],
        &[T("Ist die Geruchsempfindlichkeit auf dem Verlaufsblatt notiert und der Ernährungsberatung bekannt?")],
        &[T("Ist die Flüssigkeit aus der linken Brustfellhöhle zur Zytologie geschickt worden, und ist daraus ein Zellblock angelegt?")],
        &[T("Wie lauten Gesamteiweiss, LDH und pH im Erguss und im Blut – Transsudat oder Exsudat nach den Light-Kriterien?")],
        &[T("Läuft ein Entwässerungsmittel? Dann gilt statt der Light-Kriterien der Serum-Erguss-Albumin-Gradient.")],
        &[T("Was zeigte der Ultraschall am 2. September auf der rechten Seite – hat sich der rechte Erguss zurückgebildet, oder war er nur zu klein zum Punktieren?")],
        &[T("Wie viel Sauerstoff braucht sie nach der Punktion noch, und wie hoch ist die Sättigung ohne?")],
        &[T("Werden am Krankenbett Hausmittel gegeben – Flohsamenschalen, Kleie, Leinsamen –, und weiss die Station davon?")],
        &[T("Aus welchem Material stammen die «pathologischen Zellen» – aus dem Bauchwasser vom 30. August oder aus den Gewebeproben vom 28.?")],
        &[T("Wie lautet die Kategorie im zytologischen Befund wörtlich – Atypie unklarer Bedeutung, verdächtig oder bösartig?")],
        &[T("Ist ein Zellblock angelegt worden, und welche Immunfärbungen sind gelaufen – ist PAX8 dabei?")],
        &[T("Das Gewicht liegt bei 66 kg gegenüber sonst 58: Wird täglich gewogen, und wie sieht die Ein- und Ausfuhrbilanz dazu aus?")],
        &[T("Werden die 1500 ml Kochsalzlösung am Tag noch gebraucht, jetzt wo sie zuverlässig trinkt und sechs Kilo zugelegt hat? Das sind 1,5 Liter und gut 13 g Kochsalz täglich in das Kompartiment, das schon acht Liter zu viel hat.")],
        &[T("Welcher Blutdrucksenker läuft, seit wann, in welcher Dosis, und über welchen Weg?")],
        &[T("Kommt jetzt alles wieder heraus oder nur das Birchermüesli – bleibt gekochter Haferbrei drin?")],
        &[T("Das Erbrochene ist gelb statt schwarz: Ist das dokumentiert, und ändert es etwas an der Dringlichkeit der Magenspiegelung?")],
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
        &[T("Sie stösst Luft auf bis zum Würgen, und der Wind geht nur schwer ab: Gibt es etwas, das den Transport unterstützt, und verträgt es sich mit den übrigen Mitteln?")],
        &[T("Der geblähte Darm ist das, was sie am meisten belastet – was ist dagegen vorgesehen, solange die Peristaltik nicht in Gang kommt?")],
        &[T("Spinat hat früher geholfen: Sind Ballaststoffe bei fehlender Peristaltik jetzt sinnvoll oder eher schädlich?")],
        &[T("Poulet kam vollständig wieder heraus: Wird der Kostaufbau von der Ernährungsberatung begleitet, und in welcher Konsistenz – und wie wird der Eiweissbedarf trotzdem gedeckt?")],
        &[T("Nach dem Erbrechen: Sind Kalium und Chlorid nachkontrolliert worden?")],
        &[T("Nach Wochen ohne Nahrung: Ist Thiamin gegeben worden, bevor der Kostaufbau begann – und wird das Phosphat mitkontrolliert?")],
        &[T("Mit welcher Kalorienmenge hat der Kostaufbau begonnen, und in welchem Tempo wird gesteigert?")],
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
        &[T("Falls Aspirin zum Herzschutz läuft: Wird es 30 bis 60 Minuten vor Novalgin gegeben – und wäre bei einer blutenden Magenentzündung nicht zu prüfen, ob es überhaupt weiterlaufen soll?")],
        &[T("Sie hat nie Kortison bekommen, auch beim früheren Schub nicht: Wurde bewusst dagegen entschieden, und wogegen wurde abgewogen – Blutung, Infektrisiko, Alter?")],
        &[T("Wird die Kortisonfrage neu beurteilt, sobald die Blutungsquelle gefunden und versorgt ist?")],
        &[T("Kann der Kreatininwert durch Metamizol verfälscht sein – und wie wird die Nierenfunktion dann beurteilt?")],
        &[T("Wofür ist das Abführmittel verordnet – gegen Verstopfung oder als Vorbereitung der Darmspiegelung? Und ist es in dieser Lage überhaupt zulässig?")],
        &[T("Kann statt eines aromatisierten Präparats Movicol neutral oder Laxipeg aromafrei verschrieben werden – ohne Aroma und ohne Süssstoff?")],
        &[T("Jetzt, wo die Passage wieder offen ist: Ist die Magen- und Darmspiegelung angesetzt, und wann?")],
        &[T("Die Magenentzündung ist festgestellt – woher kommt sie? Von der Vaskulitis, von Aspirin und Novalgin, oder von Helicobacter pylori?")],
        &[T("Wurde bei der Spiegelung bis in den absteigenden Zwölffingerdarm geschaut und dort biopsiert? Dort sitzt die Vaskulitis, im Magen selbst nur selten.")],
        &[T("Ist auf Helicobacter pylori getestet worden, und mit welchem Ergebnis?")],
        &[T("Die Fachinformation von Pantoprazol verlangt bei Gewichtsverlust, wiederholtem Erbrechen und Blutarmut den Ausschluss einer bösartigen Erkrankung, weil das Mittel die Symptome kaschieren kann – ist das vorgesehen?")],
        &[T("Wird Pantoprazol eine Stunde vor dem Frühstück eingenommen, mit Wasser und nicht mit dem Tee?")],
        &[T("Wie lange soll Pantoprazol laufen, und woran wird entschieden, wann es aufhört?")],
        &[T("Falls Eisen verordnet wird: Über den Mund oder in die Vene? Unter einem Protonenpumpenhemmer ist die Aufnahme über den Mund unsicher.")],
        &[T("Wie hoch ist der Eiweissverlust im Urin, gemessen als Protein-Kreatinin-Quotient?")],
        &[T("Wurde ANCA bestimmt, also die im Alter häufigere Vaskulitisform ausgeschlossen?")],
        &[T("Der Ausschlag geht zurück: Wird die Hautbiopsie jetzt gemacht, solange noch frische Flecken da sind?")],
        &[T("Ist bei diesem Verlauf eine Nierenbiopsie angezeigt? Die KDIGO-Leitlinie nennt dafür erhebliche Organschädigung, eine Eiweissausscheidung ab 0,5 g/Tag über vier Wochen oder eine eingeschränkte Nierenfunktion.")],
        &[T("Wie hat sich das Kreatinin über die letzten Tage entwickelt? Daran hängt, ob eine rasch fortschreitende Glomerulonephritis vorliegt – und die würde anders behandelt.")],
        &[T("Kommen die roten Blutkörperchen aus dem entzündeten Filter, oder verstopfen sie als Zylinder die Nierenkanälchen? Die Behandlung ist eine andere, und nur die Biopsie trennt das.")],
        &[T("Wie ist der Blutdruck eingestellt, und ist ein RAS-Hemmer vorgesehen? Die Leitlinie nennt als Ziel 120/70 mmHg.")],
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
    ("Refeeding Syndrome. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK564513/", url: "https://www.ncbi.nlm.nih.gov/books/NBK564513/" }),
    ("Nutrition support for adults. NICE Clinical Guideline CG32",
     Verweis { text: "https://www.nice.org.uk/guidance/cg32", url: "https://www.nice.org.uk/guidance/cg32" }),
    ("Olausson EA et al.: A small particle size diet reduces upper gastrointestinal symptoms in patients with diabetic gastroparesis. Am J Gastroenterol 2014; 109: 375-85. PMID 24419482",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/24419482/", url: "https://pubmed.ncbi.nlm.nih.gov/24419482/" }),
    ("Camilleri M et al.: ACG Clinical Guideline – Gastroparesis. Am J Gastroenterol 2022; 117: 1197-1220. PMID 35926490",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/35926490/", url: "https://pubmed.ncbi.nlm.nih.gov/35926490/" }),
    ("Euthyroid Sick Syndrome. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK482219/", url: "https://www.ncbi.nlm.nih.gov/books/NBK482219/" }),
    ("Ho KS et al.: Stopping or reducing dietary fiber intake reduces constipation and its associated symptoms. World J Gastroenterol 2012; 18: 4593-6. PMID 22969234",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/22969234/", url: "https://pubmed.ncbi.nlm.nih.gov/22969234/" }),
    ("Storcksdieck genannt Bonsmann S et al.: Oxalic acid does not influence nonhaem iron absorption in humans. ETH Zürich. Eur J Clin Nutr 2008; 62: 336-41. PMID 17440529",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/17440529/", url: "https://pubmed.ncbi.nlm.nih.gov/17440529/" }),
    ("Agiolax mite (Plantago ovata, Flohsamen), vollständige Fachinformation. Swissmedic 42933",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/42933", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/42933" }),
    ("Geisler A: «Esst nicht dauernd zwischendurch etwas»: Was eine Magen-Darm-Expertin ihren Freunden rät. Tages-Anzeiger, 2. September 2026",
     Verweis { text: "tagesanzeiger.ch – Flohsamenschalen für ein gesundes Mikrobiom", url: "https://www.tagesanzeiger.ch/verdauung-flohsamenschalen-fuer-ein-gesundes-mikrobiom-211911241787" }),
    ("Bio-Flohsamenschalen 110 g, Migros-Genossenschafts-Bund. Zutaten und Nährwerte",
     Verweis { text: "migros.ch – Artikel 104236100000", url: "https://www.migros.ch/de/product/104236100000" }),
    ("van der Schoot A et al.: The Effect of Fiber Supplementation on Chronic Constipation in Adults. Am J Clin Nutr 2022; 116: 953-69. PMID 35816465",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/35816465/", url: "https://pubmed.ncbi.nlm.nih.gov/35816465/" }),
    ("Jovanovski E et al.: Effect of psyllium (Plantago ovata) fiber on LDL cholesterol. Am J Clin Nutr 2018; 108: 922-32. PMID 30239559",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/30239559/", url: "https://pubmed.ncbi.nlm.nih.gov/30239559/" }),
    ("Gibb RD et al.: Psyllium fiber improves glycemic control proportional to loss of glycemic control. Am J Clin Nutr 2015; 102: 1604-14. PMID 26561625",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/26561625/", url: "https://pubmed.ncbi.nlm.nih.gov/26561625/" }),
    ("Rao SSC, Brenner DM: Efficacy and Safety of Over-the-Counter Therapies for Chronic Constipation. Am J Gastroenterol 2021; 116: 1156-81. PMID 33767108",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/33767108/", url: "https://pubmed.ncbi.nlm.nih.gov/33767108/" }),
    ("Kang SJ et al.: Medical Management of Constipation in Elderly Patients: Systematic Review. J Neurogastroenterol Motil 2021; 27: 495-512. PMID 34642269",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/34642269/", url: "https://pubmed.ncbi.nlm.nih.gov/34642269/" }),
    ("Chey SW et al.: Exploratory Comparative Effectiveness Trial of Green Kiwifruit, Psyllium, or Prunes in US Patients With Chronic Constipation. Am J Gastroenterol 2021; 116: 1304-12",
     Verweis { text: "https://doi.org/10.14309/ajg.0000000000001149", url: "https://doi.org/10.14309/ajg.0000000000001149" }),
    ("Risk factors for the development of hypermagnesemia in patients prescribed magnesium oxide. PMC6373027",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC6373027/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC6373027/" }),
    ("Top 10 Takeaways from the KDIGO 2025 Clinical Practice Guideline for the Management of IgA Vasculitis",
     Verweis { text: "kdigo.org – Top 10 Takeaways IgAV 2025", url: "https://kdigo.org/wp-content/uploads/2024/08/KDIGO-2025-IgAV-Guideline-Key-Takeaways_IgAV.pdf" }),
    ("Sevillano AM et al.: Effect of Immunosuppressive Treatments on Kidney Outcomes After Gross Hematuria-Related AKI in Older Patients With IgA Nephropathy. Kidney Int Rep 2023; 8: 1596-1604. PMID 37547537",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/37547537/", url: "https://pubmed.ncbi.nlm.nih.gov/37547537/" }),
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
    ("Clostridioides difficile Infection. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK431054/", url: "https://www.ncbi.nlm.nih.gov/books/NBK431054/" }),
    ("The combined effect of systemic antibiotics and proton pump inhibitors on Clostridioides difficile infection and recurrence. Schweden, 43'152 Fälle. PMC10904719",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10904719/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10904719/" }),
    ("Arriola V et al.: Assessing the Risk of Hospital-Acquired Clostridium Difficile Infection With Proton Pump Inhibitor Use. Infect Control Hosp Epidemiol 2016; 37: 1408-17. PMID 27677811",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/27677811/", url: "https://pubmed.ncbi.nlm.nih.gov/27677811/" }),
    ("Proton pump inhibitors are not associated with an increased risk of Clostridioides difficile infection: Metaanalyse randomisierter Studien. PMC12502825",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12502825/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12502825/" }),
    ("Niesters M, Mahajan RP, Aarts L, Dahan A: High-inspired oxygen concentration further impairs opioid-induced respiratory depression. Br J Anaesth 2013; 110: 837-41. PMID 23293275",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/23293275/", url: "https://pubmed.ncbi.nlm.nih.gov/23293275/" }),
    ("Doufas AG et al.: Incidence of postoperative opioid-induced respiratory depression episodes in patients on room air or supplemental oxygen. BMC Anesthesiol 2023; 23: 332. PMID 37794334",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/37794334/", url: "https://pubmed.ncbi.nlm.nih.gov/37794334/" }),
    ("Metabolic Acidosis. StatPearls, NCBI Bookshelf – Winter-Formel",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK482146/", url: "https://www.ncbi.nlm.nih.gov/books/NBK482146/" }),
    ("IgA Nephropathy (Berger Disease). StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK538214/", url: "https://www.ncbi.nlm.nih.gov/books/NBK538214/" }),
    ("Restricted lung inflation as principal predictor of dyspnoea in unilateral pleural effusion. PMC6169850",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6169850/", url: "https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6169850/" }),
    ("Shen-Wagner J, Gamble C, MacGilvray P: Pleural Effusion: Diagnostic Approach in Adults. Am Fam Physician 2023; 108: 464-75. PMID 37983698",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/37983698/", url: "https://pubmed.ncbi.nlm.nih.gov/37983698/" }),
    ("Hu K et al.: Predictors of Dyspnea Relief Using Ultrasound Measurements of Diaphragmatic Function in Symptomatic Pleural Effusions: Multicenter Prospective Study. Ann Am Thorac Soc 2025; 22: 1674-9. PMID 40720164",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/40720164/", url: "https://pubmed.ncbi.nlm.nih.gov/40720164/" }),
    ("Roth BJ, O'Meara TF, Cragun WH: The serum-effusion albumin gradient in the evaluation of pleural effusions. Chest 1990; 98: 546-9. PMID 2152757",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/2152757/", url: "https://pubmed.ncbi.nlm.nih.gov/2152757/" }),
    ("Roussos A et al.: Hepatic hydrothorax: pathophysiology diagnosis and management. J Gastroenterol Hepatol 2007; 22: 1388-93. PMID 17645471",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/17645471/", url: "https://pubmed.ncbi.nlm.nih.gov/17645471/" }),
    ("Sandevski A et al.: Frequency and characteristics of pleural effusions in pulmonary embolism. Prilozi 2012; 33: 93-104. PMID 23425873",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/23425873/", url: "https://pubmed.ncbi.nlm.nih.gov/23425873/" }),
    ("Tetikkurt C et al.: The value of exfoliative cell cytology in the diagnosis of exudative pleural effusions. Monaldi Arch Chest Dis 2018; 88: 944. PMID 30203633",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/30203633/", url: "https://pubmed.ncbi.nlm.nih.gov/30203633/" }),
    ("Scott S et al.: Re-expansion Pulmonary Edema (REPE) Following Thoracentesis: Is Large-Volume Thoracentesis Associated with Increased Incidence of REPE? Cardiovasc Intervent Radiol 2024; 47: 912-7. PMID 38858252",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/38858252/", url: "https://pubmed.ncbi.nlm.nih.gov/38858252/" }),
    ("Zhang Y et al.: Predictors of inadequate bowel preparation in older patients undergoing colonoscopy. Int J Nurs Stud 2024; 149: 104631. PMID 37963423",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/37963423/", url: "https://pubmed.ncbi.nlm.nih.gov/37963423/" }),
    ("Factors affecting the quality of bowel preparation for colonoscopy in hard-to-prepare patients. World J Gastroenterol 2023; 29: 1685. PMC10107216",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10107216/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC10107216/" }),
    ("Hassan C et al.: Bowel preparation for colonoscopy. ESGE Guideline Update 2019. Endoscopy 2019; 51: 775-94",
     Verweis { text: "thieme-connect.com – ESGE Bowel preparation 2019", url: "https://www.thieme-connect.com/products/ejournals/html/10.1055/a-0959-0505" }),
    ("Dean M: Opioids in renal failure and dialysis patients. J Pain Symptom Manage 2004; 28: 497-504. PMID 15504625",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/15504625/", url: "https://pubmed.ncbi.nlm.nih.gov/15504625/" }),
    ("Ascites. StatPearls, NCBI Bookshelf – Serum-Aszites-Albumin-Gradient und Zellzahl",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK470482/", url: "https://www.ncbi.nlm.nih.gov/books/NBK470482/" }),
    ("Runyon BA: Malignancy-related ascites and ascitic fluid \"humoral tests of malignancy\". J Clin Gastroenterol 1994; 18: 94-8. PMID 8189030",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/8189030/", url: "https://pubmed.ncbi.nlm.nih.gov/8189030/" }),
    ("Small Bowel Obstruction. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK448079/", url: "https://www.ncbi.nlm.nih.gov/books/NBK448079/" }),
    ("OxyNorm (Oxycodon), vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/55352", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/55352" }),
    ("Morphin Sulfate Sintetica, vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/56400", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/56400" }),
    ("Ondansetron Sandoz, vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/67214", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/67214" }),
    ("Paspertin (Metoclopramid), vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/32733", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/32733" }),
    ("Dexamethason Zentiva, vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/41074", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/41074" }),
    ("Haldol (Haloperidol), vollständige Fachinformation",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/26891", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/26891" }),
    ("Usher MG et al.: Information handoff and outcomes of critically ill patients transferred between hospitals. J Crit Care 2016; 36: 240-5. PMC5096986",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC5096986/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC5096986/" }),
    ("Hydronephrosis and Hydroureter. StatPearls, NCBI Bookshelf",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK563217/", url: "https://www.ncbi.nlm.nih.gov/books/NBK563217/" }),
    ("Diagnosis, management, and prevention of catheter-associated urinary tract infections. Infect Dis Clin North Am 2014. PMC9580547",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9580547/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9580547/" }),
    ("Catheter-Associated Urinary Tract Infections: Current Challenges and Future Prospects. PMC8992741",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC8992741/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC8992741/" }),
    ("Hollingsworth JM et al.: Determining the noninfectious complications of indwelling urethral catheters. Ann Intern Med 2013; 159: 401-10, zusammengefasst in der DARE-Datenbank",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK159201/", url: "https://www.ncbi.nlm.nih.gov/books/NBK159201/" }),
    ("KDIGO Clinical Practice Guideline for Acute Kidney Injury 2012 – Definition über die Urinmenge",
     Verweis { text: "kdigo.org – KDIGO 2012 AKI Guideline", url: "https://kdigo.org/wp-content/uploads/2016/10/KDIGO-2012-AKI-Guideline-English.pdf" }),
    ("van Bree SH et al.: Identification of clinical outcome measures for recovery of gastrointestinal motility in postoperative ileus. Ann Surg 2014; 259: 708-14. PMID 23657087",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/23657087/", url: "https://pubmed.ncbi.nlm.nih.gov/23657087/" }),
    ("Eichelberger M, Joray ML, Perrig M, Bodmer M, Stanga Z: Management of patients during hunger strike and refeeding phase. Inselspital Bern. Nutrition 2014; 30: 1372-8. PMID 25280415",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/25280415/", url: "https://pubmed.ncbi.nlm.nih.gov/25280415/" }),
    ("Başoğlu M et al.: Neurological complications of prolonged hunger strike. Eur J Neurol 2006; 13: 1089-97. PMID 16987161",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/16987161/", url: "https://pubmed.ncbi.nlm.nih.gov/16987161/" }),
    ("Wagner P, Bui T: Wernicke-Korsakoff Syndrome in a Hunger Striker Despite Oral Thiamine Supplementation. Int Med Case Rep J 2022; 15: 399-403. PMC9359357",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9359357/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9359357/" }),
    ("KDIGO 2025 Clinical Practice Guideline for the Management of IgA Nephropathy and IgA Vasculitis. Kidney Int 2025",
     Verweis { text: "https://doi.org/10.1016/j.kint.2025.04.004", url: "https://doi.org/10.1016/j.kint.2025.04.004" }),
    ("Castilho da Silva DJ et al.: Risk of Malignancy in Effusions according to the International System for Serous Fluid Cytopathology. Acta Cytol 2024; 68: 384-93. PMID 39025059",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/39025059/", url: "https://pubmed.ncbi.nlm.nih.gov/39025059/" }),
    ("Ahuja S et al.: Diagnostic accuracy of the International System for Reporting Serous Fluid Cytopathology. Cancer Cytopathol 2024; 132: 609-20. PMID 38613789",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/38613789/", url: "https://pubmed.ncbi.nlm.nih.gov/38613789/" }),
    ("Tong GX et al.: Pax8 – a marker for carcinoma of Müllerian origin in serous effusions. Diagn Cytopathol 2011; 39: 567-74. PMID 20607683",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/20607683/", url: "https://pubmed.ncbi.nlm.nih.gov/20607683/" }),
    ("Strnad CM et al.: Peritoneal carcinomatosis of unknown primary site in women. Ann Intern Med 1989; 111: 213-7. PMID 2502058",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/2502058/", url: "https://pubmed.ncbi.nlm.nih.gov/2502058/" }),
    ("Pentheroudakis G, Pavlidis N: Serous papillary peritoneal carcinoma – a systematic review. Crit Rev Oncol Hematol 2010; 75: 27-42. PMID 19897383",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/19897383/", url: "https://pubmed.ncbi.nlm.nih.gov/19897383/" }),
    ("Ataseven B et al.: FIGO stage IV epithelial ovarian, fallopian tube and peritoneal cancer revisited. Gynecol Oncol 2016; 142: 597-607. PMID 27335253",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/27335253/", url: "https://pubmed.ncbi.nlm.nih.gov/27335253/" }),
    ("Porcel JM, Diaz JP, Chi DS: Clinical implications of pleural effusions in ovarian cancer. Respirology 2012; 17: 1060-7. PMID 22458298",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/22458298/", url: "https://pubmed.ncbi.nlm.nih.gov/22458298/" }),
    ("Faidley KN et al.: Longitudinal Outcomes of Malignant Ureteral Obstruction Secondary to Ovarian Cancer. Urology 2024; 186: 101-6. PMID 38350551",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/38350551/", url: "https://pubmed.ncbi.nlm.nih.gov/38350551/" }),
    ("Vergote I et al.: Neoadjuvant chemotherapy or primary surgery in stage IIIC or IV ovarian cancer (EORTC 55971). N Engl J Med 2010; 363: 943-53. PMID 20818904",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/20818904/", url: "https://pubmed.ncbi.nlm.nih.gov/20818904/" }),
    ("Kehoe S et al.: Primary chemotherapy versus primary surgery for newly diagnosed advanced ovarian cancer (CHORUS). Lancet 2015; 386: 249-57. PMID 26002111",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/26002111/", url: "https://pubmed.ncbi.nlm.nih.gov/26002111/" }),
    ("Falandry C et al.: Efficacy and Safety of First-line Single-Agent Carboplatin vs Carboplatin Plus Paclitaxel for Vulnerable Older Adult Women With Ovarian Cancer (EWOC-1). JAMA Oncol 2021; 7: 853-61. PMID 33885718",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/33885718/", url: "https://pubmed.ncbi.nlm.nih.gov/33885718/" }),
    ("Paraplatin (Carboplatin), Fachinformation. Swissmedic 47671",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/47671", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/47671" }),
    ("Burger RA et al.: Incorporation of bevacizumab in the primary treatment of ovarian cancer (GOG-0218). N Engl J Med 2011; 365: 2473-83. PMID 22204724",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/22204724/", url: "https://pubmed.ncbi.nlm.nih.gov/22204724/" }),
    ("Avastin (Bevacizumab), Fachinformation. Swissmedic 56922",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/56922", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/56922" }),
    ("González-Martín A et al.: Niraparib in Patients with Newly Diagnosed Advanced Ovarian Cancer (PRIMA). N Engl J Med 2019; 381: 2391-402. PMID 31562799",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/31562799/", url: "https://pubmed.ncbi.nlm.nih.gov/31562799/" }),
    ("Durno K, Powell ME: The role of radiotherapy in ovarian cancer. Int J Gynecol Cancer 2022; 32: 366-71. PMID 35256425",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/35256425/", url: "https://pubmed.ncbi.nlm.nih.gov/35256425/" }),
    ("Tew WP: Ovarian cancer in the older woman. J Geriatr Oncol 2016; 7: 354-61. PMID 27499341",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/27499341/", url: "https://pubmed.ncbi.nlm.nih.gov/27499341/" }),
    ("Betts MB et al.: Risk of Venous Thromboembolism by Cancer Type: A Network Meta-Analysis. Semin Thromb Hemost 2024; 50: 328-41. PMID 38395064",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/38395064/", url: "https://pubmed.ncbi.nlm.nih.gov/38395064/" }),
    ("Spotten L et al.: Subjective taste and smell changes in treatment-naive people with solid tumours. Support Care Cancer 2016; 24: 3201-8. PMID 26945569",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/26945569/", url: "https://pubmed.ncbi.nlm.nih.gov/26945569/" }),
    ("Self-reported and objective taste and smell evaluation in treatment-naive solid tumour patients. Support Care Cancer 2020. PMID 31486983",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/31486983/", url: "https://pubmed.ncbi.nlm.nih.gov/31486983/" }),
    ("Plus Kalium retard (Kaliumchlorid), Fachinformation. Swissmedic 29138",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/29138", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/29138" }),
    ("Magnesiocard (Magnesiumaspartat), Fachinformation. Swissmedic 51458",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/51458", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/51458" }),
    ("Ringer-Acetat Fresenius, Fachinformation. Swissmedic 58605",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/58605", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/58605" }),
    ("Topalak O et al.: Serum, pleural effusion, and ascites CA-125 levels in ovarian cancer and nonovarian benign and malignant diseases. Gynecol Oncol 2002; 85: 108-13. PMID 11925128",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/11925128/", url: "https://pubmed.ncbi.nlm.nih.gov/11925128/" }),
    ("Chow BL et al.: Weight-Based Assessment of Fluid Overload in Patients with Acute Kidney Injury. Nephron 2020; 144: 281-9. PMID 32403114",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/32403114/", url: "https://pubmed.ncbi.nlm.nih.gov/32403114/" }),
    ("Brems JH et al.: Balanced crystalloids versus saline in critically ill adults with low plasma bicarbonate. J Crit Care 2021; 63: 250-3. PMID 33500146",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/33500146/", url: "https://pubmed.ncbi.nlm.nih.gov/33500146/" }),
    ("Natriumchlorid 0,9% Baxter, Fachinformation. Swissmedic 41181",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/41181", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/41181" }),
    ("Amlodipin Viatris, Fachinformation. Swissmedic 59261",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/59261", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/59261" }),
    ("Lisinopril-Mepha, Fachinformation. Swissmedic 56777",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/56777", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/56777" }),
    ("Torasemid Mylan, Fachinformation. Swissmedic 67971",
     Verweis { text: "https://ch.oddb.org/de/gcc/fachinfo/reg/67971", url: "https://ch.oddb.org/de/gcc/fachinfo/reg/67971" }),
];

/// Das Vaskulitis-Blatt.
pub static BLATT: Dokument = Dokument {
    titel: TITEL,
    titel2: TITEL2,
    untertitel: UNTERTITEL,
    stand: STAND,
    kopfzeile: KOPFZEILE,
    blocks: DOKUMENT,
    quellen: QUELLEN,
};
