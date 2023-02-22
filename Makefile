C=gcc
CFLAGS=-O3 -mtune=znver3
LINKFLAGS=-lm -lpci
PREFIX=/usr/local
MANPATH=${PREFIX}/man

build: clean
	mkdir bin
	${C} ${CFLAGS} src/vegautils.c ${LINKFLAGS} -o bin/vegautils
	${C} ${CFLAGS} src/logoutils.c ${LINKFLAGS} -o bin/logoutils
	cp src/vega bin/vega
	cp src/config.sh bin/config.sh

install: remove build
	sudo cp -r bin /etc/vega.d
	sudo ln -sf /etc/vega.d/vega ${PREFIX}/bin/vega
	sudo mkdir -p ${MANPATH}/man1
	sudo cp vega.1 ${MANPATH}/man1

.PHONY: clean remove

remove:
	sudo rm -rf /etc/vega.d
	sudo rm -f ${PREFIX}/bin/vega
	sudo rm -f ${MANPATH}/man1/vega.1

clean:
	rm -rf bin
