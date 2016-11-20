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


showchar:
mov ah, 0Eh ; BIOS character display
mov al, 'A'

.showchar_loop:
int 10h
inc al

cmp al, 'Z' + 1
jne SHORT .showchar_loop

blerb:
mov al, 0Dh
int 10h
mov al, 0Ah
int 10h
jmp SHORT showchar


; Length of BPB - 36
; Length of EBR - 23
times (510-36-23)-($-$$) hlt
dw 0AA55h
