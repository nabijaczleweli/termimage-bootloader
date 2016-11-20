; The MIT License (MIT)
;
; Copyright (c) 2016 nabijaczleweli
;
; Permission is hereby granted, free of charge, to any person obtaining a copy of
; this software and associated documentation files (the "Software"), to deal in
; the Software without restriction, including without limitation the rights to
; use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
; the Software, and to permit persons to whom the Software is furnished to do so,
; subject to the following conditions:
;
; The above copyright notice and this permission notice shall be included in all
; copies or substantial portions of the Software.
;
; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
; IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
; FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
; COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
; IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
; CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


%include "src/boilerplate.inc"


jmp SHORT 3Ch ; jump over BPB and EBR
nop

db 'ImagBoot' ; 8-byte OEM identifier
dw 0200h      ; bytes per sector
db 01h        ; sectors per cluster
dw 0001h      ; # of reserved sectors
db 02h        ; # of FATs
dw 00E0h      ; # of directory entries
dw 0B40h      ; Total # of sectors
db 0F0h       ; Media type
dw 0009h      ; Sectors per FAT
dw 0012h      ; Sectors per track
dw 0002h      ; # of heads/sides
dd 00000000h  ; # of hidden sectors
dd 00000000h  ; # of sectors if more than 65535
