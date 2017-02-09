export MODULE_NAME := kmod
export BUILD_DIRECTORY := target/kernel
export CFILES := $(wildcard src/*.c)
export RUSTFILES := $(wildcard src/*.rs)

ifneq "$(VERBOSE)" "1"
.SILENT:
endif

all modules:
	@mkdir -p ${BUILD_DIRECTORY}/src
	cp "Makefile.in" "${BUILD_DIRECTORY}/Makefile"
	@$(MAKE) -C /lib/modules/$(shell uname -r)/build M=${PWD}/${BUILD_DIRECTORY} modules

clean:
	cargo clean
