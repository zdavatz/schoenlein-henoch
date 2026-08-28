# Erzeugt HTML und PDF aus den Rust-Quellen in src/.
#
# Beides entsteht im selben Lauf: `src/html.rs` und `src/pdf.rs` sind zwei
# Ausgaben derselben Datenstruktur in `src/inhalt.rs`.

HTML = iga-vaskulitis.html
PDF  = iga-vaskulitis.pdf
BIN  = target/release/infoblatt
QUELLEN = src/main.rs src/inhalt.rs src/html.rs src/pdf.rs src/blatt.css Cargo.toml

.PHONY: all open pruef clean

all: $(PDF)

$(BIN): $(QUELLEN)
	cargo build --release --offline

$(PDF) $(HTML): $(BIN)
	./$(BIN)

open: $(PDF)
	xdg-open $(PDF)

# Sichtpruefung Seite fuer Seite. Ab zehn Seiten wechselt die Nummerierung
# der Ausgabedateien von pruef-4.png auf pruef-04.png.
pruef: $(PDF)
	rm -f pruef*.png
	pdftoppm -png -r 70 $(PDF) pruef

clean:
	rm -f $(PDF) $(HTML) pruef*.png
	cargo clean
