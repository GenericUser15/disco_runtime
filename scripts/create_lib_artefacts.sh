#!/bin/bash

cd "$(dirname "$0")"

ASM_DIR="../src/asm"
OUT_DIR="../bin"

arm-none-linux-gnueabihf-as -march=armv7-m ${ASM_DIR}/hard_trampoline.s -o ${OUT_DIR}/hard_trampoline.o

ar crs ${OUT_DIR}/librt.a ${OUT_DIR}/hard_trampoline.o

arm-none-linux-gnueabihf-objdump -Cd ${OUT_DIR}/librt.a

cp ${OUT_DIR}/librt.a ../