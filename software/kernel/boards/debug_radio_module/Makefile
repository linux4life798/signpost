# Makefile for building the tock kernel for the Hail platform

CHIP=sam4l
PLATFORM=debug_radio_module

include ../../../kernel/tock/Makefile.common



# XXX This is a bit ugly, but is a nice feature
target/$(CHIP)/release/$(PLATFORM):	src/version.rs

.PHONY: FORCE
VCMD := echo \"pub static GIT_VERSION: &'static str = \\\"$$(git describe --always || echo notgit)\\\";\"
src/version.rs: FORCE
	@bash -c "cmp -s <($(VCMD)) <(test -e $@ && cat $@) || $(VCMD) > $@"



JLINK=JLinkExe
JLINK_OPTIONS+=-device ATSAM4LC8C -if swd -speed 1200 -AutoConnect 1
JLINK_SCRIPTS_DIR =jtag

# upload kernel over JTAG
.PHONY: flash
flash: target/sam4l/release/$(PLATFORM).hex
	$(JLINK) $(JLINK_OPTIONS) $(JLINK_SCRIPTS_DIR)/flash-kernel.jlink

.PHONY: flash-debug
flash-debug: target/sam4l/debug/$(PLATFORM).hex
	$(JLINK) $(JLINK_OPTIONS) $(JLINK_SCRIPTS_DIR)/flash-debug.jlink
