cargo asm -p faust-test-rs --bench rms --target-dir bench-baseline^
  --no-color --intel --simplify rms::mydsp::rms_faust > cargo_asm_output.asm
