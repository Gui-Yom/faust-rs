rms::mydsp::rms_faust:

	push rsi
	push rdi
	push rbp
	push rbx
	sub rsp, 88

	test r9, r9
	je .LBB279_1

	mov r9, qword ptr [r8 + 8]

	movsxd r10, edx

	cmp r9, r10
	jb .LBB279_4

	cmp qword ptr [rsp + 168], 0
	je .LBB279_7

	mov rax, qword ptr [rsp + 160]

	mov r9, qword ptr [rax + 8]

	cmp r9, r10
	jb .LBB279_16

	test edx, edx
	je .LBB279_15

	mov r8, qword ptr [r8]

	mov r9, qword ptr [rax]
	mov edx, dword ptr [rcx + 4100]

	mov eax, dword ptr [rcx + 4104]

	cmp r10, 2
	mov r11d, 1
	cmovae r11, r10
	xor r10d, r10d

	movss xmm0, dword ptr [rip + __real@49800000]
	movss xmm1, dword ptr [rip + __real@4effffff]
	movss xmm2, dword ptr [rip + __real@3083126f]
	xor esi, esi
	jmp .LBB279_11

.LBB279_13:
	ucomiss xmm3, xmm3
	cmovp ebp, r10d

	mov dword ptr [rcx + 4*rbx], ebp

	lea ebx, [rax + 24]

	and ebx, 1023

	add edx, ebp

	sub edx, dword ptr [rcx + 4*rbx]

	xorps xmm3, xmm3
	cvtsi2ss xmm3, edx
	mulss xmm3, xmm2

	sqrtss xmm3, xmm3

	movss dword ptr [r9 + 4*rsi], xmm3

	inc eax

	mov rsi, rdi

	cmp r11, rdi
	je .LBB279_14

.LBB279_11:
	lea rdi, [rsi + 1]

	mov ebx, eax
	and ebx, 1023

	movss xmm4, dword ptr [r8 + 4*rsi]

	movaps xmm3, xmm4
	mulss xmm3, xmm0

	mulss xmm3, xmm4

	ucomiss xmm3, xmm1
	mov ebp, 2147483647
	ja .LBB279_13

	cvttss2si ebp, xmm3
	jmp .LBB279_13

.LBB279_14:
	mov dword ptr [rcx + 4104], eax
	mov dword ptr [rcx + 4100], edx
	mov dword ptr [rcx + 4096], edx

.LBB279_15:
	add rsp, 88
	pop rbx
	pop rbp
	pop rdi
	pop rsi
	ret

.LBB279_1:
	lea rax, [rip + __unnamed_245]

	mov qword ptr [rsp + 56], rax
	mov qword ptr [rsp + 64], 1
	mov qword ptr [rsp + 40], 0
	lea rax, [rip + __unnamed_21]

	mov qword ptr [rsp + 72], rax
	mov qword ptr [rsp + 80], 0

	lea rdx, [rip + __unnamed_246]

	jmp .LBB279_2

.LBB279_4:
	lea r8, [rip + __unnamed_247]

	jmp .LBB279_5

.LBB279_7:
	lea rax, [rip + __unnamed_248]

	mov qword ptr [rsp + 56], rax
	mov qword ptr [rsp + 64], 1
	mov qword ptr [rsp + 40], 0
	lea rax, [rip + __unnamed_21]

	mov qword ptr [rsp + 72], rax
	mov qword ptr [rsp + 80], 0

	lea rdx, [rip + __unnamed_249]

.LBB279_2:
	lea rcx, [rsp + 40]

	call core::panicking::panic_fmt
	ud2

.LBB279_16:
	lea r8, [rip + __unnamed_250]

.LBB279_5:
	mov rcx, r10

	mov rdx, r9

	call core::slice::index::slice_end_index_len_fail
	ud2

