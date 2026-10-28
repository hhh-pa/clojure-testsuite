(def p (proxy [java.util.function.Supplier] [] (get [] :supplied)))
(println (.get p))
(println (instance? java.util.function.Supplier p))
