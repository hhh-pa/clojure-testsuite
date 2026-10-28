(prn [(= ##NaN ##NaN) (== ##NaN ##NaN) (== ##Inf ##Inf)])
(println ({##NaN :nan} ##NaN) (contains? #{##NaN} ##NaN))
(prn [##NaN ##Inf ##-Inf])
(println (= 0.0 -0.0) (str -0.0))
