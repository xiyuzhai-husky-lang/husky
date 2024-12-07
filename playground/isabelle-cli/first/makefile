# Project and Isabelle session name
SESSION = My_Session
ISABELLE = isabelle

# Default target
all: build

# Build the Isabelle session
build:
	$(ISABELLE) build -D .

# Clean Isabelle build artifacts
clean:
	rm -rf $(HOME)/.isabelle/$(shell $(ISABELLE) getenv ISABELLE_IDENTIFIER)/heaps/$(shell $(ISABELLE) getenv ML_PLATFORM)/$(SESSION)

# Run the Isabelle session interactively
jedit:
	$(ISABELLE) jedit -d . -l $(SESSION)
