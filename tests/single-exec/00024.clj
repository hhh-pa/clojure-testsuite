(println (try (/ 1 0) (catch ArithmeticException e :caught)))
(println (try (throw (ex-info "oops" {:code 7})) (catch clojure.lang.ExceptionInfo e (ex-data e))))
(println (try :body (finally :ignored)))
(let [log (atom [])]
  (try (throw (ex-info "x" {})) (catch Exception e (swap! log conj :caught)) (finally (swap! log conj :finally)))
  (println @log))
(println (try (throw (ex-info "m" {:a 1})) (catch Exception e (:a (ex-data e)))))
