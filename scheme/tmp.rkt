#lang r7rs

#|
  What i want to learn:
  1. continuation-passing style
  2. macros
|#

(import (scheme base)
       (scheme write))

; (define product
;   (lambda (ls)
;     (call/cc
;       (lambda (break)
;         (let f ((ls ls))
;           (cond
;             ((null? ls) 1)
;             ((= (car ls) 0) (break 0))
;             (else (* (car ls) (f (cdr ls))))))))))

; (define product-normal
;   (lambda (ls)
;     (let f ((ls ls))
;       (cond
;         ((null? ls) 1)
;         ((= (car ls) 0) 0)
;         (else (* (car ls) (f (cdr ls))))))))


; (let ((x (call/cc (lambda (k) k))))
;   (x (lambda (ignore) "hi")))

; (((call/cc (lambda (k) k)) (lambda (x) x)) "HEY!")

; (product-normal '(1 2 3 4 5))
; (product '(1 2 3 4 5))

; (define fib #;continuation-passing
;   (lambda (n)
;     (if (< n 2)
;        n
;        (+ (fib (- n 1))
;          (fib (- n 2))))))


; (define retry #f)

; (define factorial
;   (lambda (x)
;     (if (= x 0)
;         (call/cc (lambda (k) (set! retry k) 1))
;         (* x (factorial (- x 1))))))


; (factorial 4)
; (retry 1)
; (retry 2)


(define lwp-list '())
(define lwp
  (lambda (thunk)
    (set! lwp-list (append lwp-list (list thunk)))))

(define start
  (lambda ()
    (let ((p (car lwp-list)))
      (set! lwp-list (cdr lwp-list))
      (p))))

(define pause
  (lambda ()
    (call/cc
      (lambda (k)
        (lwp (lambda () (k #f)))
        (start)))))

(define quit
  (lambda ()
    (if (null? lwp-list)
        #f
        (start))))

(lwp (lambda () (let f () (pause) (display "h") (f))))
(lwp (lambda () (quit)))
(lwp (lambda () (let f () (pause) (display "e") (f))))
(lwp (lambda () (let f () (pause) (display "y") (f))))
(lwp (lambda () (let f () (pause) (display "!") (f))))
(lwp (lambda () (let f () (pause) (newline) (f))))
(start)


; (display (string-append "Hello, " "world!\n"))

; 3.3.1
; (define infinite-loop
;   (lambda ()
;     (let ((p (call/cc (lambda (k) (list 0 k)))))
;       (let ((n (car p))
;             (k (car (cdr p))))
;         (display n)
;         (newline)
;         (k (list (+ n 1) k))))))



; (product '(1 2 3 4 5))
; (product-normal '(1 2 3 4 5))
; (infinite-loop)


; (letrec ((f (lambda (x) (cons 'a x)))
;          (g (lambda (x) (cons 'b (f x))))
;          (h (lambda (x) (g (cons 'c x)))))
;   (cons 'd (h '())))


; (letrec ((f (lambda (x k) (k (cons 'a x))))
;          (g (lambda (x k)
;               (f x (lambda (v) (k (cons 'b v))))))
;          (h (lambda (x k) (g (cons 'c x) k))))
;   (h '() (lambda (v) (cons 'd v))))




; (letrec ((fact (lambda (n)
;                  (if (= n 0)
;                     1
;                     (* n (fact (- n 1)))))))
;   (fact 5))

; (let fact_ ((n 5))
;   (if (= n 0)
;       1
;       (* n (fact_ (- n 1)))))



; (define product
;   (lambda (ls)
;     (call/cc
;       (lambda (break)
;         (let f ((ls ls))
;           (cond
;             ((null? ls) 1)
;             ((= (car ls) 0) (break 0))
;             (else (* (car ls) (f (cdr ls))))))))))


; (display (product '(1 2 3 4 5)))

; (define product-normal
;   (lambda (ls)
;     (let f ((ls ls) (cal (lambda (x) x)))
;       (cond
;         ((null? ls) (cal 1))
;         ((= (car ls) 0) 0)
;         (else (f (cdr ls) (lambda (x) (cal (* (car ls) x)))))))))

; (display (product-normal '(1 2 3 4 5)))




(define clock 0)
(define handler #f)

(define start-timer
  (lambda (ticks new-handler)
    (set! handler new-handler)
    (set! clock ticks)))

(define stop-timer
  (lambda ()
    (let ((time-left clock))
      (set! clock 0)
      time-left)))

(define decrement-timer
  (lambda ()
    (when (> clock 0)
      (set! clock (- clock 1))
      (when (= clock 0) (handler)))))

(define-syntax timed-lambda
  (syntax-rules ()
    ((_ formals exp1 exp2 ...)
     (lambda formals (decrement-timer) exp1 exp2 ...))))


(define make-engine
  (let ((do-complete #f) (do-expire #f))
    (define timer-handler
      (lambda ()
        (start-timer (call/cc do-expire) timer-handler)))
    (define new-engine
      (lambda (resume)
        (lambda (ticks complete expire)
          ((call/cc
            (lambda (escape)
              (set! do-complete
                   (lambda (ticks value)
                     (escape (lambda () (complete ticks value)))))
              (set! do-expire
                   (lambda (resume)
                     (escape (lambda ()
                               (expire (new-engine resume))))))
              (resume ticks)))))))
    (lambda (proc)
      (new-engine
       (lambda (ticks)
         (start-timer ticks timer-handler)
         (let ((value (proc)))
           (let ((ticks (stop-timer)))
             (do-complete ticks value))))))))


(define fibonacci
  (timed-lambda (n)
    (if (< n 2)
        n
        (+ (fibonacci (- n 1))
           (fibonacci (- n 2))))))

(define eng
  (make-engine
    (lambda ()
      (fibonacci 10))))

(eng 1
  list
  (lambda (new-eng)
    (set! eng new-eng)
    "expired")) 

; (eng 50
;   list
;   (lambda (new-eng)
;     (set! eng new-eng)
;     "expired"))

; (eng 50
;   list
;   (lambda (new-eng)
;     (set! eng new-eng)
;     "expired")) 

; (eng 50
;   list
;   (lambda (new-eng)
;     (set! eng new-eng)
;     "expired")) 