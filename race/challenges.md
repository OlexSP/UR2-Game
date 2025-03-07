# Challenges

* Add a second player!
    * Variant A: The two players can overlap, harmlessly
    * Variant B: The two players are separated into their own lanes, and cannot cross to the same lane
    * Variant C: The two players crash if they touch each other. Requires implementing the "cars can move forward and backward a little".
* Powerups!  All powerups wear off after a short time, so you'll need to use `Timer`s
    * Powerup A: Boost car maneuverability
    * Powerup B: Armor - car can withstand more obstacle hits
    * Powerup C: Phase shift - car can move through obstacles
    * Powerup D: Explosion - all visible obstacles are cleared
* Hazards!  All hazards wear off after a short time, so you'll need to use `Timer`s
    * Hazard A: Oil Slick - car is unable to control movement
    * Hazard B: Anti-Powerup - hitting the anti-powerup causes a new type of obstacle to appear
    * Hazard C: Afterburners - road speed increases
* Polish
    * Make the car turn (rotate) smoothly instead of suddenly, and have the speed the car moves in the y direction vary proportianally to how much the car is rotated.
    * Randomize the rotation of the obstacles
    * Add support for driving the car via mouse input
    * Add controls to change or turn off the music
    * Add text indicators of things that happen (collisions, powerups, etc.)
    * Add the ability to pause and resume the game
    * Collect points for every obstacle that you pass, display the score in the corner during the game, add an end screen showing the final score.
    * Instead of ignoring obstacles that collide with each other, take one of the colliding items and set it to a new random starting position.
    * Make it so you can't hit the same obstacle twice (currently if you wiggle back and forth you can run into the same obstacle multiple times)