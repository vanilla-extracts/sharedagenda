export TYPST_FONT_PATHS=fonts/
build: main.typ
	typst c main.typ
watch: build
	evince main.pdf &
	typst w main.typ
