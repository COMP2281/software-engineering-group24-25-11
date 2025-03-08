#set page(numbering: "1 of 1", margin: 2cm)
#set par(leading: 0.55em, spacing: 1.2em, justify: true)
#set text(font: "New Computer Modern", lang: "en", region: "gb",size: 11pt)
#show raw: set text(font: "New Computer Modern Mono", size: 11pt)
#show heading: set block(above: 1.4em, below: 1em)

#set heading(numbering: "1.", outlined: true)
#show heading.where(level: 3): set heading(numbering: none, outlined: false)

#align(center, grid(rows: (1fr, 1fr), 
align(center + horizon, [
  #box(width: 70%,
  align(center, grid(columns: (1fr, 0.01fr, 1fr), image("Durham.svg", width: 50%), align(center + horizon, text(size: 1.5em, [$times.big$])), image("IBM.svg", width: 50%))))
  
  #text(size: 2em, weight: 900, [Technical Report])
  
  #text(size: 1.5em, weight: 450, [IBM SkillsBuild VR Game])
  
  #text(size: 1.25em, weight: 450, [Group 11])
  
  #box(width: 70%, 
    grid(columns: (1fr, 1fr), align: center, column-gutter: 2em, row-gutter: 1em,
      [*Alexandre Pinheiro Dias*\ cgfv65],
      [*Muhammad Rafay Abbas*\ djsh68],
      [*Jing Lei Wong*\ zlnm44],
      [*Siang Wei Law*\ rwbc54],
      grid.cell(colspan: 2,[*Rohab Kashif*\ dwfh45])
    )
  )

]))),

#outline(indent: auto, depth: 3)

#pagebreak()

= Introduction // subtotal: 20%
This technical report documents development, implementation, and evaluation of a web-based virtual reality (VR) game designed to educate users about IBM SkillsBuild courses. Developed by Group 11 the project integrates educational content into an interactive, space-themed gaming experience. Below, we present the project’s motivation, goals, system access instructions, and the status of its behavioural requirements.

 // 5%
== Provide usable access to the developed system, along with clear instructions on how to set it up and run it // 5%
The IBM SkillsBuild VR Game project develops an immersive VR platform to teach high-demand technical skills such as AI, cybersecurity, and data analytics—by engaging users in repairing a spaceship through quiz-based challenges derived from IBM SkillsBuild content. This gamified approach, coine by our client Mr. John Mc Namara from IBM, enhances accessibility and motivation, aligning with IBM’s mission to expand technical education. The concise prototype, inspired by Paradroid and limited to 20 minutes, integrates educational content, adaptive difficulty via AI, and a space-themed narrative to attract diverse learners while ensuring sustained engagement.

The VR game is accessible via a web browser, utilising the Godot game engine’s WebXR support to eliminate the need for local installation. To run the system, users require a compatible browser (e.g., Firefox or Chromium, latest versions) and a stable internet connection. For an immersive VR experience, a WebXR-compatible VR headset (such as Oculus Quest 2/3) is recommended, though the game also supports traditional 3D gameplay for users without VR hardware.

To set up and run the game:
1. Navigate to the project’s deployment URL (URL ENTER).
2. Ensure the browser is updated and supports WebAssembly (WASM) and WebXR standards.
3. For VR mode:
   - Connect a VR headset to the system or use a standalone device with a built-in browser.
   - Select “Play in VR” from the main menu and accept the browser’s prompt to enter immersive mode.
4. For 3D mode:
   - Select “Play in 3D” from the main menu to proceed with keyboard/mouse or controller input.
5. Follow on-screen prompts to select a course and begin the quiz-based gameplay.

The system is designed to run in a secure HTTP(s) environment, ensuring compatibility and accessibility across platforms (Windows 11, Arch Linux, and Quest 2/3 browsers). Detailed installation and deployment instructions are provided in Section 3 of this report.
== Present the status of each behavioural requirement in a table. Provide the code (e.g., BR1.2 is “behavioural requirement for feature 1, scenario 2”) and a succinct description (no need to restate the entire user story). Indicate whether the requirement remains unchanged or has been modified (if modified, provide the updated description), and explain the extent to which it is met or not met, with justification if applicable. // 10%
The table below outlines the status of the behavioural requirements as defined in the Requirements Document. Each requirement’s implementation status is assessed as of February 25, 2025, reflecting the prototype’s current development stage.

