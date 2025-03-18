#set page(numbering: "1 of 1", margin: 1in)
#set par(leading: 0.55em, spacing: 1.2em, justify: true)
#set text(font: "New Computer Modern", lang: "en", region: "gb")
#show raw: set text(font: "New Computer Modern Mono", size: 11pt)
#show heading: set block(above: 1.4em, below: 1em)

#set heading(numbering: "1.", outlined: true)
#show heading.where(level: 3): set heading(numbering: none, outlined: false)

#grid(
  rows: (1fr, 1fr),
  align(
    center + horizon,
    [
      #box(
        width: 70%,
        align(
          center,
          grid(
            columns: (1fr, 0.01fr, 1fr),
            image("assets/Durham.svg", width: 50%),
            align(center + horizon, text(size: 1.5em, [$times.big$])),
            image("assets/IBM.svg", width: 50%),
          ),
        ),
      )

      #text(size: 2em, weight: 900, [Test Plan])

      #text(size: 1.5em, weight: 450, [IBM SkillsBuild VR Game])

      #text(size: 1.25em, weight: 450, [Group 11])

      #box(
        width: 70%,
        grid(
          columns: (
            1fr,
            1fr,
          ), align: center, column-gutter: 2em, row-gutter: 1em,
          [*Alexandre Pinheiro Dias*\ cgfv65],
          [*Muhammad Rafay Abbas*\ djsh68],
          [*Jing Lei Wong*\ zlnm44],
          [*Siang Wei Law*\ rwbc54],
          grid.cell(colspan: 2, [*Rohab Kashif*\ dwfh45])
        ),
      )

    ],
  ),
  align(start, outline(indent: auto, depth: 3))
)
#pagebreak()

= Introduction

This is the test plan report for our VR Quiz Game. It outlines the objectives, scope, and methodology of our testing to ensure that the game meets its quality and performance benchmarks. This document is structured into User Acceptance Tests, Unit Tests, and System Tests, each designed to validate a range of functionalities from smooth movement and interaction to the correct implementation of adaptive difficulty. Our testing process aims to identify and categorise any issues based on five classes of test severity:



We have five classes of test severity:
- `CRITICAL`: A catastrophic failure preventing the system from being used at all.
- `MAJOR`: A serious issue that significantly disrupts core functionality and user tasks.
- `MODERATE`: An issue that reduces user experience or performance but doesn’t make the system completely unusable.
- `MINOR`: A small error that does not affect major functionality; the system remains mostly usable.
- `TRIVIAL`: A cosmetic or low impact error with no meaningful effect on usability.

This report serves as a guide to evaluate the quality and reliability of the VR Quiz Game. It details the test procedures, expected outcomes, and the environments in which the tests will be conducted, ensuring that all aspects of the game from user interaction to system performance are thoroughly assessed.

= User Acceptance Tests
== Case 1: Smooth movement and interaction
#table(
  columns: (auto, auto),
  [*ID*], [UAT_1],
  [*Description*],
  [
    Tests that the player movement and interaction are smooth and don't feel out of place, with no major tracking or usability issues.
  ],

  [*Related requirements*],
  [
    - Feature 1: Immersive VR view port
    - Feature 2: Traditional 3D view port
  ],

  [*Pre-requisites*],
  [
    - VR enabled browser (through VR headset or WebXR Emulator)
    - Game compiled successfully
  ],

  [*Test procedure*],
  [
    - Open the game in VR
      - Look around with the headset in real life
      - Move controller around in real life
      - Try teleport by pressing the trigger
      - Move using the controller joystick
      - Select over menu buttons
    - Open the game in 3D
      - Move using a controller joystick
      - Look around using second controller joystick
      - Select over menu buttons
      - Move using a keyboard (WASD)
      - Look around using mouse
      - Select over menu buttons
  ],

  [*Test material used*],
  [
    - Computer/Laptop
    - VR Headset
  ],

  [*Expected result*],
  [
    - Camera rotation changes proportional to the headset movement.
    - Hand laser correctly move in line with the controller movement
    - Player is teleported to where the controller was pointing
    - Movement with the joystick is smooth, not jittery, and in the correct direction
    - Menu interactions show visual cues (highlighted) when hovered.

    - Movement with the joystick is smooth, not jittery, and in the correct direction.
    - Camera direction changes proportional to the joystick movement
    - Menu interactions show visual cues (highlighted) when hovered.

    - Movement with the keyboard is smooth, not jittery, and in the correct direction.
    - Camera direction changes proportional to the mouse movement
    - Menu interactions show visual cues (highlighted) when hovered.

  ],

  [*Comments*],
  [
    - Test in each of the given
    - Ensure browsers and drivers are up to date.
    - Repeat the test in all specified test environments (every browser and platform combination).
    - Ensure the webpage is running in a secure environment (i.e. HTTP(s) server through Godot)
  ],

  [*Created by*], [Alexandre Pinheiro Dias],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)
