/*
MEMORY
{
    RAM : ORIGIN = 0x00000000, LENGTH = 128M
}
*/


MEMORY
{
    RAM : ORIGIN = 0x00000000, LENGTH = 8k
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
/*
We could define in which addres the stack pointer and the heap starts
exemple:
* Define o início da pilha no final da RAM 
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

* Define000100d um ponto intermediário para separar o heap e o stack 
_heap_start = ORIGIN(RAM) + 4K; Heap começa na metade da RAM 
*/

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
