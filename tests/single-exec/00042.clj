(let [al (java.util.ArrayList.)] (.add al "a") (.add al "b") (println (vec al) (.size al)))
(let [hm (java.util.HashMap.)] (.put hm "k" "v") (prn (into {} hm)))
(println (str (java.util.Arrays/asList (into-array ["x" "y"]))))
