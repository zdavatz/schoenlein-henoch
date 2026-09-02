// Kostaufbau nach langem Hungern - Begleitblatt zum Informationsblatt
// IgA-Vaskulitis.
// Copyright (C) 2026 Zeno R.R. Davatz
//
// Dieses Programm ist freie Software: Sie koennen es weitergeben und/oder
// veraendern, unter den Bedingungen der GNU General Public License, wie von
// der Free Software Foundation veroeffentlicht, entweder Version 3 der
// Lizenz oder (nach Ihrer Wahl) jeder spaeteren Version. Siehe LICENSE.
//
// Kein Ersatz fuer eine aerztliche Beurteilung.
//
// Der gesamte Text steht hier als Daten. Wer Inhalt aendert, aendert ihn nur
// in dieser Datei; gesetzt wird er von src/html.rs und src/pdf.rs.

use crate::inhalt::Block::*;
use crate::inhalt::Span::{B, L, N, T};
use crate::inhalt::{Block, Dokument, Tabelle, Verweis};

// ---------------------------------------------------------------------------
// Tabellen
// ---------------------------------------------------------------------------

static T_CHRONIK: Tabelle = Tabelle {
    kopf: &[],
    gewichte: &[12, 88],
    linien: false,
    chronik: true,
    zeilen: &[
        &[
            &[T("1945")],
            &[T("Nach der Befreiung der Lager sterben Unterernährte nicht am Hunger, sondern an der ersten Mahlzeit. Die Beobachtung steht am Anfang von allem, was hier folgt.")],
        ],
        &[
            &[T("1951")],
            &[T("Schnitker berichtet über japanische Kriegsgefangene: Ein Fünftel der Ausgehungerten stirbt plötzlich, nachdem Nahrung und Vitamine wieder gegeben wurden.")],
        ],
        &[
            &[T("2000")],
            &[T("In türkischen Gefängnissen beginnt ein Hungerstreik, der über Jahre geht. Er wird zur bestdokumentierten Sammlung neurologischer Spätschäden – und zum Beleg dafür, dass Thiamin durch den Mund nicht genügt.")],
        ],
        &[
            &[T("2012")],
            &[T("Genfer Universitätsspital: eine Handreichung für Ärztinnen und Ärzte, die Hungerstreikende begleiten. Sie enthält die Überwachungspläne, auf denen dieses Blatt beruht.")],
        ],
        &[
            &[T("2014")],
            &[T("Inselspital Bern wertet 37 Aufnahmen aus fünf Jahren aus – die einzige Serie, die zeigt, wie es ausgeht, wenn man sich an die Empfehlungen hält.")],
        ],
    ],
};

static T_WERTE: Tabelle = Tabelle {
    kopf: &["Wert", "Wann und warum"],
    gewichte: &[26, 74],
    linien: true,
    chronik: false,
    zeilen: &[
        &[
            &[B("Phosphat")],
            &[T("Der Leitwert. Vor der ersten Mahlzeit und danach täglich über vier Tage. Fällt er, ist das Refeeding-Syndrom da – noch bevor jemand etwas sieht.")],
        ],
        &[
            &[B("Kalium")],
            &[T("Derselbe Weg in die Zellen wie das Phosphat. Zusammen mit Magnesium ist der Mangel das, was Herzrhythmusstörungen macht.")],
        ],
        &[
            &[B("Magnesium")],
            &[T("Wird oft vergessen und ist die Bedingung dafür, dass sich ein Kaliummangel überhaupt ausgleichen lässt.")],
        ],
        &[
            &[B("Kalzium")],
            &[T("Gehört in dieselbe Abnahme; die Genfer Handreichung führt es in derselben Zeile.")],
        ],
        &[
            &[B("Harnstoff, Kreatinin")],
            &[T("Die Nierenfunktion entscheidet mit, wie schnell ersetzt werden darf und was der Körper an Flüssigkeit noch los wird.")],
        ],
        &[
            &[B("Natrium")],
            &[T("Kohlenhydrate drosseln die Natriumausscheidung der Niere. Wer isst, hält Wasser – deshalb gehört das Gewicht täglich dazu.")],
        ],
        &[
            &[B("Thiaminspiegel")],
            &[T("Nur, wenn eine Gabe abgelehnt wird. Sonst wird nicht gemessen, sondern gegeben.")],
        ],
        &[
            &[B("EKG")],
            &[T("Zu Beginn und bei jeder Verschiebung der Elektrolyte. Der plötzliche Herztod ist die Todesart, um die es hier geht.")],
        ],
    ],
};

