set windows-shell := ["busybox64", "sh", "-c"]

default:
    just --list

faust dsp:
    ../faust/build/bin/Debug/faust.exe -I ../faust/libraries -lang rust benches/{{dsp}}.dsp

faustvec dsp vs="4":
    ../faust/build/bin/Debug/faust.exe -I ../faust/libraries -lang rust benches/{{dsp}}.dsp -vec -vs {{vs}}

bench bench:
    cargo bench --target-dir bench-baseline --bench {{bench}} -- --save-baseline base

bench-reprc bench:
    cargo bench --target-dir bench-baseline --bench {{bench}} --features reprc -- --baseline base

bench-native bench:
    RUSTFLAGS="-C target-cpu=native" cargo bench --target-dir bench-native --bench {{bench}} -- --baseline base

bench-native-reprc bench:
    RUSTFLAGS="-C target-cpu=native" cargo bench --target-dir bench-native --bench {{bench}} --features reprc -- --baseline base

pgo bench:
    RUSTFLAGS="-Cprofile-generate=$(pwd)/pgo-data" cargo bench --target-dir bench-baseline --bench {{bench}}

asm bench="pitchshifter" func="mydsp_vec::compute_original":
    cargo asm -p faust-test-rs --bench {{bench}} --target-dir bench-baseline --no-color --intel --simplify {{bench}}::{{func}} 0 > cargo_asm_output.asm
