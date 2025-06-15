Ok this is just a basic asteroids game programmed in Rust &amp; Bevy

It was taken from the Bevy Basics Series of videos on YouTube by
[Zymartu Games](https://www.youtube.com/@ZymartuGames)

Right now it is in a basic functional state.  In addition to the features laid
out in the videos there are a couple of extra features that have been added.

- Instead of a steady stream of missiles, there is a firing rate currently
  configured to 4 shots per sec.
- There is a maximum of 3 missiles allowed on the screen at any given time.
- In the event of a collision, the missile is despawned as well as the
  asteroid.

## Proposed Features

- It would be nice if the asteroids would wrap to the other side of the screen
  instead of going off into the distance forever.
- It would be nice if the spaceship would also wrap to the other side of the
  screen.
- It would be nice if the spaceship would have zero G movement behavior, like
  it's in space.