// ---------------------------------------------------------------------------
// Das Dokument
// ---------------------------------------------------------------------------

static DOKUMENT: &[Block] = &[
    H2("Woher man das weiss"),
    P(&[T("Wie ernährt man einen Menschen, der wochenlang nichts gegessen hat? Die genaueste Antwort auf diese Frage kommt nicht aus der Ernährungsberatung, sondern aus Gefängnissen und aus dem Jahr 1945. Hungerstreikende sind die einzige Gruppe, bei der der Hunger von Tag eins an dokumentiert ist und der Wiederbeginn geplant wird – und Kriegsgefangene waren die Ersten, an denen auffiel, dass nicht das Hungern tötet, sondern das Essen danach.")]),
    Tab(&T_CHRONIK),
    Klein(&[T("Dieses Blatt gehört zum Informationsblatt zur IgA-Vaskulitis und beantwortet eine Frage daraus ausführlich. Die Patientin, um die es dort geht, war nicht im Hungerstreik – sie konnte nichts bei sich behalten. Was sich davon übertragen lässt und was nicht, steht im letzten Abschnitt.")]),

    Lead {
        werte: "Beginn 10–20 kcal/kg/Tag · Thiamin vor der ersten Kohlenhydratgabe · Phosphat täglich über vier Tage",
        blocks: &[
            P(&[T("Drei Sätze, auf die sich alles Folgende zusammenziehen lässt.")]),
            P(&[
                B("Erstens: Das Hungern ist nicht die gefährliche Phase, sondern das Wiederessen."), T(" Das Refeeding-Syndrom tritt in den ersten vier Tagen nach dem Beginn der Ernährung auf. Wer die ersten Tage übersteht, hat das Schlimmste hinter sich – und wer sie nicht ernst nimmt, verliert einen Menschen, den der Hunger nicht umgebracht hat."),
            ]),
            P(&[
                B("Zweitens: Der gefährliche Nährstoff ist der Zucker."), T(" Kohlenhydrate treiben Insulin hoch, und Insulin schiebt Phosphat, Kalium und Magnesium aus dem Blut in die Zellen. Nicht die Menge an Kalorien allein entscheidet, sondern wie schnell sie kommen und woraus sie bestehen."),
            ]),
            P(&[
                B("Drittens: Thiamin kommt vor dem Essen, nicht danach."), T(" Und der Weg ist nicht gleichgültig – das ist der Punkt, an dem die üblichen Empfehlungen nachweislich versagt haben. Er hat einen eigenen Abschnitt."),
            ]),
        ],
    },

    H2("Die kurze Antwort"),
    P(&[T("Wer nur eine Seite lesen kann, liest diese Liste. Jede Zeile steht weiter unten belegt.")]),
    Liste(&[
        &[B("Ab zwei Wochen Fasten gehört der Kostaufbau ins Spital."), T(" Die Genfer Handreichung führt ihn ausdrücklich unter den "), L("Gründen für eine Hospitalisation", "https://pubmed.ncbi.nlm.nih.gov/22987064/"), T(" auf – nicht das Hungern selbst, sondern das Wiederessen ist der Grund, aus dem jemand ins Spital gehört.")],
        &[B("Thiamin vor der ersten Kohlenhydratgabe."), T(" "), N("100 mg"), T(", mindestens eine halbe Stunde vorher, danach zweimal täglich über sieben bis zehn Tage. Zum Weg siehe den eigenen Abschnitt.")],
        &[B("Langsam beginnen."), T(" "), N("10 bis 20 kcal"), T(" je Kilogramm Körpergewicht in den ersten 24 Stunden, bei sehr schwerer Unterernährung die Hälfte davon. Danach alle ein bis zwei Tage um etwa ein Drittel des Ziels steigern.")],
        &[B("Phosphat, Kalium, Magnesium messen – vorher und dann täglich."), T(" Vier Tage lang, bei Hochrisiko in den ersten drei Tagen alle zwölf Stunden. Ergänzen, bevor der Wert fällt, nicht erst danach.")],
        &[B("Flüssigkeit zurückhaltend."), T(" Kohlenhydrate lassen die Niere Natrium und Wasser zurückhalten. Wer grosszügig infundiert, während er zu essen beginnt, überlädt den Kreislauf.")],
        &[B("Das Gewicht täglich."), T(" Eine rasche Zunahme in den ersten Tagen ist kein Ernährungserfolg, sondern Wasser – und sie ist zugleich die Zahl, auf die nicht gerechnet werden darf: Die Anfangsmenge rechnet auf das Trockengewicht, nicht auf das Gewicht mit den Ödemen.")],
        &[B("Weiche Konsistenz, nicht kleine Menge."), T(" Ein Magen, der lange nichts zu tun hatte, scheitert an festen Stücken, nicht an Kalorien.")],
    ]),

    H2("Was beim Hungern im Körper geschieht"),
    P(&[T("Ohne Nahrung stellt der Körper die Versorgung um. Zuerst verbraucht er die Zuckerspeicher, dann das Fett, zuletzt Muskel- und Organgewebe – die Genfer Handreichung beschreibt genau diese Reihenfolge in dem Merkblatt, das sie Hungerstreikenden aushändigen lässt. Insulin fällt, Glukagon steigt, der Stoffwechsel läuft auf Fett und Ketonkörper.")]),
    P(&[B("Und dabei geschieht etwas, das die Blutwerte nicht zeigen."), T(" Über eine lange Hungerzeit leeren sich die "), L("Speicher innerhalb der Zellen", "https://pubmed.ncbi.nlm.nih.gov/22987064/"), T(", allen voran das Phosphat. Die Konzentration im Blut kann dabei "), B("normal bleiben"), T(", weil der Körper sie auf Kosten der Zellen hält. Ein unauffälliges Phosphat vor der ersten Mahlzeit ist deshalb keine Entwarnung – es ist die Ausgangslage, gegen die man misst.")]),
    P(&[T("Das ist der Grund, warum dieses Blatt so viel von Messwerten handelt und so wenig von Speiseplänen. Was gefährlich wird, ist unsichtbar, bis es sich bewegt.")]),

    H2("Warum das erste Essen gefährlicher ist als das Hungern"),
    P(&[T("Mit der ersten kohlenhydrathaltigen Mahlzeit steigt der Blutzucker, das Insulin schiesst hoch, und der Körper schaltet binnen Stunden von Abbau auf Aufbau. Insulin lässt Glykogen, Fett und Eiweiss bilden – und zieht dafür Phosphat, Kalium und Magnesium in die Zellen. Die Werte im Blut fallen, obwohl doch gerade wieder gegessen wird. Das "), B("Kennzeichen"), T(" ist der "), L("Abfall des Phosphats", "https://www.ncbi.nlm.nih.gov/books/NBK564513/"), T(".")]),
    P(&[B("Was daraus wird, steht in der Genfer Handreichung als Liste."), T(" Rhabdomyolyse, gestörte Funktion der weissen Blutkörperchen, Atemversagen, Herzversagen, Blutdruckabfall, Herzrhythmusstörungen, Krampfanfälle, Koma, plötzlicher Tod. Dazu ein zweiter Weg, der leicht übersehen wird: Kohlenhydrate senken die Ausscheidung von Natrium über die Niere, und deshalb kann sich rasch eine "), L("Flüssigkeitsüberladung", "https://pubmed.ncbi.nlm.nih.gov/22987064/"), T(" entwickeln. Wer bei Beginn der Ernährung grosszügig infundiert, tut zweierlei auf einmal, was sich addiert.")]),
    P(&[B("Der Zeitrahmen ist eng und benennbar."), T(" Das Syndrom tritt "), B("innerhalb von vier Tagen"), T(" nach Beginn der Ernährung auf. Und gefährdet ist nicht erst, wer monatelang gefastet hat: Nach der Übersicht im British Medical Journal ist jeder Mensch mit "), L("vernachlässigbarer Nahrungsaufnahme über mehr als fünf Tage", "https://pmc.ncbi.nlm.nih.gov/articles/PMC2440847/"), T(" gefährdet. Fünf Tage – nicht fünf Wochen.")]),
    P(&[T("Dieselbe Arbeit nennt die Zahlen für den Beginn: höchstens "), N("0,042 MJ"), T(" je Kilogramm und Tag, also rund "), N("10 kcal/kg"), T("; bei sehr schwerer Unterernährung höchstens "), N("0,021 MJ/kg"), T(", also die Hälfte. Und Vitamine sofort, vor Beginn und über die ersten zehn Tage.")]),

    H2("Thiamin: der Punkt, an dem die übliche Empfehlung versagt hat"),
    P(&[T("Thiamin – Vitamin B1 – ist das Hilfsmolekül, mit dem der Körper Zucker verbrennt. Fehlt es und kommen Kohlenhydrate, bricht der Abbau bei der Brenztraubensäure ab, es entsteht Milchsäure, und aus dem Überschuss wird eine Azidose, die tödlich enden kann. Deshalb der Satz, der in jeder Handreichung steht: "), B("Thiamin muss vor dem Beginn der Ernährung gegeben werden"), T(". Der bleibende Schaden, um den es geht, heisst Wernicke-Korsakow-Syndrom: Bewusstseinstrübung, Augenmuskellähmung, Gangunsicherheit, und am Ende ein Gedächtnis, das nichts Neues mehr behält.")]),
    P(&[B("Und jetzt die Beobachtung, die alles daran ändert, wie man es gibt."), T(" In Izmir wurden 41 Gefangene nachuntersucht, die zwischen 2000 und 2002 im Hungerstreik waren – zwischen 130 und 324 Tagen, im Mittel 199. Alle hatten während des Streiks "), L("200 bis 600 mg Thiamin täglich durch den Mund", "https://pubmed.ncbi.nlm.nih.gov/16987161/"), T(" bekommen, im Mittel über 156 Tage. Das ist ein Vielfaches der üblichen Dosis. "), B("Alle 41 entwickelten trotzdem ein Wernicke-Korsakow-Syndrom."), T(" Bei allen war das Bewusstsein getrübt, drei bis 31 Tage lang; alle hatten Blickrichtungsnystagmus und Rumpfataxie, alle eine Amnesie. Die Autoren halten fest, dass der Hunger das Zentralnervensystem stärker traf als die Nerven der Gliedmassen – und dass die Erholung nur teilweise war.")]),
    P(&[B("Das war kein türkischer Sonderfall."), T(" 2022 beschrieben Ärzte in Pittsburgh eine 49-jährige Frau nach 237 Tagen Hungerstreik: unter ärztlicher Aufsicht, mit "), L("höher als empfohlen dosiertem Thiamin durch den Mund", "https://pmc.ncbi.nlm.nih.gov/articles/PMC9359357/"), T(" – und trotzdem eine Wernicke-Enzephalopathie mit anschliessendem Korsakow-Syndrom. Sie ist dauerhaft geschädigt. Die Autoren ziehen den Schluss, den man aus beiden Arbeiten ziehen muss: Die geltenden Empfehlungen zur Vorbeugung des Thiaminmangels bei Hungerstreikenden sind "), B("unzureichend"), T(", und Thiamin vorbeugend in die Vene oder in den Muskel gehört als neuer Standard geprüft.")]),
    P(&[B("Warum das durch den Mund nicht ankommt, hat einen einfachen Grund."), T(" Ein Darm, der lange nichts zu tun hatte, nimmt schlechter auf; die Schleimhaut bildet sich zurück. Wer also am dringendsten Thiamin braucht, ist zugleich derjenige, bei dem die Tablette am wenigsten verlässlich wirkt. "), B("Der Merksatz daraus:"), T(" Bei jemandem, der wochenlang nicht gegessen hat oder dessen Magen sich nicht entleert, ist der orale Weg für nichts Wichtiges der richtige – für Thiamin nicht und für andere unverzichtbare Mittel auch nicht. Das gehört gefragt, bevor die erste Mahlzeit kommt.")]),

    H2("Wie es in der Praxis gemacht wurde"),
    P(&[T("Zwei Serien beschreiben den Ablauf, und beide sind klein. Sie sind trotzdem das Beste, was es gibt.")]),
    P(&[B("São Paulo, acht Hungerstreikende, 43 Tage ohne Nahrung."), T(" Sie hatten ausser Wasser nichts zu sich genommen und rund 18 Prozent ihres Gewichts verloren. Der Aufbau erfolgte in "), L("vier Stufen über neun Tage", "https://pubmed.ncbi.nlm.nih.gov/11240336/"), T("; danach vertrugen alle acht eine vollständige, uneingeschränkte Kost. Ein Refeeding-Syndrom trat nicht auf – kein Phosphatabfall, keine Unverträglichkeit der Nährstoffe. Es gab etwas Wasserretention, aber nur milde, und einzelne Durchfälle. Bemerkenswert ist die Beobachtung nebenbei: Die Entzündungswerte stiegen während des Aufbaus an, ohne dass eine Infektion vorlag. Wer in dieser Phase ein steigendes CRP sieht, sucht also möglicherweise vergeblich nach einem Infekt.")]),
    P(&[B("Bern, 37 Aufnahmen aus fünf Jahren."), T(" Das Inselspital hat ausgewertet, was geschieht, wenn man sich an die Empfehlungen hält. Der Aufbau lief bei allen 30 Wiederernährten "), B("über den Mund"), T(", mit Flüssigkeitsbeschränkung, "), L("schrittweise über zehn Tage", "https://pubmed.ncbi.nlm.nih.gov/25280415/"), T("; in 25 Fällen wurden die Mikronährstoffe nach Empfehlung ersetzt. Bei 12 der 30 – also bei 40 Prozent – verschoben sich die Elektrolyte, aber ohne Folgen. Intensivmedizinische Behandlung war in keinem Fall nötig, und niemand starb.")]),
    P(&[B("Ein einziger Fall in dieser Serie zeigte ein mittelschweres Refeeding-Syndrom – und er zeigte es an den Knöcheln."), T(" Die klinische Manifestation war ein "), B("beidseitiges Knöchelödem"), T(". Das ist die praktisch wichtigste Zeile dieser ganzen Arbeit, denn sie sagt, wonach man schaut, wenn man kein Labor zur Hand hat: Wassereinlagerung an den Füssen bei jemandem, der gerade wieder zu essen begonnen hat, ist nicht einfach «lange gelegen». Sie kann die sichtbare Seite dessen sein, was die Blutwerte gleichzeitig tun.")]),
    Klein(&[T("Beide Serien sind rückblickend und klein: acht junge Männer in der einen, 33 Personen in der anderen, alle in einem Alter, in dem Herz und Nieren mitmachen. Was sie zeigen, ist, dass ein geplanter Aufbau glimpflich ausgeht – nicht, wie gross das Risiko ohne ihn wäre. Genau das lässt sich nicht mehr untersuchen, weil niemand eine Vergleichsgruppe ohne Vorsichtsmassnahmen führen würde.")]),

    H2("Die Überwachung: welche Werte, wie oft"),
    P(&[T("Die Genfer Handreichung nennt Phosphat, Magnesium, Kalzium, Kalium, Harnstoff und Kreatinin – gemessen "), B("vor"), T(" der ersten Mahlzeit und danach "), B("täglich über vier Tage"), T(". Die Kapitelübersicht in der amerikanischen Datenbank geht bei Hochrisiko weiter: in den ersten drei Tagen alle zwölf Stunden, danach täglich in der ersten Woche und dreimal in der zweiten. Fällt das Phosphat, wird es ersetzt, zusammen mit Kalium und Magnesium; das Zusammentreffen mehrerer Verschiebungen ist es, was die Rhythmusstörungen und den plötzlichen Tod ausmacht.")]),
    Tab(&T_WERTE),
    P(&[B("Und zwei Dinge, die kein Labor braucht."), T(" Das Gewicht täglich – eine rasche Zunahme in den ersten Tagen ist Wasser und kein Erfolg. Und ein Blick auf die Knöchel, aus dem Grund, der im Abschnitt davor steht.")]),

    H2("Was das Essen selbst betrifft"),
    P(&[T("Erst kommen Menge und Tempo, dann die Konsistenz, und ganz zuletzt die Speisekarte. In dieser Reihenfolge, und nicht in der umgekehrten, die sich beim Zusehen aufdrängt.")]),
    P(&[B("Zur Konsistenz."), T(" Ein Magen, der lange nichts zu tun hatte, muss feste Nahrung erst zerkleinern, bevor sie den Magenausgang passiert – dafür braucht er eine Muskelarbeit, die nach Wochen ohne Essen oft nicht da ist. Breiiges und Flüssiges passieren, ohne zerkleinert zu werden. In einer randomisierten Studie an 56 Patienten mit Magenlähmung besserten sich Übelkeit, Erbrechen, Völlegefühl und Blähung unter einer Kost aus "), L("kleinen Partikeln", "https://pubmed.ncbi.nlm.nih.gov/24419482/"), T(" deutlich stärker als unter gewöhnlicher Kost. Untersucht wurde das bei Diabetes; das Prinzip hängt an der Mechanik und nicht an der Ursache.")]),
    P(&[B("Zu den Ballaststoffen."), T(" Der Reflex, bei stockender Verdauung mehr Gemüse zu geben, geht hier fehl. Die "), L("Leitlinie der amerikanischen Gastroenterologen", "https://pubmed.ncbi.nlm.nih.gov/35926490/"), T(" rät bei Magenlähmung ausdrücklich, unverdauliche Faserstoffe aus Gemüse und Obst zu meiden. Und in einer Arbeit an 63 Menschen mit Verstopfung ohne organische Ursache stieg die Stuhlfrequenz, nachdem die Ballaststoffe "), L("ganz weggelassen wurden", "https://pubmed.ncbi.nlm.nih.gov/22969234/"), T(" – von einem Stuhlgang alle 3,75 Tage auf einen täglich. Die Arbeit ist klein und einarmig; als Warnung vor der reflexhaften Gleichung taugt sie trotzdem.")]),
    P(&[B("Zum Eiweiss."), T(" Aus alledem folgt ausdrücklich nicht, weniger Eiweiss zu geben. Nach Wochen ohne Nahrung ist das Albumin tief, und der Körper baut Muskel- und Organgewebe ab. Das Problem ist nicht das Eiweiss, sondern seine Form – Eiweiss in einer Konsistenz, die den Magen verlässt, ist eine Aufgabe für die Ernährungsberatung und keine Frage des Verzichts.")]),
    P(&[B("Zum Zucker."), T(" Er ist hier zugleich das Gefährliche und das Notwendige. Gefährlich, weil er das Insulin treibt, das die Elektrolyte in die Zellen zieht; notwendig, weil Kalorien, die freiwillig hineingehen, nach Wochen ohne Essen ein Gewinn sind. Der Widerspruch löst sich nicht über die Speisekarte, sondern über das Tempo und über die Werte, die daneben gemessen werden.")]),

    H2("Die Regel, die vor allen medizinischen steht"),
    P(&[T("Bei Hungerstreikenden geht der Frage, wie man ernährt, immer die Frage voraus, ob man darf. Die "), L("Deklaration von Malta", "https://www.wma.net/policies-post/wma-declaration-of-malta-on-hunger-strikers/"), T(" des Weltärztebundes ist an dieser Stelle unmissverständlich: Zwangsernährung ist niemals ethisch vertretbar. Gemeint ist jede Ernährung über Sonde oder Vene gegen den Willen eines urteilsfähigen Menschen; Ernährung mit seiner ausdrücklichen oder erkennbaren Zustimmung ist es sehr wohl. Nahrung, die mit Drohung, Zwang oder Fixierung verabreicht wird, gilt als unmenschliche und erniedrigende Behandlung – auch dann, wenn sie gut gemeint ist.")]),
    P(&[T("Für den Kostaufbau nach dem Streik heisst das etwas Praktisches: Er beginnt, wenn die betroffene Person ihn will, und er ist von da an eine medizinische Aufgabe wie jede andere. Die Genfer Handreichung nennt daneben die Bedingung, unter der das überhaupt gelingt – die ärztliche Unabhängigkeit von der Anstalt. Ohne Vertrauen kein Gespräch, ohne Gespräch keine Zustimmung.")]),

    H2("Was sich auf eine Kranke übertragen lässt und was nicht"),
    P(&[T("Dieses Blatt ist entstanden, weil eine 84-jährige Patientin über Wochen kaum etwas bei sich behalten hat. Sie war nicht im Hungerstreik. Der Unterschied ist wichtig, und er zeigt in beide Richtungen.")]),
    Liste(&[
        &[B("Was sich überträgt: die Mechanik."), T(" Insulin, Phosphat, Kalium, Magnesium, die vier Tage, das Thiamin vor dem Zucker – das hängt am Stoffwechsel und nicht am Anlass des Hungerns. Die Fünf-Tage-Schwelle aus dem British Medical Journal gilt für jeden.")],
        &[B("Was sich überträgt, und zwar verschärft: der Weg der Medikamente."), T(" Wer aus Krankheit nicht isst, hat oft zusätzlich einen Magen, der sich nicht entleert, oder einen Darm, der nicht transportiert. Damit ist der orale Weg noch unsicherer als bei einem Gesunden im Streik – und dort hat er nachweislich versagt.")],
        &[B("Was nicht dasselbe ist: die Ausgangslage."), T(" Hungerstreikende sind in der Regel jung und vorher gesund. Eine betagte Kranke bringt Blutarmut, eingeschränkte Nierenfunktion, ein alterndes Herz und Medikamente mit. Jede Verschiebung der Elektrolyte trifft sie härter, und die Flüssigkeitsüberladung, die in den Serien mild blieb, ist bei ihr ein eigenes Risiko.")],
        &[B("Was ebenfalls nicht dasselbe ist: die Dauer und die Kontrolle."), T(" 43 Tage totales Fasten sind etwas anderes als Wochen mit ein wenig Tee und mehrfachem Erbrechen. Erbrechen kostet zusätzlich Kalium und Flüssigkeit, und niemand hat vom ersten Tag an mitgeschrieben. Die Werte sind deshalb wichtiger und nicht weniger wichtig als in den Serien – man weiss ja weniger.")],
    ]),
    P(&[B("Und ein Satz zum Schluss, der die Richtung angibt."), T(" In den beiden Serien, in denen der Aufbau geplant wurde, ist niemand gestorben und niemand auf die Intensivstation gekommen. Das Wissen, um das es hier geht, ist weder neu noch teuer: ein Vitamin, drei Blutwerte, ein langsamer Anfang und eine Waage. Es geht nur leicht unter, wenn gleichzeitig etwas Dramatischeres behandelt wird.")]),
];

