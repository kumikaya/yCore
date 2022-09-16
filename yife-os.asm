
target/riscv64gc-unknown-none-elf/debug/yife-os:	file format elf64-littleriscv

Disassembly of section .debug_abbrev:

0000000000000000 <.debug_abbrev>:
       0: 01 11        	addi	sp, sp, -32
       2: 01 25        	sext.w	a0, a0
       4: 0e 13        	slli	t1, t1, 35
       6: 05 03        	addi	t1, t1, 1
       8: 0e 10        	c.slli	zero, 35
       a: 17 1b 0e b4  	auipc	s6, 737505
       e: 42 19        	slli	s2, s2, 48
      10: 11 01        	addi	sp, sp, 4
      12: 12 06        	slli	a2, a2, 4
      14: 00 00        	unimp	
      16: 02 39        	fld	fs2, 32(sp)
      18: 01 03        	mv	t1, t1
      1a: 0e 00        	c.slli	zero, 3
      1c: 00 03        	addi	s0, sp, 384
      1e: 04 01        	addi	s1, sp, 128
      20: 49 13        	addi	t1, t1, -14
      22: 6d 19        	addi	s2, s2, -5
      24: 03 0e 0b 0b  	lb	t3, 176(s6)
      28: 88 01        	addi	a0, sp, 192
      2a: 0f 00 00 04  	fence	o, 0
      2e: 28 00        	addi	a0, sp, 8
      30: 03 0e 1c 0f  	lb	t3, 241(s8)
      34: 00 00        	unimp	
      36: 05 13        	addi	t1, t1, -31
      38: 01 03        	mv	t1, t1
      3a: 0e 0b        	slli	s6, s6, 3
      3c: 0b 88 01 0f  	<unknown>
      40: 00 00        	unimp	
      42: 06 0d        	slli	s10, s10, 1
      44: 00 03        	addi	s0, sp, 384
      46: 0e 49        	lw	s2, 192(sp)
      48: 13 88 01 0f  	addi	a6, gp, 240
      4c: 38 0b        	addi	a4, sp, 408
      4e: 00 00        	unimp	
      50: 07 33 01 15  	fld	ft6, 336(sp)
      54: 13 00 00 08  	li	zero, 128
      58: 0d 00        	c.nop	3
      5a: 49 13        	addi	t1, t1, -14
      5c: 88 01        	addi	a0, sp, 192
      5e: 0f 38 0b 34  	<unknown>
      62: 19 00        	c.nop	6
      64: 00 09        	addi	s0, sp, 144
      66: 19 01        	addi	sp, sp, 6
      68: 16 0b        	slli	s6, s6, 5
      6a: 00 00        	unimp	
      6c: 0a 13        	slli	t1, t1, 34
      6e: 00 03        	addi	s0, sp, 384
      70: 0e 0b        	slli	s6, s6, 3
      72: 0b 88 01 0f  	<unknown>
      76: 00 00        	unimp	
      78: 0b 19 01 00  	<unknown>
      7c: 00 0c        	addi	s0, sp, 528
      7e: 2f 00 49 13  	<unknown>
      82: 03 0e 00 00  	lb	t3, 0(zero)
      86: 0d 24        	addiw	s0, s0, 3
      88: 00 03        	addi	s0, sp, 384
      8a: 0e 3e        	fld	ft8, 224(sp)
      8c: 0b 0b 0b 00  	<unknown>
      90: 00 0e        	addi	s0, sp, 784
      92: 2e 01        	slli	sp, sp, 11
      94: 11 01        	addi	sp, sp, 4
      96: 12 06        	slli	a2, a2, 4
      98: 40 18        	addi	s0, sp, 52
      9a: 6e 0e        	slli	t3, t3, 27
      9c: 03 0e 3a 0b  	lb	t3, 179(s4)
      a0: 3b 0b 87 01  	addw	s6, a4, s8
      a4: 19 00        	c.nop	6
      a6: 00 0f        	addi	s0, sp, 912
      a8: 05 00        	c.nop	1
      aa: 02 18        	slli	a6, a6, 32
      ac: 03 0e 3a 0b  	lb	t3, 179(s4)
      b0: 3b 0b 49 13  	<unknown>
      b4: 00 00        	unimp	
      b6: 10 0f        	addi	a2, sp, 912
      b8: 00 49        	lw	s0, 16(a0)
      ba: 13 03 0e 33  	addi	t1, t3, 816
      be: 06 00        	c.slli	zero, 1
      c0: 00 11        	addi	s0, sp, 160
      c2: 0f 00 49 13  	<unknown>
      c6: 33 06 00 00  	add	a2, zero, zero
      ca: 12 01        	slli	sp, sp, 4
      cc: 01 49        	li	s2, 0
      ce: 13 00 00 13  	li	zero, 304
      d2: 21 00        	c.nop	8
      d4: 49 13        	addi	t1, t1, -14
      d6: 22 0d        	slli	s10, s10, 8
      d8: 37 0b 00 00  	lui	s6, 0
      dc: 14 24        	fld	fa3, 8(s0)
      de: 00 03        	addi	s0, sp, 384
      e0: 0e 0b        	slli	s6, s6, 3
      e2: 0b 3e 0b 00  	<unknown>
      e6: 00 15        	addi	s0, sp, 672
      e8: 15 01        	addi	sp, sp, 5
      ea: 49 13        	addi	t1, t1, -14
      ec: 00 00        	unimp	
      ee: 16 05        	slli	a0, a0, 5
      f0: 00 49        	lw	s0, 16(a0)
      f2: 13 00 00 00  	nop

Disassembly of section .debug_info:

