I havent programmed in rust in a while
wanted to make something basic to get up to speed with it again

I have a strategy in wordle that I use and wanted to implement it in code.

most people when they find a letter than matches will only try words with that letter in that place

I think this is inefficient as our objective should be to narrow down the scope of what words are possible
not confirm letters that we know for sure.


## Plan(in scope)
1. Choose a word from a list of words
2. character wise comparison and feedback of chosen word vs guessed word
3. implement a solving algorithim, broadly using the strategy above

## Possible ideas(out of scope, for now)
1. Train a model to solve wordle(some kind of reinforcement learning thing is probably going to be best here
    you might be able to get away with a loss function where every letter match is -n, word match -m, and no match is 0)
2. TUI to play yourself, with toggleable AI/algo help
3. create and compare a bunch of these algorithims