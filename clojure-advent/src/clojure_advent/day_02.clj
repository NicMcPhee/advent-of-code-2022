(ns day-02
  (:require [clojure.string :as str]))

(defn char->move
  [c]
  (case c
    ("A" "X") :rock
    ("B" "Y") :paper
    ("C" "Z") :scissors))

(defn move->output-diff
  [move]
  (case move
    :rock -1     ; We should lose 
    :paper 0     ; We should draw
    :scissors +1 ; We should win
    ))

(def move-scores
  {:rock 1,
   :paper 2 
   :scissors 3})

(defn i-won
  [their-score my-score]
  (or (= 1 (- my-score their-score))
      (= 2 (- their-score my-score))))

(defn score-moves-part-1
  [[their-move my-move]]
  (let [their-score (get move-scores their-move)
        my-score (get move-scores my-move)]
    (+ my-score
       (cond
         (i-won their-score my-score) 6
         (= their-score my-score) 3
         :else 0))))

(defn make-legal
  [score]
  (cond
    (< score 1) 3
    (> score 3) 1
    :else score))

(defn score->move
  [score]
  (case score
    1 :rock
    2 :paper
    3 :scissors))

(defn compute-my-move
  [their-move target-outcome]
  (->> their-move
       (get move-scores) ; their score
       (+ (move->output-diff target-outcome))
       make-legal
       score->move
       )
  )

(defn score-moves-part-2
  [[their-move target-outcome]]
  (let [my-move (compute-my-move their-move target-outcome)]
    (score-moves-part-1 [their-move my-move])))

(defn score-line
  [score-moves line-str]
  (as-> line-str $
    (str/split $ #" ")
    (map char->move $)
    (score-moves $)))

(defn total-score-part-1
  [input-text]
  (->>
   input-text
   str/split-lines
   (map (partial score-line score-moves-part-1))
   (reduce +)))

(defn total-score-part-2
  [input-text]
  (->> input-text
       str/split-lines
       (map (partial score-line score-moves-part-2))
       (reduce +)))

(def demo-str
  "A Y\nB X\nC Z")

(defn day-02
  []
  (println (str "Total is " (total-score-part-1 (slurp "../inputs/day_02.input"))))
  (println (str "Part 2 total is " (total-score-part-2 (slurp "../inputs/day_02.input")))))
