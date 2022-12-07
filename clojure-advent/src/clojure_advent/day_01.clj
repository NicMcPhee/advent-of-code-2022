(ns day-01
  (:require [clojure.string :as str]))

(defn totals
  []
  (->> "../inputs/day_01.input"
       slurp
       str/split-lines
       (partition-by #(= % ""))
       (remove #(= % '(""))) 
       (map #(map (fn [s] (Integer/parseInt s)) %))
       (map #(reduce + %))
       ))

(defn day-01
  []
  (let [tots (reverse (sort (totals)))
        biggest (apply max tots)
        biggest-3 (reduce + (take 3 tots))]
    (println (str "Biggest is " biggest))
    (println (str "Sum of biggest 3 is " biggest-3))))