=== Equivalence classes
This test has multiple equivalence classes: movement, camera control, and menu interaction.

=== Failure severity
This test has multiple classes of failure severity depending on the scope of failure, if there is an issue with movement (unable to move correctly, clipping through boundaries) then the severity is *MAJOR* as it can stop or severely impact the user's ability to play the game. If the issue is with menu visual cues, then the impact is *MODERATE* as it can make it difficult for players to select their intended answers on the quiz. If the issue is with the smoothness/jitteriness of movement then the issue is *TRIVIAL* as it does not impede the player from playing the game and answering questions.


// Failure of this test is of severity *MAJOR*, as improper movement and interaction in either the VR or 3D viewport can severely impact the user's ability to play the quiz. If the test fails for movement smoothness or correctness it would be difficult to get to the next area, whilst if it failed for menu interaction issues it would be hard to know which button was being selected and clicked, possibly leading to wrong answers.

== Case 2: Selecting quiz answers
#table(
  columns: (auto, auto),
  [*ID*], [UAT_2],
  [*Description*],
  [
    Verify that users can select quiz answers within the game, observe correct or incorrect feedback, and proceed without issues.
  ],

  [*Related requirements*],
  [
    - Feature 6: Course Selection
    - Feature 8: Scoring System for Player Performance
    - Feature 10: Adaptive Difficulty Within Game
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - A loaded question bank with multiple possible answers per question
  ],

  [*Test procedure*],
  [
    1. Launch the game in a supported environment (VR or 3D).
    2. Select any course from the main menu or course selection screen.
    3. Begin the quiz sequence.
    4. For each question displayed:
      - Highlight or point at the desired answer using controller (VR) or mouse/keyboard (3D).
      - Confirm the selection by clicking or pressing the interaction button.
      - Observe immediate feedback (e.g., correct or incorrect) and any score changes.
    5. Continue until the quiz is finished or the user decides to exit.
    6. Verify no blocking issues (e.g., unresponsive UI, glitching) during answer selection.
  ],

  [*Test material used*],
  [
    - Computer/Laptop
    - VR Headset (for VR tests)
  ],

  [*Expected result*],
  [
    - Users can freely select any of the multiple-choice answers.
    - Correct or incorrect feedback is displayed promptly.
    - Game does not freeze or block if the user selects an answer too quickly or multiple times.
    - Score and difficulty adjustments (if any) reflect correct or incorrect answers.
    - User can seamlessly progress to the next question or exit the quiz as intended.
  ],

  [*Comments*],
  [
    - Ensure each answer choice is clearly selectable.
    - If adaptive difficulty is active, subsequent questions may become easier or harder based on correct or incorrect responses.
  ],

  [*Created by*], [Rohab Kashif],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)
=== Equivalence classes
This test has one equivalence class, which is course selection. The test should give the same result regardless of the course selected.

=== Failure severity
Failure of this test is of severity *MAJOR*, as the user cannot effectively proceed with the quiz if answer selection fails. This hinders the core functionality of learning and assessment, thereby causing a major disruption to gameplay.



== Case 3: Accessibility Settings Applied Correctly
#table(
  columns: (auto, auto),
  [*ID*], [UAT_3],
  [*Description*],
  [
    Validates that the game is able to apply accessibility settings when prompted by the user through the settings menu.
  ],

  [*Related requirements*],
  [
    - Feature 11: Accessibility options available in the game
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - VR and 3D viewport able to be accessed and used
  ],

  [*Test procedure*],
  [
    1. Launch the game in a supported environment (VR or 3D).
    2. Start the game and navigate to the settings page
    3. Adjust accessibility settings individually as well as in conjunction with one another and see if it works as expected
  ],

  [*Test material used*],
  [
    - Desktop/laptop and (optionally) VR headset
    - Keyboard/mouse or VR controllers
  ],

  [*Expected result*],
  [
    - Accessibility settings (large font, colourblind Mode, etc) can be applied successfully as well as in conjunction with each other if needed.
  ],

  [*Comments*],
  [
    - Would be useful to have user with need of these accessibility settings to see if they are effective.
  ],

  [*Created by*], [Wong Jing Lei],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)

