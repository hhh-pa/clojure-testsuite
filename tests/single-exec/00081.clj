(def q81 (conj clojure.lang.PersistentQueue/EMPTY 1 2 3))
(println (peek q81) (peek (pop q81)))
(println (seq q81))
(println (count q81) (empty? (into clojure.lang.PersistentQueue/EMPTY [])))
