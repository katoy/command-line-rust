/* micro:bit v2 (nRF52833) メモリレイアウト */
MEMORY
{
  /* フラッシュメモリ: 512KB */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  /* RAM: 128KB */
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