=== Failure severity
A failure in accessibility settings would make the game less accessible to those with disabilities/ailments that inhibit gameplay, decreasing the amount of players that are able to play the game and learn from it. Thus, a failure is deemed *MODERATE* in severity.


== Case 4: Accessible through the web
#table(
  columns: (auto, auto),
  [*ID*], [UAT_4],
  [*Description*],
  [
    Verify that users can access and interact with the game through a web browser without issues.
  ],

  [*Related requirements*],
  [
    - Feature 3: Accessible through the web
    - Feature 6: Course Selection for Customised Learning Paths
    - Feature 8: Scoring System for Player Performance
  ],

  [*Pre-requisites*],
  [
    - A stable internet connection
    - Supported web browsers installed
  ],

  [*Test procedure*],
  [
    1. Open a supported web browser on a desktop.
    2. Navigate to the game's web URL.
    3. Verify that the main menu loads without significant delays.
    4. Select any course from the course selection screen.
    5. Start the quiz sequence and ensure that the questions load properly.
    6. Interact with the quiz by selecting answers and observing the feedback.
    7. Verify that the score updates and navigation between questions works as expected.
    8. Complete the quiz or exit and ensure that the progress is saved if applicable.
  ],

  [*Test material used*],
  [
    - Computer/Laptop
    - VR Headset (for VR tests)
    - Stable internet connection
  ],

  [*Expected result*],
  [
    - Game loads correctly in the browser.
    - Users can navigate the menu and select courses without issues.
    - Quiz questions load and functions as expected.
    - Answer selection and feedback display correctly.
    - No major performance issues, crashes or unresponsiveness during gameplay.
  ],

  [*Comments*],
  [
    - Test on both desktop and mobile devices to ensure responsive design.
    - Test on different screen resolutions and network conditions, if applicable.
  ],

  [*Created by*], [Siang Wei Law],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)

=== Failure severity
A failure in web accessibility would prevent users from being able to access and play the game through a browser if they do not own a VR headset. This will reduce its reach and usability, leading to limited learning opportunities for users who rely on web-based access. Thus, the severity of this failure is classified as *MODERATE*

== Case 5: Access to Previous Results
#table(
  columns: (auto, auto),
  [*ID*], [UAT_5],
  [*Description*],
  [
    Ensure that users can access previously recorded results such as high scores and previously recorded wrong and correct answers.
  ],

  [*Related requirements*],
  [
    - Feature 7: Ability to review previous quiz answers
    - Feature 9: Localised High Score with Time Tracking
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - Game fully played once to record initial scores and answers
  ],

  [*Test procedure*],
  [
    1. Launch the game in a supported environment (VR or 3D).
    2. Navigate to the Records section.
    3. Ensure high scores and past answers and their classification display correctly
    4. Complete a play-through to verify high score updates and answer recording.
  ],

  [*Test material used*],
  [
    - Computer/Laptop
    - VR Headset (for VR tests)
    - Stable internet connection
  ],

  [*Expected result*],
  [
    - High scores and past answers correctly displayed.
    - Past answers and their correctness are displayed accurately.
    - Quiz questions load and functions as expected.
    - Answer selection and feedback display correctly.
    - High scores and answers section updates on subsequent playthrough's.
  ],

  [*Comments*],
  [
    - Ensure that there's an option for the user to clear records in the case that they want a fresh start.
  ],

  [*Created by*], [Muhammad Rafay Abbas],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)

=== Failure Severity
Failure to access previously recorded results would hinder a user's ability to reflect and revise on their skills, stopping their progress and limiting their learning opportunities. Since it doesn't affect core gameplay the failure of this test is of severity *MODERATE*.


