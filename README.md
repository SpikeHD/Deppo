<div align=center>
  <h1>Deppo</h1>
  <p>
    Little friends on your computer!!
  </p>

  <a href="https://discord.gg/agQ9mRdHMZ">
    https://discord.gg/agQ9mRdHMZ
  </a>
</div>

<div align="center">
 <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/Deppo/build.yml" />
 <img src="https://img.shields.io/github/repo-size/SpikeHD/Deppo" />
</div>
<div align="center">
 <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/Deppo" />
 <img src="https://img.shields.io/github/release-date/SpikeHD/Deppo" />
 <img src="https://img.shields.io/github/stars/SpikeHD/Deppo" />
 <img src="https://img.shields.io/github/downloads/SpikeHD/Deppo/total" />
</div>

# Table of Contents

* [Table of Contents](#table-of-contents)
* [Download](#download)
* [Quickstart](#quickstart)
* [Make your own!](#make-your-own)
  * [Deppo Configuration](#deppo-configuration)
    * [Syntax](#syntax)
    * [Reference](#reference)
* [TODO](#todo)
* [Contributing](#contributing)

# Download

All downloads can be found on the [releases page](https://github.com/SpikeHD/Deppo/releases). If you're unsure, download the **Windows x86** version.

# Quickstart

TODO

# Make Your Own!

Creating a Deppo is easy! All you need is a single `.gif` file (or more, for different animations!) and a text editor. Here's the step-by-step:

1. Create a new folder for your Deppo.
2. Place your `.gif` file(s) in the folder. It might help to name them accordingly (`idle.gif`, `walk.gif`, `fall.gif`, etc.).
3. Create a new file in the folder called `deppo.json`.
4. Open `deppo.json` in a text editor and paste the following:

```json
{
  "name": "YOUR_DEPPOS_NAME",
  "fps": 30,
  "timescale": 1,
  "scale": 1,

  "behaviour_change_rarity": 40,
  "move_speed": 3,

  "can_move": true,
  "can_drag": true,
  "can_click": false,
  "can_fall": true,

  "animations": {
    "idle": ["idle.gif"],
    "fall": ["fall.gif"],
    "drag": ["drag.gif"],
    "walk": ["walk.gif"]
  }
}
```

5. Edit the configuration to fit your Deppo's style! See [the Deppo configuration guide](#deppo-configuration) for more information.

## Deppo Configuration

Below is a description of each field. Most are self-explanatory, but some may need require a little bit of insight.

### Syntax

Deppo configurations use typical JSON format. If you are unfamiliar, that's okay! Below is a quick guide to the syntax:

* Everything on the left side of a `:` is a "key". In the above example, `"name"`, `"fps"`, `"timescale"`, etc. are all keys.
* Everything on the right side of a `:` is a "value". In the above example, `"YOUR_DEPPOS_NAME"`, `30`, `1`, etc. are all values.
* Values can be text (like `"YOUR_DEPPOS_NAME"`), numbers (like `30`), or even lists (like `["idle.gif"]`).
* Lists are surrounded by square brackets `[]`, and each item in the list is separated by a comma `,`.
  * For example, to have two idle animations, you would write `["idle1.gif", "idle2.gif"]`, replacing the filenames with your file's names.
* The values that you see in the above example cannot change their types. For example, do **NOT** change `"fps": 30` to `"fps": "30"`. This will cause an error.
* Each key-value pair is separated by a comma `,`. These are also required, except for the last line in the configuration.

> [!NOTE]
> ***Having Trouble?*** Try pasting your configuration into a JSON validator, such as [JSONLint.com](https://jsonlint.com/), to see if there are any errors.


### Reference

* `name`: The name of your Deppo, of course!
* `fps`: The frames-per-second of your Deppo's animations. The animation will run faster if set higher.
* `timescale`: The speed of the Deppo's animations.
  * A value of `1` is normal speed, `2` is twice as fast, `0.5` is half as fast, etc.
  * This is useful when you have a low-FPS animation, but you want the Deppo window to run at a higher framerate!
* `scale`: The size of your Deppo.
  * A value of `1` is normal size, `2` is twice as big, `0.5` is half as big, etc.
* `behaviour_change_rarity`: The rarity of the Deppo changing its behaviour, such as changing from "Idle" to "Moving".
  * 20 is frequent, 40 is somewhat frequent, 100 is rare, etc.
* `move_speed`: The speed at which the Deppo moves. This is only used if `can_move` is set to `true`.
  * 2 is slow, 4 is normal, 8 is fast, etc.
* `can_move`: Whether the Deppo can move around the screen.
* `can_drag`: Whether the Deppo can be dragged around the screen.
* `can_click`: Whether the Deppo can be clicked on.
* `can_fall`: Whether the Deppo can fall if not sitting on the bottom of the screen.
* `animations`: A list of animations that the Deppo can play. Each animation is a list of `.gif` files, and the animation will be randomly chosen from the list.
  * `idle`: The Deppo's default animation(s).
  * `fall`: The Deppo's falling animation(s).
  * `drag`: The Deppo's being-dragged animation(s).
  * `walk`: The Deppo's walking animation(s).

# TODO

* [ ] Transition animations
* [ ] Single-file loading (basically just renamed `.zip` files)
* [ ] UI for choosing Deppos
* [ ] Configuration options
* [ ] Click behaviour
* [ ] ARM Linux support
* [ ] Example Deppos
* [ ] Always-on-top
* [ ] More documentation

# Contributing

Issues, PRs, etc. are all welcome!