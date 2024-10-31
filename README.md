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


## Results
I completed what I had planned, and my strat was by far the worst

------------------------------------------------------------
Average guess count: 5.083 #Basic strat
------------------------------------------------------------
Average guess count: 4.553 #Stake, then count, then basic
------------------------------------------------------------
Average guess count: 5.713 #Custom strat
------------------------------------------------------------

I had chatgpt implement a basic parrallelizing method in another branch
I would like to give that a shot myself

------------------------------------------------------------
Average guess count: 5.0476, Time Taken: 27.32s 2.73ms/iter
------------------------------------------------------------
Average guess count: 4.5954, Time Taken: 2.65s 265.18µs/iter
------------------------------------------------------------
Average guess count: 4.5546, Time Taken: 38.24s 3.82ms/iter
------------------------------------------------------------

After multithreading them, they are quite a bit faster to run through, I have really just done a first pass on multithreading though
Not sure how much more I want to perfect/understand it quite yet