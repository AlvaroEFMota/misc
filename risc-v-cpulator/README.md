# To compile
cargo rustc --release --target riscv32imac-unknown-none-elf -- -C opt-level=3  --emit=asm


# Set up the stack address
remove the fist line in the _start function
`addi	sp, sp, -16`
and replace it for
`li sp, 0x400`
where the 0x400 is the initial address for the stack pointer.

### maybe util code 
`unsafe {
    let reg_base: *mut u8 = 0x4001_0000 as *mut u8;

    // Escrevendo nos 4 bytes de um registrador de 32 bits usando `*mut u8`
    ptr::write_volatile(reg_base.offset(0), 0xAA); // Primeiro byte
    ptr::write_volatile(reg_base.offset(1), 0xBB); // Segundo byte
    ptr::write_volatile(reg_base.offset(2), 0xCC); // Terceiro byte
    ptr::write_volatile(reg_base.offset(3), 0xDD); // Quarto byte
}
Nesse exemplo, usamos *mut u8 e offset() para mover o ponteiro para as próximas posições de memória e escrever um valor em cada byte.

Usando *mut u32 para Acessar Todos os 32 Bits de Uma Vez
Se quisermos acessar todos os 4 bytes (32 bits) de uma vez, podemos usar *mut u32:

rust
Copy code
unsafe {
    let reg_base: *mut u32 = 0x4001_0000 as *mut u32;

    // Escrevendo diretamente um valor de 32 bits no registrador
    ptr::write_volatile(reg_base, 0xDDCCBBAA); // Escreve os 4 bytes de uma vez
}
Nesse exemplo, estamos escrevendo todos os 32 bits de uma vez no registrador, o que é mais conveniente quando se trata de um registrador de hardware que precisa de um valor de 32 bits.
` 
