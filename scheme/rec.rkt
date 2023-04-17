#lang r7rs
(import (scheme base)
        (scheme write))


; Y-combinator
(define Y (lambda (phi)
            ((lambda (f) (f f))
              (lambda (f)
                (phi (lambda (x) ((f f) x)))))))

(define factY (Y (lambda (f)
                   (lambda (n)
                     (if (= n 0)
                         1
                         (* n (f (- n 1))))))))
(define fact (lambda (n)
               (if (= n 0)
                   1
                   (* n (fact (- n 1))))))

(display (fact 5))
(newline)
(display (factY 5))
(newline)

; lazy list
(define ones_lr
  (letrec ((x (cons 1 (lambda () x))))
    x))

(define ones
  (cons 1
        (lambda ()
          ones)))

(display (car ones_lr))

(display (car ones))