= Unit Tests
== Case 1: Game resume-ability
#table(
  columns: (auto, auto),
  [*ID*], [UT_1],
  [*Description*],
  [
    Checks whether game state is the same before and after resuming
  ],

  [*Related requirements*],
  [
    - Feature 4: Game resume-ability
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
  ],

  [*Test procedure*],
  [
    - Open the game in any one of the specified test environments
    - Enter the game scene
    - Snapshot the game state before any interactions with the game
    - Answer one question correctly
    - Answer one question incorrectly
    - Move character forwards within question room
    - Snapshot the game state
    - Pause game
    - Un-pause game
    - Snapshot the game state again
    - Assert that the last snapshot state is the same as the snapshot state before pausing
    - Assert that the last snapshot state is different from the first snapshot state taken before any interactions.
  ],

  [*Test material used*],
  [
    - Computer/Laptop
    - VR Headset
  ],

  [*Expected result*],
  [
    - State after pausing is the same as the state before pausing.
    - State after pausing is different from the state before any interactions.
  ],

  [*Comments*], [],
  [*Created by*], [Alexandre Pinheiro Dias],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)

=== Failure severity
Failure of this test is of severity *MAJOR*, since it would imply that data is not stored within the scene tree, this means that the design is deviating from Godot's Node system, which could lead to undefined behaviour when, swapping scenes, creating game saves, etc.

// The discussion of text context, and the choice of classes of
// failure severity should be appropriate to the nature and form of
// your application, and where appropriate, you should give
// reasons for your choices.

== Case 2: Randomisation of questions
#table(
  columns: (auto, auto),
  [*ID*], [UT_2],
  [*Description*],
  [
    To make sure that quiz questions are shuffled properly so that no two playthrough's have an identical question order.
  ],

  [*Related requirements*],
  [
    - Feature 6: Course Selection
    - Feature 10: Adaptive Difficulty Within Game
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - A loaded question bank with multiple questions
  ],

  [*Test procedure*],
  [
    1. Launch the game in a test environment.
    2. Select a course with at least 10 questions.
    3. Begin the quiz
    4. Store the question order.
    5. Complete or quit the quiz.
    6. Relaunch the game, selecting the same course again.
    7. Begin the quiz
    8. Store the new question order.
    9. Repeat steps 5-8 at least 5 times, each time comparing the new list of questions to the original list from the first quiz attempt.
  ],

  [*Test material used*],
  [
    - Computer or VR headset
  ],

  [*Expected result*],
  [
    - Each new repetition presents the quiz questions in a different or partially varied order.
    - The combined set of question orders have less than 5% repetition.
    - No discernible predictable pattern for question order.
  ],

  [*Comments*],
  [
    - Minor differences in question selection are acceptable as long as not always identical
  ],

  [*Created by*], [Rohab Kashif],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)
=== Failure severity
Failure to randomise the quiz is *MODERATE* because while the game remains functional, predictable question orders may reduce replay value and learning engagement.


== Case 3: Difficulty is adjusted appropriately based on answers supplied
#table(
  columns: (auto, auto),
  [*ID*], [UT_3],
  [*Description*],
  [
    Validates that the game is adjusting the adaptive difficulty correctly when the user inputs a correct or wrong answer.
  ],

  [*Related requirements*],
  [
    - Feature 10: Adaptive Difficulty Within Game
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - VR and 3D viewport able to be accessed and used
    - A loaded question bank with multiple questions
    - Game can be completed from start to finish
  ],

  [*Test procedure*],
  [
    // 1. Launch the game in a test environment.
    // 2. Select any course with a sufficient number of questions.
    // 3. Begin the quiz and note the difficulty of questions.
    // 4. See if the questions get easier if you answer wrongly (answer all the questions wrongly and note the difficulty of the questions after).
    // 5. Relaunch the game, select the same quiz again, answer the questions again but correctly and see if the difficulty of the questions get harder.
    // 6. Compare and contrast the questions given to the player for difficulty.
    1. Launch the game in a test environment.
    2. Select any course with a sufficient number of questions and begin the quiz.
    4. Note the difficulty score associated with the given question (internal).
    4. Answer all of the questions correctly
    5. Check that the difficulty score for the last question has increased
    6. Relaunch the game, select the same quiz again, answer the questions again but incorrectly.
    5. Check that the difficulty score for the last question has decreased
  ],

  [*Test material used*],
  [
    - Desktop/laptop and (optionally) VR headset
    - Keyboard/mouse or VR controllers
  ],

  [*Expected result*],
  [
    - Questions should get harder if the player gets more questions correct (difficulty score increases)
    - Questions should get easier if the player gets more questions wrong (difficulty score decreases)
  ],

  [*Comments*],
  [
    - As there is randomisation in questions being asked, difficulty of questions should be compared in more extreme circumstances (for example, comparing the first question to the last question if you are answering all questions wrongly).
    - Due to the adaptive difficulty, the first question is always of medium difficulty, and hence can become easier or harder.
  ],

  [*Created by*], [Wong Jing Lei],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)
