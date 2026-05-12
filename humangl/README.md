# HumanGL

HumanGL is a small Rust and OpenGL project that demonstrates hierarchical modeling with a simple animated human figure. The program opens a window, draws a humanoid body from cube meshes, and uses a custom matrix stack so that body parts move together in a logical way.

## Description

A Rust-based OpenGL hierarchical modeling demonstration featuring skeletal animation of a humanoid figure. Implements custom matrix transformations and a matrix stack for proper joint articulation. The animated human supports walking, jumping, and idle states controlled via keyboard input. All body parts are rendered from a single cube mesh with independent transformations, showcasing efficient hierarchical rendering techniques.

# Project Overview

This project is an introduction to skeletal animation and hierarchical modeling. The main idea is to build a character from separate body parts and control those parts with matrices instead of OpenGL's old built-in matrix functions.

The program solves a very specific graphics problem: how to make a model move as one connected structure while still allowing each limb to rotate and scale independently. The torso, head, arms, and legs are all drawn from the same cube mesh, but each one gets its own transformation.

The program receives input from the keyboard while the window is open. It does not read command-line arguments. The main input is the current time plus key presses such as `1`, `2`, `3`, `R`, and `Escape`.

The output is a rendered 3D humanoid in an OpenGL window. The console also prints initialization information such as the shader compilation message and the OpenGL version/renderer strings.

Before reading the code, it helps to understand a few beginner-level ideas:

- A 3D model can be built from simple shapes.
- Matrices are used to move, rotate, and scale those shapes.
- A matrix stack lets child body parts inherit the transform of their parent.
- OpenGL draws geometry through shaders, vertex buffers, and draw calls.

# PDF / Subject Requirements

The PDF explains the assignment requirements for a hierarchical modeling project. The important points are:

- The project is an introduction to hierarchical modeling and matrix stack manipulation.
- You must implement your own matrix stack and your own matrix transformations.
- The goal is to create a skeletal animation.
- OpenGL 4.0 or higher must be used.
- A build system is required. The PDF allows a Makefile or a similar build system.
- Any language and any graphics library may be used.

Mandatory requirements from the PDF:

- Body parts must be correctly articulated using a matrix stack.
- If the torso rotates, the limbs must follow that rotation.
- If the upper arm moves, only the forearm should follow it.
- If the size of a limb changes, related parts must automatically reposition themselves.
- The model must include:
  - a head
  - a torso
  - two arms, each with an upper arm and a forearm
  - two legs, each with a thigh and a lower part
- The model must be able to walk, jump, and stay still.
- Each body part must be drawn by one and only one function call.
- That function must draw a 1x1x1 geometric shape at the origin of the current matrix.

Bonus requirements mentioned in the PDF:

- Add more body parts.
- Add other movement patterns such as dance or kung-fu style motion.
- Add a graphical interface that can modify part sizes and colors.
- Bonus points are only evaluated if the mandatory part is perfect.

Rules and constraints from the PDF:

- The project must use custom matrices and transformations, not the deprecated OpenGL matrix helpers.
- Upper and lower parts of the same limb are considered different parts.
- The hierarchy must be visible in the way the limbs are drawn and linked.
- The drawing function and matrix stack should be easy to explain during evaluation.

Expected behavior from the PDF:

- The body should move as a connected hierarchy.
- Limb motion should follow parent motion naturally.
- The project should demonstrate walking, jumping, and an idle state.

Input rules:

- The assignment itself does not define a file input format.
- In this implementation, the only runtime input is keyboard input in the OpenGL window.

Output rules:

- The visible output is a 3D animated human model rendered in a window.
- The model should be built from separate parts and transformed hierarchically.

Error-handling rules:

- The PDF does not define a formal error protocol.
- This implementation stops with a panic if initialization or shader compilation fails.

Edge cases mentioned in the PDF:

- Changing limb size must keep related parts positioned correctly.
- The upper and lower parts of the same limb must remain logically separate.

# Features Implemented

| Feature | Description | Related Files | Main Functions / Classes |
|---|---|---|---|
| OpenGL window setup | Creates the application window and OpenGL context | `src/main.rs`, `Cargo.toml` | `main()` |
| Shader compilation | Compiles and links a basic vertex and fragment shader | `src/shader.rs`, `src/main.rs` | `compile_program()`, `compile_shader()` |
| Unit cube mesh | Builds one cube mesh that is reused for every body part | `src/cube.rs` | `UnitCube::new()`, `UnitCube::draw()`, `create_unit_cube_vao_vbo()`, `unit_cube_positions()` |
| Matrix math | Provides custom vector and matrix operations | `src/math.rs` | `Vec3`, `Mat4` and their methods |
| Matrix stack | Stores parent transforms and applies child transforms on top | `src/stack.rs` | `MatrixStack::new()`, `push()`, `pop()`, `apply()` |
| Human animation state | Tracks idle, walk, and jump motion states | `src/human.rs` | `Motion`, `Pose`, `HumanAnimation::new()`, `start_jump()`, `update()` |
| Hierarchical human drawing | Draws torso, head, arms, and legs with inherited transforms | `src/human.rs` | `draw_human()` |
| Keyboard controls | Lets the user switch animation states at runtime | `src/main.rs` | Event handling inside `main()` |
| Debug spin toggle | Adds a simple rotation toggle for the whole figure | `src/main.rs`, `src/human.rs` | `debug_spin` handling in `main()`, `draw_human()` |

