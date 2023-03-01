(qualif EqZero ((a0 int)) (a0 = 0))
(qualif GtZero ((a0 int)) (a0 > 0))
(qualif GeZero ((a0 int)) (a0 >= 0))
(qualif LtZero ((a0 int)) (a0 < 0))
(qualif LeZero ((a0 int)) (a0 <= 0))
(qualif Eq ((a0 int) (a1 int)) (a0 = a1))
(qualif Gt ((a0 int) (a1 int)) (a0 > a1))
(qualif Ge ((a0 int) (a1 int)) (a0 >= a1))
(qualif Lt ((a0 int) (a1 int)) (a0 < a1))
(qualif Le ((a0 int) (a1 int)) (a0 <= a1))
(qualif Le1 ((a0 int) (a1 int)) (a0 <= (a1 - 1)))
(qualif Add2 ((a0 int) (a1 int) (a2 int)) (a0 = (a1 + a2)))
(qualif Sub2 ((a0 int) (a1 int) (a2 int)) (a0 = (a1 - a2)))
(qualif MyQ1 ((a4 int) (a5 int) (a6 int)) ((a4 + a5) <= (a6 + a3)))
(data Pair 2 = [| Pair { fst: @(0), snd: @(1) } ])
(data Unit 0 = [| Unit { }])
(constant a3 int)(constant a2 int)(constant a1 int)(constant a0 int)(var $k0 ((bool) (int) (int) (int)))
(var $k1 ((bool) (int) (int) (int) (bool)))

(constraint
  (forall ((_ Unit) (a0 = 1048576))
    (forall ((_ Unit) (a1 = -2147483648))
      (forall ((_ Unit) (a2 = 4096))
        (forall ((_ Unit) (a3 = 4294965096))
          (forall ((a4 int) (true))
            (forall ((a5 int) (true))
              (forall ((a6 int) (true))
                (forall ((_ Unit) (a4 >= 0))
                  (forall ((_ Unit) (a5 >= 0))
                    (and
                      (forall ((_ Unit) (~((a4 + a5) >= a3)))
                        (and
                          (forall ((_ Unit) (~((0 <= a4) && (a4 < a3))))
                            (forall ((a7 bool) (a7 = false))
                              (tag ($k0 a7 a4 a5 a6) "0")
                            )
                          )
                          (forall ((_ Unit) (~(~((0 <= a4) && (a4 < a3)))))
                            (forall ((a8 bool) (a8 = ((0 <= a5) && (a5 < a3))))
                              (tag ($k0 a8 a4 a5 a6) "0")
                            )
                          )
                          (forall ((a9 bool) (true))
                            (forall ((_ Unit) ($k0 a9 a4 a5 a6))
                              (and
                                (forall ((_ Unit) (~a9))
                                  (forall ((a10 bool) (a10 = false))
                                    (tag ($k1 a10 a4 a5 a6 a9) "1")
                                  )
                                )
                                (forall ((_ Unit) (~(~a9)))
                                  (forall ((a11 bool) (a11 = (a4 <= (a4 + a5))))
                                    (tag ($k1 a11 a4 a5 a6 a9) "1")
                                  )
                                )
                                (forall ((a12 bool) (true))
                                  (forall ((_ Unit) ($k1 a12 a4 a5 a6 a9))
                                    (tag (a12 = ((((0 <= a4) && (0 <= a5)) && (a4 <= (a4 + a5))) && ((a4 + a5) < a3))) "2")
                                  )
                                )
                              )
                            )
                          )
                        )
                      )
                      (forall ((_ Unit) (~(~((a4 + a5) >= a3))))
                        (tag (false = ((((0 <= a4) && (0 <= a5)) && (a4 <= (a4 + a5))) && ((a4 + a5) < a3))) "3")
                      )
                    )
                  )
                )
              )
            )
          )
        )
      )
    )
  )
)
