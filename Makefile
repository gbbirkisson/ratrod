SRC=$(shell find src -name "*.rs") build.rs

LIB_DIR=${PWD}/target/release
RATROD_LIB=target/release/libratrod.so
RATROD_HEADERS=target/release/libratrod.h

VCL_NAME=ratrod

export LD_LIBRARY_PATH=${LIB_DIR}

CC_EXTRA=-I ${LIB_DIR} -L ${LIB_DIR} -l ratrod
CC_COMMAND=gcc -g -O2 -fno-omit-frame-pointer -mno-omit-leaf-frame-pointer -ffile-prefix-map=/varnish-cache=. -fstack-protector-strong -fstack-clash-protection -Wformat -Werror=format-security -fcf-protection -fdebug-prefix-map=/varnish-cache=/usr/src/varnish-6.0.13-1~noble -Wall -Werror -Wno-error=unused-result -pthread -fpic -shared -Wl,-x

$(RATROD_LIB) $(RATROD_HEADERS): $(SRC)
	cargo build --release
	touch ${RATROD_HEADERS}

${VCL_NAME}.c: ${VCL_NAME}.vcl $(RATROD_HEADERS) $(RATROD_LIB)
	varnishd \
		-n ${PWD}/varnish-pwd \
		-C \
		-f ${PWD}/${VCL_NAME}.vcl \
		-p vcc_allow_inline_c=on \
		-p cc_command="exec ${CC_COMMAND} -o %o %s ${CC_EXTRA}" \
		2>&1 | tail -n +4 | tee ${VCL_NAME}.c

${VCL_NAME}: ${VCL_NAME}.c
	${CC_COMMAND} -o ${VCL_NAME} ${VCL_NAME}.c ${CC_EXTRA}

.PHONY: run
run: $(RATROD_HEADERS) $(RATROD_LIB)
	varnishd \
		-n ${PWD}/varnish-pwd \
		-F \
		-a 127.0.0.1:7777 \
		-f ${PWD}/${VCL_NAME}.vcl \
		-p cc_command="exec ${CC_COMMAND} -o %o %s ${CC_EXTRA}" \
		-p vcc_allow_inline_c=on

.PHONY: clean
clean:
	rm -rf varnish-pwd
	rm -f ${VCL_NAME}
	rm -f ${VCL_NAME}.c
	cargo clean
