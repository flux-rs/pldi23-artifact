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
(constant a3 int)(constant a2 int)(constant a1 int)(constant a0 int)(var $k0 ((int) (int) (int)))
(var $k1 ((int) (int) (int)))
(var $k2 ((int) (int) (int) (int)))
(var $k3 ((int) (int) (int) (int) (int)))
(var $k4 ((int) (int) (int)))
(var $k5 ((int) (int) (int) (int)))
(var $k6 ((int) (int) (int) (int) (int)))
(var $k7 ((int) (int) (int) (int) (int) (int)))
(var $k8 ((int) (int) (int) (int) (int)))
(var $k9 ((int) (int) (int) (int) (int) (int) (int) (int)))
(var $k10 ((int) (int) (int) (int) (int) (int) (int) (int) (int)))

(constraint
  (forall ((_ Unit) (a0 = 1048576))
    (forall ((_ Unit) (a1 = -2147483648))
      (forall ((_ Unit) (a2 = 4096))
        (forall ((_ Unit) (a3 = 4294965096))
          (forall ((a4 int) (true))
            (forall ((a5 int) (true))
              (and
                (forall ((a6 int) (true))
                  (tag ($k0 a6 a4 a5) "0")
                )
                (forall ((a7 int) (a7 = 0))
                  (forall ((a8 int) (a8 = 0))
                    (and
                      (tag ($k1 a7 a4 a5) "1")
                      (tag ($k2 a8 a4 a5 a7) "1")
                    )
                  )
                )
                (forall ((a9 int) (true))
                  (forall ((a10 int) (a10 = 0))
                    (forall ((a11 int) (a11 = 0))
                      (tag ($k3 a9 a4 a5 a10 a11) "1")
                    )
                  )
                )
                (forall ((a12 int) (true))
                  (forall ((a13 int) (true))
                    (forall ((_ Unit) (and ($k4 a12 a4 a5) ($k5 a13 a4 a5 a12)))
                      (forall ((a14 int) (a14 = 0))
                        (forall ((a15 int) (a15 = 0))
                          (and
                            (tag ($k6 a12 a4 a5 a14 a15) "1")
                            (tag ($k7 a13 a4 a5 a14 a15 a12) "1")
                          )
                        )
                      )
                    )
                  )
                )
                (forall ((a16 int) (true))
                  (forall ((a17 int) (true))
                    (forall ((_ Unit) (and ($k1 a16 a4 a5) ($k2 a17 a4 a5 a16)))
                      (and
                        (forall ((_ Unit) (~(a17 < a5)))
                          (forall ((a18 int) (true))
                            (forall ((a19 int) (true))
                              (forall ((_ Unit) (and ($k6 a18 a4 a5 a16 a17) ($k7 a19 a4 a5 a16 a17 a18)))
                                (tag ((a18 + a19) <= (a4 + a3)) "2")
                              )
                            )
                          )
                        )
                        (forall ((_ Unit) (~(~(a17 < a5))))
                          (and
                            (and
                              (forall ((a20 int) (true))
                                (forall ((_ Unit) ($k3 a20 a4 a5 a16 a17))
                                  (tag ($k8 a20 a4 a5 a16 a17) "3")
                                )
                              )
                              (tag ((0 <= a17) && (a17 < a5)) "3")
                            )
                            (forall ((a21 int) (true))
                              (forall ((_ Unit) ($k8 a21 a4 a5 a16 a17))
                                (forall ((a22 int) (true))
                                  (forall ((a23 int) (true))
                                    (forall ((_ Unit) ((a22 + a23) <= (a4 + a3)))
                                      (and
                                        (and
                                          (tag ($k9 a22 a4 a5 a16 a17 a21 a22 a23) "4")
                                          (tag ($k10 a23 a4 a5 a16 a17 a21 a22 a23 a22) "4")
                                        )
                                        (forall ((a24 int) (true))
                                          (forall ((a25 int) (true))
                                            (forall ((_ Unit) (and ($k6 a24 a4 a5 a16 a17) ($k7 a25 a4 a5 a16 a17 a24)))
                                              (and
                                                (tag ($k9 a24 a4 a5 a16 a17 a21 a22 a23) "4")
                                                (tag ($k10 a25 a4 a5 a16 a17 a21 a22 a23 a24) "4")
                                              )
                                            )
                                          )
                                        )
                                        (forall ((a26 int) (a26 = (a16 + 1)))
                                          (forall ((a27 int) (a27 = (a17 + 1)))
                                            (and
                                              (tag ($k1 a26 a4 a5) "5")
                                              (tag ($k2 a27 a4 a5 a26) "5")
                                            )
                                          )
                                        )
                                        (forall ((a28 int) (true))
                                          (forall ((_ Unit) ($k3 a28 a4 a5 a16 a17))
                                            (forall ((a29 int) (a29 = (a16 + 1)))
                                              (forall ((a30 int) (a30 = (a17 + 1)))
                                                (tag ($k3 a28 a4 a5 a29 a30) "5")
                                              )
                                            )
                                          )
                                        )
                                        (forall ((a31 int) (true))
                                          (forall ((a32 int) (true))
                                            (forall ((_ Unit) (and ($k9 a31 a4 a5 a16 a17 a21 a22 a23) ($k10 a32 a4 a5 a16 a17 a21 a22 a23 a31)))
                                              (forall ((a33 int) (a33 = (a16 + 1)))
                                                (forall ((a34 int) (a34 = (a17 + 1)))
                                                  (and
                                                    (tag ($k6 a31 a4 a5 a33 a34) "5")
                                                    (tag ($k7 a32 a4 a5 a33 a34 a31) "5")
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
  )
)