# Project Structure

```text
humangl/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src/
│   ├── cube.rs
│   ├── human.rs
│   ├── main.rs
│   ├── math.rs
│   ├── shader.rs
│   └── stack.rs
└── target/
```

Important note:

- `target/` is generated by Cargo during builds.
- `target/` is ignored by `.gitignore` and is not part of the hand-written source.

| File / Folder | Purpose | Important Content | Used By |
|---|---|---|---|
| `Cargo.toml` | Cargo manifest and build configuration | Package name, edition, dependencies | Cargo, all source files indirectly |
| `Cargo.lock` | Locked dependency versions | Exact versions resolved by Cargo | Cargo |
| `.gitignore` | Git ignore rules | Ignores `target/` and `CONCEPTS_TO_RECAP.md` | Git |
| `README.md` | Project documentation | This guide | Readers |
| `src/main.rs` | Program entry point | Window setup, event loop, render loop | Starts the full application |
| `src/shader.rs` | Shader utilities | Shader/program compilation and log handling | `main.rs`, OpenGL setup |
| `src/cube.rs` | Shared cube geometry | Vertex buffer, vertex array, cube draw helper | `human.rs` |
| `src/stack.rs` | Matrix stack implementation | Push/pop/apply transform logic | `human.rs` |
| `src/math.rs` | Math utilities | `Vec3`, `Mat4`, transform helpers | `main.rs`, `stack.rs`, `human.rs`, `cube.rs` |
| `src/human.rs` | Animation and scene assembly | Motion state, pose generation, human drawing | `main.rs` |
| `target/` | Generated build artifacts | Cargo build output | Cargo |

# How the Project Works

When the program starts, `main()` does the following:

1. Initializes GLFW.
2. Requests an OpenGL 4.1 core profile context.
3. Creates a 1024x768 window titled `HumanGL`.
4. Makes the window's OpenGL context current.
5. Loads OpenGL function pointers.
6. Compiles and links the shaders.
7. Creates the reusable unit cube geometry.
8. Enables depth testing and sets the viewport.
9. Creates the animation state.
10. Enters the render loop.

During each frame, the program:

1. Polls keyboard and window events.
2. Updates the current motion state when a key is pressed.
3. Computes the frame delta time.
4. Clears the color and depth buffers.
5. Recomputes the viewport and camera matrices.
6. Updates the pose based on the current motion state.
7. Draws the human model using hierarchical transforms.
8. Swaps buffers to display the frame.

Input handling is simple:

- `Escape` closes the window.
- `1` switches to idle.
- `2` switches to walk.
- `3` starts a jump.
- `R` toggles a debug rotation for the whole model.

Error handling is mostly immediate failure:

- If GLFW initialization fails, the program panics.
- If window creation fails, the program panics.
- If shader compilation or linking fails, the program panics with the log.
- If a required uniform location cannot be found, the program panics.

# High-Level Program Flow Diagram

Normal success flow:

```text
main()
  |
  v
glfw::init()
  |
  v
create window + load OpenGL
  |
  v
compile_program()
  |
  v
UnitCube::new()
  |
  v
event loop
  |
  +--> keyboard events
  |
  +--> HumanAnimation::update(dt)
  |
  v
draw_human()
  |
  v
UnitCube::draw() repeated for each body part
  |
  v
swap_buffers()
```

Error flow:

```text
main()
  |
  +--> GLFW init fails -> panic
  |
  +--> window creation fails -> panic
  |
  +--> shader compile/link fails -> panic with log
  |
  +--> required uniform missing -> panic
```

Explanation:

- `main()` controls the whole application.
- `HumanAnimation::update()` turns time into a pose.
- `draw_human()` turns the pose into model transforms.
- `UnitCube::draw()` performs the actual OpenGL draw call for each part.

# File Dependency Diagram

```text
main.rs
  ├── uses math.rs
  ├── uses shader.rs
  ├── uses cube.rs
  └── uses human.rs

human.rs
  ├── uses math.rs
  ├── uses stack.rs
  └── uses cube.rs

cube.rs
  └── uses math.rs

stack.rs
  └── uses math.rs

shader.rs
  └── uses std::ffi::CString and OpenGL calls

math.rs
  └── stands alone as the shared math layer
```

What to read first:

