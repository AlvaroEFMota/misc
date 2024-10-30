# To compile
cargo rustc --release --target riscv32imac-unknown-none-elf -- -C opt-level=3  --emit=asm


# Set up the stack address
remove the fist line in the _start function
`addi	sp, sp, -16`
and replace it for
`li sp, 0x400`
where the 0x400 is the initial address for the stack pointer.