=== Equivalence classes
This test has one equivalence class, which is the course selection. The test should give the same result regardless of the course selected.

=== Failure severity
A failure in the adaptive difficulty would increase the likelihood of the user feeling the game to be too easy or too hard, decreasing enjoyment of the game and learning engagement. Thus, the severity of its failure can be classified as *MODERATE*.
== Case 4: Quiz System Detects Wrong and Right Answers
#table(
  columns: (auto, auto),
  [*ID*], [UT_4],
  [*Description*],
  [
    Ensures that the quiz system correctly identifies and processes both correct and incorrect answers
  ],

  [*Related requirements*],
  [
    - Feature 7: Ability to review previous quiz answers
    - Feature 8: Scoring system for Player Performance
    - Feature 10: Adaptive Difficulty Within Game
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - A loaded question bank with correct answers
  ],

  [*Test procedure*],
  [
    1. Launch the game in a test environment.
    2. Select any course with a sufficient number of questions.
    3. Begin the quiz and note the difficulty of questions.
    4. Observe the system's feedback on each response
    5. Check whether the correct answers are acknowledged and incorrect answers are flagged appropriately.
    6. Complete the quiz and review the final results.
  ],

  [*Test material used*],
  [
    - Desktop/laptop and (optionally) VR headset
    - Keyboard/mouse or VR controllers
  ],

  [*Expected result*],
  [
    - Correct answers are identified and acknowledged by the system.
    - Incorrect answers are flagged with appropriate feedback.
    - The scoring system updates accordingly based on the correctness of the responses.
  ],

  [*Comments*],
  [
    - Unintended input interactions should be considered in testing.
  ],

  [*Created by*], [Siang Wei Law],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)
=== Failure severity
A failure in the in the quiz system's ability to detect correct and incorrect answers would lead to incorrect scoring and misrepresentation of the player's performance. This could cause frustration among users, reducing trust in the game's educational value which may impact the learning engagement of the game. Thus, the severity of its failure can be classified as *MODERATE*.

== Case 5: Player Movement Synchronisation Accuracy test
#table(
  columns: (auto, auto),
  [*ID*], [UT_5],
  [*Description*],
  [
    Ensures that player movements are accurately synchronised across all clients in a multiplayer session, with minimal lag and no desynchronisation. The test defines specific latency and packet loss thresholds to measure acceptable performance.
  ],

  [*Related requirements*],
  [
    - Feature 12: Online Multiplayer Support
    - Feature 15: Real-Time Synchronisation
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - Online servers running
    - Two or more test devices available
  ],

  [*Test procedure*],
  [
    1. Launch the game on two or more devices.
    2. Create and join a multiplayer session.
    3. Perform controlled player movements (e.g., forward, backward, strafing, jumping) and measure synchronisation delay.
      - Player position updates must be reflected within 100ms on all clients.
    4. Simulate network latency of 50ms, 100ms, and 200ms and observe movement smoothness.
    5. Introduce packet loss of 5% and 10% and check if movements remain predictable and smooth.
    6. Force a temporary disconnect (e.g., 5 seconds) and measure the time for movement synchronisation to recover upon reconnection. Recovery must occur within 5 seconds.
    7. Observe movement interpolation/smoothing mechanisms when data packets are delayed or lost.
  ],

  [*Test material used*],
  [
    - Computers/Laptops
    - Network simulation tools (e.g., NetLimiter, Clumsy)
    - Stable and unstable network conditions (WiFi, Ethernet)
  ],

  [*Expected result*],
  [
    - Player movement updates are synchronised across clients within 100ms under normal conditions.
    - Under 50ms latency, movement remains smooth with no noticeable lag.
    - Under 100ms latency, minor but acceptable delays occur, with no major desynchronisation.
    - Under 200ms latency, movement prediction mechanisms compensate for lag, keeping gameplay functional.
    - At 5% packet loss, movement synchronisation is slightly affected but remains playable.
    - At 10% packet loss, movement jitter may be present but should not cause full desynchronisation.
    - If a player disconnects for 5 seconds, movement synchronisation must resume within 5 seconds of reconnection.
  ],

  [*Comments*],
  [
    - Observe interpolation effects when lag spikes occur.
    - Test under both high-speed (10 Mbps) and low-speed (1 Mbps) connections.
  ],

  [*Created by*], [Muhammad Rafay Abbas],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)

