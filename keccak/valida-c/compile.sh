# To be run in valida-shell
clang -c -O2 -target delendum keccak-valida.c keccak-tiny.c
ld.lld --script=/valida-toolchain/valida.ld \
    /valida-toolchain/lib/delendum-unknown-baremetal-gnu/libc.a \
    /valida-toolchain/DelendumEntryPoint.o \
    keccak-valida.o keccak-tiny.o \
    -o keccak-valida.exe
