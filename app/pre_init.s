.section .text.__pre_init
.global __pre_init
.thumb_func
__pre_init:
		// we booted from ram or flash copied into ram, so the stack pointer wasn't
		// read from the vector table like it would be on a standard reset
		ldr r0, =_stack_start
		mov sp, r0

		bx lr
