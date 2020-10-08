sw	t1, 0(zero)
lw	t2, 0(zero)
addi	t1, t2, 5
slti	t2, t1, 4
slti	t2, t1, 7
slti	t2, t1, -1
sltiu	t2, t2, -1
xori	s0, t1, 0
andi	s0, t1, 15
ori	s1, t1, 15
slli	s1, s1, 1
srli	s1, s1, 1
srli	s1, s0, 1
add	s1, s1, s0
sub	s1, s1, s0
sll	s0, s0, s1
slt	s1, s0, s1
sltu	s1, s0, s1
xor	s1, s1, s1
srl	s1, s0, t2
sra	s1, s0, t2
or	s1, s1, s0
and	s1, s1, s0
addi	ra, zero, 12
jalr	zero, ra, 0
