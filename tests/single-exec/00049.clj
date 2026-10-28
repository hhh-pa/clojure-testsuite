;; count of a string is its UTF-16 length, so a combining character
;; sequence (e + U+0301) counts more than its code point count.
(println (count (str (char 101) (char 769))))
(println (count (str (char 104) (char 233) (char 108) (char 108) (char 111))))
