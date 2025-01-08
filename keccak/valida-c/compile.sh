# To be run in valida-shell
clang -c -O2 -target valida keccak-valida.c keccak-tiny.c
ld.lld --script=/valida-toolchain/valida.ld \
    /valida-toolchain/lib/valida-unknown-baremetal-gnu/libc.a \
    /valida-toolchain/ValidaEntryPoint.o \
    keccak-valida.o keccak-tiny.o \
    -o keccak-valida.exe