- `src/main.rs` is the entry point and top-level controller.
- `src/human.rs` contains the animation logic and the full body hierarchy.
- `src/stack.rs` explains the matrix stack behavior.
- `src/math.rs` shows how transformations are represented.
- `src/cube.rs` shows how one cube mesh is reused for every body part.
- `src/shader.rs` shows how the OpenGL shaders are compiled and linked.

# Function and Class Documentation

## `main()`

Location: `src/main.rs`

Purpose:
Sets up the OpenGL application, handles keyboard input, updates animation, and draws the frame loop.

Parameters:

- None.

Returns:

- Nothing. The program runs until the window is closed.

Main Logic:

1. Initialize GLFW.
2. Request OpenGL 4.1 core profile settings.
3. Create a window.
4. Make the context current and load OpenGL symbols.
5. Compile the shaders.
6. Build the cube mesh and animation state.
7. In each frame, poll input, update time, compute camera matrices, update the pose, and draw the human.

Called By:

- The Rust runtime.

Calls:

- `glfw::init()`
- `glfw::create_window()`
- `gl::load_with()`
- `shader::compile_program()`
- `cube::UnitCube::new()`
- `human::HumanAnimation::new()`
- `human::HumanAnimation::update()`
- `human::draw_human()`

Error Handling:

- Panics if GLFW cannot initialize.
- Panics if the window cannot be created.
- Panics if shader compilation or linking fails.

Example:

- Run the program, press `2` to switch to walking, then press `3` to start a jump.

## `cstr_to_string(ptr: *const u8) -> String`

Location: `src/main.rs`

Purpose:
Converts a null-terminated OpenGL C string pointer into a Rust `String` for console output.

Parameters:

- `ptr`: Pointer returned by OpenGL, expected to point to a null-terminated string.

Returns:

- A Rust `String` containing the OpenGL text.

Main Logic:

1. Cast the pointer to `*const i8`.
2. Read it as a `CStr`.
3. Convert the bytes to a lossless string.

Called By:

- `main()` when printing OpenGL version and renderer strings.

Calls:

- `std::ffi::CStr::from_ptr()`

Error Handling:

- Assumes the pointer is valid and null-terminated.
- This should be verified if OpenGL returns unexpected data.

## `compile_program(vertex_src, fragment_src)`

Location: `src/shader.rs`

Purpose:
Compiles a vertex shader and a fragment shader, links them into one OpenGL program, and returns the program ID.

Parameters:

- `vertex_src`: Vertex shader source code.
- `fragment_src`: Fragment shader source code.

Returns:

- A valid OpenGL program object ID.

Main Logic:

1. Compile the vertex shader.
2. Compile the fragment shader.
3. Create a program.
4. Attach both shaders.
5. Link the program.
6. Check the link status.
7. If linking fails, fetch the log and panic.
8. Detach and delete the shader objects after linking.

Called By:

- `main()`.

Calls:

- `compile_shader()`
- `get_program_info_log()`

Error Handling:

- Panics if program linking fails.
- Deletes temporary shader objects before returning or panicking.

Example:

- Used once at startup to compile the built-in shaders.

## `compile_shader(kind, src)`

Location: `src/shader.rs`

Purpose:
Compiles a single GLSL shader of the given type.

Parameters:

- `kind`: Shader type such as vertex or fragment.
- `src`: GLSL source string.

Returns:

- An OpenGL shader object ID.

Main Logic:

1. Convert the source string to a C string.
2. Create the shader object.
3. Upload the source to OpenGL.
4. Compile the shader.
5. Check compile status.
6. If compilation fails, fetch the log and panic.

Called By:

- `compile_program()`.

Calls:

- `get_shader_info_log()`

Error Handling:

- Panics if the source contains a null byte.
- Panics if shader compilation fails.

Example:

- The vertex shader defines `gl_Position` using the MVP matrix.

## `get_shader_info_log(shader)`

Location: `src/shader.rs`

Purpose:
Reads the compiler log for a shader object.

Parameters:

- `shader`: OpenGL shader object ID.

Returns:

- A string containing the shader log, or a small fallback message if no log exists.

Main Logic:

1. Ask OpenGL for the log length.
2. Allocate a buffer.
3. Read the log text.
4. Convert it to a Rust string.

Called By:

- `compile_shader()`.

Calls:

- `gl::GetShaderiv()`
- `gl::GetShaderInfoLog()`

Error Handling:

- Falls back to `"(no shader log)"` when the log is empty.

## `get_program_info_log(program)`

Location: `src/shader.rs`

Purpose:
Reads the linker log for an OpenGL program object.

Parameters:

- `program`: OpenGL program object ID.

Returns:

- A string containing the program log, or a fallback message if no log exists.

Main Logic:

1. Ask OpenGL for the program log length.
2. Allocate a buffer.
3. Read the log text.
4. Convert it to a Rust string.

Called By:

- `compile_program()`.

Calls:

- `gl::GetProgramiv()`
- `gl::GetProgramInfoLog()`

