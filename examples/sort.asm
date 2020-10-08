main:
addi	sp,sp,-144
sw	ra,140(sp)
sw	s0,136(sp)
addi	s0,sp,144
sw	zero,-20(s0)
j	main0x44
main0x18:
li	a4,30
lw	a5,-20(s0)
sub	a4,a4,a5
lw	a5,-20(s0)
slli	a5,a5,0x2
addi	a3,s0,-16
add	a5,a3,a5
sw	a4,-124(a5)
lw	a5,-20(s0)
addi	a5,a5,1
sw	a5,-20(s0)
main0x44:
lw	a4,-20(s0)
li	a5,29
ble	a4,a5,main0x18
addi	a5,s0,-140
li	a1,30
mv	a0,a5
jal	sort
li	a5,0
mv	a0,a5
lw	ra,140(sp)
lw	s0,136(sp)
addi	sp,sp,144
ret

sort:
addi	sp,sp,-48
sw	s0,44(sp)
addi	s0,sp,48
sw	a0,-36(s0)
sw	a1,-40(s0)
sw	zero,-24(s0)
j	sort0xe4
sort0x1c:
lw	a5,-24(s0)
sw	a5,-20(s0)
lw	a5,-24(s0)
addi	a5,a5,1
sw	a5,-28(s0)
j	sort0x74
sort0x34:
lw	a5,-20(s0)
slli	a5,a5,0x2
lw	a4,-36(s0)
add	a5,a4,a5
lw	a4,0(a5)
lw	a5,-28(s0)
slli	a5,a5,0x2
lw	a3,-36(s0)
add	a5,a3,a5
lw	a5,0(a5)
ble	a4,a5,sort0x68
lw	a5,-28(s0)
sw	a5,-20(s0)
sort0x68:
lw	a5,-28(s0)
addi	a5,a5,1
sw	a5,-28(s0)
sort0x74:
lw	a4,-28(s0)
lw	a5,-40(s0)
blt	a4,a5,sort0x34
lw	a5,-24(s0)
slli	a5,a5,0x2
lw	a4,-36(s0)
add	a5,a4,a5
lw	a5,0(a5)
sw	a5,-32(s0)
lw	a5,-24(s0)
slli	a5,a5,0x2
lw	a4,-36(s0)
add	a5,a4,a5
lw	a4,-20(s0)
slli	a4,a4,0x2
lw	a3,-36(s0)
add	a4,a3,a4
lw	a4,0(a4)
sw	a4,0(a5)
lw	a5,-20(s0)
slli	a5,a5,0x2
lw	a4,-36(s0)
add	a5,a4,a5
lw	a4,-32(s0)
sw	a4,0(a5)
lw	a5,-24(s0)
addi	a5,a5,1
sw	a5,-24(s0)
sort0xe4:
lw	a4,-24(s0)
lw	a5,-40(s0)
blt	a4,a5,sort0x1c
nop
lw	s0,44(sp)
addi	sp,sp,48
ret
