; Basic arithmetic
(+ 1 1) ; >2
(* 2 3) ; >6
(/ 4 2) ; >2
(- 4 2) ; >2
(= 2 2) ; >true
(= 2 3) ; >false
(> 2 3) ; >false
(< 2 3) ; >true

; Nested expressions
(+ 1 (+ 2 3)) ; >6
(* (+ 1 1) 3) ; >6
(/ (+ 1 1) 3) ; >0

; control flow
(if (= 2 2) 1 2) ; >1

; functions
(define plus1 (lambda (x) (+ x 1))) ;>None
(define add (lambda (x y) (+ x y))) ;>None
(plus1 1) ; >2
(add 1 2) ; >3


; list 
(list 1 2 3) ; >(1 2 3)
(car (list 1 2 3)) ; >1
(cdr (list 1 2 3)) ; >(2 3)
(cons 1 (list 2 3)) ; >(1 2 3)
(cons (list 1 2) (list 3 4)) ; >((1 2) 3 4)

; complex
(define fact (lambda (n) (if (< n 2) 1 (* n (fact (- n 1)))))) ;>None
(fact 10) ;>3628800
(((lambda (mk-length) (mk-length mk-length)) (lambda (mk-length) (lambda (l) (if (null? l) 0 (+ 1 ((mk-length mk-length) (cdr l))))))) (list 1 2 3)) ;>None
(define Y (lambda (le) ((lambda (f) (f f)) (lambda (f) (le (lambda (x) ((f f) x))))))) ;>None
(define Y2 (lambda (le) ((lambda (f) (f f)) (lambda (f) (le (lambda (x1 x2) ((f f) x1 x2))))))) ;>None
(define YS (lambda (le) ((lambda (f) (f f)) (lambda (f) (le (lambda x (apply (f f) x))))))) ;>None
(define facty (Y (lambda (facty) (lambda (n) (if (< n 2) 1 (* n (facty (- n 1)))))))) ;>None
(facty 10) ;>3628800