=== Failure severity
If movement updates exceed 100ms delay, the issue is *MODERATE*. If movement desynchronisation causes incorrect player positioning, the issue is *MAJOR*. If movement fails entirely or players experience rubber-banding beyond 200ms, the issue is *CRITICAL*.


= System Tests
== Case 1: Complete Quiz Session Test
#table(
  columns: (auto, auto),
  [*ID*], [ST_1],
  [*Description*],
  [
    Validates the system’s handling of a complete quiz session, including course selection, randomised questions, adaptive difficulty, scoring, and final summary. Ensures the entire flow integrates smoothly without errors.
  ],

  [*Related requirements*],
  [
    - Feature 6: Course Selection
    - Feature 8: Scoring System for Player Performance
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - At least two courses available with multiple questions each
    - Working question bank, supporting easy, medium, and hard questions
  ],

  [*Test procedure*],
  [
    1. Launch the game in a supported environment (VR or 3D).
    2. From the main menu, select "Course Selection" and choose a course.
    3. Begin the quiz:
      - Observe that the question order is randomised.
    4. Continue the quiz, mixing correct and incorrect answers to ensure:
      - Points are consistently tallied for correct answers only.
    5. Complete the quiz or reach the end of the question set:
      - Observe final results screen (score, time taken, possibly accuracy).
    6. Return to the main menu:
      - Confirm no error messages or crashes occur, and the final score is cleared or stored as needed.
  ],

  [*Test material used*],
  [
    - Desktop/laptop and (optionally) VR headset
    - Keyboard/mouse or VR controllers
  ],

  [*Expected result*],
  [
    - Course selection loads the correct question bank.
    - Randomised question order each session.
    - Scoring system reflects each user response accurately.
    - Final summary screen displays correct total score and relevant metrics.
    - Game transitions back to the main menu without errors.
  ],

  [*Comments*],
  [
    - If multiple courses are available, repeating the test with different courses is recommended to confirm system consistency.
    - In VR mode, ensure that all menus and question prompts are accessible via controllers.
  ],

  [*Created by*], [Rohab Kashif],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)

=== Failure severity
A failure in any major element of this test such as course selection failing, randomised questions not loading, scoring failing to record correct answers, or the final summary not displaying—renders the core quiz functionality unusable. Because this test represents the primary user interaction and educational purpose of the game, any fundamental failure in these steps is deemed *MAJOR*, as it prevents players from completing the quiz experience.


== Case 2: Performance Testing
#table(
  columns: (auto, auto),
  [*ID*], [ST_2],
  [*Description*],
  [
    Validates that the game is running at a stable and smooth framerate with valid rendering.
  ],

  [*Related requirements*],
  [
    - Game being able to be played without issue.
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - VR and 3D viewport able to be accessed and used
    - Game can be completed from start to finish
  ],

  [*Test procedure*],
  [
    1. Launch the game in a supported environment (VR or 3D).
    2. Start the game and play through the game until the end.
    3. During the playthrough, record performance metrics (frame rate, RAM usage, loading times)
    4. Process performance metric data and look for averages and spikes in the metrics.
  ],

  [*Test material used*],
  [
    - Desktop/laptop and (optionally) VR headset
    - Keyboard/mouse or VR controllers
  ],

  [*Expected result*],
  [
    - FPS is stable and provides smooth performance with little to no stutter/lag (Above 30FPS constantly)
    - Loading times should be quick (under 10 seconds)
  ],

  [*Comments*],
  [
    - As performance differs in different environments, give leeways according to strength of system and strength of method used (VR should take more processing power than 3D for example)
  ],

  [*Created by*], [Wong Jing Lei],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
      - Minimum Spec: Ryzen 7 7730U with Integrated Graphics or equivalent
    - Arch Linux: Firefox (latest), Chromium (latest)
      - Minimum Spec: Ryzen 7 7730U with Integrated Graphics or equivalent
    - Quest 2/3 Browser
  ],
)