0000000000000000 <.debug_info>:
       0: bb 05 00 00  	addw	a1, zero, zero
       4: 04 00        	<unknown>
       6: 00 00        	unimp	
       8: 00 00        	unimp	
       a: 08 01        	addi	a0, sp, 128
       c: 72 02        	slli	tp, tp, 28
       e: 00 00        	unimp	
      10: 1c 00        	<unknown>
      12: ac 00        	addi	a1, sp, 72
      14: 00 00        	unimp	
      16: 00 00        	unimp	
      18: 00 00        	unimp	
      1a: 3f 03 00 00  	<unknown>
		...
      26: 08 00        	<unknown>
      28: 00 00        	unimp	
      2a: 02 59        	lw	s2, 32(sp)
      2c: 01 00        	nop
      2e: 00 02        	addi	s0, sp, 256
      30: 48 02        	addi	a0, sp, 260
      32: 00 00        	unimp	
      34: 02 3a        	fld	fs4, 32(sp)
      36: 01 00        	nop
      38: 00 02        	addi	s0, sp, 256
      3a: 7e 04        	slli	s0, s0, 31
      3c: 00 00        	unimp	
      3e: 03 e2 03 00  	lwu	tp, 0(t2)
      42: 00 25        	fld	fs0, 8(a0)
      44: 01 00        	nop
      46: 00 01        	addi	s0, sp, 128
      48: 01 04        	mv	s0, s0
      4a: 20 01        	addi	s0, sp, 136
      4c: 00 00        	unimp	
      4e: 00 04        	addi	s0, sp, 512
      50: 5c 03        	addi	a5, sp, 388
      52: 00 00        	unimp	
      54: 01 04        	mv	s0, s0
      56: d1 01        	addi	gp, gp, 20
      58: 00 00        	unimp	
      5a: 02 04        	c.slli64	s0
      5c: 81 04        	mv	s1, s1
      5e: 00 00        	unimp	
      60: 03 00 05 36  	lb	zero, 864(a0)
      64: 03 00 00 38  	lb	zero, 896(zero)
      68: 08 06        	addi	a0, sp, 768
      6a: 17 01 00 00  	auipc	sp, 0
      6e: 72 04        	slli	s0, s0, 28
      70: 00 00        	unimp	
      72: 08 00        	<unknown>
      74: 06 e0        	sd	ra, 0(sp)
      76: 01 00        	nop
      78: 00 80        	<unknown>
      7a: 00 00        	unimp	
      7c: 00 08        	addi	s0, sp, 16
      7e: 08 00        	<unknown>
      80: 05 0f        	addi	t5, t5, 1
      82: 02 00        	c.slli64	zero
      84: 00 30        	fld	fs0, 32(s0)
      86: 08 06        	addi	a0, sp, 768
      88: 5e 00        	c.slli	zero, 23
      8a: 00 00        	unimp	
      8c: 09 05        	addi	a0, a0, 2
      8e: 00 00        	unimp	
      90: 04 20        	fld	fs1, 0(s0)
      92: 06 69        	ld	s2, 64(sp)
      94: 03 00 00 3e  	lb	zero, 992(zero)
      98: 00 00        	unimp	
      9a: 00 01        	addi	s0, sp, 128
      9c: 28 06        	addi	a0, sp, 776
      9e: de 02        	slli	t0, t0, 23
      a0: 00 00        	unimp	
      a2: 10 05        	addi	a2, sp, 640
      a4: 00 00        	unimp	
      a6: 04 24        	fld	fs1, 8(s0)
      a8: 06 e7        	sd	ra, 392(sp)
      aa: 02 00        	c.slli64	zero
      ac: 00 bf        	fsd	fs0, 56(a4)
      ae: 00 00        	unimp	
      b0: 00 08        	addi	s0, sp, 16
      b2: 00 06        	addi	s0, sp, 768
      b4: d1 00        	addi	ra, ra, 20
      b6: 00 00        	unimp	
      b8: bf 00 00 00  	<unknown>
      bc: 08 10        	addi	a0, sp, 32
      be: 00 05        	addi	s0, sp, 640
      c0: 30 03        	addi	a2, sp, 392
      c2: 00 00        	unimp	
      c4: 10 08        	addi	a2, sp, 16
      c6: 07 cb 00 00  	<unknown>
      ca: 00 08        	addi	s0, sp, 16
      cc: 80 04        	addi	s0, sp, 576
      ce: 00 00        	unimp	
      d0: 08 00        	<unknown>
      d2: 09 00        	c.nop	2
      d4: 06 64        	ld	s0, 64(sp)
      d6: 01 00        	nop
      d8: 00 fd        	sd	s0, 56(a0)
      da: 00 00        	unimp	
      dc: 00 08        	addi	s0, sp, 16
      de: 00 00        	unimp	
      e0: 09 01        	addi	sp, sp, 2
      e2: 06 f9        	sd	ra, 176(sp)
      e4: 02 00        	c.slli64	zero
      e6: 00 10        	addi	s0, sp, 32
      e8: 01 00        	nop
      ea: 00 08        	addi	s0, sp, 16
      ec: 00 00        	unimp	
      ee: 09 02        	addi	tp, tp, 2
      f0: 06 a8        	fsd	ft1, 16(sp)
      f2: 01 00        	nop
      f4: 00 23        	fld	fs0, 0(a4)
      f6: 01 00        	nop
      f8: 00 08        	addi	s0, sp, 16
      fa: 00 00        	unimp	
      fc: 00 05        	addi	s0, sp, 640
      fe: 64 01        	addi	s1, sp, 140
     100: 00 00        	unimp	
     102: 10 08        	addi	a2, sp, 16
     104: 06 0a        	slli	s4, s4, 1
     106: 04 00        	<unknown>
     108: 00 72        	ld	s0, 32(a2)
     10a: 04 00        	<unknown>
     10c: 00 08        	addi	s0, sp, 16
     10e: 08 00        	<unknown>
     110: 05 f9        	bnez	a0, 0x40 <.debug_info+0x40>
     112: 02 00        	c.slli64	zero
     114: 00 10        	addi	s0, sp, 32
     116: 08 06        	addi	a0, sp, 768
     118: 0a 04        	slli	s0, s0, 2
     11a: 00 00        	unimp	
     11c: 72 04        	slli	s0, s0, 28
     11e: 00 00        	unimp	
     120: 08 08        	addi	a0, sp, 16
     122: 00 0a        	addi	s0, sp, 272
     124: a8 01        	addi	a0, sp, 200
     126: 00 00        	unimp	
     128: 10 08        	addi	a2, sp, 16
     12a: 00 00        	unimp	
     12c: 00 05        	addi	s0, sp, 640
     12e: 3e 02        	slli	tp, tp, 15
     130: 00 00        	unimp	
     132: 30 08        	addi	a2, sp, 24
     134: 06 49        	lw	s2, 64(sp)
     136: 01 00        	nop
     138: 00 94        	<unknown>
     13a: 04 00        	<unknown>
     13c: 00 08        	addi	s0, sp, 16
     13e: 00 06        	addi	s0, sp, 768
     140: 48 02        	addi	a0, sp, 260
     142: 00 00        	unimp	
     144: a4 02        	addi	s1, sp, 328
     146: 00 00        	unimp	
     148: 08 10        	addi	a0, sp, 32
     14a: 06 39        	fld	fs2, 96(sp)
     14c: 02 00        	c.slli64	zero
     14e: 00 17        	addi	s0, sp, 928
     150: 05 00        	c.nop	1
     152: 00 08        	addi	s0, sp, 16
     154: 20 00        	addi	s0, sp, 8
     156: 05 03        	addi	t1, t1, 1
     158: 01 00        	nop
     15a: 00 10        	addi	s0, sp, 32
     15c: 08 06        	addi	a0, sp, 768
     15e: 12 04        	slli	s0, s0, 4
     160: 00 00        	unimp	
     162: 3e 05        	slli	a0, a0, 15
     164: 00 00        	unimp	
     166: 08 00        	<unknown>
     168: 06 dc        	sw	ra, 56(sp)
     16a: 00 00        	unimp	
     16c: 00 4b        	lw	s0, 16(a4)
     16e: 05 00        	c.nop	1
     170: 00 08        	addi	s0, sp, 16
     172: 08 00        	<unknown>
     174: 02 b3        	fsd	ft0, 416(sp)
     176: 02 00        	c.slli64	zero
     178: 00 0a        	addi	s0, sp, 272
     17a: 18 04        	addi	a4, sp, 512
     17c: 00 00        	unimp	
     17e: 00 01        	addi	s0, sp, 128
     180: 00 0a        	addi	s0, sp, 272
     182: 5e 01        	slli	sp, sp, 23
     184: 00 00        	unimp	
     186: 00 01        	addi	s0, sp, 128
     188: 05 97        	srai	a4, a4, 33
     18a: 04 00        	<unknown>
     18c: 00 40        	lw	s0, 0(s0)
     18e: 08 06        	addi	a0, sp, 768
     190: de 02        	slli	t0, t0, 23
     192: 00 00        	unimp	
     194: 10 05        	addi	a2, sp, 640
     196: 00 00        	unimp	
     198: 04 30        	fld	fs1, 32(s0)
     19a: 06 5e        	lw	t3, 96(sp)
     19c: 00 00        	unimp	
     19e: 00 09        	addi	s0, sp, 144
     1a0: 05 00        	c.nop	1
     1a2: 00 04        	addi	s0, sp, 512
     1a4: 34 06        	addi	a3, sp, 776
     1a6: 69 03        	addi	t1, t1, 26
     1a8: 00 00        	unimp	
     1aa: 3e 00        	c.slli	zero, 15
     1ac: 00 00        	unimp	
     1ae: 01 38        	addiw	a6, a6, -32
     1b0: 06 d1        	sw	ra, 160(sp)
     1b2: 00 00        	unimp	
     1b4: 00 01        	addi	s0, sp, 128
     1b6: 03 00 00 08  	lb	zero, 128(zero)
     1ba: 00 06        	addi	s0, sp, 768
     1bc: e7 02 00 00  	jalr	t0, zero
     1c0: 01 03        	mv	t1, t1
     1c2: 00 00        	unimp	
     1c4: 08 10        	addi	a0, sp, 32
     1c6: 06 a0        	fsd	ft1, 0(sp)
     1c8: 00 00        	unimp	
     1ca: 00 7c        	ld	s0, 56(s0)
     1cc: 05 00        	c.nop	1
     1ce: 00 08        	addi	s0, sp, 16
     1d0: 20 00        	addi	s0, sp, 8
     1d2: 00 02        	addi	s0, sp, 256
     1d4: 09 02        	addi	tp, tp, 2
     1d6: 00 00        	unimp	
     1d8: 02 2f        	fld	ft10, 0(sp)
     1da: 01 00        	nop
     1dc: 00 05        	addi	s0, sp, 640
     1de: 8a 00        	slli	ra, ra, 2
     1e0: 00 00        	unimp	
     1e2: 28 08        	addi	a0, sp, 24
     1e4: 06 d8        	sw	ra, 48(sp)
     1e6: 01 00        	nop
     1e8: 00 2a        	fld	fs0, 16(a2)
     1ea: 04 00        	<unknown>
     1ec: 00 08        	addi	s0, sp, 16
     1ee: 00 06        	addi	s0, sp, 768
     1f0: a0 01        	addi	s0, sp, 200
     1f2: 00 00        	unimp	
     1f4: 47 02 00 00  	fmsub.s	ft4, ft0, ft0, ft0, rne
     1f8: 08 10        	addi	a0, sp, 32
     1fa: 06 50        	<unknown>
     1fc: 01 00        	nop
     1fe: 00 aa        	fsd	fs0, 16(a2)
     200: 05 00        	c.nop	1
     202: 00 08        	addi	s0, sp, 16
     204: 18 06        	addi	a4, sp, 768
     206: d3 02 00 00  	fadd.s	ft5, ft0, ft0, rne
     20a: b7 05 00 00  	lui	a1, 0
     20e: 01 20        	<unknown>
     210: 00 00        	unimp	
     212: 02 50        	<unknown>
     214: 01 00        	nop
     216: 00 05        	addi	s0, sp, 640
     218: 30 02        	addi	a2, sp, 264
     21a: 00 00        	unimp	
     21c: 18 08        	addi	a4, sp, 16
     21e: 06 e7        	sd	ra, 392(sp)
     220: 01 00        	nop
     222: 00 bb        	fsd	fs0, 48(a4)
     224: 04 00        	<unknown>
     226: 00 08        	addi	s0, sp, 16
     228: 00 06        	addi	s0, sp, 768
     22a: 39 04        	addi	s0, s0, 14
     22c: 00 00        	unimp	
     22e: 10 05        	addi	a2, sp, 640
     230: 00 00        	unimp	
     232: 04 10        	addi	s1, sp, 32
     234: 06 0e        	slli	t3, t3, 1
     236: 01 00        	nop
     238: 00 10        	addi	s0, sp, 32
     23a: 05 00        	c.nop	1
     23c: 00 04        	addi	s0, sp, 512
     23e: 14 00        	<unknown>
     240: 00 00        	unimp	
     242: 02 99        	jalr	s2
     244: 00 00        	unimp	
     246: 00 05        	addi	s0, sp, 640
     248: 6e 01        	slli	sp, sp, 27
     24a: 00 00        	unimp	
     24c: 08 08        	addi	a0, sp, 16
     24e: 07 53 02 00  	<unknown>
     252: 00 08        	addi	s0, sp, 16
     254: 80 04        	addi	s0, sp, 576
     256: 00 00        	unimp	
     258: 08 00        	<unknown>
     25a: 09 00        	c.nop	2
     25c: 06 3e        	fld	ft8, 96(sp)
     25e: 04 00        	<unknown>
     260: 00 76        	ld	s0, 40(a2)
     262: 02 00        	c.slli64	zero
     264: 00 08        	addi	s0, sp, 16
     266: 00 00        	unimp	
     268: 0b 06 d7 00  	<unknown>
     26c: 00 00        	unimp	
     26e: 87 02 00 00  	<unknown>
     272: 08 00        	<unknown>
     274: 00 00        	unimp	
     276: 05 3e        	addiw	t3, t3, -31
     278: 04 00        	<unknown>
     27a: 00 08        	addi	s0, sp, 16
     27c: 08 0c        	addi	a0, sp, 528
     27e: 87 04 00 00  	<unknown>
     282: 3d 01        	addi	sp, sp, 15
     284: 00 00        	unimp	
     286: 00 05        	addi	s0, sp, 640
     288: d7 00 00 00  	<unknown>
     28c: 08 08        	addi	a0, sp, 16
     28e: 0c 87        	<unknown>
     290: 04 00        	<unknown>
     292: 00 3d        	fld	fs0, 56(a0)
     294: 01 00        	nop
     296: 00 06        	addi	s0, sp, 768
     298: 0a 04        	slli	s0, s0, 2
     29a: 00 00        	unimp	
     29c: 87 04 00 00  	<unknown>
     2a0: 08 00        	<unknown>
     2a2: 00 00        	unimp	
     2a4: 05 63        	lui	t1, 1
     2a6: 00 00        	unimp	
     2a8: 00 10        	addi	s0, sp, 32
     2aa: 08 07        	addi	a0, sp, 896
     2ac: b0 02        	addi	a2, sp, 328
     2ae: 00 00        	unimp	
     2b0: 08 80        	<unknown>
     2b2: 04 00        	<unknown>
     2b4: 00 08        	addi	s0, sp, 16
     2b6: 00 09        	addi	s0, sp, 144
     2b8: 00 06        	addi	s0, sp, 768
     2ba: 3e 04        	slli	s0, s0, 15
     2bc: 00 00        	unimp	
     2be: d3 02 00 00  	fadd.s	ft5, ft0, ft0, rne
     2c2: 08 00        	<unknown>
     2c4: 00 0b        	addi	s0, sp, 400
     2c6: 06 d7        	sw	ra, 172(sp)
     2c8: 00 00        	unimp	
     2ca: 00 e4        	sd	s0, 8(s0)
     2cc: 02 00        	c.slli64	zero
     2ce: 00 08        	addi	s0, sp, 16
     2d0: 00 00        	unimp	
     2d2: 00 05        	addi	s0, sp, 640
     2d4: 3e 04        	slli	s0, s0, 15
     2d6: 00 00        	unimp	
     2d8: 10 08        	addi	a2, sp, 16
     2da: 0c e2        	sd	a1, 0(a2)
     2dc: 04 00        	<unknown>
     2de: 00 3d        	fld	fs0, 56(a0)
     2e0: 01 00        	nop
     2e2: 00 00        	unimp	
     2e4: 05 d7        	beqz	a4, 0x20c <.debug_info+0x20c>
     2e6: 00 00        	unimp	
     2e8: 00 10        	addi	s0, sp, 32
     2ea: 08 0c        	addi	a0, sp, 528
     2ec: e2 04        	slli	s1, s1, 24
     2ee: 00 00        	unimp	
     2f0: 3d 01        	addi	sp, sp, 15
     2f2: 00 00        	unimp	
     2f4: 06 0a        	slli	s4, s4, 1
     2f6: 04 00        	<unknown>
     2f8: 00 e2        	sd	s0, 0(a2)
     2fa: 04 00        	<unknown>
     2fc: 00 08        	addi	s0, sp, 16
     2fe: 00 00        	unimp	
     300: 00 05        	addi	s0, sp, 640
     302: 6b 04 00 00  	<unknown>
     306: 10 08        	addi	a2, sp, 16
     308: 07 0d 03 00  	<unknown>
     30c: 00 08        	addi	s0, sp, 16
     30e: 80 04        	addi	s0, sp, 576
     310: 00 00        	unimp	
     312: 08 00        	<unknown>
     314: 09 00        	c.nop	2
     316: 06 3e        	fld	ft8, 96(sp)
     318: 04 00        	<unknown>
     31a: 00 31        	fld	fs0, 32(a0)
     31c: 03 00 00 08  	lb	zero, 128(zero)
     320: 00 00        	unimp	
     322: 09 01        	addi	sp, sp, 2
     324: 06 d7        	sw	ra, 172(sp)
     326: 00 00        	unimp	
     328: 00 42        	lw	s0, 0(a2)
     32a: 03 00 00 08  	lb	zero, 128(zero)
     32e: 00 00        	unimp	
     330: 00 05        	addi	s0, sp, 640
     332: 3e 04        	slli	s0, s0, 15
     334: 00 00        	unimp	
     336: 10 08        	addi	a2, sp, 16
     338: 0c 72        	ld	a1, 32(a2)
     33a: 04 00        	<unknown>
     33c: 00 3d        	fld	fs0, 56(a0)
     33e: 01 00        	nop
     340: 00 00        	unimp	
     342: 05 d7        	beqz	a4, 0x26a <.debug_info+0x26a>
     344: 00 00        	unimp	
     346: 00 10        	addi	s0, sp, 32
     348: 08 0c        	addi	a0, sp, 528
     34a: 72 04        	slli	s0, s0, 28
     34c: 00 00        	unimp	
     34e: 3d 01        	addi	sp, sp, 15
     350: 00 00        	unimp	
     352: 06 0a        	slli	s4, s4, 1
     354: 04 00        	<unknown>
     356: 00 72        	ld	s0, 32(a2)
     358: 04 00        	<unknown>
     35a: 00 08        	addi	s0, sp, 16
     35c: 08 00        	<unknown>
     35e: 00 00        	unimp	
     360: 02 3f        	fld	ft10, 32(sp)
     362: 01 00        	nop
     364: 00 05        	addi	s0, sp, 640
     366: e6 00        	slli	ra, ra, 25
     368: 00 00        	unimp	
     36a: 01 01        	mv	sp, sp
     36c: 07 71 03 00  	<unknown>
     370: 00 08        	addi	s0, sp, 16
     372: e2 03        	slli	t2, t2, 24
     374: 00 00        	unimp	
     376: 01 00        	nop
     378: 09 00        	c.nop	2
     37a: 06 67        	ld	a4, 64(sp)
     37c: 01 00        	nop
     37e: 00 95        	<unknown>
     380: 03 00 00 01  	lb	zero, 16(zero)
     384: 00 00        	unimp	
     386: 09 01        	addi	sp, sp, 2
     388: 06 6a        	ld	s4, 64(sp)
     38a: 01 00        	nop
     38c: 00 ba        	fsd	fs0, 48(a2)
     38e: 03 00 00 01  	lb	zero, 16(zero)
     392: 00 00        	unimp	
     394: 00 05        	addi	s0, sp, 640
     396: 67 01 00 00  	jalr	sp, zero
     39a: 01 01        	mv	sp, sp
     39c: 0c 68        	ld	a1, 16(s0)
     39e: 05 00        	c.nop	1
     3a0: 00 3d        	fld	fs0, 56(a0)
     3a2: 01 00        	nop
     3a4: 00 0c        	addi	s0, sp, 528
     3a6: 81 01        	mv	gp, gp
     3a8: 00 00        	unimp	
     3aa: 4c 02        	addi	a1, sp, 260
     3ac: 00 00        	unimp	
     3ae: 06 0a        	slli	s4, s4, 1
     3b0: 04 00        	<unknown>
     3b2: 00 68        	ld	s0, 16(s0)
     3b4: 05 00        	c.nop	1
     3b6: 00 01        	addi	s0, sp, 128
     3b8: 01 00        	nop
     3ba: 05 6a        	lui	s4, 1
     3bc: 01 00        	nop
     3be: 00 01        	addi	s0, sp, 128
     3c0: 01 0c        	mv	s8, s8
     3c2: 68 05        	addi	a0, sp, 652
     3c4: 00 00        	unimp	
     3c6: 3d 01        	addi	sp, sp, 15
     3c8: 00 00        	unimp	
     3ca: 0c 81        	<unknown>
     3cc: 01 00        	nop
     3ce: 00 4c        	lw	s0, 24(s0)
     3d0: 02 00        	c.slli64	zero
     3d2: 00 06        	addi	s0, sp, 768
     3d4: 0a 04        	slli	s0, s0, 2
     3d6: 00 00        	unimp	
     3d8: 81 01        	mv	gp, gp
     3da: 00 00        	unimp	
     3dc: 01 01        	mv	sp, sp
     3de: 00 00        	unimp	
     3e0: 00 00        	unimp	
     3e2: 0d 46        	li	a2, 3
     3e4: 01 00        	nop
     3e6: 00 07        	addi	s0, sp, 896
     3e8: 01 02        	mv	tp, tp
     3ea: 89 04        	addi	s1, s1, 2
     3ec: 00 00        	unimp	
     3ee: 02 fe        	sd	zero, 312(sp)
     3f0: 01 00        	nop
     3f2: 00 0e        	addi	s0, sp, 784
		...
     3fc: 08 00        	<unknown>
     3fe: 00 00        	unimp	
     400: 01 52        	li	tp, -32
     402: ec 01        	addi	a1, sp, 204
     404: 00 00        	unimp	
     406: 09 02        	addi	tp, tp, 2
     408: 00 00        	unimp	
     40a: 01 04        	mv	s0, s0
     40c: 0f 02 91 08  	<unknown>
     410: cb 00 00 00  	fnmsub.s	ft1, ft0, ft0, ft0, rne
     414: 01 04        	mv	s0, s0
     416: 1d 04        	addi	s0, s0, 7
     418: 00 00        	unimp	
     41a: 00 00        	unimp	
     41c: 00 10        	addi	s0, sp, 32
     41e: dd 01        	addi	gp, gp, 23
     420: 00 00        	unimp	
     422: 4e 02        	slli	tp, tp, 19
     424: 00 00        	unimp	
     426: 00 00        	unimp	
     428: 00 00        	unimp	
     42a: 05 00        	c.nop	1
     42c: 00 00        	unimp	
     42e: 00 10        	addi	s0, sp, 32
     430: 08 06        	addi	a0, sp, 768
     432: a4 00        	addi	s1, sp, 72
     434: 00 00        	unimp	
     436: 48 04        	addi	a0, sp, 516
     438: 00 00        	unimp	
     43a: 08 00        	<unknown>
     43c: 06 62        	ld	tp, 64(sp)
     43e: 03 00 00 58  	lb	zero, 1408(zero)
     442: 04 00        	<unknown>
     444: 00 08        	addi	s0, sp, 16
     446: 08 00        	<unknown>
     448: 11 51        	li	sp, -28
     44a: 04 00        	<unknown>
     44c: 00 00        	unimp	
     44e: 00 00        	unimp	
     450: 00 0a        	addi	s0, sp, 272
     452: ff 02 00 00  	<unknown>
     456: 00 01        	addi	s0, sp, 128
     458: 10 65        	ld	a2, 8(a0)
     45a: 04 00        	<unknown>
     45c: 00 fe        	sd	s0, 56(a2)
     45e: 03 00 00 00  	lb	zero, 0(zero)
     462: 00 00        	unimp	
     464: 00 12        	addi	s0, sp, 288
     466: 72 04        	slli	s0, s0, 28
     468: 00 00        	unimp	
     46a: 13 79 04 00  	andi	s2, s0, 0
     46e: 00 00        	unimp	
     470: 03 00 0d 91  	lb	zero, -1776(s10)
     474: 04 00        	<unknown>
     476: 00 07        	addi	s0, sp, 896
     478: 08 14        	addi	a0, sp, 544
     47a: 8c 01        	addi	a1, sp, 192
     47c: 00 00        	unimp	
     47e: 08 07        	addi	a0, sp, 896
     480: 0d fa        	bnez	a2, 0x3b2 <.debug_info+0x3b2>
     482: 03 00 00 07  	lb	zero, 112(zero)
     486: 08 10        	addi	a0, sp, 32
     488: 2d 01        	addi	sp, sp, 11
     48a: 00 00        	unimp	
     48c: 1a 02        	slli	tp, tp, 6
     48e: 00 00        	unimp	
     490: 00 00        	unimp	
     492: 00 00        	unimp	
     494: 05 f1        	bnez	a0, 0x3b4 <.debug_info+0x3b4>
     496: 02 00        	c.slli64	zero
     498: 00 10        	addi	s0, sp, 32
     49a: 08 06        	addi	a0, sp, 768
     49c: 43 04 00 00  	fmadd.s	fs0, ft0, ft0, ft0, rne
     4a0: b2 04        	slli	s1, s1, 12
     4a2: 00 00        	unimp	
     4a4: 08 00        	<unknown>
     4a6: 06 29        	fld	fs2, 64(sp)
     4a8: 03 00 00 72  	lb	zero, 1824(zero)
     4ac: 04 00        	<unknown>
     4ae: 00 08        	addi	s0, sp, 16
     4b0: 08 00        	<unknown>
     4b2: 11 bb        	j	0x1c6 <.debug_info+0x1c6>
     4b4: 04 00        	<unknown>
     4b6: 00 00        	unimp	
     4b8: 00 00        	unimp	
     4ba: 00 05        	addi	s0, sp, 640
     4bc: 12 01        	slli	sp, sp, 4
     4be: 00 00        	unimp	
     4c0: 10 08        	addi	a2, sp, 16
     4c2: 06 43        	lw	t1, 64(sp)
     4c4: 04 00        	<unknown>
     4c6: 00 d9        	sw	s0, 48(a0)
     4c8: 04 00        	<unknown>
     4ca: 00 08        	addi	s0, sp, 16
     4cc: 00 06        	addi	s0, sp, 768
     4ce: 29 03        	addi	t1, t1, 10
     4d0: 00 00        	unimp	
     4d2: 72 04        	slli	s0, s0, 28
     4d4: 00 00        	unimp	
     4d6: 08 08        	addi	a0, sp, 16
     4d8: 00 11        	addi	s0, sp, 160
     4da: e2 03        	slli	t2, t2, 24
     4dc: 00 00        	unimp	
     4de: 00 00        	unimp	
     4e0: 00 00        	unimp	
     4e2: 05 6f        	lui	t5, 1
     4e4: 03 00 00 10  	lb	zero, 256(zero)
     4e8: 08 06        	addi	a0, sp, 768
     4ea: 43 04 00 00  	fmadd.s	fs0, ft0, ft0, ft0, rne
     4ee: 00 05        	addi	s0, sp, 640
     4f0: 00 00        	unimp	
     4f2: 08 00        	<unknown>
     4f4: 06 29        	fld	fs2, 64(sp)
     4f6: 03 00 00 72  	lb	zero, 1824(zero)
     4fa: 04 00        	<unknown>
     4fc: 00 08        	addi	s0, sp, 16
     4fe: 08 00        	<unknown>
     500: 11 62        	lui	tp, 4
     502: 00 00        	unimp	
     504: 00 00        	unimp	
     506: 00 00        	unimp	
     508: 00 0d        	addi	s0, sp, 656
     50a: 94 00        	addi	a3, sp, 64
     50c: 00 00        	unimp	
     50e: 10 04        	addi	a2, sp, 512
     510: 0d 0e        	addi	t3, t3, 3
     512: 04 00        	<unknown>
     514: 00 07        	addi	s0, sp, 896
     516: 04 05        	addi	s1, sp, 640
     518: 45 00        	c.nop	17
     51a: 00 00        	unimp	
     51c: 10 08        	addi	a2, sp, 16
     51e: 06 43        	lw	t1, 64(sp)
     520: 04 00        	<unknown>
     522: 00 35        	fld	fs0, 40(a0)
     524: 05 00        	c.nop	1
     526: 00 08        	addi	s0, sp, 16
     528: 00 06        	addi	s0, sp, 768
     52a: 29 03        	addi	t1, t1, 10
     52c: 00 00        	unimp	
     52e: 72 04        	slli	s0, s0, 28
     530: 00 00        	unimp	
     532: 08 08        	addi	a0, sp, 16
     534: 00 11        	addi	s0, sp, 160
     536: 56 01        	slli	sp, sp, 21
     538: 00 00        	unimp	
     53a: 00 00        	unimp	
     53c: 00 00        	unimp	
     53e: 10 79        	ld	a2, 48(a0)
     540: 01 00        	nop
     542: 00 4c        	lw	s0, 24(s0)
     544: 04 00        	<unknown>
     546: 00 00        	unimp	
     548: 00 00        	unimp	
     54a: 00 10        	addi	s0, sp, 32
     54c: 58 05        	addi	a4, sp, 644
     54e: 00 00        	unimp	
     550: 8e 03        	slli	t2, t2, 3
     552: 00 00        	unimp	
     554: 00 00        	unimp	
     556: 00 00        	unimp	
     558: 15 65        	lui	a0, 5
     55a: 03 00 00 16  	lb	zero, 352(zero)
     55e: 3e 05        	slli	a0, a0, 15
     560: 00 00        	unimp	
     562: 16 6f        	ld	t5, 320(sp)
     564: 05 00        	c.nop	1
     566: 00 00        	unimp	
     568: 0d e4        	bnez	s0, 0x592 <.debug_info+0x592>
     56a: 02 00        	c.slli64	zero
     56c: 00 07        	addi	s0, sp, 896
     56e: 00 10        	addi	s0, sp, 32
     570: 88 01        	addi	a0, sp, 192
     572: 00 00        	unimp	
     574: 2b 00 00 00  	<unknown>
     578: 00 00        	unimp	
     57a: 00 00        	unimp	
     57c: 05 1f        	addi	t5, t5, -31
     57e: 04 00        	<unknown>
     580: 00 10        	addi	s0, sp, 32
     582: 08 06        	addi	a0, sp, 768
     584: a4 00        	addi	s1, sp, 72
     586: 00 00        	unimp	
     588: 9a 05        	slli	a1, a1, 6
     58a: 00 00        	unimp	
     58c: 08 00        	<unknown>
     58e: 06 62        	ld	tp, 64(sp)
     590: 03 00 00 58  	lb	zero, 1408(zero)
     594: 04 00        	<unknown>
     596: 00 08        	addi	s0, sp, 16
     598: 08 00        	<unknown>
     59a: 11 a3        	j	0xa9e <.debug_info+0xa9e>
     59c: 05 00        	c.nop	1
     59e: 00 00        	unimp	
     5a0: 00 00        	unimp	
     5a2: 00 0a        	addi	s0, sp, 272
     5a4: be 02        	slli	t0, t0, 15
     5a6: 00 00        	unimp	
     5a8: 00 01        	addi	s0, sp, 128
     5aa: 10 17        	addi	a2, sp, 928
     5ac: 02 00        	c.slli64	zero
     5ae: 00 b0        	fsd	fs0, 32(s0)
     5b0: 01 00        	nop
     5b2: 00 00        	unimp	
     5b4: 00 00        	unimp	
     5b6: 00 0d        	addi	s0, sp, 656
     5b8: 79 04        	addi	s0, s0, 30
     5ba: 00 00        	unimp	
     5bc: 02 01        	c.slli64	sp
     5be: 00           	<unknown>

