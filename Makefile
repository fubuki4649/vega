C=gcc
CFLAGS=-O3 -mtune=znver3
LINKFLAGS=-lm -lpci
PREFIX=/usr/local

build: clean
	mkdir bin
	${C} ${CFLAGS} src/vegautils.c ${LINKFLAGS} -o bin/vegautils
	${C} ${CFLAGS} src/logoutils.c ${LINKFLAGS} -o bin/logoutils
	cp src/vega bin/vega
	cp src/config.sh bin/config.sh

install: build
	sudo cp -r bin /etc/vega.d
	sudo ln -sf /etc/vega.d/vega ${PREFIX}/bin/vega

.PHONY: clean remove

remove:
	sudo rm -rf /etc/vega.d
	sudo rm -f ${PREFIX}/bin/vega

clean:
	rm -rf bin
