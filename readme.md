# Neutrino-demo [WIP]

A (work in progress) game engine written in Rust WebAssembly and WebGL. 

Javascript handles WebGL rendering, keypress event handlers and calling the WASM code every frame, whereas all 3D math, matrix calculations, physics simulation, entity logic and the event loop are written in Rust compiled to WebAssembly.

#### Rendering
- Javascript + A mini WebGL layer handle the rendering. It has Support for multiple shaders, buffers, uniform blocks and entities. 
- JS and Rust interface directly and expose functions to each other through FFI (Foreign Function Interface) with no translation or bindgen layer in between them.
- On `init()` Rust sends over all shader code, vertices, attributes, entity details etc. as binary which are parsed and initialized in webgl.
- On `render()` called each frame, Rust sends over a large UInt32Array containing matrices and uniforms for each entity in a custom encoding, which is parsed and sent to webgl.

#### Math
- The engine includes implementations for `Vec3` and `Matrix` which handle all 3D Math.
- `Matrix` is also used for implementing a camera, calculating perspective matrix, world matrices, view-projection matrices etc. for each entity and transforms for position/rotation/scale.

#### Physics
 - Implements `RigidBody` which supports the following concepts - mass, force, velocity, acceleration, torque, angular velocity, angular acceleration, moment of inertia.
 - Uses velocity verlet integration to run the simulation.
 - Implements collision detection with 3 collider variants - Circle, Axis Aligned Bounding Box (aabb), Polygon.

#### Engine
- The core of the engine is a event loop that is called each frame by JS. It receives time and input data each frame, executes all entity logic and collision detection, runs the physics simulation forward by 1 step, updates the camera and all matrices and then send all matrices and entity uniforms back to JS.
- Includes a stack based generic arena allocator that is used to store arrays (the intention is for the engine to be `nostd` when finished).
- Implements `Entity` and a few basic entities - 
    - `Object3d` - Static objects in the world
    - `Ship` - The main object controlled by the player. Parses user input and triggers actions. Has rigidbody physics.
    - `Thrusters` - Can apply physics based thrust force and torque to it's parent taking into account its own position, angle and power.
    - `Gun` - Has an array of bullets and handles their spawning and cleanup. It is also a physics object and can apply impulse forces to it's parent.
    - `Bullet` - Small objects that have physics and can collide with asteroids.
    - `Asteroid` - Dynamic objects with collision colliders to interact with the ship and its bullets.
- Keys
    - `WASD` - Movement
    - `QE` - Strafe
    - `Space` - Shoot

#### Compile Command
```
cargo build --target=wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/neutrino_demo.wasm ./js/
```