Disassembly of section .debug_aranges:

0000000000000000 <.debug_aranges>:
       0: 2c 00        	addi	a1, sp, 8
       2: 00 00        	unimp	
       4: 02 00        	c.slli64	zero
       6: 00 00        	unimp	
       8: 00 00        	unimp	
       a: 08 00        	<unknown>
       c: ff ff ff ff  	<unknown>
		...
      18: 08 00        	<unknown>
		...
      2e: 00 00        	unimp	

Disassembly of section .debug_str:

0000000000000000 <.debug_str>:
       0: 26 28        	fld	fa6, 72(sp)
       2: 64 79        	ld	s1, 240(a0)
       4: 6e 20        	fld	ft0, 216(sp)
       6: 63 6f 72 65  	bltu	tp, s7, 0x664 <.debug_info+0x664>
       a: 3a 3a        	fld	fs4, 424(sp)
       c: 61 6e        	lui	t3, 24
       e: 79 3a        	addiw	s4, s4, -2
      10: 3a 41        	lw	sp, 140(sp)
      12: 6e 79        	ld	s2, 248(sp)
      14: 20 2b        	fld	fs0, 80(a4)
      16: 20 63        	ld	s0, 64(a4)
      18: 6f 72 65 3a  	jal	tp, 0x573be <.debug_info+0x573be>
      1c: 3a 6d        	ld	s10, 392(sp)
      1e: 61 72        	lui	tp, 1048568
      20: 6b 65 72 3a  	<unknown>
      24: 3a 53        	lw	t1, 172(sp)
      26: 65 6e        	lui	t3, 25
      28: 64 29        	fld	fs1, 208(a0)
      2a: 00 26        	fld	fs0, 8(a2)
      2c: 6d 75        	lui	a0, 1048571
      2e: 74 20        	fld	fa3, 192(s0)
      30: 63 6f 72 65  	bltu	tp, s7, 0x68e <.debug_info+0x68e>
      34: 3a 3a        	fld	fs4, 424(sp)
      36: 66 6d        	ld	s10, 88(sp)
      38: 74 3a        	fld	fa3, 240(a2)
      3a: 3a 46        	lw	a2, 140(sp)
      3c: 6f 72 6d 61  	jal	tp, 0xd7652 <.debug_info+0xd7652>
      40: 74 74        	ld	a3, 232(s0)
      42: 65 72        	lui	tp, 1048569
      44: 00 26        	fld	fs0, 8(a2)
      46: 5b 63 6f 72  	<unknown>
      4a: 65 3a        	addiw	s4, s4, -7
      4c: 3a 66        	ld	a2, 392(sp)
      4e: 6d 74        	lui	s0, 1048571
      50: 3a 3a        	fld	fs4, 424(sp)
      52: 41 72        	lui	tp, 1048560
      54: 67 75 6d 65  	<unknown>
      58: 6e 74        	ld	s0, 248(sp)
      5a: 56 31        	fld	ft2, 368(sp)
      5c: 5d 00        	c.nop	23
      5e: 66 69        	ld	s2, 88(sp)
      60: 6c 6c        	ld	a1, 216(s0)
      62: 00 4f        	lw	s0, 24(a4)
      64: 70 74        	ld	a2, 232(s0)
      66: 69 6f        	lui	t5, 26
      68: 6e 3c        	fld	fs8, 248(sp)
      6a: 26 5b        	lw	s6, 104(sp)
      6c: 63 6f 72 65  	bltu	tp, s7, 0x6ca <.debug_info+0x6ca>
      70: 3a 3a        	fld	fs4, 424(sp)
      72: 66 6d        	ld	s10, 88(sp)
      74: 74 3a        	fld	fa3, 240(a2)
      76: 3a 72        	ld	tp, 424(sp)
      78: 74 3a        	fld	fa3, 240(a2)
      7a: 3a 76        	ld	a2, 424(sp)
      7c: 31 3a        	addiw	s4, s4, -20
      7e: 3a 41        	lw	sp, 140(sp)
      80: 72 67        	ld	a4, 280(sp)
      82: 75 6d        	lui	s10, 29
      84: 65 6e        	lui	t3, 25
      86: 74 5d        	lw	a3, 124(a0)
      88: 3e 00        	c.slli	zero, 15
      8a: 50 61        	ld	a2, 128(a0)
      8c: 6e 69        	ld	s2, 216(sp)
      8e: 63 49 6e 66  	blt	t3, t1, 0x700 <.debug_info+0x700>
      92: 6f 00 63 68  	j	0x30718 <.debug_info+0x30718>
      96: 61 72        	lui	tp, 1048568
      98: 00 6f        	ld	s0, 24(a4)
      9a: 70 74        	ld	a2, 232(s0)
      9c: 69 6f        	lui	t5, 26
      9e: 6e 00        	c.slli	zero, 27
      a0: 62 75        	ld	a0, 56(sp)
      a2: 66 00        	c.slli	zero, 25
      a4: 70 6f        	ld	a2, 216(a4)
      a6: 69 6e        	lui	t3, 26
      a8: 74 65        	ld	a3, 200(a0)
      aa: 72 00        	c.slli	zero, 28
      ac: 73 72 63 2f  	csrrci	tp, 758, 6
      b0: 6d 61        	addi	sp, sp, 240
      b2: 69 6e        	lui	t3, 26
      b4: 2e 72        	ld	tp, 232(sp)
      b6: 73 2f 40 2f  	csrr	t5, 756
      ba: 32 32        	fld	ft4, 296(sp)
      bc: 78 64        	ld	a4, 200(s0)
      be: 63 67 37 64  	bltu	a4, gp, 0x70c <.debug_info+0x70c>
      c2: 6a 71        	ld	sp, 184(sp)
      c4: 65 66        	lui	a2, 25
      c6: 75 39        	addiw	s2, s2, -3
      c8: 7a 37        	fld	fa4, 440(sp)
      ca: 00 5f        	lw	s0, 56(a4)
      cc: 69 6e        	lui	t3, 26
      ce: 66 6f        	ld	t5, 88(sp)
      d0: 00 77        	ld	s0, 40(a4)
      d2: 69 64        	lui	s0, 26
      d4: 74 68        	ld	a3, 208(s0)
      d6: 00 53        	lw	s0, 32(a4)
      d8: 6f 6d 65 00  	jal	s10, 0x560de <.debug_info+0x560de>
      dc: 66 6f        	ld	t5, 88(sp)
      de: 72 6d        	ld	s10, 280(sp)
      e0: 61 74        	lui	s0, 1048568
      e2: 74 65        	ld	a3, 200(a0)
      e4: 72 00        	c.slli	zero, 28
      e6: 52 65        	ld	a0, 272(sp)
      e8: 73 75 6c 74  	csrrci	a0, 1862, 24
      ec: 3c 28        	fld	fa5, 80(s0)
      ee: 29 2c        	addiw	s8, s8, 10
      f0: 20 63        	ld	s0, 64(a4)
      f2: 6f 72 65 3a  	jal	tp, 0x57498 <.debug_info+0x57498>
      f6: 3a 66        	ld	a2, 392(sp)
      f8: 6d 74        	lui	s0, 1048571
      fa: 3a 3a        	fld	fs4, 424(sp)
      fc: 45 72        	lui	tp, 1048561
      fe: 72 6f        	ld	t5, 280(sp)
     100: 72 3e        	fld	ft8, 312(sp)
     102: 00 41        	lw	s0, 0(a0)
     104: 72 67        	ld	a4, 280(sp)
     106: 75 6d        	lui	s10, 29
     108: 65 6e        	lui	t3, 25
     10a: 74 56        	lw	a3, 108(a2)
     10c: 31 00        	c.nop	12
     10e: 63 6f 6c 00  	bltu	s8, t1, 0x12c <.debug_info+0x12c>
     112: 26 73        	ld	t1, 104(sp)
     114: 74 72        	ld	a3, 224(a2)
     116: 00 70        	ld	s0, 32(s0)
     118: 6f 73 69 74  	jal	t1, 0x9785e <.debug_info+0x9785e>
     11c: 69 6f        	lui	t5, 26
     11e: 6e 00        	c.slli	zero, 27
     120: 4c 65        	ld	a1, 136(a0)
     122: 66 74        	ld	s0, 120(sp)
     124: 00 41        	lw	s0, 0(a0)
     126: 6c 69        	ld	a1, 208(a0)
     128: 67 6e 6d 65  	<unknown>
     12c: 6e 74        	ld	s0, 248(sp)
     12e: 00 70        	ld	s0, 32(s0)
     130: 61 6e        	lui	t3, 24
     132: 69 63        	lui	t1, 26
     134: 5f 69 6e 66  	<unknown>
     138: 6f 00 72 74  	j	0x2107e <.debug_info+0x2107e>
     13c: 00 54        	lw	s0, 40(s0)
     13e: 00 72        	ld	s0, 32(a2)
     140: 65 73        	lui	t1, 1048569
     142: 75 6c        	lui	s8, 29
     144: 74 00        	addi	a3, sp, 12
     146: 75 38        	addiw	a6, a6, -3
     148: 00 70        	ld	s0, 32(s0)
     14a: 69 65        	lui	a0, 26
     14c: 63 65 73 00  	bltu	t1, t2, 0x156 <.debug_info+0x156>
     150: 6c 6f        	ld	a1, 216(a4)
     152: 63 61 74 69  	bltu	s0, s7, 0x7d4 <.debug_info+0x7d4>
     156: 6f 6e 00 63  	jal	t3, 0x6786 <.debug_info+0x6786>
     15a: 6f 72 65 00  	jal	tp, 0x57160 <.debug_info+0x57160>
     15e: 45 72        	lui	tp, 1048561
     160: 72 6f        	ld	t5, 280(sp)
     162: 72 00        	c.slli	zero, 28
     164: 49 73        	lui	t1, 1048562
     166: 00 4f        	lw	s0, 24(a4)
     168: 6b 00 45 72  	<unknown>
     16c: 72 00        	c.slli	zero, 28
     16e: 4f 70 74 69  	fnmadd.s	ft0, fs0, fs7, fa3
     172: 6f 6e 3c 26  	jal	t3, 0xc6bd4 <.debug_info+0xc6bd4>
     176: 63 6f 72 65  	bltu	tp, s7, 0x7d4 <.debug_info+0x7d4>
     17a: 3a 3a        	fld	fs4, 424(sp)
     17c: 66 6d        	ld	s10, 88(sp)
     17e: 74 3a        	fld	fa3, 240(a2)
     180: 3a 41        	lw	sp, 140(sp)
     182: 72 67        	ld	a4, 280(sp)
     184: 75 6d        	lui	s10, 29
     186: 65 6e        	lui	t3, 25
     188: 74 73        	ld	a3, 224(a4)
     18a: 3e 00        	c.slli	zero, 15
     18c: 5f 5f 41 52  	<unknown>
     190: 52 41        	lw	sp, 20(sp)
     192: 59 5f        	li	t5, -10
     194: 53 49 5a 45  	<unknown>
     198: 5f 54 59 50  	<unknown>
     19c: 45 5f        	li	t5, -15
     19e: 5f 00 6d 65  	<unknown>
     1a2: 73 73 61 67  	csrrci	t1, 1654, 2
     1a6: 65 00        	c.nop	25
     1a8: 49 6d        	lui	s10, 18
     1aa: 70 6c        	ld	a2, 216(s0)
     1ac: 69 65        	lui	a0, 26
     1ae: 64 00        	addi	s1, sp, 12
     1b0: 26 63        	ld	t1, 72(sp)
     1b2: 6f 72 65 3a  	jal	tp, 0x57558 <.debug_info+0x57558>
     1b6: 3a 70        	<unknown>
     1b8: 61 6e        	lui	t3, 24
     1ba: 69 63        	lui	t1, 26
     1bc: 3a 3a        	fld	fs4, 424(sp)
     1be: 6c 6f        	ld	a1, 216(a4)
     1c0: 63 61 74 69  	bltu	s0, s7, 0x842 <.debug_info+0x842>
     1c4: 6f 6e 3a 3a  	jal	t3, 0xa6d66 <.debug_info+0xa6d66>
     1c8: 4c 6f        	ld	a1, 152(a4)
     1ca: 63 61 74 69  	bltu	s0, s7, 0x84c <.debug_info+0x84c>
     1ce: 6f 6e 00 43  	jal	t3, 0x65fe <.debug_info+0x65fe>
     1d2: 65 6e        	lui	t3, 25
     1d4: 74 65        	ld	a3, 200(a0)
     1d6: 72 00        	c.slli	zero, 28
     1d8: 70 61        	ld	a2, 192(a0)
     1da: 79 6c        	lui	s8, 30
     1dc: 6f 61 64 00  	jal	sp, 0x461e2 <.debug_info+0x461e2>
     1e0: 66 6f        	ld	t5, 88(sp)
     1e2: 72 6d        	ld	s10, 280(sp)
     1e4: 61 74        	lui	s0, 1048568
     1e6: 00 66        	ld	s0, 8(a2)
     1e8: 69 6c        	lui	s8, 26
     1ea: 65 00        	c.nop	25
     1ec: 72 75        	ld	a0, 312(sp)
     1ee: 73 74 5f 62  	csrrci	s0, 1573, 30
     1f2: 65 67        	lui	a4, 25
     1f4: 69 6e        	lui	t3, 26
     1f6: 5f 75 6e 77  	<unknown>
     1fa: 69 6e        	lui	t3, 26
     1fc: 64 00        	addi	s1, sp, 12
     1fe: 6c 61        	ld	a1, 192(a0)
     200: 6e 67        	ld	a4, 216(sp)
     202: 5f 69 74 65  	<unknown>
     206: 6d 73        	lui	t1, 1048571
     208: 00 70        	ld	s0, 32(s0)
     20a: 61 6e        	lui	t3, 24
     20c: 69 63        	lui	t1, 26
     20e: 00 46        	lw	s0, 8(a2)
     210: 6f 72 6d 61  	jal	tp, 0xd7826 <.debug_info+0xd7826>
     214: 74 53        	lw	a3, 100(a4)
     216: 70 65        	ld	a2, 200(a0)
     218: 63 00 26 63  	beq	a2, s2, 0x838 <.debug_info+0x838>
     21c: 6f 72 65 3a  	jal	tp, 0x575c2 <.debug_info+0x575c2>
     220: 3a 66        	ld	a2, 392(sp)
     222: 6d 74        	lui	s0, 1048571
     224: 3a 3a        	fld	fs4, 424(sp)
     226: 41 72        	lui	tp, 1048560
     228: 67 75 6d 65  	<unknown>
     22c: 6e 74        	ld	s0, 248(sp)
     22e: 73 00 4c 6f  	<unknown>
     232: 63 61 74 69  	bltu	s0, s7, 0x8b4 <.debug_info+0x8b4>
     236: 6f 6e 00 61  	jal	t3, 0x6846 <.debug_info+0x6846>
     23a: 72 67        	ld	a4, 280(sp)
     23c: 73 00 41 72  	<unknown>
     240: 67 75 6d 65  	<unknown>
     244: 6e 74        	ld	s0, 248(sp)
     246: 73 00 66 6d  	<unknown>
     24a: 74 00        	addi	a3, sp, 12
     24c: 45 00        	c.nop	17
     24e: 26 63        	ld	t1, 72(sp)
     250: 6f 72 65 3a  	jal	tp, 0x575f6 <.debug_info+0x575f6>
     254: 3a 70        	<unknown>
     256: 61 6e        	lui	t3, 24
     258: 69 63        	lui	t1, 26
     25a: 3a 3a        	fld	fs4, 424(sp)
     25c: 70 61        	ld	a2, 192(a0)
     25e: 6e 69        	ld	s2, 216(sp)
     260: 63 5f 69 6e  	bge	s2, t1, 0x95e <.debug_info+0x95e>
     264: 66 6f        	ld	t5, 88(sp)
     266: 3a 3a        	fld	fs4, 424(sp)
     268: 50 61        	ld	a2, 128(a0)
     26a: 6e 69        	ld	s2, 216(sp)
     26c: 63 49 6e 66  	blt	t3, t1, 0x8de <.debug_info+0x8de>
     270: 6f 00 63 6c  	j	0x30936 <.debug_info+0x30936>
     274: 61 6e        	lui	t3, 24
     276: 67 20 4c 4c  	<unknown>
     27a: 56 4d        	lw	s10, 84(sp)
     27c: 20 28        	fld	fs0, 80(s0)
     27e: 72 75        	ld	a0, 312(sp)
     280: 73 74 63 20  	csrrci	s0, 518, 6
     284: 76 65        	ld	a0, 344(sp)
     286: 72 73        	ld	t1, 312(sp)
     288: 69 6f        	lui	t5, 26
     28a: 6e 20        	fld	ft0, 216(sp)
     28c: 31 2e        	addiw	t3, t3, 12
     28e: 36 35        	fld	fa0, 360(sp)
     290: 2e 30        	fld	ft0, 232(sp)
     292: 2d 6e        	lui	t3, 11
     294: 69 67        	lui	a4, 26
     296: 68 74        	ld	a0, 232(s0)
     298: 6c 79        	ld	a1, 240(a0)
     29a: 20 28        	fld	fs0, 80(s0)
     29c: 32 32        	fld	ft4, 296(sp)
     29e: 38 37        	fld	fa4, 104(a4)
     2a0: 31 30        	<unknown>
     2a2: 37 35 38 20  	lui	a0, 131971
     2a6: 32 30        	fld	ft0, 296(sp)
     2a8: 32 32        	fld	ft4, 296(sp)
     2aa: 2d 30        	<unknown>
     2ac: 39 2d        	addiw	s10, s10, 14
     2ae: 31 30        	<unknown>
     2b0: 29 29        	addiw	s2, s2, 10
     2b2: 00 7b        	ld	s0, 48(a4)
     2b4: 65 78        	lui	a6, 1048569
     2b6: 74 65        	ld	a3, 200(a0)
     2b8: 72 6e        	ld	t3, 280(sp)
     2ba: 23 30 7d 00  	sd	t2, 0(s10)
     2be: 64 79        	ld	s1, 240(a0)
     2c0: 6e 20        	fld	ft0, 216(sp)
     2c2: 63 6f 72 65  	bltu	tp, s7, 0x920 <.debug_info+0x920>
     2c6: 3a 3a        	fld	fs4, 424(sp)
     2c8: 66 6d        	ld	s10, 88(sp)
     2ca: 74 3a        	fld	fa3, 240(a2)
     2cc: 3a 57        	lw	a4, 172(sp)
     2ce: 72 69        	ld	s2, 280(sp)
     2d0: 74 65        	ld	a3, 200(a0)
     2d2: 00 63        	ld	s0, 0(a4)
     2d4: 61 6e        	lui	t3, 24
     2d6: 5f 75 6e 77  	<unknown>
     2da: 69 6e        	lui	t3, 26
     2dc: 64 00        	addi	s1, sp, 12
     2de: 66 6c        	ld	s8, 88(sp)
     2e0: 61 67        	lui	a4, 24
     2e2: 73 00 28 29  	<unknown>
     2e6: 00 70        	ld	s0, 32(s0)
     2e8: 72 65        	ld	a0, 280(sp)
     2ea: 63 69 73 69  	bltu	t1, s7, 0x97c <.debug_info+0x97c>
     2ee: 6f 6e 00 26  	jal	t3, 0x654e <.debug_info+0x654e>
     2f2: 5b 26 73 74  	<unknown>
     2f6: 72 5d        	lw	s10, 60(sp)
     2f8: 00 50        	lw	s0, 32(s0)
     2fa: 61 72        	lui	tp, 1048568
     2fc: 61 6d        	lui	s10, 24
     2fe: 00 28        	fld	fs0, 16(s0)
     300: 64 79        	ld	s1, 240(a0)
     302: 6e 20        	fld	ft0, 216(sp)
     304: 63 6f 72 65  	bltu	tp, s7, 0x962 <.debug_info+0x962>
     308: 3a 3a        	fld	fs4, 424(sp)
     30a: 61 6e        	lui	t3, 24
     30c: 79 3a        	addiw	s4, s4, -2
     30e: 3a 41        	lw	sp, 140(sp)
     310: 6e 79        	ld	s2, 248(sp)
     312: 20 2b        	fld	fs0, 80(a4)
     314: 20 63        	ld	s0, 64(a4)
     316: 6f 72 65 3a  	jal	tp, 0x576bc <.debug_info+0x576bc>
     31a: 3a 6d        	ld	s10, 392(sp)
     31c: 61 72        	lui	tp, 1048568
     31e: 6b 65 72 3a  	<unknown>
     322: 3a 53        	lw	t1, 172(sp)
     324: 65 6e        	lui	t3, 25
     326: 64 29        	fld	fs1, 208(a0)
     328: 00 6c        	ld	s0, 24(s0)
     32a: 65 6e        	lui	t3, 25
     32c: 67 74 68 00  	<unknown>
     330: 43 6f 75 6e  	<unknown>
     334: 74 00        	addi	a3, sp, 12
     336: 41 72        	lui	tp, 1048560
     338: 67 75 6d 65  	<unknown>
     33c: 6e 74        	ld	s0, 248(sp)
     33e: 00 2f        	fld	fs0, 24(a4)
     340: 68 6f        	ld	a0, 216(a4)
     342: 6d 65        	lui	a0, 27
     344: 2f 79 69 66  	<unknown>
     348: 65 2f        	addiw	t5, t5, 25
     34a: 72 75        	ld	a0, 312(sp)
     34c: 73 74 2d 77  	csrrci	s0, 1906, 26
     350: 6f 72 6b 2f  	jal	tp, 0xb7646 <.debug_info+0xb7646>
     354: 79 69        	lui	s2, 30
     356: 66 65        	ld	a0, 88(sp)
     358: 2d 6f        	lui	t5, 11
     35a: 73 00 52 69  	<unknown>
     35e: 67 68 74 00  	<unknown>
     362: 76 74        	ld	s0, 376(sp)
     364: 61 62        	lui	tp, 24
     366: 6c 65        	ld	a1, 200(a0)
     368: 00 61        	ld	s0, 0(a0)
     36a: 6c 69        	ld	a1, 208(a0)
     36c: 67 6e 00 26  	<unknown>
     370: 5b 63 6f 72  	<unknown>
     374: 65 3a        	addiw	s4, s4, -7
     376: 3a 66        	ld	a2, 392(sp)
     378: 6d 74        	lui	s0, 1048571
     37a: 3a 3a        	fld	fs4, 424(sp)
     37c: 72 74        	ld	s0, 312(sp)
     37e: 3a 3a        	fld	fs4, 424(sp)
     380: 76 31        	fld	ft2, 376(sp)
     382: 3a 3a        	fld	fs4, 424(sp)
     384: 41 72        	lui	tp, 1048560
     386: 67 75 6d 65  	<unknown>
     38a: 6e 74        	ld	s0, 248(sp)
     38c: 5d 00        	c.nop	23
     38e: 66 6e        	ld	t3, 88(sp)
     390: 28 26        	fld	fa0, 72(a2)
     392: 63 6f 72 65  	bltu	tp, s7, 0x9f0 <.debug_info+0x9f0>
     396: 3a 3a        	fld	fs4, 424(sp)
     398: 66 6d        	ld	s10, 88(sp)
     39a: 74 3a        	fld	fa3, 240(a2)
     39c: 3a 7b        	ld	s6, 424(sp)
     39e: 65 78        	lui	a6, 1048569
     3a0: 74 65        	ld	a3, 200(a0)
     3a2: 72 6e        	ld	t3, 280(sp)
     3a4: 23 30 7d 3a  	sd	t2, 928(s10)
     3a8: 3a 4f        	lw	t5, 140(sp)
     3aa: 70 61        	ld	a2, 192(a0)
     3ac: 71 75        	lui	a0, 1048572
     3ae: 65 2c        	addiw	s8, s8, 25
     3b0: 20 26        	fld	fs0, 72(a2)
     3b2: 6d 75        	lui	a0, 1048571
     3b4: 74 20        	fld	fa3, 192(s0)
     3b6: 63 6f 72 65  	bltu	tp, s7, 0xa14 <.debug_info+0xa14>
     3ba: 3a 3a        	fld	fs4, 424(sp)
     3bc: 66 6d        	ld	s10, 88(sp)
     3be: 74 3a        	fld	fa3, 240(a2)
     3c0: 3a 46        	lw	a2, 140(sp)
     3c2: 6f 72 6d 61  	jal	tp, 0xd79d8 <.debug_info+0xd79d8>
     3c6: 74 74        	ld	a3, 232(s0)
     3c8: 65 72        	lui	tp, 1048569
     3ca: 29 20        	<unknown>
     3cc: 2d 3e        	addiw	t3, t3, -21
     3ce: 20 63        	ld	s0, 64(a4)
     3d0: 6f 72 65 3a  	jal	tp, 0x57776 <.debug_info+0x57776>
     3d4: 3a 72        	ld	tp, 424(sp)
     3d6: 65 73        	lui	t1, 1048569
     3d8: 75 6c        	lui	s8, 29
     3da: 74 3a        	fld	fa3, 240(a2)
     3dc: 3a 52        	lw	tp, 172(sp)
     3de: 65 73        	lui	t1, 1048569
     3e0: 75 6c        	lui	s8, 29
     3e2: 74 3c        	fld	fa3, 248(s0)
     3e4: 28 29        	fld	fa0, 80(a0)
     3e6: 2c 20        	fld	fa1, 64(s0)
     3e8: 63 6f 72 65  	bltu	tp, s7, 0xa46 <.debug_info+0xa46>
     3ec: 3a 3a        	fld	fs4, 424(sp)
     3ee: 66 6d        	ld	s10, 88(sp)
     3f0: 74 3a        	fld	fa3, 240(a2)
     3f2: 3a 45        	lw	a0, 140(sp)
     3f4: 72 72        	ld	tp, 312(sp)
     3f6: 6f 72 3e 00  	jal	tp, 0xe7bf8 <.debug_info+0xe7bf8>
     3fa: 75 36        	addiw	a2, a2, -3
     3fc: 34 00        	addi	a3, sp, 8
     3fe: 26 5b        	lw	s6, 104(sp)
     400: 75 73        	lui	t1, 1048573
     402: 69 7a        	lui	s4, 1048570
     404: 65 3b        	addiw	s6, s6, -7
     406: 20 33        	fld	fs0, 96(a4)
     408: 5d 00        	c.nop	23
     40a: 5f 5f 30 00  	<unknown>
     40e: 75 33        	addiw	t1, t1, -3
     410: 32 00        	c.slli	zero, 12
     412: 76 61        	ld	sp, 344(sp)
     414: 6c 75        	ld	a1, 232(a0)
     416: 65 00        	c.nop	25
     418: 4f 70 61 71  	fnmadd.s	ft0, ft2, fs6, fa4
     41c: 75 65        	lui	a0, 29
     41e: 00 26        	fld	fs0, 8(a2)
     420: 6d 75        	lui	a0, 1048571
     422: 74 20        	fld	fa3, 192(s0)
     424: 64 79        	ld	s1, 240(a0)
     426: 6e 20        	fld	ft0, 216(sp)
     428: 63 6f 72 65  	bltu	tp, s7, 0xa86 <.debug_info+0xa86>
     42c: 3a 3a        	fld	fs4, 424(sp)
     42e: 66 6d        	ld	s10, 88(sp)
     430: 74 3a        	fld	fa3, 240(a2)
     432: 3a 57        	lw	a4, 172(sp)
     434: 72 69        	ld	s2, 280(sp)
     436: 74 65        	ld	a3, 200(a0)
     438: 00 6c        	ld	s0, 24(s0)
     43a: 69 6e        	lui	t3, 26
     43c: 65 00        	c.nop	25
     43e: 4e 6f        	ld	t5, 208(sp)
     440: 6e 65        	ld	a0, 216(sp)
     442: 00 64        	ld	s0, 8(s0)
     444: 61 74        	lui	s0, 1048568
     446: 61 5f        	li	t5, -8
     448: 70 74        	ld	a2, 232(s0)
     44a: 72 00        	c.slli	zero, 28
     44c: 26 63        	ld	t1, 72(sp)
     44e: 6f 72 65 3a  	jal	tp, 0x577f4 <.debug_info+0x577f4>
     452: 3a 66        	ld	a2, 392(sp)
     454: 6d 74        	lui	s0, 1048571
     456: 3a 3a        	fld	fs4, 424(sp)
     458: 7b 65 78 74  	<unknown>
     45c: 65 72        	lui	tp, 1048569
     45e: 6e 23        	fld	ft6, 216(sp)
     460: 30 7d        	ld	a2, 120(a0)
     462: 3a 3a        	fld	fs4, 424(sp)
     464: 4f 70 61 71  	fnmadd.s	ft0, ft2, fs6, fa4
     468: 75 65        	lui	a0, 29
     46a: 00 4f        	lw	s0, 24(a4)
     46c: 70 74        	ld	a2, 232(s0)
     46e: 69 6f        	lui	t5, 26
     470: 6e 3c        	fld	fs8, 248(sp)
     472: 75 73        	lui	t1, 1048573
     474: 69 7a        	lui	s4, 1048570
     476: 65 3e        	addiw	t3, t3, -7
     478: 00 62        	ld	s0, 0(a2)
     47a: 6f 6f 6c 00  	jal	t5, 0xc6480 <.debug_info+0xc6480>
     47e: 76 31        	fld	ft2, 376(sp)
     480: 00 55        	lw	s0, 40(a0)
     482: 6e 6b        	ld	s6, 216(sp)
     484: 6e 6f        	ld	t5, 216(sp)
     486: 77 6e 00 79  	<unknown>
     48a: 69 66        	lui	a2, 26
     48c: 65 5f        	li	t5, -7
     48e: 6f 73 00 75  	jal	t1, 0x7bde <.debug_info+0x7bde>
     492: 73 69 7a 65  	csrrsi	s2, 1623, 20
     496: 00 46        	lw	s0, 8(a2)
     498: 6f 72 6d 61  	jal	tp, 0xd7aae <.debug_info+0xd7aae>
     49c: 74 74        	ld	a3, 232(s0)
     49e: 65 72        	lui	tp, 1048569
     4a0: 00           	<unknown>

