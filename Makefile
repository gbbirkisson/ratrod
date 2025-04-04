SRC=$(call rwildcard,src,*.rs)

LIB_DIR=${PWD}/target/release
RATROD_LIB=${LIB_DIR}/libratrod.so
RATROD_HEADERS=${LIB_DIR}/libratrod.h

CC_EXTRA=-I ${LIB_DIR} -L ${LIB_DIR} -l ratrod
CC_COMMAND=gcc -g -O2 -fno-omit-frame-pointer -mno-omit-leaf-frame-pointer -ffile-prefix-map=/varnish-cache=. -fstack-protector-strong -fstack-clash-protection -Wformat -Werror=format-security -fcf-protection -fdebug-prefix-map=/varnish-cache=/usr/src/varnish-6.0.13-1~noble -Wall -Werror -Wno-error=unused-result -pthread -fpic -shared -Wl,-x ${CC_EXTRA}

$(RATROD_LIB): $(SRC)
	cargo build --release

$(RATROD_HEADERS): $(RATROD_LIB)
	cbindgen --crate ratrod --output $(RATROD_HEADERS) --lang c

dummy: $(RATROD_HEADERS) $(RATROD_LIB)
	gcc -o dummy dummy.c ${CC_EXTRA} -lpthread -ldl
	ldd dummy

test.c: test.vcl $(RATROD_HEADERS) $(RATROD_LIB)
	varnishd \
		-n ${PWD}/varnish-pwd \
		-C \
		-f ${PWD}/test.vcl \
		-p vcc_allow_inline_c=on \
		-p cc_command="exec ${CC_COMMAND} -o %o %s" \
		2>&1 | tail -n +4 | tee test.c

.PHONY: test
test:
	${CC_COMMAND} -o test test.c
	ldd test

.PHONY: run
run: $(RATROD_HEADERS) $(RATROD_LIB)
	varnishd \
		-n ${PWD}/varnish-pwd \
		-d \
		-a 127.0.0.1:7777 \
		-f ${PWD}/test.vcl \
		-p cc_command="exec ${CC_COMMAND} -o %o %s" \
		-p vcc_allow_inline_c=on

.PHONY: clean
clean:
	rm -f dummy
	rm -f test
	rm -f test.c
	cargo clean