#table(
  columns: (auto, auto, auto, auto, auto),
  align: (left, left, left, left, left),
  table.header(
    [*Code*], [*Description*], [*Status*], [*Met/Not Met*], [*Justification*]
  ),
  [BR1.1], [Look around with headset], [Unchanged], [Fully Met], [Implemented using WebXR API in Godot; headset movement accurately rotates the viewport.],
  [BR1.2], [Look around with left thumbstick], [Unchanged], [Fully Met], [Controller input mapped to camera rotation, tested across VR setups.],
  [BR1.3], [Move with right thumbstick], [Unchanged], [Fully Met], [Smooth movement implemented via Godot’s input system, functional in VR.],
  [BR1.4], [Teleport via thumbstick click], [Unchanged], [Not Met], [Teleportation logic code still in the works.],
  [BR2.1], [Move with WASD keys], [Unchanged], [Fully Met], [3D mode supports standard keyboard controls, verified across platforms.],
  [BR2.2], [Look around with mouse], [Unchanged], [Fully Met], [Mouse input adjusts camera direction smoothly in 3D mode.],
  [BR2.3], [Teleport with right-click], [Unchanged], [Fully Met], [Right-click triggers teleportation in 3D mode, consistent with VR.],
  [BR3.1], [VR prompt with headset], [Unchanged], [Fully Met], [WebXR prompt displays correctly on VR-enabled browsers.],
  [BR3.2], [3D prompt without headset], [Unchanged], [Fully Met], [Non-VR users receive a fallback option to play in 3D mode.],
  [BR3.3], [WASM support check], [Unchanged], [Partially Met], [Detects unsupported browsers but lacks a full list of compatible alternatives; under refinement.],
  [BR4.1], [Auto-save progress], [Unchanged], [Fully Met], [Progress saved locally after each question via Godot’s file system.],
  [BR4.2], [Pause on browser focus loss], [Unchanged], [Fully Met], [Game pauses and saves when tab loses focus, tested across browsers.],
  [BR4.3], [Pause with menu button], [Unchanged], [Fully Met], [Manual pause implemented with controller/keyboard input.],
  [BR4.4], [Resume from pause], [Unchanged], [Fully Met], [Game resumes seamlessly from pause state.],
  [BR4.5], [Resume from save], [Unchanged], [Fully Met], [Saved progress loads correctly upon reopening the game.],
  [BR5.1], [Session completes in 15 min], [Modified: Adjusted to 10-20 min], [Fully Met], [Sessions timed to fit within 20 minutes, aligning with client feedback.],
  [BR5.2], [Notify session length], [Unchanged], [Fully Met], [Start screen displays estimated duration.],
  [BR5.3], [Continue after session], [Unchanged], [Fully Met], [Option to start a new session provided post-completion.],
  [BR6.1], [Select a course], [Unchanged], [Fully Met], [Course selection menu loads SkillsBuild content dynamically.],
  [BR6.2], [Return to course menu], [Unchanged], [Fully Met], [Accessible from pause or session end.],
  [BR6.3], [Show course details], [Unchanged], [Partially Met], [Basic descriptions implemented; detailed content pending.],
  [BR6.4], [Recommend related courses], [Unchanged], [Not Met], [Feature planned but not implemented due to time constraints.],
  [BR7.1], [Review incorrect answers], [Unchanged], [Fully Met], [Post-session review shows incorrect answers with explanations.],
  [BR7.2], [Compare performance trends], [Unchanged], [Partially Met], [Basic trends displayed; graphical representation incomplete.],
  [BR7.3], [Review time per question], [Unchanged], [Not Met], [Time tracking implementation and display is being tested.],
  [BR8.1], [Points for correct answers], [Unchanged], [Fully Met], [Scoring system awards points accurately.],
  [BR8.2], [No points for incorrect], [Unchanged], [Fully Met], [Incorrect answers do not increment score.],
  [BR8.3], [Bonus for fast completion], [Unchanged], [Fully Met], [Bonus points calculated based on session time.],
  [BR9.1], [Display high score], [Unchanged], [Fully Met], [Local high score list updates with completion time.],
  [BR9.2], [View high score list], [Unchanged], [Fully Met], [Accessible from main menu.],
  [BR9.3], [Handle tied scores], [Unchanged], [Partially Met], [Currently implementing, faster time ranks higher.],
  [BR10.1], [Easier questions after poor performance], [Unchanged], [Fully Met], [AI adjusts difficulty downward after three wrong answers.],
  [BR10.2], [Harder questions after good performance], [Unchanged], [Partially Met], [Difficulty increases after three correct answers. Untested],
  [BR10.3], [Maintain difficulty for average], [Unchanged], [Fully Met], [Stable difficulty for mixed performance.],
  [BR11.1], [Visual accessibility options], [Unchanged], [Not Met], [Colourblind mode and text resizing not implemented yet.],
  [BR11.2], [Auditory accessibility options], [Unchanged], [Fully Met], [ Volume controls functional.],
  [BR11.3], [Motor accessibility options], [Unchanged], [Partially Met], [Basic control remapping available; sensitivity adjustments pending.],
  [BR12.1], [Set up local multiplayer], [Unchanged], [Not Met], [Optional feature not implemented due to focus on core single-player mode.]
)