Disassembly of section .debug_pubnames:

0000000000000000 <.debug_pubnames>:
       0: bd 00        	addi	ra, ra, 15
       2: 00 00        	unimp	
       4: 02 00        	c.slli64	zero
       6: 00 00        	unimp	
       8: 00 00        	unimp	
       a: bf 05 00 00  	<unknown>
       e: 55 00        	c.nop	21
      10: 00 00        	unimp	
      12: 43 65 6e 74  	<unknown>
      16: 65 72        	lui	tp, 1048569
      18: 00 74        	ld	s0, 40(s0)
      1a: 01 00        	nop
      1c: 00 7b        	ld	s0, 48(a4)
      1e: 65 78        	lui	a6, 1048569
      20: 74 65        	ld	a3, 200(a0)
      22: 72 6e        	ld	t3, 280(sp)
      24: 23 30 7d 00  	sd	t2, 0(s10)
      28: ee 03        	slli	t2, t2, 27
      2a: 00 00        	unimp	
      2c: 6c 61        	ld	a1, 192(a0)
      2e: 6e 67        	ld	a4, 216(sp)
      30: 5f 69 74 65  	<unknown>
      34: 6d 73        	lui	t1, 1048571
      36: 00 34        	fld	fs0, 40(s0)
      38: 00 00        	unimp	
      3a: 00 72        	ld	s0, 32(a2)
      3c: 74 00        	addi	a3, sp, 12
      3e: 2f 00 00 00  	<unknown>
      42: 66 6d        	ld	s10, 88(sp)
      44: 74 00        	addi	a3, sp, 12
      46: 39 00        	c.nop	14
      48: 00 00        	unimp	
      4a: 76 31        	fld	ft2, 376(sp)
      4c: 00 2a        	fld	fs0, 16(a2)
      4e: 00 00        	unimp	
      50: 00 63        	ld	s0, 0(a4)
      52: 6f 72 65 00  	jal	tp, 0x57058 <.debug_info+0x57058>
      56: 49 00        	c.nop	18
      58: 00 00        	unimp	
      5a: 4c 65        	ld	a1, 136(a0)
      5c: 66 74        	ld	s0, 120(sp)
      5e: 00 d3        	sw	s0, 32(a4)
      60: 01 00        	nop
      62: 00 70        	ld	s0, 32(s0)
      64: 61 6e        	lui	t3, 24
      66: 69 63        	lui	t1, 26
      68: 00 e9        	sd	s0, 16(a0)
      6a: 03 00 00 79  	lb	zero, 1936(zero)
      6e: 69 66        	lui	a2, 26
      70: 65 5f        	li	t5, -7
      72: 6f 73 00 5b  	jal	t1, 0x7622 <.debug_info+0x7622>
      76: 00 00        	unimp	
      78: 00 55        	lw	s0, 40(a0)
      7a: 6e 6b        	ld	s6, 216(sp)
      7c: 6e 6f        	ld	t5, 216(sp)
      7e: 77 6e 00 d8  	<unknown>
      82: 01 00        	nop
      84: 00 70        	ld	s0, 32(s0)
      86: 61 6e        	lui	t3, 24
      88: 69 63        	lui	t1, 26
      8a: 5f 69 6e 66  	<unknown>
      8e: 6f 00 42 02  	j	0x200b2 <.debug_info+0x200b2>
      92: 00 00        	unimp	
      94: 6f 70 74 69  	j	0x47f2a <.debug_info+0x47f2a>
      98: 6f 6e 00 12  	jal	t3, 0x61b8 <.debug_info+0x61b8>
      9c: 02 00        	c.slli64	zero
      9e: 00 6c        	ld	s0, 24(s0)
      a0: 6f 63 61 74  	jal	t1, 0x167e6 <.debug_info+0x167e6>
      a4: 69 6f        	lui	t5, 26
      a6: 6e 00        	c.slli	zero, 27
      a8: 4f 00 00 00  	fnmadd.s	ft0, ft0, ft0, ft0, rne
      ac: 52 69        	ld	s2, 272(sp)
      ae: 67 68 74 00  	<unknown>
      b2: 60 03        	addi	s0, sp, 396
      b4: 00 00        	unimp	
      b6: 72 65        	ld	a0, 280(sp)
      b8: 73 75 6c 74  	csrrci	a0, 1862, 24
      bc: 00 00        	unimp	
      be: 00 00        	unimp	
      c0: 00           	<unknown>

