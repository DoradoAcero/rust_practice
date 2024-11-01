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
I completed what I had planned, and my strat was by far the worst(when I programmed it wrong)

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


Ok, we got multithreading working well, enough
time to move onto one of the three options I had for later down the line or move onto something else
I might have a look at more wordle strats. Look on the internet and try to implement them.
If I go with option 1. I want to use something with a ML lean aka not Rust, Something like mojo.
I could look into a tui, but im not feeling motivated to do that atm, idk why.

So lets find/make some algorithims.
https://www.poirrier.ca/notes/wordle-optimal/
They say with a decision tree based algo, you can solve the 12972 word dict with 4.077 guesses on average, I am running a ~9000 word dict, with a average of about 4.5
All of these appear to be a decision tree construction, something I kinda want to do in mojo.

One thing they talk about in comments following the strats are positioning of the chrs in the words limiting the search space further down the line and why one permutation of a set of letters is superior to another
I want to try and build in the position of the letters into the heuristic, somehow.

------------------------------------------------------------
Average guess count: 5.0982, Time Taken: 28.68s 2.87ms/iter
------------------------------------------------------------
Average guess count: 4.5731, Time Taken: 3.40s 340.42µs/iter
------------------------------------------------------------
Average guess count: 4.5502, Time Taken: 99.14s 9.91ms/iter
------------------------------------------------------------
Average guess count: 4.4103, Time Taken: 153.77s 15.38ms/iter
------------------------------------------------------------

it worked, but by golly it is slow
it makes sense though, im looping through way too much.
added a log term to the position to slightly deincentivize it, ill increase the log term to do this more, see if it helps at all
log 2 seems to be the most based

------------------------------------------------------------
Average guess count: 5.078, Time Taken: 8.83s 2.94ms/iter
------------------------------------------------------------
Average guess count: 4.595666666666666, Time Taken: 846.73ms 282.24µs/iter
------------------------------------------------------------
Average guess count: 4.544, Time Taken: 8.14s 2.71ms/iter
------------------------------------------------------------
Average guess count: 4.369, Time Taken: 8.23s 2.74ms/iter
------------------------------------------------------------