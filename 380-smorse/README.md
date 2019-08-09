# `smorse`: Smooshed Morse

## [Part 1](https://www.reddit.com/r/dailyprogrammer/comments/cmd1hb/20190805_challenge_380_easy_smooshed_morse_code_1/)

## [Part 2](https://www.reddit.com/r/dailyprogrammer/comments/cn6gz5/20190807_challenge_380_intermediate_smooshed/)

## Incremental Progress

Because this quite obviously is going to take a while, it dumps its state when
handling SIGINT, and can restart itself from a previous state. The most recent such dump:

```
Checked 472145920 inputs; continue with
  smorse --bonus-2-2 17803283686748604
no valid input was found
```