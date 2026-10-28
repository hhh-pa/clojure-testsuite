(println `a)
(println `(1 2 ~(+ 1 2)))
(println (let [g (gensym "pre")] (symbol? g)))
(println (macroexpand '(when true 1)))
