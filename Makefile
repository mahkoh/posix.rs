PROJECT_NAME = posix

# Sources
LIB_SRC = src/lib.rs
DEPS_FILE = target/.$(PROJECT_NAME).deps
LIB_DEPS = $(shell head -n1 $(DEPS_FILE) 2> /dev/null)

# Objects
LIB = target/$(shell rustc --print-file-name ${LIB_SRC})
TEST = target/posix

# CFG Directive Options
CFG_OPT ?= -O

all: ${LIB}

test: ${TEST}

${LIB}: ${LIB_DEPS}
	@mkdir -p target
	rustc ${CFG_OPT} --out-dir target ${LIB_SRC}
	@rustc --no-trans --dep-info $(DEPS_FILE) ${LIB_SRC}
	@sed -i 's/.*: //' $(DEPS_FILE)

${TEST}: ${LIB_DEPS}
	@mkdir -p target
	rustc --test ${CFG_OPT} --out-dir target ${LIB_SRC}
	@rustc --no-trans --dep-info $(DEPS_FILE) ${LIB_SRC}
	@sed -i 's/.*: //' $(DEPS_FILE)

doc: ${LIB_DEPS}
	rm -rf doc
	rustdoc ${LIB_SRC}

clean:
	rm -rf target bin doc

.PHONY: all clean
