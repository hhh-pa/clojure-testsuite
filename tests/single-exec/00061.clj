(def ^:dynamic *d61* :root)
(def bf61 (binding [*d61* :captured] (bound-fn [] *d61*)))
(println (bf61) *d61*)
(println (with-local-vars [x 10] (var-set x 20) @x))
