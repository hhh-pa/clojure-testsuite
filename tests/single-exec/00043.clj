(require '[clojure.java.io :as io])
(let [f (java.io.File/createTempFile "cljst" ".txt")]
  (spit f "line1\nline2\n")
  (print (slurp f))
  (prn (with-open [r (io/reader f)] (doall (line-seq r))))
  (println (.delete f)))
