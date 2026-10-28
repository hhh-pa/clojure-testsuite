(defn f [{:keys [a] [b c] :pair {d :deep} :nest :as whole}] [a b c d (sort (keys whole))])
(println (f {:a 1 :pair [2 3] :nest {:deep 4} :extra 5}))
(let [[[x1 x2] & xs] [[1 2] 3 4]] (println x1 x2 xs))
(println (nth [10 20 30] 1))
