(println (persistent! (-> (transient [1]) (conj! 2) (conj! 3))))
(println (sort (persistent! (assoc! (transient {:a 1}) :b 2))))
(println (sort (persistent! (conj! (transient #{1}) 2))))
(println (persistent! (assoc! (transient [1 2]) 0 :z)))
