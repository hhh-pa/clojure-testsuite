;; with-redefs cannot intercept direct-linked core fns, but rebinds user vars.
(println (with-redefs [inc dec] (inc 5)))
(def w80 1)
(println (with-redefs [w80 2] w80))
