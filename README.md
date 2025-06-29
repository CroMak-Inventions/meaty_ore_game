Ok this is just a basic asteroids game programmed in Rust &amp; Bevy

It was taken from the Bevy Basics Series of videos on YouTube by
[Zymartu Games](https://www.youtube.com/@ZymartuGames)

Right now it is in a basic functional state.  In addition to the features laid
out in the videos there are a couple of extra features that have been added.

- Instead of a steady stream of missiles, there is a firing rate currently
  configured to 4 shots per sec.
- There is a maximum of 3 missiles allowed on the screen at any given time.
- In the event of a collision:
    - The missile is despawned
    - Asteroid debris is spawned in with a random size & velocity.
    - Sound effect for a missile hitting an Asteroid.
- Asteroids and the spaceship wrap to the opposite side of the screen
  instead of going off into the distance forever.
- Sound effect for shooting.
- Ambient music in the background.

## Proposed Features

- It would be nice if the spaceship would have zero G movement behavior, like
  it's in space.
