# Erzeugt HTML und PDF aus den Rust-Quellen in src/.
#
# Zwei Blaetter, ein Lauf, dieselbe Pipeline: `src/html.rs` und `src/pdf.rs`
# setzen jedes Dokument, das ihnen gereicht wird. Das Vaskulitis-Blatt steht
# in `src/inhalt.rs`, das Begleitblatt zum Kostaufbau in `src/hunger.rs`.

HTML  = iga-vaskulitis.html
PDF   = iga-vaskulitis.pdf
HHTML = kostaufbau-nach-hungern.html
HPDF  = kostaufbau-nach-hungern.pdf
BIN   = target/release/infoblatt
QUELLEN = src/main.rs src/inhalt.rs src/hunger.rs src/html.rs src/pdf.rs src/blatt.css Cargo.toml

.PHONY: all open pruef pruef-hunger clean

all: $(PDF) $(HPDF)

$(BIN): $(QUELLEN)
	cargo build --release --offline

$(PDF) $(HTML) $(HPDF) $(HHTML): $(BIN)
	./$(BIN)

open: $(PDF)
	xdg-open $(PDF)

# Sichtpruefung Seite fuer Seite. Ab zehn Seiten wechselt die Nummerierung
# der Ausgabedateien von pruef-4.png auf pruef-04.png.
pruef: $(PDF)
	rm -f pruef*.png
	pdftoppm -png -r 70 $(PDF) pruef

pruef-hunger: $(HPDF)
	rm -f hunger-pruef*.png
	pdftoppm -png -r 70 $(HPDF) hunger-pruef

clean:
	rm -f $(PDF) $(HTML) $(HPDF) $(HHTML) pruef*.png hunger-pruef*.png
	cargo clean
