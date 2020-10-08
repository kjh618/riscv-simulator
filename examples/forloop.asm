main:
addi	sp,sp,-48
sw	s0,44(sp)
addi	s0,sp,48
sw	zero,-20(s0)
j	main0x38
main0x14:
lw	a5,-20(s0)
slli	a5,a5,0x2
addi	a4,s0,-16
add	a5,a4,a5
lw	a4,-20(s0)
sw	a4,-24(a5)
lw	a5,-20(s0)
addi	a5,a5,1
sw	a5,-20(s0)
main0x38:
lw	a4,-20(s0)
li	a5,4
ble	a4,a5,main0x14
li	a5,0
mv	a0,a5
lw	s0,44(sp)
addi	sp,sp,48
ret
