(println (ex-message (ex-info "hello" {:n 1})))
(println (ex-data (ex-cause (ex-info "m" {} (ex-info "c" {:n 7})))))
(println (ex-data (ex-info "m" nil)) (ex-cause (ex-info "m" {})))