Error Handling:

- Falls back to `"(no programme log)"` when the log is empty.

## `UnitCube`

Location: `src/cube.rs`

Purpose:
Stores the OpenGL objects needed to draw one reusable cube mesh.

Fields:

- `vao`: Vertex array object used to describe the vertex layout.
- `vbo`: Vertex buffer object containing cube vertices.
- `program`: Shader program used for drawing.
- `u_mvp`: Uniform location for the model-view-projection matrix.
- `u_colour`: Uniform location for the RGB color.

Why it exists:

- Every body part in the human model uses the same cube geometry.
- The model is built by transforming that cube differently for each part.

## `UnitCube::new(program)`

Location: `src/cube.rs`

Purpose:
Creates the cube geometry and fetches shader uniform locations.

Parameters:

- `program`: The linked OpenGL shader program.

Returns:

- A fully initialized `UnitCube`.

Main Logic:

1. Create the VAO and VBO.
2. Query the `u_mvp` uniform location.
3. Query the `u_colour` uniform location.
4. Panic if either uniform is missing.

Called By:

- `main()`.

Calls:

- `create_unit_cube_vao_vbo()`
- `gl::GetUniformLocation()`

Error Handling:

- Panics if a required uniform is not found.

Example:

- Called once during startup, before the render loop begins.

## `UnitCube::draw(&self, mvp, colour)`

Location: `src/cube.rs`

Purpose:
Draws the cube with a specific transform and color.

Parameters:

- `mvp`: The full transform matrix for the current body part.
- `colour`: The RGB color used for the part.

Returns:

- Nothing. The side effect is an OpenGL draw call.

Main Logic:

1. Bind the shader program.
2. Upload the MVP matrix.
3. Upload the color.
4. Bind the cube VAO.
5. Draw 36 vertices as triangles.
6. Unbind the VAO.

Called By:

- `draw_human()`.

Calls:

- `gl::UseProgram()`
- `gl::UniformMatrix4fv()`
- `gl::Uniform3f()`
- `gl::BindVertexArray()`
- `gl::DrawArrays()`

Error Handling:

- Assumes the OpenGL state is valid.
- This should be verified if the rendering context changes.

Example:

- Used for the torso, head, upper arms, forearms, thighs, and lower legs.

## `create_unit_cube_vao_vbo()`

Location: `src/cube.rs`

Purpose:
Creates and fills the OpenGL vertex array and vertex buffer for the cube.

Parameters:

- None.

Returns:

- A tuple containing `(vao, vbo)`.

Main Logic:

1. Generate the VAO and VBO.
2. Bind them.
3. Upload the cube vertex positions.
4. Describe the vertex layout as 3 floats per vertex.
5. Enable the attribute.
6. Unbind the objects.

Called By:

- `UnitCube::new()`.

Calls:

- `unit_cube_positions()`

Error Handling:

- Relies on OpenGL being available.

## `unit_cube_positions()`

Location: `src/cube.rs`

Purpose:
Builds the 36 cube triangles as a flat list of vertex positions.

Parameters:

- None.

Returns:

- A fixed array of 108 floats, which represents 36 vertices with 3 coordinates each.

Main Logic:

1. Define the half-size of the cube as `0.5`.
2. Build each face with two triangles.
3. Convert the dynamic vector into a fixed array.

Called By:

- `create_unit_cube_vao_vbo()`.

Calls:

- A small local helper named `quad()`.

Error Handling:

- Uses `try_into().unwrap()` and assumes the vertex count is correct.

Example:

- The result is one centered unit cube that can be scaled into any limb size.

## `MatrixStack`

Location: `src/stack.rs`

Purpose:
Stores a stack of matrices so child parts can inherit their parent transform.

Why it exists:

- Hierarchical modeling needs a safe way to return to a parent transform after drawing a child part.

## `MatrixStack::new()`

Location: `src/stack.rs`

Purpose:
Creates a matrix stack containing one identity matrix.

Returns:

- A new `MatrixStack`.

Called By:

- `draw_human()`.

## `MatrixStack::top()`

Location: `src/stack.rs`

Purpose:
Returns the current matrix on the top of the stack.

Returns:

- The active `Mat4`.

Called By:

- `push()` and `apply()`.

## `MatrixStack::push()`

Location: `src/stack.rs`

Purpose:
Duplicates the current transform so a child branch can modify it safely.

Main Logic:

1. Read the current top matrix.
2. Push a copy onto the stack.

Called By:

- `draw_human()` before drawing each body branch.

## `MatrixStack::pop()`

Location: `src/stack.rs`

Purpose:
Removes the most recent matrix and returns to the parent transform.

Main Logic:

1. Remove the top matrix.
2. If the stack ever becomes empty, restore the identity matrix.

Error Handling:

- Panics if the stack underflows before the identity fallback is restored.

Called By:

- `draw_human()`.

## `MatrixStack::apply(m)`