Disassembly of section .debug_pubtypes:

0000000000000000 <.debug_pubtypes>:
       0: 68 03        	addi	a0, sp, 396
       2: 00 00        	unimp	
       4: 02 00        	c.slli64	zero
       6: 00 00        	unimp	
       8: 00 00        	unimp	
       a: bf 05 00 00  	<unknown>
       e: 17 05 00 00  	auipc	a0, 0
      12: 26 5b        	lw	s6, 104(sp)
      14: 63 6f 72 65  	bltu	tp, s7, 0x672 <.debug_info+0x672>
      18: 3a 3a        	fld	fs4, 424(sp)
      1a: 66 6d        	ld	s10, 88(sp)
      1c: 74 3a        	fld	fa3, 240(a2)
      1e: 3a 41        	lw	sp, 140(sp)
      20: 72 67        	ld	a4, 280(sp)
      22: 75 6d        	lui	s10, 29
      24: 65 6e        	lui	t3, 25
      26: 74 56        	lw	a3, 108(a2)
      28: 31 5d        	li	s10, -20
      2a: 00 51        	lw	s0, 32(a0)
      2c: 04 00        	<unknown>
      2e: 00 28        	fld	fs0, 16(s0)
      30: 64 79        	ld	s1, 240(a0)
      32: 6e 20        	fld	ft0, 216(sp)
      34: 63 6f 72 65  	bltu	tp, s7, 0x692 <.debug_info+0x692>
      38: 3a 3a        	fld	fs4, 424(sp)
      3a: 61 6e        	lui	t3, 24
      3c: 79 3a        	addiw	s4, s4, -2
      3e: 3a 41        	lw	sp, 140(sp)
      40: 6e 79        	ld	s2, 248(sp)
      42: 20 2b        	fld	fs0, 80(a4)
      44: 20 63        	ld	s0, 64(a4)
      46: 6f 72 65 3a  	jal	tp, 0x573ec <.debug_info+0x573ec>
      4a: 3a 6d        	ld	s10, 392(sp)
      4c: 61 72        	lui	tp, 1048568
      4e: 6b 65 72 3a  	<unknown>
      52: 3a 53        	lw	t1, 172(sp)
      54: 65 6e        	lui	t3, 25
      56: 64 29        	fld	fs1, 208(a0)
      58: 00 6f        	ld	s0, 24(a4)
      5a: 05 00        	c.nop	1
      5c: 00 26        	fld	fs0, 8(a2)
      5e: 6d 75        	lui	a0, 1048571
      60: 74 20        	fld	fa3, 192(s0)
      62: 63 6f 72 65  	bltu	tp, s7, 0x6c0 <.debug_info+0x6c0>
      66: 3a 3a        	fld	fs4, 424(sp)
      68: 66 6d        	ld	s10, 88(sp)
      6a: 74 3a        	fld	fa3, 240(a2)
      6c: 3a 46        	lw	a2, 140(sp)
      6e: 6f 72 6d 61  	jal	tp, 0xd7684 <.debug_info+0xd7684>
      72: 74 74        	ld	a3, 232(s0)
      74: 65 72        	lui	tp, 1048569
      76: 00 aa        	fsd	fs0, 16(a2)
      78: 05 00        	c.nop	1
      7a: 00 26        	fld	fs0, 8(a2)
      7c: 63 6f 72 65  	bltu	tp, s7, 0x6da <.debug_info+0x6da>
      80: 3a 3a        	fld	fs4, 424(sp)
      82: 70 61        	ld	a2, 192(a0)
      84: 6e 69        	ld	s2, 216(sp)
      86: 63 3a 3a 6c  	<unknown>
      8a: 6f 63 61 74  	jal	t1, 0x167d0 <.debug_info+0x167d0>
      8e: 69 6f        	lui	t5, 26
      90: 6e 3a        	fld	fs4, 248(sp)
      92: 3a 4c        	lw	s8, 140(sp)
      94: 6f 63 61 74  	jal	t1, 0x167da <.debug_info+0x167da>
      98: 69 6f        	lui	t5, 26
      9a: 6e 00        	c.slli	zero, 27
      9c: 4b 05 00 00  	fnmsub.s	fa0, ft0, ft0, ft0, rne
      a0: 66 6e        	ld	t3, 88(sp)
      a2: 28 26        	fld	fa0, 72(a2)
      a4: 63 6f 72 65  	bltu	tp, s7, 0x702 <.debug_info+0x702>
      a8: 3a 3a        	fld	fs4, 424(sp)
      aa: 66 6d        	ld	s10, 88(sp)
      ac: 74 3a        	fld	fa3, 240(a2)
      ae: 3a 7b        	ld	s6, 424(sp)
      b0: 65 78        	lui	a6, 1048569
      b2: 74 65        	ld	a3, 200(a0)
      b4: 72 6e        	ld	t3, 280(sp)
      b6: 23 30 7d 3a  	sd	t2, 928(s10)
      ba: 3a 4f        	lw	t5, 140(sp)
      bc: 70 61        	ld	a2, 192(a0)
      be: 71 75        	lui	a0, 1048572
      c0: 65 2c        	addiw	s8, s8, 25
      c2: 20 26        	fld	fs0, 72(a2)
      c4: 6d 75        	lui	a0, 1048571
      c6: 74 20        	fld	fa3, 192(s0)
      c8: 63 6f 72 65  	bltu	tp, s7, 0x726 <.debug_info+0x726>
      cc: 3a 3a        	fld	fs4, 424(sp)
      ce: 66 6d        	ld	s10, 88(sp)
      d0: 74 3a        	fld	fa3, 240(a2)
      d2: 3a 46        	lw	a2, 140(sp)
      d4: 6f 72 6d 61  	jal	tp, 0xd76ea <.debug_info+0xd76ea>
      d8: 74 74        	ld	a3, 232(s0)
      da: 65 72        	lui	tp, 1048569
      dc: 29 20        	<unknown>
      de: 2d 3e        	addiw	t3, t3, -21
      e0: 20 63        	ld	s0, 64(a4)
      e2: 6f 72 65 3a  	jal	tp, 0x57488 <.debug_info+0x57488>
      e6: 3a 72        	ld	tp, 424(sp)
      e8: 65 73        	lui	t1, 1048569
      ea: 75 6c        	lui	s8, 29
      ec: 74 3a        	fld	fa3, 240(a2)
      ee: 3a 52        	lw	tp, 172(sp)
      f0: 65 73        	lui	t1, 1048569
      f2: 75 6c        	lui	s8, 29
      f4: 74 3c        	fld	fa3, 248(s0)
      f6: 28 29        	fld	fa0, 80(a0)
      f8: 2c 20        	fld	fa1, 64(s0)
      fa: 63 6f 72 65  	bltu	tp, s7, 0x758 <.debug_info+0x758>
      fe: 3a 3a        	fld	fs4, 424(sp)
     100: 66 6d        	ld	s10, 88(sp)
     102: 74 3a        	fld	fa3, 240(a2)
     104: 3a 45        	lw	a0, 140(sp)
     106: 72 72        	ld	tp, 312(sp)
     108: 6f 72 3e 00  	jal	tp, 0xe790a <.debug_info+0xe790a>
     10c: 79 01        	addi	sp, sp, 30
     10e: 00 00        	unimp	
     110: 4f 70 61 71  	fnmadd.s	ft0, ft2, fs6, fa4
     114: 75 65        	lui	a0, 29
     116: 00 b7        	fsd	fs0, 40(a4)
     118: 05 00        	c.nop	1
     11a: 00 62        	ld	s0, 0(a2)
     11c: 6f 6f 6c 00  	jal	t5, 0xc6122 <.debug_info+0xc6122>
     120: e2 03        	slli	t2, t2, 24
     122: 00 00        	unimp	
     124: 75 38        	addiw	a6, a6, -3
     126: 00 72        	ld	s0, 32(a2)
     128: 04 00        	<unknown>
     12a: 00 75        	ld	s0, 40(a0)
     12c: 73 69 7a 65  	csrrsi	s2, 1623, 20
     130: 00 68        	ld	s0, 16(s0)
     132: 05 00        	c.nop	1
     134: 00 28        	fld	fs0, 16(s0)
     136: 29 00        	c.nop	10
     138: 87 04 00 00  	<unknown>
     13c: 26 63        	ld	t1, 72(sp)
     13e: 6f 72 65 3a  	jal	tp, 0x574e4 <.debug_info+0x574e4>
     142: 3a 66        	ld	a2, 392(sp)
     144: 6d 74        	lui	s0, 1048571
     146: 3a 3a        	fld	fs4, 424(sp)
     148: 41 72        	lui	tp, 1048560
     14a: 67 75 6d 65  	<unknown>
     14e: 6e 74        	ld	s0, 248(sp)
     150: 73 00 80 00  	<unknown>
     154: 00 00        	unimp	
     156: 46 6f        	ld	t5, 80(sp)
     158: 72 6d        	ld	s10, 280(sp)
     15a: 61 74        	lui	s0, 1048568
     15c: 53 70 65 63  	<unknown>
     160: 00 dd        	sw	s0, 56(a0)
     162: 01 00        	nop
     164: 00 50        	lw	s0, 32(s0)
     166: 61 6e        	lui	t3, 24
     168: 69 63        	lui	t1, 26
     16a: 49 6e        	lui	t3, 18
     16c: 66 6f        	ld	t5, 88(sp)
     16e: 00 88        	<unknown>
     170: 01 00        	nop
     172: 00 46        	lw	s0, 8(a2)
     174: 6f 72 6d 61  	jal	tp, 0xd778a <.debug_info+0xd778a>
     178: 74 74        	ld	a3, 232(s0)
     17a: 65 72        	lui	tp, 1048569
     17c: 00 58        	lw	s0, 48(s0)
     17e: 04 00        	<unknown>
     180: 00 26        	fld	fs0, 8(a2)
     182: 5b 75 73 69  	<unknown>
     186: 7a 65        	ld	a0, 408(sp)
     188: 3b 20 33 5d  	<unknown>
     18c: 00 94        	<unknown>
     18e: 04 00        	<unknown>
     190: 00 26        	fld	fs0, 8(a2)
     192: 5b 26 73 74  	<unknown>
     196: 72 5d        	lw	s10, 60(sp)
     198: 00 09        	addi	s0, sp, 144
     19a: 05 00        	c.nop	1
     19c: 00 63        	ld	s0, 0(a4)
     19e: 68 61        	ld	a0, 192(a0)
     1a0: 72 00        	c.slli	zero, 28
     1a2: 3e 00        	c.slli	zero, 15
     1a4: 00 00        	unimp	
     1a6: 41 6c        	lui	s8, 16
     1a8: 69 67        	lui	a4, 26
     1aa: 6e 6d        	ld	s10, 216(sp)
     1ac: 65 6e        	lui	t3, 25
     1ae: 74 00        	addi	a3, sp, 12
     1b0: 80 04        	addi	s0, sp, 576
     1b2: 00 00        	unimp	
     1b4: 75 36        	addiw	a2, a2, -3
     1b6: 34 00        	addi	a3, sp, 8
     1b8: a4 02        	addi	s1, sp, 328
     1ba: 00 00        	unimp	
     1bc: 4f 70 74 69  	fnmadd.s	ft0, fs0, fs7, fa3
     1c0: 6f 6e 3c 26  	jal	t3, 0xc6c22 <.debug_info+0xc6c22>
     1c4: 5b 63 6f 72  	<unknown>
     1c8: 65 3a        	addiw	s4, s4, -7
     1ca: 3a 66        	ld	a2, 392(sp)
     1cc: 6d 74        	lui	s0, 1048571
     1ce: 3a 3a        	fld	fs4, 424(sp)
     1d0: 72 74        	ld	s0, 312(sp)
     1d2: 3a 3a        	fld	fs4, 424(sp)
     1d4: 76 31        	fld	ft2, 376(sp)
     1d6: 3a 3a        	fld	fs4, 424(sp)
     1d8: 41 72        	lui	tp, 1048560
     1da: 67 75 6d 65  	<unknown>
     1de: 6e 74        	ld	s0, 248(sp)
     1e0: 5d 3e        	addiw	t3, t3, -9
     1e2: 00 1d        	addi	s0, sp, 688
     1e4: 04 00        	<unknown>
     1e6: 00 26        	fld	fs0, 8(a2)
     1e8: 63 6f 72 65  	bltu	tp, s7, 0x846 <.debug_info+0x846>
     1ec: 3a 3a        	fld	fs4, 424(sp)
     1ee: 70 61        	ld	a2, 192(a0)
     1f0: 6e 69        	ld	s2, 216(sp)
     1f2: 63 3a 3a 70  	<unknown>
     1f6: 61 6e        	lui	t3, 24
     1f8: 69 63        	lui	t1, 26
     1fa: 5f 69 6e 66  	<unknown>
     1fe: 6f 3a 3a 50  	jal	s4, 0xa3f00 <.debug_info+0xa3f00>
     202: 61 6e        	lui	t3, 24
     204: 69 63        	lui	t1, 26
     206: 49 6e        	lui	t3, 18
     208: 66 6f        	ld	t5, 88(sp)
     20a: 00 62        	ld	s0, 0(a2)
     20c: 00 00        	unimp	
     20e: 00 41        	lw	s0, 0(a0)
     210: 72 67        	ld	a4, 280(sp)
     212: 75 6d        	lui	s10, 29
     214: 65 6e        	lui	t3, 25
     216: 74 00        	addi	a3, sp, 12
     218: 01 03        	mv	t1, t1
     21a: 00 00        	unimp	
     21c: 4f 70 74 69  	fnmadd.s	ft0, fs0, fs7, fa3
     220: 6f 6e 3c 75  	jal	t3, 0xc7172 <.debug_info+0xc7172>
     224: 73 69 7a 65  	csrrsi	s2, 1623, 20
     228: 3e 00        	c.slli	zero, 15
     22a: 47 02 00 00  	fmsub.s	ft4, ft0, ft0, ft0, rne
     22e: 4f 70 74 69  	fnmadd.s	ft0, fs0, fs7, fa3
     232: 6f 6e 3c 26  	jal	t3, 0xc6c94 <.debug_info+0xc6c94>
     236: 63 6f 72 65  	bltu	tp, s7, 0x894 <.debug_info+0x894>
     23a: 3a 3a        	fld	fs4, 424(sp)
     23c: 66 6d        	ld	s10, 88(sp)
     23e: 74 3a        	fld	fa3, 240(a2)
     240: 3a 41        	lw	sp, 140(sp)
     242: 72 67        	ld	a4, 280(sp)
     244: 75 6d        	lui	s10, 29
     246: 65 6e        	lui	t3, 25
     248: 74 73        	ld	a3, 224(a4)
     24a: 3e 00        	c.slli	zero, 15
     24c: bf 00 00 00  	<unknown>
     250: 43 6f 75 6e  	<unknown>
     254: 74 00        	addi	a3, sp, 12
     256: 56 01        	slli	sp, sp, 21
     258: 00 00        	unimp	
     25a: 41 72        	lui	tp, 1048560
     25c: 67 75 6d 65  	<unknown>
     260: 6e 74        	ld	s0, 248(sp)
     262: 56 31        	fld	ft2, 368(sp)
     264: 00 2a        	fld	fs0, 16(a2)
     266: 04 00        	<unknown>
     268: 00 26        	fld	fs0, 8(a2)
     26a: 28 64        	ld	a0, 72(s0)
     26c: 79 6e        	lui	t3, 30
     26e: 20 63        	ld	s0, 64(a4)
     270: 6f 72 65 3a  	jal	tp, 0x57616 <.debug_info+0x57616>
     274: 3a 61        	ld	sp, 392(sp)
     276: 6e 79        	ld	s2, 248(sp)
     278: 3a 3a        	fld	fs4, 424(sp)
     27a: 41 6e        	lui	t3, 16
     27c: 79 20        	<unknown>
     27e: 2b 20 63 6f  	<unknown>
     282: 72 65        	ld	a0, 280(sp)
     284: 3a 3a        	fld	fs4, 424(sp)
     286: 6d 61        	addi	sp, sp, 240
     288: 72 6b        	ld	s6, 280(sp)
     28a: 65 72        	lui	tp, 1048569
     28c: 3a 3a        	fld	fs4, 424(sp)
     28e: 53 65 6e 64  	<unknown>
     292: 29 00        	c.nop	10
     294: 81 01        	mv	gp, gp
     296: 00 00        	unimp	
     298: 45 72        	lui	tp, 1048561
     29a: 72 6f        	ld	t5, 280(sp)
     29c: 72 00        	c.slli	zero, 28
     29e: e2 04        	slli	s1, s1, 24
     2a0: 00 00        	unimp	
     2a2: 26 5b        	lw	s6, 104(sp)
     2a4: 63 6f 72 65  	bltu	tp, s7, 0x902 <.debug_info+0x902>
     2a8: 3a 3a        	fld	fs4, 424(sp)
     2aa: 66 6d        	ld	s10, 88(sp)
     2ac: 74 3a        	fld	fa3, 240(a2)
     2ae: 3a 72        	ld	tp, 424(sp)
     2b0: 74 3a        	fld	fa3, 240(a2)
     2b2: 3a 76        	ld	a2, 424(sp)
     2b4: 31 3a        	addiw	s4, s4, -20
     2b6: 3a 41        	lw	sp, 140(sp)
     2b8: 72 67        	ld	a4, 280(sp)
     2ba: 75 6d        	lui	s10, 29
     2bc: 65 6e        	lui	t3, 25
     2be: 74 5d        	lw	a3, 124(a0)
     2c0: 00 a3        	fsd	fs0, 0(a4)
     2c2: 05 00        	c.nop	1
     2c4: 00 64        	ld	s0, 8(s0)
     2c6: 79 6e        	lui	t3, 30
     2c8: 20 63        	ld	s0, 64(a4)
     2ca: 6f 72 65 3a  	jal	tp, 0x57670 <.debug_info+0x57670>
     2ce: 3a 66        	ld	a2, 392(sp)
     2d0: 6d 74        	lui	s0, 1048571
     2d2: 3a 3a        	fld	fs4, 424(sp)
     2d4: 57 72 69 74  	<unknown>
     2d8: 65 00        	c.nop	25
     2da: 7c 05        	addi	a5, sp, 652
     2dc: 00 00        	unimp	
     2de: 26 6d        	ld	s10, 72(sp)
     2e0: 75 74        	lui	s0, 1048573
     2e2: 20 64        	ld	s0, 72(s0)
     2e4: 79 6e        	lui	t3, 30
     2e6: 20 63        	ld	s0, 64(a4)
     2e8: 6f 72 65 3a  	jal	tp, 0x5768e <.debug_info+0x5768e>
     2ec: 3a 66        	ld	a2, 392(sp)
     2ee: 6d 74        	lui	s0, 1048571
     2f0: 3a 3a        	fld	fs4, 424(sp)
     2f2: 57 72 69 74  	<unknown>
     2f6: 65 00        	c.nop	25
     2f8: 2d 01        	addi	sp, sp, 11
     2fa: 00 00        	unimp	
     2fc: 41 72        	lui	tp, 1048560
     2fe: 67 75 6d 65  	<unknown>
     302: 6e 74        	ld	s0, 248(sp)
     304: 73 00 65 03  	<unknown>
     308: 00 00        	unimp	
     30a: 52 65        	ld	a0, 272(sp)
     30c: 73 75 6c 74  	csrrci	a0, 1862, 24
     310: 3c 28        	fld	fa5, 80(s0)
     312: 29 2c        	addiw	s8, s8, 10
     314: 20 63        	ld	s0, 64(a4)
     316: 6f 72 65 3a  	jal	tp, 0x576bc <.debug_info+0x576bc>
     31a: 3a 66        	ld	a2, 392(sp)
     31c: 6d 74        	lui	s0, 1048571
     31e: 3a 3a        	fld	fs4, 424(sp)
     320: 45 72        	lui	tp, 1048561
     322: 72 6f        	ld	t5, 280(sp)
     324: 72 3e        	fld	ft8, 312(sp)
     326: 00 17        	addi	s0, sp, 928
     328: 02 00        	c.slli64	zero
     32a: 00 4c        	lw	s0, 24(s0)
     32c: 6f 63 61 74  	jal	t1, 0x16a72 <.debug_info+0x16a72>
     330: 69 6f        	lui	t5, 26
     332: 6e 00        	c.slli	zero, 27
     334: 10 05        	addi	a2, sp, 640
     336: 00 00        	unimp	
     338: 75 33        	addiw	t1, t1, -3
     33a: 32 00        	c.slli	zero, 12
     33c: 3e 05        	slli	a0, a0, 15
     33e: 00 00        	unimp	
     340: 26 63        	ld	t1, 72(sp)
     342: 6f 72 65 3a  	jal	tp, 0x576e8 <.debug_info+0x576e8>
     346: 3a 66        	ld	a2, 392(sp)
     348: 6d 74        	lui	s0, 1048571
     34a: 3a 3a        	fld	fs4, 424(sp)
     34c: 7b 65 78 74  	<unknown>
     350: 65 72        	lui	tp, 1048569
     352: 6e 23        	fld	ft6, 216(sp)
     354: 30 7d        	ld	a2, 120(a0)
     356: 3a 3a        	fld	fs4, 424(sp)
     358: 4f 70 61 71  	fnmadd.s	ft0, ft2, fs6, fa4
     35c: 75 65        	lui	a0, 29
     35e: 00 bb        	fsd	fs0, 48(a4)
     360: 04 00        	<unknown>
     362: 00 26        	fld	fs0, 8(a2)
     364: 73 74 72 00  	csrrci	s0, 7, 4
     368: 00 00        	unimp	
     36a: 00 00        	unimp	

