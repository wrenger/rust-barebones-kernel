ENTRY(_start)
SECTIONS
{
    . = 0x40000000;
    .text.boot : {
        *(.text.boot)
    }
    .text : {
        *(.text .text.*)
    }
    .data : {
        *(.padata)
		*(.data .data.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .bss : {
        *(.bss .bss.*)
    }
}