Most requirements have been fully met within the prototype, leveraging Godot’s capabilities and WebXR integration. Partial implementations (e.g., BR3.3, BR6.3, BR7.2, BR9.3, BR11.3) reflect ongoing rfforts, while unmet features (e.g., BR1.4, BR6.4, BR7.3, BR11.1, BR12.1) were postponed due to prioritisation of core functionality.
#pagebreak()

= Technical Development // subtotal: 40%
== Clearly describe the source materials that form the basis for the conceptualisation and development of the system. //  5%

In the initial project specification provided by IBM, we were given the base ideas needed to make the game. Which is a space-themed VR Quiz Game that promotes IBM SkillsBuild to the players, which also has an adaptive difficulty based on the users' gameplay.

Using the specification, as well as the initial contact (emails) and meetings (virtual), our team, alongside our practical instructor, constructed a short description of our minimum viable product (MVP) to be delivered to the client during the handover:

"A space themed vr quiz game that works with adaptive difficulty targeting IBM users using IBM SkillsBuild Platform
"

This formed the basis of our development, as well as giving us a goal to work towards.

No initial source code or database was given to us, so we had to create everything from scratch.

Client asked us to use the courses within IBM SkillsBuild to source our questions to display on the game. We initially tried asking for a JSON or a database of the questions and answers used within the SkillsBuild website, which was denied due to security reasons, as well as the client wanting the team do source the questions by doing the SkillsBuild courses ourselves, so this added an additional hurdle in our game design.
== Provide a clear and appropriately detailed technical description of how each system functionality was developed. Include, where relevant, some or all of the following aspects: // 30%
- A high-level overview of the system  architecture.
- The design principles and patterns used.
- The technologies and platforms used (e.g., programming languages, databases, frameworks).

For the game engine, we chose to use GODOT due to the open source nature, extensive documentation and it being a relatively lightweight development environment.

Natively, GODOT used its own language, GDScript for everything, both frontend and backend. However, we chose to use Rust for most of the game, due to the team wanting the game to be on webxr for accessibility reasons, Rust gave us the performance boost we needed to run the game in a web environment without losing too much framerate.

GDScript was still used for some of the smaller places, such as menus and displays, but there was enough frameworks within Godot that 
- Relevant diagrams (e.g., UML diagrams, class diagrams, sequence diagrams) to illustrate system design.
- The software development process followed by the team.
- The role of the behaviour-driven development (BDD) approach in the implementation phase.
== Clearly describe how the system's usability and user experience aspects were addressed, providing an appropriate level of detail. // 5%
=== Concern 1: User might not have VR headset, limiting access to playing the game

Our team has also developed a 3D mode alongside the VR mode of the game to allow even people without a VR headset to be able to try out the game and experience it, although the game was still made for VR in mind, having a 3D mode expands the potential user base, as well as enhance the user experience of those who might experience motion sickness in a VR environment.

=== Concern 2: System requirements might be too high for some users

