(let fibo (λ (x)
    (assert (> x 0))
    (if (< x 2) 
        (ret x)
        (ret (+ (fibo (- x 1)) (fibo (- x 2)))))))

(assert (= (fibo 13) 233))