Disassembly of section .riscv.attributes:

0000000000000000 <.riscv.attributes>:
       0: 41 34        	addiw	s0, s0, -16
       2: 00 00        	unimp	
       4: 00 72        	ld	s0, 32(a2)
       6: 69 73        	lui	t1, 1048570
       8: 63 76 00 01  	bgeu	zero, a6, 0x14 <.debug_info+0x14>
       c: 2a 00        	c.slli	zero, 10
       e: 00 00        	unimp	
      10: 04 10        	addi	s1, sp, 32
      12: 05 72        	lui	tp, 1048545
      14: 76 36        	fld	fa2, 376(sp)
      16: 34 69        	ld	a3, 80(a0)
      18: 32 70        	<unknown>
      1a: 30 5f        	lw	a2, 120(a4)
      1c: 6d 32        	addiw	tp, tp, -5
      1e: 70 30        	fld	fa2, 224(s0)
      20: 5f 61 32 70  	<unknown>
      24: 30 5f        	lw	a2, 120(a4)
      26: 66 32        	fld	ft4, 120(sp)
      28: 70 30        	fld	fa2, 224(s0)
      2a: 5f 64 32 70  	<unknown>
      2e: 30 5f        	lw	a2, 120(a4)
      30: 63 32 70 30  	<unknown>
      34: 00           	<unknown>

