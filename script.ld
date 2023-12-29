/* OUTPUT_FORMAT("elf64-x86-64", "elf64-x86-64",
 	      "elf64-x86-64") */
OUTPUT_FORMAT("binary")
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)

SECTIONS {
	. = 0x10000;
	.text : { *(.text._start) *(.text) }
}