It is an issue within the VR game development scene that these games have relatively high system requirements compared to their 3D counterparts, due to the increased amount of rendering needed in a VR environment. Due to this, we limited ourselves in developing the game for a webxr (browser) environment. 
= Use Instructions // subtotal: 20%
== Installation: Describe system requirements, including the minimum and recommended hardware requirements (e.g., CPU, RAM, and storage) and operating systems. Provide step-by-step instructions for installing and configuring the software. // 5% 
=== System Requirements:
=== 1. Hardware Requirements


#block(width: 100%, align(center)[
  #table(
    columns: (auto, auto),
    align: center, // Center-align the content of the table
    table.header(
      [*Component*], [*Minimum Requirement*]
    ),
    [VR Headset], [Meta Quest (2, 3, Pro) (Optional for 3D Play)],
    [CPU], [Intel Core i5-7500 / AMD Ryzen 5 1600 or better],
    [GPU], [NVIDIA GTX 1060 / AMD RX 580 or better],
    [RAM], [8GB or more],
    [Storage], [5GB (SSD Preferred)]
  )
])

Settings aim for 60+ frames per second to reduce induced motion sickness.

=== 2. Software Requirements

#block(width: 100%, align(center)[
  #table(
    columns: (auto, auto),
    align: center, // Center-align the content of the table
    table.header(
      [*Category*], [*Requirements*]
    ),
    [*Operating System*], [Windows 10/11, Linux],
    [*Browser *], [Must support WebXR e.g: Chrome
    ])  
])

=== 3. Installation Instructions:

  1. *Open a Supported Browser*:
     - Use a browser that supports *WebXR* for the best experience.
     - Recommended browsers:
       - *Google Chrome*
       - *Mozilla Firefox*
       - *Brave*
  
  2. *Go to the Game URL*:
     - Open a new tab in your browser.
     - Type the following URL in the address bar and press *Enter*:
       ```
       https://xyz.com
       ```
  
  3. *Wait for the Game to Load*:
     - The game will start loading automatically.
     - Ensure you have a stable internet connection for smooth loading.
  
  4. *Allow Permissions (if prompted)*:
     - If the browser asks for permissions (e.g., access to your microphone, camera, or VR headset), click *Allow* to proceed.
  
  5. *Launch the Game*:
     - Once the game loads, choose to either play it in *3D* or *VR*
     - Alternatively, you may choose to select *Options* to adjust some settings to your liking, or view *credits*.


== Deployment: Explain how to deploy the system on a local machine and how to set up the database. If relevant, include instructions for setting up virtual machines, containers or cloud functions. // 5%

=== 3.2.1 Automatic Installation of Environment:
  === Windows:

  1. *Install Git*:
     - Git is required for cloning the repository and managing version control.
     - Download and install Git from the official website: 
       ```
       https://git-scm.com/downloads
       ```
       
     - During installation, ensure you select the option to add Git to your system PATH.

  2. *Install Just (Command Runner)*:
     - Just simplifies the process of running commands and managing dependencies.
     - Install Just using Winget (Windows Package Manager):
       ```
       winget install -e --id Casey.Just
       ```
       
     - If you choose not to use Just, you will need to manually install dependencies, set up environment variables, and configure the project.
  3. *Open a Terminal*:
     - Navigate to the root project directory (the folder containing the project structure).
     - You can open a terminal by:
       - Opening File Explorer, navigating to the project folder, clicking the address bar, and typing `wt` (Windows Terminal) or `cmd` (Command Prompt).

  4. *Install Dependencies*:
     - Run the following command to install all required dependencies:
     
       ```
       just setup
       ```
       
     - If the setup is successful, you will see a confirmation message.

  === Linux & Mac:

    1. *Ensure Your System Packages are Installed and Up To Date*:
     - Ensure the following system packages are up to date with the latest versions to avoid compatibility issues (they should be available in your package manager):
     
       ```
      - Git
      - just
       ```
       
     - If the aforementioned packages are not installed, they can be installed using the following commands in the terminal:

        ```
       -sudo apt install git
       -sudo apt install just
        ```
     - If your package manager does not have the latest version of ` just `available, download it from the GitHub release and and place at` ~/.local/bin/just` (Linux) or `/usr/local/bin/just` (Mac).

  2. *Open a Terminal*:
     - Navigate to the root project directory using the terminal.

  3. *Install Dependencies*:
     - Run the following command to install all required dependencies:
       ```
       just setup
       ```
       
     - If the setup is successful, you will see a confirmation message.
     
    