// ---------------------------------------------------------------------------
// Quellen
// ---------------------------------------------------------------------------

static QUELLEN: &[(&str, Verweis)] = &[
    ("Gétaz L et al.: Hunger strike among detainees: guidance for good medical practice. Swiss Med Wkly 2012; 142: w13675. PMID 22987064",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/22987064/", url: "https://pubmed.ncbi.nlm.nih.gov/22987064/" }),
    ("Eichelberger M, Joray ML, Perrig M, Bodmer M, Stanga Z: Management of patients during hunger strike and refeeding phase. Inselspital Bern. Nutrition 2014; 30: 1372-8. PMID 25280415",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/25280415/", url: "https://pubmed.ncbi.nlm.nih.gov/25280415/" }),
    ("Faintuch J et al.: Refeeding procedures after 43 days of total fasting. Nutrition 2001; 17: 100-4. PMID 11240336",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/11240336/", url: "https://pubmed.ncbi.nlm.nih.gov/11240336/" }),
    ("Başoğlu M et al.: Neurological complications of prolonged hunger strike. Eur J Neurol 2006; 13: 1089-97. PMID 16987161",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/16987161/", url: "https://pubmed.ncbi.nlm.nih.gov/16987161/" }),
    ("Wagner P, Bui T: Wernicke-Korsakoff Syndrome in a Hunger Striker Despite Oral Thiamine Supplementation. Int Med Case Rep J 2022; 15: 399-403. PMC9359357",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9359357/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC9359357/" }),
    ("Mehanna HM, Moledina J, Travis J: Refeeding syndrome: what it is, and how to prevent and treat it. BMJ 2008; 336: 1495-8. PMC2440847",
     Verweis { text: "https://pmc.ncbi.nlm.nih.gov/articles/PMC2440847/", url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC2440847/" }),
    ("Refeeding Syndrome. StatPearls, NCBI Bookshelf – enthält den Bericht von Schnitker 1951 über japanische Kriegsgefangene",
     Verweis { text: "https://www.ncbi.nlm.nih.gov/books/NBK564513/", url: "https://www.ncbi.nlm.nih.gov/books/NBK564513/" }),
    ("WMA Declaration of Malta on Hunger Strikers, Weltärztebund",
     Verweis { text: "wma.net – Declaration of Malta on Hunger Strikers", url: "https://www.wma.net/policies-post/wma-declaration-of-malta-on-hunger-strikers/" }),
    ("Olausson EA et al.: A small particle size diet reduces upper gastrointestinal symptoms in patients with diabetic gastroparesis. Am J Gastroenterol 2014; 109: 375-85. PMID 24419482",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/24419482/", url: "https://pubmed.ncbi.nlm.nih.gov/24419482/" }),
    ("Camilleri M et al.: ACG Clinical Guideline – Gastroparesis. Am J Gastroenterol 2022; 117: 1197-1220. PMID 35926490",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/35926490/", url: "https://pubmed.ncbi.nlm.nih.gov/35926490/" }),
    ("Ho KS et al.: Stopping or reducing dietary fiber intake reduces constipation and its associated symptoms. World J Gastroenterol 2012; 18: 4593-6. PMID 22969234",
     Verweis { text: "https://pubmed.ncbi.nlm.nih.gov/22969234/", url: "https://pubmed.ncbi.nlm.nih.gov/22969234/" }),
];

pub static BLATT: Dokument = Dokument {
    titel: "Kostaufbau nach langem Hungern",
    titel2: "(Refeeding-Syndrom)",
    untertitel: "Was die Betreuung von Hungerstreikenden über den Wiederbeginn des Essens weiss",
    stand: "Begleitblatt zum Informationsblatt IgA-Vaskulitis · Stand 2. September 2026",
    kopfzeile: "Kostaufbau nach langem Hungern (Refeeding-Syndrom)",
    blocks: DOKUMENT,
    quellen: QUELLEN,
};
