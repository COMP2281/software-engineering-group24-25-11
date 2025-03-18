# Project structure
├── godot: The base godot game project.\
├── rust: All rust code for the GDExtension for the Godot project, provides custom classes.\
├── docs: All required documentation, such as requirements etc...\
├── web_assets: Extra assets for web builds such as headers and favicons...\
├── .deps: A local collection of dependencies needed for building/developing.\
└── target (optional): Default location of exports, created when needed.\

# Lifecycle of the game
We have created a diagram of the lifecycle of the game (accurate as of 18/3/25). This shows the full lifecycle of the game.

![Game flow diagram](./docs/assets/game_flow_diagram.svg)

# Main components
This describes the main components of the project, most of which are found in the rust library.
## Scene manager
### Base Architecture

This is at the core of the application, it handles the primary state of the game such as input method, course selection, initial difficulty, and xr interfaces.

This class handles the major transitions, such as between the title screen and the world, and vice versa, and dealing with setting up VR/3D. It is also a gateway to get root nodes such as the sound effects class, providing safe fallible ways to get them from a generic base pointer of any node in the scene tree.

This architecture reduces duplication of code since all handling/fetching of the major nodes is done through the scene manager, which is easily obtainable, and also helps to ensure node paths are all correct since they are done from a central location.

The scene manager also handles resuming/pausing the game, by providing functions that apply the relevant actions, which can be easily called from elsewhere.


## Overarching scene classes
### Base Architecture

There are several main states of the program, which are the: title screen, world scene, and completion screen.

These all require some state, such as which screen is shown, or shared data required throughout all child scenes. To make these interactions easier, each overarching scene implements its own custom class, which provide functions to do state transitions within themselves, such as transitioning to another submenu (e.g. course selection), or dealing with spawning a new question room after a question has been answered.

Where appropriate, these scenes implement custom constructors requiring the necessary data (such as completion information), ensuring that they are always initialised correctly.

There is a global helper button class for returning to a different subscene, used throughout the game, it allows assigning a return node from within the editor, which it then returns to (and hides the other relevant nodes within the screen).

Transitions between these main states are delegated to the scene manager.

### Title screen

This provides some helper utilities that help swap between different sub screens of the title screen, such as the main menu, course selection, message box, leaderboard, options and credits.

### World

The world scene has a custom constructor which requires a selected course, it then uses this to construct a question bank, and stores it within its state. It then setups the world as needed, by creating the start room and an initial question room, and then provides a callback function when a question is answered to add a new question room, or spawn the end room.

### Completion screen

This has a custom constructor that requires a list of the answered questions, this is then used to populate the respective menus with data. This also provides transition functions between the different subscreens (question list, question details, and save score).

## Question room & panel
### Base Architecture

The question room class handles spawning in a room, with a question panel at its core. It provides some helper functions to open/close the front and back door, preventing users from jumping off the spaceship.

It uses a custom constructor which takes in an instance of the `Question` struct, along with the current time, which is passed to the question panel.

### Question Panel

The question panel is a core component of the system, it is a custom class, which creates a 3D scene, dynamically adding buttons and text based on the provided `Question`. These buttons are all set to a collision layer of 8, allowing them to be collided against by the relevant `RayCast3D` from the player.

The creation of the question panel is divided into reusable functions, such as create button, create label, create body, etc. This allows us to reuse the common functionality e.g. for the choice buttons, and the submit button.

It also provides some functions for working out if an object provided by a `RayCast3D` is one of the buttons on this question panel, which it then uses to highlight the hovered selection, select (if clicked), or submit, these are split into further functions making it easier to follow.

On submission, the button on the question panel are disabled, and it calls `question_answered` on the world scene with the answered question, which handles further state transitions (such as open front door, and spawn next question panel).

## VR Environment and Movement
### Base Architecture

When the game is loaded, utilising Godot's `XrServer` singletons, we probe for VR support, which responds with a signal, allowing us to listen and enable playing in VR if it is supported.

When a user tries to play in VR, we fetch an `XrInterface`, which we use to configure the HMD, we then listen to signals provided by the `XrServer` to tell if initialising VR has succeeded (with the configuration) or failed, and also triggers signals telling us when the user has entered and exited VR. These signals are used to know when to enter the world, pause, or show a corresponding message.

All of these signals are listened to within the scene manager, which handles the relevant transitions (i.e. into the world).

Our VR character has its own subscene, which is injected into the world when the world is created and played in VR.

