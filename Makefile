RUSTC=rustc

TARGET_NAME=soundex
TEST_BIN_NAME=${BUILD_DIR}/${TARGET_NAME}-test
EXAMPLE_BIN_NAME=${BUILD_DIR}/${TARGET_NAME}-example

SRC_DIR=src
BUILD_DIR=build
MAIN_SRC=${SRC_DIR}/lib.rs
TEST_SRC=${SRC_DIR}/tests.rs
EXAMPLE_SRC=${SRC_DIR}/examples/example.rs
DUMMY_FILE=${BUILD_DIR}/lib${TARGET_NAME}.dummy

SRCS = $(shell find src/. -type f -name '*.rs')

all: lib test
lib: ${DUMMY_FILE}
test: ${TEST_BIN_NAME}
examples: ${EXAMPLE_BIN_NAME}

${BUILD_DIR}:
	mkdir -p $@

${DUMMY_FILE}: ${MAIN_SRC} ${SRCS} ${BUILD_DIR}
	${RUSTC} ${MAIN_SRC} --out-dir ${BUILD_DIR}/
	touch $@

${TEST_BIN_NAME}: ${TEST_SRC} ${SRCS} ${BUILD_DIR}
	${RUSTC} ${TEST_SRC} --test -o ${TEST_BIN_NAME}
	./${TEST_BIN_NAME}

${EXAMPLE_BIN_NAME}: ${EXAMPLE_SRC} lib ${BUILD_DIR}
	${RUSTC} ${EXAMPLE_SRC} -o ${EXAMPLE_BIN_NAME} -L ${BUILD_DIR}

clean:
	rm -f ${BUILD_DIR}/*rlib ${BUILD_DIR}/*.o ${TEST_BIN_NAME} ${DUMMY_FILE}
	rmdir --ignore-fail-on-non-empty ${BUILD_DIR}
