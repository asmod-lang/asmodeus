; Calculate factorial of 5
; Uses stack and recursive-like approach

start:
    POB n           ; Load number
    factorial:
        SOM done    ; If negative (done), exit
        SDP         ; Push current number
        ODE one     ; Subtract 1
        SOB factorial ; Loop
    
    done:
        PZS         ; Pop from stack
        WYJSCIE     ; Output
        STP         ; Stop

n: RST 5
one: RST 1
