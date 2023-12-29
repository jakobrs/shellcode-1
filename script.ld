/* OUTPUT_FORMAT("elf64-x86-64", "elf64-x86-64",
 	      "elf64-x86-64") */
OUTPUT_FORMAT("binary")
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)

SECTIONS {
	/* override with -C link-arg=-Wl,--defsym=loadaddr=ADDRESS */
	PROVIDE(loadaddr = 0x1000000);
	. = loadaddr;

	/* /DISCARD/ : { *(*unlikely*) } */

	/* Note: much of this is not really necessary */
	/* ld automatically adds the parts that we forget most of the time */
	/* *(.text._start) is needed to ensure that the entry point is the very first thing in the file */
	.text : { *(.text._start) *(.text.*) }
	.rodata : { *(.rodata.*) }
}
