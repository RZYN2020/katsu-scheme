; Tests for tr7rs
(+ 1 1)
(* 2 3)
(* (+ 1 1) 3)
(= 1 2)
(if (= 1 1) 1 2)
(define list (lambda l l))
(list 1 2 3)
(car (list 1 2 3))
(cdr (list 1 2 3))
(cons 1 (list 2 3))
(cons (list 1 2) (list 3 4))
(define a (list 1 2))
(car a)
(cdr a)
(null? (list))
(null? (list 1))
(null? 3)
(define plus1 (lambda (x) (+ x 1)))
(define add (lambda (x y) (+ x y)))
(plus1 1)
(add 2 3)
(define fact (lambda (n) (if (< n 2) 1 (* n (fact (- n 1))))))
(define S 5)
(foo)
(null? foo1)
(+ 1 foo2)
(fact 10)
(((lambda (mk-length) (mk-length mk-length)) (lambda (mk-length) (lambda (l) (if (null? l) 0 (+ 1 ((mk-length mk-length) (cdr l))))))) (list 1 2 3))
(define Y (lambda (le) ((lambda (f) (f f)) (lambda (f) (le (lambda (x) ((f f) x)))))))
(define Y2 (lambda (le) ((lambda (f) (f f)) (lambda (f) (le (lambda (x1 x2) ((f f) x1 x2)))))))
(define YS (lambda (le) ((lambda (f) (f f)) (lambda (f) (le (lambda x (apply (f f) x)))))))
(define facty (Y (lambda (facty) (lambda (n) (if (< n 2) 1 (* n (facty (- n 1))))))))
(facty 10)
(define fooy2 (Y2 (lambda (fooy2) (lambda (l a) (cond ((null? l) a) (else (+ 1 (fooy2 (cdr l) a))))))))
(fooy2 '(a b) 5)
(define fooys (YS (lambda (fooys) (lambda k (cond ((null? (car k)) (car (cdr k))) (else (+ 1 (fooys (cdr (car k)) (car (cdr k))))))))))
(define foos                      (lambda k (cond ((null? (car k)) (car (cdr k))) (else (+ 1 (foos  (cdr (car k)) (car (cdr k))))))))
(fooys '(a b c) 9)
(foos '(a b c) 9)
(quote a)
(quote (1 a))
'a
(list 'a 'b)
'(a b c)
(define zero? (lambda (x) (= x 0)))
(zero? 0)
(zero? 4)
(not #f)
(not #t)
(eqv? #f #f)
(eqv? 'a 'b)
(eqv? 'a 'a)
(eqv? '() '())
(number? 1)
(number? 'a)
(pair? '(a))
(pair? 3)
(cond ((> 3 2) 'greater)  ((< 3 2) 'less))
(cond ((> 3 4) 'greater)  ((< 3 4) 'less))
(cond ((> 3 3) 'greater)  ((< 3 3) 'less))
(cond ((> 3 3) 'greater) ((< 3 3) 'less) (else 'equal))
(and (= 1 1) (= 2 2))
(and (= 1 1))
(and)
(or (= 1 1) (= 1 2))
(or #f #f)
(or #t)
(or)
(define m 3)
(define fact_part (lambda (factya) (lambda (n) (if (< n 2) 1 (* n (factya (- n 1)))))))
(define factya (Y fact_part))
(factya 10)
(boolean? #f)
(boolean? 'a)
(symbol? 'a)
(symbol? #t)
(procedure? factya)
(procedure? 1)
(apply + '(3 4))
(define compose (lambda (f g) (lambda args (f (apply g args)))))
((compose - *) 3 4)
(define add1 (lambda (x) (+ 1 x)))
(apply add1 '(3))
((lambda x (define a (+ (car x) 2)) a) 3)
((lambda (x) (define a (+ x 3)) a) 6)
(let ((a 2) (b 3)) (+ a b))
(+ 41
1)
(let ((numbers '(3 -2 1 6 -5))
      (nonneg '())
      (neg '()))
  (define loop (lambda (numbers nonneg neg)
	       (cond ((null? numbers) (list nonneg neg))
		     ((>= (car numbers) 0)
		      (loop (cdr numbers)
			    (cons (car numbers) nonneg)
			    neg))
		     ((< (car numbers) 0)
		      (loop (cdr numbers)
			    nonneg
			    (cons (car numbers) neg))))))
  (loop numbers nonneg neg))
(let ((numbers '(3 -2 1 6 -5))
      (nonneg '())
      (neg '()))
  (define loop (lambda (numbers nonneg neg)
	       (cond ((null? numbers) (list nonneg neg))
		     ((or (> (car numbers) 0) (= (car numbers) 0))
		      (loop (cdr numbers)
			    (cons (car numbers) nonneg)
			    neg))
		     ((< (car numbers) 0)
		      (loop (cdr numbers)
			    nonneg
			    (cons (car numbers) neg))))))
  (loop numbers nonneg neg))
(+ 9223372036854775807 9223372036854775807)
(- -9223372036854775807  9223372036854775807)
(* 9223372036854775807 9223372036854775807)
(- -9223372036854775808)
92233720368547758074
(display '(Hello World!))
(newline)
;from spec
'struct
(* 5 8)
'expr
(define x 28)
x
(quote a)
(quote (a b c))
(quote (+ 1 2))
'a
'(a b c)
'()
'(+ 1 2)
'(quote a)
''a
'145932
145932
'#t
#t
(+ 3 4)
((if #f + *) 3 4)
(lambda (x) (+ x x))
((lambda (x) (+ x x)) 4)
(define reverse-subtract
(lambda (x y) (- y x)))
(reverse-subtract 7 10)
(define add4
(let ((x 4))
(lambda (y) (+ x y))))
(add4 6)
((lambda x x) 3 4 5 6)
(if (> 3 2) 'yes 'no)
(if (> 2 3) 'yes 'no)
(if (> 3 2)
(- 3 2)
(+ 3 2))
(cond ((> 3
 2) 'greater)
((< 3
 2) 'less))
(cond ((> 3
 3) 'greater)
((< 3
 3) 'less)
(else
 'equal))
(and (= 2 2) (> 2 1))
(and (= 2 2) (< 2 1))
(and 1 2 'c '(f g))
(and)
(or
 (= 2 2) (> 2 1))
(or
 (= 2 2) (< 2 1))
(or
 #f #f #f)
(or
 '(b c) (car 'a))
(let ((x 2) (y 3))
(* x y))
(let ((x 2) (y 3))
(let ((x 7)
(z (+ x y)))
(* z x)))
'prog
(define add3
(lambda (x) (+ x 3)))
(add3 3)
(define first car)
(first '(1 2))
(let ((x 5))
(define bar (lambda (a b) (+ (* a b) a)))
(define foo (lambda (y) (bar x y)))
(foo (+ x 3)))
'procs
(eqv? 'a 'a)
(eqv? 'a 'b)
(eqv? '(a) '(a))
(eqv? (list 'a) (list 'a))
(eqv? '() '())
(eqv? 2 2)
(eqv? car car)
(let ((n (+ 2 3)))
(eqv? n n))
(let ((x '(a)))
(eqv? x x))
(let ((x '()))
(eqv? x x))
(let ((p (lambda (x) x)))
(eqv? p p))
(eqv? #f 'nil)
(number? 3)
(number? '(1))
(+ 3 4)
(* 4 5)
(- 3 4)
(- 3)
#t
#f
'#f
(not #t)
(not 3)
(not '(3))
(not #f)
(not '())
(not 'nil)
(boolean? #f)
(boolean? 0)
(boolean? '())
(pair? '('a . 'b))
(pair? '(a b c))
(pair? '())
(cons 'a '())
(cons '(a) '(b c d))
(cons 'a 3)
(cons '(a b) 'c)
(car '(a b c))
(car '((a) b c d))
(car '(1 . 2))
(car '())
(cdr '((a) b c d))
(cdr '(1 . 2))
(cdr '())
(symbol? 'foo)
(symbol? (car '(a b)))
(symbol? 'nil)
(symbol? '())
(symbol? #f)
(procedure? car)
(procedure? 'car)
(procedure? (lambda (x) (* x x)))
(procedure? '(lambda (x) (* x x)))
(apply + '(3 4))
(define compose
(lambda (f g)
(lambda args
(f (apply g args)))))
((compose - *) 3 4)
'examples
(define list (lambda l l))
(list 'a 'b 'c)
(define list? (lambda (l)
  (cond ((null? l) #t)
	((not (pair? l)) #f)
	(else (list? (cdr l)))
	)
))
(list? '(a b c))
(list? (cons 'a 'b))
(define append (lambda (l t)
  (cond ((null? l) t)
	(else (cons (car l) (append (cdr l) t))))
))
(append '() '(a))
(append '(a b) '(c d))
(define assv (lambda (obj l)
  (cond ((null? l) #f)
	((eqv? obj (car (car l))) (car l))
	(else (assv obj (cdr l)))
)))
(define e '((a 1) (b 2) (c 3)))
(assv 'a e)
(assv 'b e)
(assv 'd e)
(assv 5 '((2 3) (5 7) (11 13)))
(let ((Y (lambda (phi)
	   ((lambda (f) (f f))
	    (lambda (f)
	      (phi (lambda x (apply (f f) x))))))))
  (let ((fact
	 (Y (lambda (fact)
	      (lambda (n)
		(if (< n 2) 1
		    (* n (fact (- n 1)))))))))
    (fact 5)))
; stack test (check that no stack used on recursive call) (loop 1000000 0)
(define loop (lambda (x y) (if (< x y) x (loop (- x 1) y))))
(loop 10 0)
(loop 2000 0)