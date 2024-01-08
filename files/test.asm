global main                 ; declare main() method
extern printf               ; link to external library

section .data
message db 'Hello world', 0xA, 0 ; text message
                                 ; 0xA (10) is hex for (NL), carriage return
                                 ; 0 terminates the line

section .text
main:                          ; the entry point! int main()
    push rdi                   ; save rdi register (callee-save)
    sub rsp, 32                ; allocate stack space for printf arguments

    lea rdi, [rel message]     ; load the address of the message into rdi
    call printf                ; call printf with the message address

    add rsp, 32                ; deallocate stack space
    pop rdi                    ; restore rdi register
    ret                        ; return
