(println (nil? nil) (nil? false) (some? nil) (some? 0))
(println (true? true) (false? false) (boolean nil) (boolean "x"))
(println (first []) (get {} :k) (nth nil 0 nil))
(println (if-let [x false] :t :f) (if-some [x false] :t :f))