The root node of the subscene is a custom class which inherits from Godot's CharacterBody3D, allowing us to take advantage of Godot's physics engine to deal with collisions (i.e. with the floor). This class then listens for serveral signals, such as `physics_process`, which runs everytime physics are calculated. Within this signal we process any provided user input (such as joystick movement) and move the character correspondingly, and also trigger a raycast from the user's primary controller, which we use to detect when buttons with the game world are hovered, and likewise selected.

Within the subscene we have several primitive Godot nodes, used for VR, which are a `XrOrigin3D`, `XrCamera3D`, and `XrController3D`. The origin provides the centre of the character in VR, whilst the camera provides the viewport for the HMD. The controller nodes show the position of the controllers within the game, and are used to poll for input events on them.

### Movement System

Input is handled through VR controllers controllers via `XrController3D` nodes, with the right thumbstick used for directional movement and left thumbstick for horizontal rotation. The "A" button on the right controller is used for jumping, and the right trigger was used for interactions within the scene such as selecting an answer.

This input handling differs from the normal Godot paradigm so input events are fetched directly from the controller nodes, instead of the global `Input` singleton.

Movement direction is derived from main player basis, which we multiply by a the direction of the thumbstick, and then normalise. This then is converted into a vector with only horizontal values, where we then apply a speed modifier.
We then check if the jump button is pressed and if the user is currently on a floor, then if so, we apply a vertical impulse to the velocity. Otherwise we take the current vertical velocity and reduce it in accordance with a defined gravity constant (9.8 m/s#super[2]) and the time elapsed.

This is then applied using Godot's `move_and_slide` primitive which then tries to move the character sliding after a collision (such as with a wall).

### Rotation System

Similar to the movement system, we fetch the left thumbstick values for the VR controller. Using this we apply scaling to reduce the sensitivity, and then apply the rotation to the character body, only to the Y plane (since users can look up using their heads). The system directly modifies character body rotation whilst maintaining HMD-independent view through the `XrCamera3D` node.

### Interaction System

For interacting with objects, the right controller has a `RayCast3D` mounted to its tip, which interacts only with collision layer 8 (where all buttons are located in), this allows us to poll for collisions on the `RayCast3D`, and relate the collision with any of the active buttons in the world. When a button is found, a corresponding function is applied on the relevant target class, such as the question panel.

## 3D Environment and Movement

A traditional first-person movement system implemented through Rust/Godot integration with these components:

### Base Architecture

Our 3D character also has its own subscene, which is injected into the world when the world is created and played in 3D.

The root node of the subscene is a custom class which inherits from Godot's CharacterBody3D, allowing us to take advantage of Godot's physics engine to deal with collisions (i.e. with the floor). This class then listens for serveral signals, such as `physics_process`, which runs everytime physics are calculated. Within this signal we process any provided user input (such as keyboard input and  joystick movement) and move the character correspondingly.

Unlike in VR, a raycast is triggered whenever there is any user input, this provides more responsive feedback. We utilise the raycast in the same manner to detect when buttons with the game world are hovered, and likewise selected.

Within the subscene we have use a primitive Godot node for the camera (`Camera3D`), and provide simple meshes for the player body, along with a reticle in the centre of the screen to help users know what they are selecting.

Unlike the VR scene, this is able to use Godot's input system which allows us to easily remap buttons to actions, and likewise isn't dependent on nodes (such as XrController3D) to get input from.

### Movement System

Input is handled by using WASD keyboard input utilising the Godot `Input` singleton to get directional movement based of our action map, spacebar for jumping action ("jump" input action), and left clicking the mouse for interactions such as selecting a question choice. Also within web build, this class handles recapturing the mouse on clicking back into the game.

Movement direction is derived from main player basis, which we multiply by a the direction of the input (from keyboard WASD, or controller joystick), and then normalise. This then is converted into a vector with only horizontal values, where we then apply a speed modifier.
We then check if the jump button is pressed and if the user is currently on a floor, then if so, we apply a vertical impulse to the velocity. Otherwise we take the current vertical velocity and reduce it in accordance with a defined gravity constant (9.8 m/s#super[2]) and the time elapsed.

This is then applied using Godot's `move_and_slide` primitive which then tries to move the character sliding after a collision (such as with a wall).

Since these affect the actual player body, they are listened to within the `physics_process` signal.

### Rotation System
Since we can listen to input events properly within 3D, and rotation does not effect the player position, we can handle camera rotation from the `on_unhandled_input` signal. For this, we get the current relative rotation of the player base, and apply only the horizontal movement to it (after applying a sensitivity multiplier). Which also rotates its children (including the head/camera), this is also what the movement direction is based on.

For vertical rotation, we get the player `Head` node, and get the current rotation, applying the vertical relative movement (with a sensitivity multiplier). This is then clamped to a maximum of $plus.minus pi/2 (plus.minus 90 degree )$ to prevent users from rotating their heads too far (leading to an upside down viewport).
This is then applied to the `Head` node.


### Interaction System

For interacting with objects, the right controller has a `RayCast3D` mounted to the head of the player body, which interacts only with collision layer 8 (where all buttons are located in), this allows us to poll for collisions on the `RayCast3D`, and relate the collision with any of the active buttons in the world. When a button is found, a corresponding function is applied on the relevant target class, such as the question panel.


## Adaptive Quiz Difficulty
### Base Architecture:

Adaptive difficulty is implemented through our question bank struct, this struct loads a provided course's question data, and deserialises it into `Question` structs. The question bank struct also handles keeping track of the current score, answered questions, user difficulty, and overall time taken.

The question bank then provides public functions for getting a question and submitting an answer, along with two internal functions which are used to modify the user difficulty, and score upon receiving an answer submission.

The question bank expects a course, initial difficulty mode, question limit (i.e. how many questions to provide before ending), and the initial time it was created at. We currently provide 3 initial preset difficulties (Easy - 0.3, Normal - 0.5, Hard - 0.7), which  _currently_ defaults to the Normal mode.

### Loading course data

Course data is loaded using Godot's `FileAccess` singleton, where we read the entire course data into a buffer, and deserialise it utilising the `serde_json` library.

### Dynamic Difficulty System

The question is selected by first taking the user's current difficulty level, applying a ±15% random jitter on it, then finding a question with difficulty closest to that specific level, ensuring some random elements while ensuring appropriate difficulty for user.

When question is answered correctly/wrongly, the user's current difficulty is adjusted based on time taken and question difficulty, this is capped at a limit of $plus.minus 0.2$ ensuring that the difficulty does not change dramatically.

The answered question is also stored (and could be sent out), allowing the client to adjust the difficulty of the questions based on how often users fail it, ensuring that the question's difficulty is not arbitrarily set and instead is based on users answering the questions.

### Performance Tracking

For scoring, the correctness of answer, the time taken for user to answer, as well as the difficulty of the questions is taken into account, providing either 0 points, for an incorrect answer, or up to a limit of 250 points for a correct question (with a minimum of 10). To score highly, users must answer difficult questions fast. This information is shown within the HUD.

## Leaderboard

### Base Architecture

The leaderboard scene has its own class, which updates it's contents when loaded with data from a leaderboard JSON file within the user data path (i.e. cookies and local storage on web). If this file doesn't exist it provided an empty leaderboard with a message stating the leaderboard is empty.

The leaderboard entries are stored as an array of JSON objects, and similar to the rest of the code, are serialised and deserialised utilising `serde_json`.
Each entry of the leaderboard has the following information: name (associated with the entry), score, and time taken (in milliseconds) to complete the entire session.

[{"name": "Fastest AI", "score": 1040, "time_taken": 2023440}]

The entries are sorted in a descending order, ensuring that the top scorers are displayed first, with a random tie breaker for even scores. Since scores also take into account difficulty of questions, and time taken it provides a useful metric for which sessions were the best.

### Submitting an entry

Within the completion screen after finishing a course, an button is shown allowing users to save their score, this leads to a subscene with a text box to enter a display name for the leaderboard, and a button to save it, after clicking the button, the leaderboard file is read (or created if it doesn't exist), the entry is appended to the end of it, and then subsequently written back into the leaderboard file. After submission the save button is disabled to prevent duplicate submissions in the leaderboard

## Question Bank Tooling

A desktop GUI application for managing quiz question bank, implemented with Python/Tkinter, converting the provided questions and answers into a correct JSON file usable by the quiz system.

### Base Architecture

Tool uses Tkinter framework for cross-platform GUI development using JSON-based storage system, it allows for loading an existing question bank, and creating a new one. It provides an interface to add questions, and select the right answers, and set the question difficulty. After which it can be exported into a JSON file.

### JSON Operations

The tool outputs the question bank data into a JSON file with the following format, which is deserializable by the game code: an array of question objects, where each question object has a: question, list of choices, indices of the correct choices, and the difficulty of the question.

[{"question": "some question", choices: ["choice one", "choice 2"], answers: [0], difficulty: 0.1}, {...}]

### User Interface Components

Main window contains a Listbox for question preview (question text only), action buttons with vertical flow layout and a label for the output file path.

When editing, there is a toplevel window for detailed editing, which uses a grid-layout form with 7 entry fields (QuestionType, Question, Choice1, Choice2, Choice3, Choice4, Answer, Difficulty) as well as a unified save handler for create/update
