![openbattlesim_logo_dark](https://github.com/skalimoi/OpenBattlesim/assets/53193415/b0c6ce41-7849-4fab-afd6-4d19f683bdf7)

-------------------------

<a href="https://godotengine.org/"><img src="https://img.shields.io/badge/Godot-4.1.1-2ea44f?logo=godotengine&logoColor=%23478CBF" alt="Godot - 4.1.1"></a>
![contributions - welcome](https://img.shields.io/badge/contributions-welcome-blue)
[![Made with Rust](https://img.shields.io/badge/Rust-1-blue?logo=rust&logoColor=white)](https://www.rust-lang.org/ "Go to Rust homepage")
![Currently working on - Compute shader for cloud movement!](https://img.shields.io/badge/Currently_working_on-Compute_shader_for_cloud_movement!-2ea44f)


A military simulator written in Rust, GDScript and made in Godot.

It aims to be a complete 3D environment simulation geared towards military games, including:
* Dynamic, procedural weather system which simulates everything from temperature to humidity, together with volumetric and interactive clouds that react to the environment parameters each day. (Work-in-progress)
* Erosion system which converts any heightmap to a realistic landscape and automatically creates lakes and rivers. (Done)
* Vegetation system which takes into account average yearly sunlight, soil type, and overall climate to populate the world with realistic trees, bushes, and other kinds of vegetation. (Work-in-progress)
* Accurate terrain texturing based on real-life satellite data and user-chosen premade climates. (Done)

The actual gameplay is also on the works, and will have its own description soon.

OpenBattlesim is a revamp of my previous project Battlesim. This is being implemented in Godot, with performance-critical systems written in Rust and ported from the previous project.

This is in a very early development stage and isn't meant to be playable yet!