Location: `src/stack.rs`

Purpose:
Multiplies the current top matrix by another transform.

Parameters:

- `m`: The transform to apply.

Main Logic:

1. Read the current top matrix.
2. Multiply it by `m`.
3. Store the result back on top of the stack.

Called By:

- `draw_human()`.

## `Vec3`

Location: `src/math.rs`

Purpose:
Represents a 3D vector and provides helper operations used by camera math.

Fields:

- `x`, `y`, `z`: The vector components.

## `Vec3::new(x, y, z)`

Location: `src/math.rs`

Purpose:
Creates a new 3D vector.

## `Vec3::sub(a, b)`

Location: `src/math.rs`

Purpose:
Subtracts one vector from another.

Used For:

- Building the forward direction in `Mat4::look_at()`.

## `Vec3::dot(a, b)`

Location: `src/math.rs`

Purpose:
Computes the dot product of two vectors.

Used For:

- Camera math in `Mat4::look_at()`.

## `Vec3::cross(a, b)`

Location: `src/math.rs`

Purpose:
Computes the cross product of two vectors.

Used For:

- Building a camera basis in `Mat4::look_at()`.

## `Vec3::length()`

Location: `src/math.rs`

Purpose:
Returns the vector length.

## `Vec3::normalise()`

Location: `src/math.rs`

Purpose:
Returns a unit-length version of the vector.

Error Handling:

- If the length is too small, it returns a safe fallback vector.

## `Mat4`

Location: `src/math.rs`

Purpose:
Represents a 4x4 matrix in column-major form and provides transformation helpers.

Why it exists:

- OpenGL transforms and camera setup are built from 4x4 matrices.

## `Mat4::identity()`

Location: `src/math.rs`

Purpose:
Creates the identity matrix.

## `Mat4::as_ptr()`

Location: `src/math.rs`

Purpose:
Returns a raw pointer to the matrix data for OpenGL uniform upload.

## `Mat4::at(row, col)`

Location: `src/math.rs`

Purpose:
Reads a matrix element using the project's column-major layout.

## `Mat4::set(row, col, v)`

Location: `src/math.rs`

Purpose:
Writes a matrix element using the project's column-major layout.

## `Mat4::mul(a, b)`

Location: `src/math.rs`

Purpose:
Multiplies two 4x4 matrices.

Main Logic:

1. Loop over columns.
2. Loop over rows.
3. Multiply corresponding elements and sum them.
4. Store the result in the output matrix.

Used For:

- Combining parent and child transforms.
- Building final MVP matrices.

## `Mat4::translation(tx, ty, tz)`

Location: `src/math.rs`

Purpose:
Creates a translation matrix.

Used For:

- Moving the body root, head, shoulders, elbows, hips, knees, and camera-related transforms.

## `Mat4::rotation_y(radians)`

Location: `src/math.rs`

Purpose:
Creates a rotation matrix around the Y axis.

Used For:

- The debug spin and leg facing direction.

## `Mat4::scale(sx, sy, sz)`

Location: `src/math.rs`

Purpose:
Creates a scale matrix.

Used For:

- Turning the unit cube into body parts with different sizes.

## `Mat4::rotation_x(radians)`

Location: `src/math.rs`

Purpose:
Creates a rotation matrix around the X axis.

Used For:

- Arms, elbows, hips, and knees.

## `Mat4::rotation_z(radians)`

Location: `src/math.rs`

Purpose:
Creates a rotation matrix around the Z axis.

Status:

- It is currently not used anywhere in the code.

## `Mat4::perspective(fov_y_radians, aspect, near, far)`

Location: `src/math.rs`

Purpose:
Creates a perspective projection matrix.

Used For:

- Building the camera lens in `main()`.

## `Mat4::look_at(eye, centre, up)`

Location: `src/math.rs`

Purpose:
Creates a view matrix that points the camera toward a target.

Main Logic:

1. Compute the forward direction from eye to center.
2. Compute the side vector using a cross product.
3. Compute the corrected up vector.
4. Fill the camera matrix.

Used For:

- Positioning the camera in the render loop.

## `Motion`

Location: `src/human.rs`

Purpose:
Describes the current animation state of the human.

Variants:

- `Idle`
- `Walk`
- `Jump`

## `Pose`

Location: `src/human.rs`

Purpose:
Stores the per-frame joint angles and root height for the human model.

Fields:

- `root_y`: Vertical offset of the body root.
- `hip_l`, `hip_r`: Left and right hip angles.
- `knee_l`, `knee_r`: Left and right knee angles.
- `shoulder_l`, `shoulder_r`: Left and right shoulder angles.
- `elbow_l`, `elbow_r`: Left and right elbow angles.

## `HumanAnimation`

Location: `src/human.rs`

Purpose:
Tracks motion state over time and turns that state into a pose.

Fields:

- `motion`: Current motion mode.
- `walk_phase`: Phase used for sine-wave walking movement.
- `jump_t`: Normalized jump timer.

## `HumanAnimation::new()`

Location: `src/human.rs`

Purpose:
Creates an animation controller in the idle state.

Returns:

- A new `HumanAnimation`.

## `HumanAnimation::start_jump()`

Location: `src/human.rs`

Purpose:
Starts a jump animation from the beginning.

Main Logic:

1. Switch motion to `Jump`.
2. Reset the jump timer.

Called By:

- `main()` when the user presses `3`.

## `HumanAnimation::update(dt)`

Location: `src/human.rs`

Purpose:
Advances the animation timers and produces the current pose.

Parameters:

- `dt`: Time passed since the previous frame.

Returns:

- A `Pose` containing the current joint angles and body height.

Main Logic:

1. Update the timing state depending on the active motion.
2. Compute a default pose.
3. Adjust the pose for idle, walk, or jump.
4. Clamp angles to reasonable ranges.
5. Return the pose.

Motion behavior:

- Idle: keeps the body mostly still.
- Walk: swings hips, knees, shoulders, and elbows in a cycle.
- Jump: raises the body and bends the knees near the top of the jump.

Called By:

- `main()` once per frame.

Calls:

- Trigonometric functions such as `sin()` and `cos()`.
- `clamp()` on several pose values.

Error Handling:

- There is no explicit error return.
- The function assumes `dt` is a sensible non-negative frame time.

Example:

- If the motion is `Walk`, the left and right limbs swing in opposite directions.

## `draw_human(cube, proj, view, debug_spin, t_now, pose)`

Location: `src/human.rs`

Purpose:
Draws the full humanoid character using hierarchical transforms and the shared cube mesh.

Parameters:

- `cube`: The reusable unit cube renderer.
- `proj`: The projection matrix.
- `view`: The camera/view matrix.
- `debug_spin`: If true, rotates the whole model slowly around Y.
- `t_now`: Current time, used for the debug spin.
- `pose`: The current animation pose.

Returns:

- Nothing. The side effect is rendering all visible body parts.

Main Logic:

1. Create a matrix stack.
2. Apply the root translation.
3. Optionally apply debug spin.
4. Draw the torso.
5. Draw the head as a child of the torso.
6. Draw the right arm, then the left arm.
7. Draw the right leg, then the left leg.
8. For each part, scale the unit cube and send an MVP matrix to the shader.

How the hierarchy works:

- The torso is the parent of the head, arms, and legs.
- The upper arm is the parent of the forearm.
- The thigh is the parent of the lower leg.
- Each branch uses `push()` before local transforms and `pop()` after drawing.

Called By:

- `main()`.

Calls:

- `MatrixStack::new()`
- `MatrixStack::push()`
- `MatrixStack::pop()`
- `MatrixStack::apply()`
- `Mat4::translation()`
- `Mat4::rotation_x()`
- `Mat4::rotation_y()`
- `Mat4::scale()`
- `Mat4::mul()`
- `UnitCube::draw()`

Error Handling:

- Assumes the matrix stack operations are balanced.
- A stack underflow would indicate a logic bug.

Example:

- If the torso is rotated, the head and limbs inherit that rotation because they are drawn on top of the torso transform.

# Detailed Logic Explanation

The main idea of the project is hierarchical modeling.

This means the model is not drawn as one fixed object. Instead, it is built from parts that depend on each other. A head is attached to a torso. An arm is attached to the torso. A forearm is attached to an upper arm. The same is true for the legs.

The code does this in a few clear stages:

1. Create one cube mesh.
2. Create a transform for the whole body.
3. Push the current transform before drawing a child part.
4. Apply local transforms for that part.
5. Draw the cube.
6. Pop back to the parent transform.

The same cube is reused everywhere. The only thing that changes is the matrix and color.

Why this logic is needed:

- It avoids building separate geometry for every limb.
- It makes child parts follow parent parts naturally.
- It satisfies the subject requirement that the body should be articulated with a matrix stack.

What data it uses:

- The current time from GLFW.
- The current motion state.
- The current pose angles.
- The camera matrices.
- The cube vertex data.

How the data is transformed:

- Raw motion state becomes a `Pose`.
- The `Pose` becomes a series of matrices.
- The matrices become final MVP transforms.
- The MVP transforms are uploaded to the shader.
- The shader draws the cube with the correct shape, size, and position.

What decisions it makes:

- Which motion state is active.
- How far the body should move vertically during a jump.
- How much each joint should bend during a walk cycle.
- Where each limb should be attached relative to the torso.

What happens when the input is valid:

- The pose updates smoothly.
- The body parts move together correctly.
- The window continues rendering frames.

What happens when the input is invalid:

- The project does not have a formal runtime input parser.
- Invalid OpenGL initialization or shader errors cause a panic.

# Data Flow

The data flow is simple and linear:

```text
Keyboard input + frame time
  -> HumanAnimation state
  -> Pose
  -> MatrixStack transforms
  -> MVP matrices
  -> UnitCube::draw()
  -> OpenGL framebuffer output
```

Where the data starts:

- Keyboard events start the control flow.
- Time from GLFW drives animation progression.

How the data is stored:

- `HumanAnimation` stores the current motion and animation timers.
- `Pose` stores the current angles.
- `MatrixStack` stores the active transform chain.
- `UnitCube` stores the OpenGL buffers and shader locations.

Which functions modify the data:

- `main()` changes the motion state when keys are pressed.
- `HumanAnimation::update()` changes the animation timers and returns a new pose.
- `draw_human()` builds the transform hierarchy.

Which functions only read the data:

- `UnitCube::draw()` reads the matrix and color values.
- `Mat4::mul()` reads two matrices and returns a new one.

How the final result is produced:

- The program draws the torso first.
- Then it draws the head, arms, and legs using the torso as the parent transform.
- Each part is just a scaled and rotated unit cube.

# Error Handling and Edge Cases

| Error / Edge Case | Checked In | Behavior |
|---|---|---|
| GLFW initialization fails | `src/main.rs` in `main()` | Panics with `Failed to initialise GLFW` |
| Window creation fails | `src/main.rs` in `main()` | Panics with `Failed to create GLFW window` |
| Shader source contains a null byte | `src/shader.rs` in `compile_shader()` | Panics with `Shader source contains a NULL byte` |
| Shader compilation fails | `src/shader.rs` in `compile_shader()` | Panics and prints the shader log |
| Program linking fails | `src/shader.rs` in `compile_program()` | Panics and prints the program log |
| Required uniform not found | `src/cube.rs` in `UnitCube::new()` | Panics for `u_mvp` or `u_colour` |
| Shader log is empty | `src/shader.rs` | Returns a fallback message instead of failing |
| Program log is empty | `src/shader.rs` | Returns a fallback message instead of failing |
| Vector normalization length is nearly zero | `src/math.rs` in `Vec3::normalise()` | Returns a safe fallback vector |
| Matrix stack becomes empty | `src/stack.rs` in `MatrixStack::pop()` | Restores the identity matrix after underflow handling |
| Framebuffer height is zero | `src/main.rs` in `main()` | Uses `.max(1.0)` to avoid division by zero in aspect ratio |
| Unused `rotation_z()` helper | `src/math.rs` | Not harmful, but currently unused |

Notes:

- The project uses panics for startup failures instead of recoverable error values.
- That is acceptable for a small graphics demo, but it should be verified if the project ever needs stronger runtime recovery.

# Build, Compile, and Run Instructions

This is a Cargo-based Rust project. There is no Makefile in the repository.

Required tools:

- Rust toolchain with Cargo
- A working OpenGL 4.1-capable graphics environment
- GLFW runtime support on the system

Dependencies:

- `glfw`
- `gl`

Cargo fetches the Rust crates automatically during the first build.

Build command:

```bash
cargo build
```

Run command:

```bash
cargo run
```

Check-only command:

```bash
cargo check
```

Clean command:

```bash
cargo clean
```

Rebuild from scratch:

```bash
cargo clean
cargo build
```

What the build does:

- Compiles the Rust source.
- Links the GLFW and OpenGL-related crates.
- Produces the executable in Cargo's `target/` directory.

What the run command does:

- Opens the `HumanGL` window.
- Starts the render loop.
- Shows the animated human model.

Important environment note:

- This project needs a graphical session. It will not run correctly in a headless terminal without OpenGL support.

# Example Usage

## Valid Example 1

Command:

```bash
cargo run
```

Input:

- No command-line input.
- The user can press keys inside the window.

Expected output:

- A window titled `HumanGL` opens.
- The console prints shader and OpenGL initialization information.
- A humanoid model appears in the window.

Explanation:

- The application initializes OpenGL, builds the cube mesh, and starts rendering the human model.

## Valid Example 2

Command:

```bash
cargo run
```

Input:

- Press `2` while the window is focused.

Expected output:

- The model switches to a walking motion.

Explanation:

- Key `2` sets the motion state to `Walk`, which causes limb swinging in `HumanAnimation::update()`.

## Valid Example 3

Command:

```bash
cargo run
```

Input:

- Press `3` while the window is focused.

Expected output:

- The model performs a jump animation.

Explanation:

- Key `3` calls `start_jump()`, which resets the jump timer and switches the motion state.

## Invalid / Error Example

Command:

```bash
cargo run
```

Input:

- Run the program on a system without a working OpenGL 4.1 context or without GLFW support.

Expected output:

- The program stops during startup with a panic.

Explanation:

- Initialization depends on GLFW and OpenGL. If either one fails, the program cannot continue.

# Testing

There are no dedicated automated test files in this repository.

Existing test files:

- None were found in the project source tree.

Useful manual checks:

