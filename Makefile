HTML = src/iga-vaskulitis.html
PDF  = iga-vaskulitis.pdf

.PHONY: all open clean

all: $(PDF)

# Die Warnung "Unable to subset font with Harfbuzz" ist harmlos.
$(PDF): $(HTML)
	weasyprint $< $@ 2>&1 | grep -v Harfbuzz || true

open: $(PDF)
	xdg-open $(PDF)

clean:
	rm -f $(PDF)
