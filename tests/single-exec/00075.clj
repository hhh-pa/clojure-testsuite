(println (case 2 (1 2) :low (3 4) :high :other))
(println (case \a \a :is-a :other))
(println (condp (fn [a b] (some a b)) [1 2 3] #{2} :>> identity :none))
