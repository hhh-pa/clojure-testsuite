(prn (sort (frequencies "abba")))
(println (sort (map (fn [[k v]] [k (sort v)]) (group-by odd? (range 6)))))
(println (sort (zipmap [:a :b] [1 2])))
(println (vals {:a 1}))
