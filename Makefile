SRC=$(wildcard src/*.rs) build.rs

LIB_DIR=${PWD}/target/release
RATROD_LIB=${LIB_DIR}/libratrod.so
RATROD_HEADERS=${LIB_DIR}/libratrod.h

export LD_LIBRARY_PATH=${LIB_DIR}

CC_EXTRA=-I ${LIB_DIR} -L ${LIB_DIR} -l ratrod
CC_FILES=-o %o %s
CC_COMMAND=gcc -g -O2 -fno-omit-frame-pointer -mno-omit-leaf-frame-pointer -ffile-prefix-map=/varnish-cache=. -fstack-protector-strong -fstack-clash-protection -Wformat -Werror=format-security -fcf-protection -fdebug-prefix-map=/varnish-cache=/usr/src/varnish-6.0.13-1~noble -Wall -Werror -Wno-error=unused-result -pthread -fpic -shared -Wl,-x

$(RATROD_LIB) $(RATROD_HEADERS): $(SRC)
	cargo build --release

dummy: $(RATROD_HEADERS) $(RATROD_LIB)
	gcc -o dummy dummy.c ${CC_EXTRA} -lpthread -ldl
	ldd dummy

test.c: test.vcl $(RATROD_HEADERS) $(RATROD_LIB)
	varnishd \
		-n ${PWD}/varnish-pwd \
		-C \
		-f ${PWD}/test.vcl \
		-p vcc_allow_inline_c=on \
		-p cc_command="exec ${CC_COMMAND} -o %o %s ${CC_EXTRA}" \
		2>&1 | tail -n +4 | tee test.c

test: test.c
	${CC_COMMAND} -o test test.c ${CC_EXTRA}
	ldd test

.PHONY: run
run: $(RATROD_HEADERS) $(RATROD_LIB)
	varnishd \
		-n ${PWD}/varnish-pwd \
		-F \
		-a 127.0.0.1:7777 \
		-f ${PWD}/test.vcl \
		-p cc_command="exec ${CC_COMMAND} -o %o %s ${CC_EXTRA}" \
		-p vcc_allow_inline_c=on

.PHONY: clean
clean:
	rm -rf varnish-pwd
	rm -f dummy
	rm -f test
	rm -f test.c
	cargo clean
