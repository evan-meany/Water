section .data
    ; Define your variable here, for example:
    my_variable dq 42  ; 64-bit double-precision floating-point variable with an initial value of 42

section .text
global main

main:
    ; Prologue: Save the base pointer and set up the stack frame
    push rbp
    mov rbp, rsp

    ; Push the variable onto the stack
    mov rax, qword [my_variable]
    push rax

    ; Call your function or perform any other operations
    ; In this example, we'll just return the value

    ; Epilogue: Clean up the stack and return
    mov rsp, rbp
    pop rbp
    ret