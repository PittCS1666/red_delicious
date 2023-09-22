# Jordquest

by Red Delicious

## Team Members
* Advanced Topic Subteam 1: Networking
	* Ian Whitfield
	* Jordan Brudenell
	* Ruoheng Xu

* Advanced Topic Subteam 2: Procedural Generation
	* Sam Durigon
	* Alex Lampe
	* Brendan Szewczyk
	* Garrett DiCenzo

## Game Description

Multiplayer Hack n Slash in a randomly generated arena with PvE camps you can
kill to earn items, and other players you can kill to earn points. Most
points at the end of 5 minutes wins!

## Advanced Topic Description

### Networking

UDP networking connects together players with a listen server on the host
player's computer. Connecting over LAN directly by IP. Focus on reliability
and performance.
    
### Procedural Generation

Each round starts with a randomly generated arena, placing enemy camps, items,
decorations, shops, obstacles, and terrain throughout the map. Focus on balance,
complexity, and natural appearance.

## Midterm Goals

* Networking: Players can see each other in a lobby 
* ProcGen: One mostly gameplay-complete map should be produced, not necessarily good. Basic minimap.
* Gameplay: Sword should work to do damage, enemies should be able to kill you
* Scoring: Score system for killing enemies is implemented and there is a timer to encourage risk-taking and offensive tactics.
* UI supports currently built features.

## Final Goals

* 25%: Networking: Complete listen server, network should not be an overbearing and domineering issue for gameplay
* 25%: ProcGen: Maps are generated so that they appear to be varied. They should also look somewhat natural by not repeating too many objects or entities. There should be roughly 5 different camp types and maybe 10 different decorations.
* 15%: Gameplay: Sword combat finished, at least one extra ability, upgrades such as increased damage or reduced damage taken work, enemies should be able to kill you and some enemies will have extra powers such as increased range or health.
* 5%: Scoring: Working leaderboard with statistics, scoring, and timer. The score should increase when the player kills enemies, captures bases, or ganks enemy players.
* 5%: UI supports all required features such as play, inventory management, ability usage, attacks, powers, viewing upgrades, and rooms. The game has visual/auditory feedback for players' and enemies' actions as well as environmental sounds such as ambiance and background sounds.
* 5%: The game runs at an acceptable speed and all abilities work together with minimal to no bugs.

## Stretch Goals

* Rollback and prediction net code. Specifically this stretch goal should solve reducing latency, improving hit registration, and enhance overall multiplayer responsiveness. A specific library will most likely be required to implement this feature. Network architecture design will have to be planned around our networked gameplay. We have to define critical game events and states that need to be synchronized among players. We need to decide the level of authority and synchronization needed for different game elements, such as character movement, combat actions, and environmental changes. We'll need to develop prediction algorithms that allow clients to simulate future game states based on their input and the current game states. We will also need to implement techniques like client-side prediction, interpolation, and extrapolation to smooth out discrepancies between the local client's view and the server's authoritative state.
* _Epic_ boss battle which includes custom model, animation, and attacks for the enemy boss. The boss should have a specific roll in the game, as well as a suitable backstory. The boss should have unique mechanics and abilities. These should be challenging, engaging, and different from regular enemies to make the encounter memorable. The boss should have a dedicated arena or chamber within the dungeon for the players to fight in. The environment should be visually distinct and conducive to the boss fight. The arena should have interactive elements that players can use strategically during the battle. The boss requires unique sounds and music as well. The boss will require a new AI and behavior patterns to implement its strategy. There should be various phases for the boss fight, each with different attack patterns and behaviors, to keep players engaged. The boss will require finely-tuned balance to ensure the battle is challenging, but not unfairly punishing towards the player. 
