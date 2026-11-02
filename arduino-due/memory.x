/* Arduino Due: SAM3X8E.
   Flash is two 256 KB banks, contiguous from 0x00080000 (bank 1 starts at
   0x000C0000); the SAM-BA bootloader lives in ROM, not in flash, so the
   whole 512 KB is the firmware's. RAM is linked at the 0x20070000 mirror,
   where SRAM0 (64 KB) and SRAM1 (32 KB) appear contiguous. */
MEMORY
{
  FLASH : ORIGIN = 0x00080000, LENGTH = 512K
  RAM   : ORIGIN = 0x20070000, LENGTH = 96K
}
