cargo asm -p faust-test-rs --bench karplus --target-dir bench-baseline^
  --no-color --intel --simplify karplus::mydsp::karplus_faust > cargo_asm_output.asm