=== Failure severity
A failure in terms of performance would be detrimental to the player being able to complete or even play the game in certain scenarios. However, severity of failure is also dependant on scale of failure. Performance issues that lead to player being unable to play the game is deemed *MAJOR*, while those that affect the quality of experience offered (framerate dips, long loading times) can be deemed *MODERATE*, while those that mildly inconvenience that player (infrequent minor stuttering during gameplay, mildly long loading times) can be deemed *MINOR*.

== Case 3: Viewports work on all target environments
#table(
  columns: (auto, auto),
  [*ID*], [ST_3],
  [*Description*],
  [
    Test that both viewports of the game (VR and 3D) load in all expected target environments.
  ],

  [*Related requirements*],
  [
    - Feature 1: Immersive VR view port
    - Feature 2: Traditional 3D view port
  ],

  [*Pre-requisites*],
  [
    - VR enabled browser (through VR headset or WebXR Emulator)
    - Game compiled successfully
  ],

  [*Test procedure*],
  [
    - Open the game in one of the specified test environments
    - Select the option "Play in VR" in the main menu
    - Ensure that a system prompt is shown to enter VR
    - Select the option "Play in VR" in the main menu
    - Accept the system prompt to enter VR
    - Press the menu button on the VR controller
    - Select "Return to main menu"
    - Select the option "Play in 3D" in the main menu
    - See that you are in the game world.
    - Press Esc or the menu button on a controller
    - Select "Return to main menu"
  ],

  [*Test material used*],
  [
    - Computer/Laptop
    - VR Headset
  ],

  [*Expected result*],
  [
    - Main menu shows with buttons: `Play in VR`, `Play in 3D`, `Settings`, `Quit`
    - Ensure system prompt is shown to enter VR
    - Returned to the main menu after denying
    - Check if you are inside the game world in immersive VR after accepting the system prompt
    - Pause menu should show options: `Resume`, `Settings`, `Return to main menu`
    - Should exit the game world and return to the initial main menu
    - Check if you are inside the game world (3D character and traditional input methods) after clicking `Play in 3D`.
    - Pause menu should show options: `Resume`, `Settings`, `Return to main menu`
    - Should exit the game world and return to the initial main menu
  ],

  [*Comments*],
  [
    - Ensure browsers are up to date.
    - Open the game in all major browsers.
    - Ensure the webpage is running in a secure environment (i.e. HTTP(s) server through Godot)

    - This should be tested in all test environments defined below.
  ],

  [*Created by*], [Alexandre],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)
=== Failure severity
Failure of this test is of severity *CRITICAL*, as the user cannot proceed to play the game if they are unable to enter the game world.

== Case 4: Audio Feedback and Synchronisation
#table(
  columns: (auto, auto),
  [*ID*], [ST_4],
  [*Description*],
  [
    Ensures that audio feedback (sound effects, voiceovers, music) plays correctly, synchronises with in-game events, and enhances the user experience.
  ],

  [*Related requirements*],
  [
    - Feature 1: Immersive VR view port
    - Feature 2: Traditional 3D view port
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - Audio drivers and hardware properly configured
  ],

  [*Test procedure*],
  [
    1. Launch the game and ensure that audio is enabled.
    2. Test different game scenes: main menu, quiz, game events (correct/incorrect answers, question changes).
    3. Confirm that background music plays correctly, and any audio cues match the corresponding actions (e.g., correct answer feedback, score updates, etc.).
    4. Adjust volume levels and test whether sound balances appropriately across all audio channels (voice, music, effects).
    5. Check that no audio cuts off unexpectedly or gets delayed during gameplay.
  ],

  [*Test material used*],
  [
    - Desktop/laptop
    - VR headset (Quest 2/3)
    - Headphones or speakers
  ],

  [*Expected result*],
  [
    - All audio elements should play correctly and at the right time during gameplay.
    - Background music, sound effects, and voiceovers should be balanced and clear.
    - No audio glitches, delays, or mismatched synchronisation between audio and visuals.
  ],

  [*Comments*],
  [
    - Test audio on different devices to ensure consistency (e.g., PC speakers, headphones, VR headset).
  ],

  [*Created by*], [Muhammad Rafay Abbas],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)
