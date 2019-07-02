# Include the Rust library
LIBS := -ldigest -L../rust-digest/target/release

ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip $(LIBS)
else
    LDFLAGS := -Wl,--gc-sections $(LIBS)
endif

all: target/main

target:
	@mkdir -p $@

target/main: target/main.o 
	@echo "Linking ... "
	$(CC) -o $@ $^ $(LDFLAGS)

target/main.o: src/main.c | target
	@echo "Compiling ..."
	$(CC) -o $@ -c $<

clean:
	@echo "Removing target/"
	@rm -rf target
