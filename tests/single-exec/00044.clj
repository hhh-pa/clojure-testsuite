(let [f (future (+ 1 2))] (println @f (realized? f)))
(let [p (promise)] (future (deliver p :done)) (println @p))
(println (deref (future :x) 1000 :timeout))
(println (sort (pmap inc [1 2 3])))
