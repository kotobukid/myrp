MEMORY
{
  FLASH : ORIGIN = 0x10000000, LENGTH = 2M
  RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}

SECTIONS
{
  /* cortex-m-rt の既定リンカスクリプトを使いつつ、FLASH/RAMだけ上書き */
}
