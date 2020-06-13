INCLUDE memory-fe310.x
MEMORY
{
    FLASH : ORIGIN = 0x20000000, LENGTH = 4M
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

/* We do not have bootloader, start from the beginnig of the flash. */
_stext = 0x20000000;