| Test Case | Command / Input | Expected Result |
|---|---|---|
| Build check | `cargo check` | Project compiles successfully |
| Start the program | `cargo run` | Window opens and model renders |
| Idle motion | Press `1` | Model stays mostly still |
| Walk motion | Press `2` | Arms and legs swing in a cycle |
| Jump motion | Press `3` | Model performs a jump and returns to idle |
| Debug spin | Press `R` | Whole model slowly rotates around Y |
| Close window | Press `Escape` | Application exits |

Important edge cases to test manually:

- Verify that the torso rotation affects all attached limbs.
- Verify that the upper arm moves the forearm but not the torso.
- Verify that changing a scale still keeps child parts attached correctly.
- Verify that the window keeps its aspect ratio when resized.

# Reading Guide for New Developers

If you are new to this project, read the files in this order:

1. `src/main.rs`
   - Learn how the window is created.
   - Learn which keys control the animation.
   - Learn how the render loop is organized.

2. `src/human.rs`
   - Learn how motion states become poses.
   - Learn how the body is assembled from parts.
   - Focus on how `push()` and `pop()` define the hierarchy.

3. `src/stack.rs`
   - Learn how matrix stacking works.
   - Understand how child transforms inherit parent transforms.

4. `src/math.rs`
   - Learn how vectors and matrices are represented.
   - Focus on translation, rotation, scaling, perspective, and camera math.

5. `src/cube.rs`
   - Learn how one cube mesh is reused for every body part.
   - Focus on how the VAO, VBO, and uniforms are set up.

6. `src/shader.rs`
   - Learn how the GLSL shaders are compiled and linked.
   - Focus on the error logs, since they matter if rendering fails.

What to focus on while reading:

- How each body part gets its own local transform.
- How parent transforms are preserved with the matrix stack.
- How the same geometry is reused instead of duplicated.
- How animation state changes the pose values every frame.

# Important Design Decisions

- The project uses one cube mesh for the entire human model. This keeps the geometry simple and makes the hierarchy easier to understand.
- The project separates math, shader setup, mesh setup, stacking, and animation into different modules. This keeps the code easier to read.
- The project uses a matrix stack instead of hand-building every world transform from scratch. That matches the subject and makes parent-child relationships natural.
- The animation logic is separated from the drawing logic. `update()` computes the pose, and `draw_human()` uses the pose.
- The code uses direct panics for startup failures. That is simple for a small graphics demo, but it is not a full error recovery system.

# Common Mistakes / Things to Be Careful About

- Do not forget to call `push()` before applying local transforms for a child body part.
- Do not forget to call `pop()` after drawing a branch of the hierarchy.
- Do not change the order of transforms casually. Translation, rotation, and scaling order matters.
- Do not assume the matrix layout is row-major. This code uses a column-major style layout.
- Do not remove the OpenGL version hints unless you know the target system supports the change.
- Do not expect the project to run without a graphics display and working OpenGL context.
- Do not delete the shared cube mesh logic unless you replace it with another complete mesh path.

# Known Limitations

- There is no automated test suite in the repository.
- There is no Makefile; the project relies on Cargo instead.
- There is no user interface for changing colors or body sizes at runtime.
- The motion set is small: idle, walk, jump, and a debug spin toggle.
- OpenGL resource cleanup on exit is not explicit in the source.
- `rotation_z()` is present in the math module but is currently unused.

No major limitation was found from the current code, but this should be verified with more testing.

# Glossary

| Term | Meaning |
|---|---|
| Hierarchical modeling | Building a model from parts where child parts inherit the transform of parent parts |
| Matrix stack | A stack of transformation matrices used to save and restore parent transforms |
| Transform | A move, rotation, or scale applied to an object |
| Pose | The current set of joint angles and body offsets for one animation frame |
| Shader | Small GPU program that controls how vertices and pixels are processed |
| Vertex | One point in 3D space used to build a mesh |
| VAO | Vertex Array Object, which stores vertex attribute setup |
| VBO | Vertex Buffer Object, which stores vertex data |
| Uniform | A value sent from the CPU to a shader program |
| MVP | Model-View-Projection matrix, used to place geometry on screen |
| GLFW | Library used here to create the window and handle input |
| OpenGL core profile | A modern OpenGL context without old fixed-function matrix helpers |

# Final Summary

HumanGL is a Rust OpenGL demo about hierarchical modeling and skeletal animation. It draws a human figure from one reusable cube mesh and uses a custom matrix stack so the torso, head, arms, and legs move together correctly.

The most important files are `src/main.rs` for startup and input, `src/human.rs` for animation and body assembly, `src/stack.rs` for matrix hierarchy behavior, `src/math.rs` for transforms, `src/cube.rs` for reusable geometry, and `src/shader.rs` for shader setup.

After reading this README, a new developer should understand what the project is supposed to do, how the body hierarchy is built, how the animation state flows into rendering, and how to build and run the program.