=== Failure severity
A failure in the audio feedback system can detract from the overall experience, affecting immersion and user engagement. If audio cues fail to synchronise or play at all, the failure can be classified as *MODERATE*, as it may negatively impact user experience but not prevent gameplay.

== Case 5: VR headset Disconnection
#table(
  columns: (auto, auto),
  [*ID*], [ST_5],
  [*Description*],
  [
    Ensures that the game pauses successfully when the VR headset is disconnected
  ],

  [*Related requirements*],
  [
    - Feature 4: Game resume-ability
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
  ],

  [*Test procedure*],
  [
    1. Launch the game and ensure that game runs.
    2. Start the game
    3. Disconnect the VR headset.
    4. Reconnect the VR headset.
    5. Game should be paused.
  ],

  [*Test material used*],
  [
    - Desktop/laptop
    - VR headset
  ],

  [*Expected result*],
  [
    - Game screen should be paused upon reconnection of the VR headset.
  ],

  [*Comments*], [],
  [*Created by*], [Siang Wei Law],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)

=== Failure severity
A failure due to VR headset disconnection can impact the gameplay experience by causing interruptions or forcing the user to restart. This could lead to loss of progress, leaving players demoralised and unwilling to play again. Thus, since disconnecting/taking off a VR headset is common occurrence, the failure is classified as *MAJOR*.


== Case 6: Multiplayer Latency test
#table(
  columns: (auto, auto),
  [*ID*], [ST_6],
  [*Description*],
  [
    Ensures that multiplayer connections remain stable with measurable latency, synchronisation, and reconnection thresholds. Players should be able to join, stay connected, and communicate in real-time without unexpected disconnections or excessive lag. The test defines specific numerical limits for acceptable performance.
  ],

  [*Related requirements*],
  [
    - Feature 12: Online Multiplayer Support
  ],

  [*Pre-requisites*],
  [
    - Game compiled successfully
    - Online servers running
    - Two or more test devices available
  ],

  [*Test procedure*],
  [
    1. Launch the game on two or more devices.
    2. Attempt to create and join a multiplayer session.
    3. Perform various in-game actions (e.g., moving characters, using abilities) and measure synchronisation delay between clients. Synchronisation must occur within 100ms.
    4. Simulate network instability by disconnecting the network for 5 seconds and then reconnecting. Measure time taken for players to rejoin. Reconnection must occur within 10 seconds.
    5. Measure response time for:
      - Player actions (e.g., movement, attacks): Must be processed within 100ms.
    6. Test under different network conditions:
      - Bandwidth: 1 Mbps, 5 Mbps, 10 Mbps
      - Packet loss: 0%, 5%, 10%
      - Latency spikes: Simulate 50ms, 100ms, and 200ms delays and observe game stability.
  ],

  [*Test material used*],
  [
    - Computers/Laptops
    - Network simulation tools (e.g., NetLimiter, Clumsy)
    - Stable and unstable network conditions (WiFi, Ethernet)
  ],

  [*Expected result*],
  [
    - Players successfully connect and remain in the session for at least 30 minutes without forced disconnections.
    - No significant lag or desynchronisation occurs; actions are synchronised within 100ms.
    - Players can rejoin within 10 seconds after an unintentional disconnection.
    - The game remains playable under at least 5% packet loss without severe degradation.
  ],

  [*Comments*],
  [
    - Test under different network conditions, including low bandwidth and high packet loss scenarios.
  ],

  [*Created by*], [Muhammad Rafay Abbas],
  [*Test environment(s)*],
  [
    - Windows 11: Firefox (latest), Chromium (latest)
    - Arch Linux: Firefox (latest), Chromium (latest)
    - Quest 2/3 Browser
  ],
)

=== Failure severity
Failure to join a multiplayer session at all is *MODERATE*, since multiplayer is not a core section of the game. If latency exceeds 150ms or synchronisation delays exceed 100ms, then the severity is *MAJOR*, as the game remains playable but degrades significantly. If players cannot rejoin within 10 seconds after an unintentional disconnection, this is a *MAJOR* issue, as this can lead to lost progression.

