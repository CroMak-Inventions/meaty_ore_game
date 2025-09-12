This is an Asteroids-styled arcade game programmed in Rust &amp; Bevy

It was inspired from the Bevy Basics Series of videos on YouTube by
[Zymartu Games](https://www.youtube.com/@ZymartuGames)
(*This is a great place to start your journey into making Bevy games in Rust.
Highly recommended.*)

<img width="1680" height="1050" alt="image" src="https://github.com/user-attachments/assets/d3166153-f8d9-4b48-be52-f8666f70b509" />

Right now it is mostly complete in terms of features.  In addition to the
features laid out in the videos there are a few extra features that have
been added.

- The spaceship has zero G movement behavior.
- The spaceship &amp; asteroids wrap to the opposite side of the screen
  instead of going off into the distance forever.
- Instead of a steady stream of missiles, there is a firing rate currently
  configured to 4 shots per second.
- There is a maximum of 3 missiles allowed on the screen at any given time.
- In the event of a collision:
    - The missile is despawned
    - Asteroid debris is spawned in with a random size & velocity.
- Sound effects for shooting &amp; asteroid collisions.
- Ambient music in the background.
- Score is displayed in the top-left.
- Last score is displayed in the top-right.
- High score is displayed in the top-center.
- Asteroids are spawned in waves, making it more challenging.
