(fixpoint "--rewrite")
(fixpoint "--interpreter=false")
(fixpoint "--fuel=3")

(constant sum  (func(0, [int, int])))

(define sum(n : int) : int = { if (n <= 0) then (0) else (n + sum (n-1)) })

(constraint 
   (forall ((x int) ((7 <= x) && (0 <= (sum (x-7)))))
       ((28 <= (sum x)))
   )
)

