+ picor7rs: pure functional subset
+ add side effect
+ disable var len(only builtin function can be varlen)

+ 对Scheme语法理解有误的地方
  + ((lambda x x) 3 4 5 6) => (3 4 5 6)
  + (define list (lambda l l))
  + (list 1 2 3) => (1 2 3)
  + disable了算了