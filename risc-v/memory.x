MEMORY
{
    RAM : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
    .text : {
        *(.text*)
    } > RAM

    .rodata : {
        *(.rodata*)
    } > RAM

    .data : {
        *(.data*)
    } > RAM

    .bss : {
        *(.bss*)
    } > RAM
}

