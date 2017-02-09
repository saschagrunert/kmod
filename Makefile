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

test: clean all modules
	sudo insmod ${BUILD_DIRECTORY}/${MODULE_NAME}.ko
	sudo rmmod ${MODULE_NAME}
	dmesg -H | tail