Disassembly of section .debug_frame:

0000000000000000 <.debug_frame>:
       0: 14 00        	<unknown>
       2: 00 00        	unimp	
       4: ff ff ff ff  	<unknown>
       8: 04 00        	<unknown>
       a: 08 00        	<unknown>
       c: 01 78        	lui	a6, 1048544
       e: 01 0c        	mv	s8, s8
      10: 02 00        	c.slli64	zero
      12: 00 00        	unimp	
      14: 00 00        	unimp	
      16: 00 00        	unimp	
      18: 1c 00        	<unknown>
		...
      26: 00 00        	unimp	
      28: 08 00        	<unknown>
      2a: 00 00        	unimp	
      2c: 00 00        	unimp	
      2e: 00 00        	unimp	
      30: 42 0e        	slli	t3, t3, 16
      32: 10 00        	<unknown>
      34: 00 00        	unimp	
      36: 00 00        	unimp	

Disassembly of section .debug_line:

0000000000000000 <.Lline_table_start0>:
       0: 4a 00        	c.slli	zero, 18
       2: 00 00        	unimp	
       4: 04 00        	<unknown>
       6: 29 00        	c.nop	10
       8: 00 00        	unimp	
       a: 01 01        	mv	sp, sp
       c: 01 fb        	bnez	a4, 0xffffffffffffff1c <.debug_info+0xffffffffffffff1c>
       e: 0e 0d        	slli	s10, s10, 3
      10: 00 01        	addi	s0, sp, 128
      12: 01 01        	mv	sp, sp
      14: 01 00        	nop
      16: 00 00        	unimp	
      18: 01 00        	nop
      1a: 00 01        	addi	s0, sp, 128
      1c: 73 72 63 00  	csrrci	tp, 6, 6
      20: 00 6c        	ld	s0, 24(s0)
      22: 61 6e        	lui	t3, 24
      24: 67 5f 69 74  	<unknown>
      28: 65 6d        	lui	s10, 25
      2a: 73 2e 72 73  	csrrs	t3, 1847, tp
      2e: 00 01        	addi	s0, sp, 128
      30: 00 00        	unimp	
      32: 00 00        	unimp	
      34: 09 02        	addi	tp, tp, 2
		...
      3e: 15 05        	addi	a0, a0, 5
      40: 05 0a        	addi	s4, s4, 1
      42: 03 01 09 04  	lb	sp, 64(s2)
      46: 00 01        	addi	s0, sp, 128
      48: 09 04        	addi	s0, s0, 2
      4a: 00 00        	unimp	
      4c: 01 01        	mv	sp, sp

Disassembly of section .comment:

0000000000000000 <.comment>:
       0: 4c 69        	ld	a1, 144(a0)
       2: 6e 6b        	ld	s6, 216(sp)
       4: 65 72        	lui	tp, 1048569
       6: 3a 20        	fld	ft0, 392(sp)
       8: 4c 4c        	lw	a1, 28(s0)
       a: 44 20        	fld	fs1, 128(s0)
       c: 31 35        	addiw	a0, a0, -20
       e: 2e 30        	fld	ft0, 232(sp)
      10: 2e 30        	fld	ft0, 232(sp)
      12: 00           	<unknown>

Disassembly of section .symtab:

