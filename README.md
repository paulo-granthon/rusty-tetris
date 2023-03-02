# Rusty Tetris
Tetris implementation with Rust

## Try it out!
1. Download the latest release. [Click here for direct download](https://github.com/paulo-granthon/rusty-tetris/releases/latest/download/release.rar).
2. Extract the *.rar* file 
3. Open *"rusty_tetris.exe"*

# About
Fully working implementation of Tetris in Rust following official guidelines for the behaviour of the game, including the Super Rotation System that describes how the pieces should rotate and wall-kick.

Local Versus Mode to challenge friends on a 1v1 Tetris match.

Score system that tracks the score history as well as the best scores and saves to binary files.

Profile system capable of storing up to 16 different players by name to differentiate the scores generated during gameplay. Able to create, delete and rename profiles.

Settings screen to customize the controls for singleplayer and versus mode.



<details><summary>

# Screenshots

</summary>


Title screen:  
![alt text](https://github.com/paulo-granthon/rusty-tetris/blob/main/captures/title.png?raw=true)

Singleplayer game:  
![alt text](https://github.com/paulo-granthon/rusty-tetris/blob/main/captures/new.png?raw=true)
![alt text](https://github.com/paulo-granthon/rusty-tetris/blob/main/captures/single.png?raw=true)

Versus mode:  
![alt text](https://github.com/paulo-granthon/rusty-tetris/blob/main/captures/versus.png?raw=true)

Game over:  
![alt text](https://github.com/paulo-granthon/rusty-tetris/blob/main/captures/over.png?raw=true)

Profiles:  
![alt text](https://github.com/paulo-granthon/rusty-tetris/blob/main/captures/profiles.png?raw=true)

Scores:  
![alt text](https://github.com/paulo-granthon/rusty-tetris/blob/main/captures/scores.png?raw=true)

Settings:  
![alt text](https://github.com/paulo-granthon/rusty-tetris/blob/main/captures/settings.png?raw=true)

Rebinding a key:  
![alt text](https://github.com/paulo-granthon/rusty-tetris/blob/main/captures/rebind.png?raw=true)

</details>

# Backlog
 - [x] Single player mode
 - [x] Local versus Mode
 - [x] Score history and best scores tracking
 - [x] Persisting data with binary files
 - [x] Profile system
 - [x] Settings
 - [ ] ~~Mouse support~~
 
# References
[Tetris](https://pt.wikipedia.org/wiki/Tetris) - Wikipedia page  
[Tetromino](https://tetris.fandom.com/wiki/Tetromino) - Naming convention for the Tetris pieces  
[Super Rotation System](https://tetris.fandom.com/wiki/SRS) - Tetromino's rotation system guidelines  
[Random Generator](https://tetris.fandom.com/wiki/Random_Generator) - Pseudo random sequence generator guidelines  

# Technologies
[Rust](https://www.rust-lang.org/) - the programming language  
[doryen-rs](https://github.com/jice-nospam/doryen-rs) - rendering library  