=== 3.2.2 Manual Installation of Environment:

In the case of a user wishing to manually install the environment, you'll have to make sure these pre-requisite dependencies are installed:

- *emsdk + emscripten v3.1.74* (required for web) 
- *blender* (required)  
- *rust nightly toolchain + rust-src component* (required):  
  - *wasm32-unknown-emscripten* (web)  
  - *x86_64-pc-windows-msvc* (if on windows)  
  - *x86_64-unknown-linux-gnu* (if on linux)  
- *just* (optional - highly recommended - makes it easy to setup, develop, and release)  

You should ensure that these dependencies are in your PATH.  

Then you can look at the justfile to find commands you may want to run, or `just --list`

=== 3.2.3 Getting Started with the program post environment installation:
To launch godot with the correct environment run: 

```
just godot 
```
To launch an editor of your choice (e.g. VSCode) with the correct environment run: 
```
just env "code""
```
(note: you need to pass the command as a string)

To compile rust extension (in debug mode) do just or alternatively (more verbose) just rust-debug. To compile rust extension (in release mode) do just rust-release.

If you wish to automatically compile the rust extension when you make changes, see automatic rebuilding







== Launching: Provide instructions on creating a user account or logging into the system. Explain different user roles (if applicable). Guide users through any first-time setup steps, such as creating admin accounts or configuring user settings. // 5%
== Troubleshooting: List common error messages and solutions for resolving them. Explain where users can find logs or diagnostic information to troubleshoot issues. // 5%
= Maintenance and Implications // subtotal: 20%
== Provide useful and usable information how the system can be maintained // 5%
- *Regular Content Updates*
  - Update and create new question banks for more topics (e.g. theoretical computer science, cloud computing).
  - Use built-in question bank editing tool for adjusting current questions to be more topical.
  
- *Software/Hardware Compatibility*
  - Test updates on Quest, different operating systems and web browsers before pushing to production.
  - Expand VR headset compatibility to other headsets such as Valve Index, HTC Vive etc.
  
- *Performance Optimisation*
  - Monitor VR metrics: Frame rate, Latency to ensure new updates and features do not reduce user performance.
  - Clean redundant data (deprecated topics) from database.
  
- *User Feedback Integration*
  - In-game bug reporting.
  - Question flagging system for inappropriate or wrong questions.
  
== Provide useful and usable information how the system’s possible future development can be implemented // 5%

- *AI-Driven Question Generation*
  - Adopt generative AI in the generation of questions, use academic papers and textbooks (with permission) as training material.
  
- *Collaborative Features*
  - Multiplayer mode for collaborative or competitive learning environment.
  - Allow importation of question banks created by other users
  
- *Cross-Platform Expansion*
  - Allow web version to be accessed through mobile to enhance accessibility among those without normal VR headsets
  - Decrease system requirements through introducing graphics settings to widen user base.
  
- *Advanced Analytics*
  - Create a individualised user dashboard with focus areas on subjects that need improvement, as well as strong areas for users to reflect and learn from.

== Describe potential ethical and societal impacts of the system in its current and possible future status // 10%

- *Current System:*

    - Accessibility Divide:
        - VR hardware costs may exclude low-income users, exacerbating inequities in tech education.
        
    - Data Privacy Risks:
        - User performance data (e.g., quiz scores, learning patterns) must be anonymised and encrypted to prevent misuse.

    - Educational Integrity:
        - Ensure question banks are fact-checked to avoid spreading misinformation (e.g., outdated coding practices).

- *Future Scenarios:*

    - Bias in AI-Personalised Learning:
        - If adaptive algorithms prioritise certain topics/users, they may reinforce skill gaps (e.g., favouring advanced learners).

    - Ethical Content Moderation:
        - User-generated questions (e.g., on AI ethics) must be moderated to filter harmful or biased content.

    - Societal Benefits:
        - Democratising CS education through gamification could inspire underrepresented groups to pursue tech careers.
        - Over-reliance on VR learning might reduce hands-on traditional learning, need to reinforce to users that traditional learning is still needed to succeed.