0000000000000000 <.symtab>:
		...
      18: 01 00        	nop
      1a: 00 00        	unimp	
      1c: 04 00        	<unknown>
      1e: f1 ff        	bnez	a5, 0xfffffffffffffffa <.symtab+0xfffffffffffffffa>
		...
      34: 00 00        	unimp	
      36: 02 00        	c.slli64	zero
		...
      4c: 00 00        	unimp	
      4e: 04 00        	<unknown>
      50: 72 02        	slli	tp, tp, 28
		...
      66: 04 00        	<unknown>
      68: ac 00        	addi	a1, sp, 72
		...
      76: 00 00        	unimp	
      78: 12 00        	c.slli	zero, 4
      7a: 00 00        	unimp	
      7c: 00 00        	unimp	
      7e: 09 00        	c.nop	2
		...
      94: 00 00        	unimp	
      96: 04 00        	<unknown>
      98: 3f 03 00 00  	<unknown>
		...
      ac: 00 00        	unimp	
      ae: 04 00        	<unknown>
      b0: 59 01        	addi	sp, sp, 22
		...
      c6: 04 00        	<unknown>
      c8: 48 02        	addi	a0, sp, 260
		...
      de: 04 00        	<unknown>
      e0: 3a 01        	slli	sp, sp, 14
		...
      f6: 04 00        	<unknown>
      f8: 7e 04        	slli	s0, s0, 31
		...
     10e: 04 00        	<unknown>
     110: 25 01        	addi	sp, sp, 9
		...
     126: 04 00        	<unknown>
     128: 20 01        	addi	s0, sp, 136
		...
     13e: 04 00        	<unknown>
     140: 5c 03        	addi	a5, sp, 388
		...
     156: 04 00        	<unknown>
     158: d1 01        	addi	gp, gp, 20
		...
     16e: 04 00        	<unknown>
     170: 81 04        	mv	s1, s1
		...
     186: 04 00        	<unknown>
     188: 36 03        	slli	t1, t1, 13
		...
     19e: 04 00        	<unknown>
     1a0: 17 01 00 00  	auipc	sp, 0
		...
     1b4: 00 00        	unimp	
     1b6: 04 00        	<unknown>
     1b8: e0 01        	addi	s0, sp, 204
		...
     1ce: 04 00        	<unknown>
     1d0: 0f 02 00 00  	<unknown>
		...
     1e4: 00 00        	unimp	
     1e6: 04 00        	<unknown>
     1e8: 5e 00        	c.slli	zero, 23
		...
     1fe: 04 00        	<unknown>
     200: 69 03        	addi	t1, t1, 26
		...
     216: 04 00        	<unknown>
     218: de 02        	slli	t0, t0, 23
		...
     22e: 04 00        	<unknown>
     230: e7 02 00 00  	jalr	t0, zero
		...
     244: 00 00        	unimp	
     246: 04 00        	<unknown>
     248: d1 00        	addi	ra, ra, 20
		...
     25e: 04 00        	<unknown>
     260: 30 03        	addi	a2, sp, 392
		...
     276: 04 00        	<unknown>
     278: 64 01        	addi	s1, sp, 140
		...
     28e: 04 00        	<unknown>
     290: f9 02        	addi	t0, t0, 30
		...
     2a6: 04 00        	<unknown>
     2a8: a8 01        	addi	a0, sp, 200
		...
     2be: 04 00        	<unknown>
     2c0: 0a 04        	slli	s0, s0, 2
		...
     2d6: 04 00        	<unknown>
     2d8: 3e 02        	slli	tp, tp, 15
		...
     2ee: 04 00        	<unknown>
     2f0: 49 01        	addi	sp, sp, 18
		...
     306: 04 00        	<unknown>
     308: 39 02        	addi	tp, tp, 14
		...
     31e: 04 00        	<unknown>
     320: 03 01 00 00  	lb	sp, 0(zero)
		...
     334: 00 00        	unimp	
     336: 04 00        	<unknown>
     338: 12 04        	slli	s0, s0, 4
		...
     34e: 04 00        	<unknown>
     350: dc 00        	addi	a5, sp, 68
		...
     366: 04 00        	<unknown>
     368: b3 02 00 00  	add	t0, zero, zero
		...
     37c: 00 00        	unimp	
     37e: 04 00        	<unknown>
     380: 18 04        	addi	a4, sp, 512
		...
     396: 04 00        	<unknown>
     398: 5e 01        	slli	sp, sp, 23
		...
     3ae: 04 00        	<unknown>
     3b0: 97 04 00 00  	auipc	s1, 0
		...
     3c4: 00 00        	unimp	
     3c6: 04 00        	<unknown>
     3c8: a0 00        	addi	s0, sp, 72
		...
     3de: 04 00        	<unknown>
     3e0: 09 02        	addi	tp, tp, 2
		...
     3f6: 04 00        	<unknown>
     3f8: 2f 01 00 00  	<unknown>
		...
     40c: 00 00        	unimp	
     40e: 04 00        	<unknown>
     410: 8a 00        	slli	ra, ra, 2
		...
     426: 04 00        	<unknown>
     428: d8 01        	addi	a4, sp, 196
		...
     43e: 04 00        	<unknown>
     440: a0 01        	addi	s0, sp, 200
		...
     456: 04 00        	<unknown>
     458: 50 01        	addi	a2, sp, 132
		...
     46e: 04 00        	<unknown>
     470: d3 02 00 00  	fadd.s	ft5, ft0, ft0, rne
		...
     484: 00 00        	unimp	
     486: 04 00        	<unknown>
     488: 30 02        	addi	a2, sp, 264
		...
     49e: 04 00        	<unknown>
     4a0: e7 01 00 00  	jalr	gp, zero
		...
     4b4: 00 00        	unimp	
     4b6: 04 00        	<unknown>
     4b8: 39 04        	addi	s0, s0, 14
		...
     4ce: 04 00        	<unknown>
     4d0: 0e 01        	slli	sp, sp, 3
		...
     4e6: 04 00        	<unknown>
     4e8: 99 00        	addi	ra, ra, 6
		...
     4fe: 04 00        	<unknown>
     500: 6e 01        	slli	sp, sp, 27
		...
     516: 04 00        	<unknown>
     518: 3e 04        	slli	s0, s0, 15
		...
     52e: 04 00        	<unknown>
     530: d7 00 00 00  	<unknown>
		...
     544: 00 00        	unimp	
     546: 04 00        	<unknown>
     548: 3d 01        	addi	sp, sp, 15
		...
     55e: 04 00        	<unknown>
     560: 63 00 00 00  	beqz	zero, 0x560 <.symtab+0x560>
		...
     574: 00 00        	unimp	
     576: 04 00        	<unknown>
     578: 6b 04 00 00  	<unknown>
		...
     58c: 00 00        	unimp	
     58e: 04 00        	<unknown>
     590: 3f 01 00 00  	<unknown>
		...
     5a4: 00 00        	unimp	
     5a6: 04 00        	<unknown>
     5a8: e6 00        	slli	ra, ra, 25
		...
     5be: 04 00        	<unknown>
     5c0: 67 01 00 00  	jalr	sp, zero
		...
     5d4: 00 00        	unimp	
     5d6: 04 00        	<unknown>
     5d8: 6a 01        	slli	sp, sp, 26
		...
     5ee: 04 00        	<unknown>
     5f0: 4c 02        	addi	a1, sp, 260
		...
     606: 04 00        	<unknown>
     608: 46 01        	slli	sp, sp, 17
		...
     61e: 04 00        	<unknown>
     620: 89 04        	addi	s1, s1, 2
		...
     636: 04 00        	<unknown>
     638: fe 01        	slli	gp, gp, 31
		...
     64e: 04 00        	<unknown>
     650: ec 01        	addi	a1, sp, 204
		...
     666: 04 00        	<unknown>
     668: cb 00 00 00  	fnmsub.s	ft1, ft0, ft0, ft0, rne
		...
     67c: 00 00        	unimp	
     67e: 04 00        	<unknown>
     680: 4e 02        	slli	tp, tp, 19
		...
     696: 04 00        	<unknown>
		...
     6ac: 00 00        	unimp	
     6ae: 04 00        	<unknown>
     6b0: a4 00        	addi	s1, sp, 72
		...
     6c6: 04 00        	<unknown>
     6c8: 62 03        	slli	t1, t1, 24
		...
     6de: 04 00        	<unknown>
     6e0: ff 02 00 00  	<unknown>
		...
     6f4: 00 00        	unimp	
     6f6: 04 00        	<unknown>
     6f8: fe 03        	slli	t2, t2, 31
		...
     70e: 04 00        	<unknown>
     710: 91 04        	addi	s1, s1, 4
		...
     726: 04 00        	<unknown>
     728: 8c 01        	addi	a1, sp, 192
		...
     73e: 04 00        	<unknown>
     740: fa 03        	slli	t2, t2, 30
		...
     756: 04 00        	<unknown>
     758: 1a 02        	slli	tp, tp, 6
		...
     76e: 04 00        	<unknown>
     770: f1 02        	addi	t0, t0, 28
		...
     786: 04 00        	<unknown>
     788: 43 04 00 00  	fmadd.s	fs0, ft0, ft0, ft0, rne
		...
     79c: 00 00        	unimp	
     79e: 04 00        	<unknown>
     7a0: 29 03        	addi	t1, t1, 10
		...
     7b6: 04 00        	<unknown>
     7b8: 12 01        	slli	sp, sp, 4
		...
     7ce: 04 00        	<unknown>
     7d0: 6f 03 00 00  	jal	t1, 0x7d0 <.symtab+0x7d0>
		...
     7e4: 00 00        	unimp	
     7e6: 04 00        	<unknown>
     7e8: 94 00        	addi	a3, sp, 64
		...
     7fe: 04 00        	<unknown>
     800: 0e 04        	slli	s0, s0, 3
		...
     816: 04 00        	<unknown>
     818: 45 00        	c.nop	17
		...
     82e: 04 00        	<unknown>
     830: 4c 04        	addi	a1, sp, 516
		...
     846: 04 00        	<unknown>
     848: 8e 03        	slli	t2, t2, 3
		...
     85e: 04 00        	<unknown>
     860: e4 02        	addi	s1, sp, 332
		...
     876: 04 00        	<unknown>
     878: 2b 00 00 00  	<unknown>
		...
     88c: 00 00        	unimp	
     88e: 04 00        	<unknown>
     890: 1f 04 00 00  	<unknown>
		...
     8a4: 00 00        	unimp	
     8a6: 04 00        	<unknown>
     8a8: be 02        	slli	t0, t0, 15
		...
     8be: 04 00        	<unknown>
     8c0: b0 01        	addi	a2, sp, 200
		...
     8d6: 04 00        	<unknown>
     8d8: 79 04        	addi	s0, s0, 30
		...
     8ee: 08 00        	<unknown>
		...
     904: 00 00        	unimp	
     906: 08 00        	<unknown>
     908: 18 00        	<unknown>
		...
     916: 00 00        	unimp	

Disassembly of section .shstrtab:

0000000000000000 <.shstrtab>:
       0: 00 2e        	fld	fs0, 24(a2)
       2: 64 65        	ld	s1, 200(a0)
       4: 62 75        	ld	a0, 56(sp)
       6: 67 5f 61 62  	<unknown>
       a: 62 72        	ld	tp, 56(sp)
       c: 65 76        	lui	a2, 1048569
       e: 00 2e        	fld	fs0, 24(a2)
      10: 64 65        	ld	s1, 200(a0)
      12: 62 75        	ld	a0, 56(sp)
      14: 67 5f 69 6e  	<unknown>
      18: 66 6f        	ld	t5, 88(sp)
      1a: 00 2e        	fld	fs0, 24(a2)
      1c: 64 65        	ld	s1, 200(a0)
      1e: 62 75        	ld	a0, 56(sp)
      20: 67 5f 61 72  	<unknown>
      24: 61 6e        	lui	t3, 24
      26: 67 65 73 00  	<unknown>
      2a: 2e 64        	ld	s0, 200(sp)
      2c: 65 62        	lui	tp, 25
      2e: 75 67        	lui	a4, 29
      30: 5f 73 74 72  	<unknown>
      34: 00 2e        	fld	fs0, 24(a2)
      36: 64 65        	ld	s1, 200(a0)
      38: 62 75        	ld	a0, 56(sp)
      3a: 67 5f 70 75  	<unknown>
      3e: 62 6e        	ld	t3, 24(sp)
      40: 61 6d        	lui	s10, 24
      42: 65 73        	lui	t1, 1048569
      44: 00 2e        	fld	fs0, 24(a2)
      46: 64 65        	ld	s1, 200(a0)
      48: 62 75        	ld	a0, 56(sp)
      4a: 67 5f 70 75  	<unknown>
      4e: 62 74        	ld	s0, 56(sp)
      50: 79 70        	c.lui	zero, -2
      52: 65 73        	lui	t1, 1048569
      54: 00 2e        	fld	fs0, 24(a2)
      56: 72 69        	ld	s2, 280(sp)
      58: 73 63 76 2e  	csrrsi	t1, 743, 12
      5c: 61 74        	lui	s0, 1048568
      5e: 74 72        	ld	a3, 224(a2)
      60: 69 62        	lui	tp, 26
      62: 75 74        	lui	s0, 1048573
      64: 65 73        	lui	t1, 1048569
      66: 00 2e        	fld	fs0, 24(a2)
      68: 64 65        	ld	s1, 200(a0)
      6a: 62 75        	ld	a0, 56(sp)
      6c: 67 5f 66 72  	<unknown>
      70: 61 6d        	lui	s10, 24
      72: 65 00        	c.nop	25
      74: 2e 64        	ld	s0, 200(sp)
      76: 65 62        	lui	tp, 25
      78: 75 67        	lui	a4, 29
      7a: 5f 6c 69 6e  	<unknown>
      7e: 65 00        	c.nop	25
      80: 2e 63        	ld	t1, 200(sp)
      82: 6f 6d 6d 65  	jal	s10, 0xd66d8 <.symtab+0xd66d8>
      86: 6e 74        	ld	s0, 248(sp)
      88: 00 2e        	fld	fs0, 24(a2)
      8a: 73 79 6d 74  	csrrci	s2, 1862, 26
      8e: 61 62        	lui	tp, 24
      90: 00 2e        	fld	fs0, 24(a2)
      92: 73 68 73 74  	csrrsi	a6, mseccfg, 6
      96: 72 74        	ld	s0, 312(sp)
      98: 61 62        	lui	tp, 24
      9a: 00 2e        	fld	fs0, 24(a2)
      9c: 73 74 72 74  	csrrci	s0, mseccfg, 4
      a0: 61 62        	lui	tp, 24
      a2: 00           	<unknown>

Disassembly of section .strtab:

0000000000000000 <.strtab>:
       0: 00 32        	fld	fs0, 32(a2)
       2: 32 78        	ld	a6, 296(sp)
       4: 64 63        	ld	s1, 192(a4)
       6: 67 37 64 6a  	<unknown>
       a: 71 65        	lui	a0, 28
       c: 66 75        	ld	a0, 120(sp)
       e: 39 7a        	lui	s4, 1048558
      10: 37 00 2e 4c  	lui	zero, 312032
      14: 6c 69        	ld	a1, 208(a0)
      16: 6e 65        	ld	a0, 216(sp)
      18: 5f 74 61 62  	<unknown>
      1c: 6c 65        	ld	a1, 200(a0)
      1e: 5f 73 74 61  	<unknown>
      22: 72 74        	ld	s0, 312(sp)
      24: 30 00        	addi	a2, sp, 8
