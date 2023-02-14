C=gcc
CFLAGS=-O3 -mtune=znver3
EXEC=bin/vegautils
OBJECTS=src/vegautils.o
LINKFLAGS=-lm -lpci
DEPENDS=${OBJECTS:.o=.d}
PREFIX=/usr/local

${EXEC}: ${OBJECTS}
	mkdir bin
	${C} ${CFLAGS} ${OBJECTS} $(LINKFLAGS) -o ${EXEC}
	cp src/vega bin/vega
	cp src/config.sh bin/config.sh

-include ${DEPENDS}

.PHONY: clean

install: ${EXEC}
	sudo cp -r bin /etc/vega.d
	sudo ln -sf /etc/vega.d/vega ${PREFIX}/bin/vega

remove:
	sudo rm -rf /etc/vega.d
	sudo rm -f ${PREFIX}/bin/vega

clean:
	rm -f ${OBJECTS} ${EXEC} ${DEPENDS}
	rm -rf bin
