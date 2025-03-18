#set page(numbering: "1 of 1", margin: 1in)
#set par(leading: 0.55em, spacing: 1.2em, justify: true)
#set text(font: "New Computer Modern", lang: "en", region: "gb")
#show raw: set text(font: "New Computer Modern Mono", size: 11pt)
#set raw(syntaxes: "assets/gherkin.sublime-syntax")
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

      #text(size: 2em, weight: 900, [Requirements Document])

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
== Overview and justification// Recommended 1/2 page.
// Specify the purpose and need for the proposed system in high-level terms, and
// precisely who it will benefit if delivered as proposed. You should include
// details about your client, including any relevant organisational details (e.g.
// who your contact within the organisation is) and their aims and motivations. You
// should also briefly state how the remainder of the document is structured,
// including the rest of the introduction section.
The project we are delivering is an web-based VR game educating potential new
users about the courses presented on *IBM SkillsBuild*.

The premise of the game is to repair a broken spaceship, which requires
answering short questions (in varying difficulty) to repair each sector of the
ship.

Our client is _Mr John Mc Namara_ from IBM, who serves as our primary contact for the project. IBM aims to increase access to technical education, particularly for students and professionals seeking high-demand skills. By presenting this content in a VR format, IBM aims to sustain user attention and foster motivation through an innovative approach to learning.

This document outlines the project’s scope, system description, solution requirements, and project management strategies, providing a comprehensive view of the proposed solution and how it addresses both IBM’s educational goals and user needs.

== Project scope// Recommended 1/2 page.
// Specify the exact project scope, indicating the project’s boundaries. This
// should also include the purpose of the software project, your overall goals, and
// how these align with the interests of your and other stakeholders. This section
// should contain your vision for your product or service and should indicate the
// exact user base of the proposed product.With an emphasis on AI, cybersecurity, and data analytics, the VR
=== Purpose of the Project
SkillsBuild Reimagined project aims to develop an entertaining educational VR game that makes use of IBM's SkillsBuild content. The goal is to incorporate these subjects into an interactive, story-driven game that will improve learning by allowing players to progress through immersive stages by responding to quiz questions based on SkillsBuild.

=== Overall Goals and Stakeholder Alignment
Our primary goal is to create an innovative educational experience that motivates users to develop their skills in high-demand fields like AI and Cybersecurity. This supports IBM's goal of advancing technical education by delivering information in a way that sustains attention and motivates more research. The game offers learners a new visual and inspiring method to acquire in-demand skills. Additionally, it enables interdisciplinary collaboration among university students from fields like computer science, design, education, and business, providing real-world project experience and fostering IBM’s relationship with academia.

The *project scope* is confined to developing a prototype VR game with *key features*, including:

- *Educational Content Integration*: Using IBM SkillsBuild topics, a dynamic question bank is produced and seamlessly incorporated into the game.
- *Adaptive Difficulty through AI Mechanics*: Using AI-powered difficulty adjustment to tailor the learning experience according to player performance.
- *Immersive Game Design*: Drawing inspiration from the game Paradroid, the VR experience will feature a space-themed setting where players navigate and unlock new levels by completing progressively challenging levels. The game will take no longer than 20 minutes to complete in order to retain active player engagement.
- *Multiplayer Capability (Optional)*: Although the game is primarily single-player, a local multiplayer feature might be added as an ambitious objective.

=== User Base
The primary users are *IBM SkillsBuild* learners and students who want to develop technical skills interactively. Secondary users may include educators and IBM stakeholders interested in exploring a fresh approach to engage learners through VR and adaptive learning.
#pagebreak()

== System description// Recommended 1 page.
// You should provide an overview of the system to be built. Where appropriate, you
// should first briefly detail any existing/legacy systems. You should demonstrate
// what research into alternative solutions the team has undertaken by providing a
// brief description of comparable commercial or non-commercial solutions. You
// should evaluate the usefulness. In this section, you should focus on the
// technical features of the proposed solution itself, rather than on its
// behavioural requirements. Where these have not yet been determined, note that
// this is the case.
Our primary objective is to build a comprehensive educational space themed VR game which covers basic content about *IBM SkillsBuild*, in each sector players are provided information on the chosen course through their surroundings, where they have to complete a quiz to progress to further sectors.
=== Existing/legacy systems
We were not provided access to any existing or legacy systems, though we were given viewing access to a web 3D showroom of some IBM SkillsBuild courses which was developed as part of another project. However, this is too different from our project for it to be of any use. There is also a lack of open source educational VR games, and choosing to choose an open source game to start from means inheriting their design choices, many of which do not fit our project goals (space theme, web based, quizzes, etc.).
=== Similar solutions
While there are no games that aim to achieve the exact same aims as our project, there are many games (educative & no educative) that we are using as inspiration aspects of our game.

In our game we need to present content to players in a fun and interactive way, the VR game `Trivia Crack World` and 2D `Paradroid` are both successful games which we will be using as inspiration for presenting questions. We also need to display content for players to learn, so to do this we are placing the content around the room for users to explore and learn, this takes inspiration from traditional escape rooms alongside VR puzzle games such as `The Room VR: A Dark Matter`, and games like `Subnautica` where you learn information about the story (in our case, course content) by exploring your environment.

=== Game engine
We chose to use Godot, primarily because it supports *WebXR* the best, which will allow us to have users play through the browser without downloading the game, allowing the game to have further reach. Godot is also open source with a large user base and amount of contributors, which gives us confidence that our game will still be maintainable for the foreseeable future.

Finally, Godot also supports a variety of extra programming languages through `GDExtensions`, which provides a FFI which allows for binds to be made for lots of popular programming languages such as C++, Python, and Rust. This gives us the flexibility to move away from GDScript if we find it hard to learn, slow, or less comfortable to develop in than languages we are familiar with.

=== Alternative game engines
- *Unity:* While Unity was also a popular choice among us due to the large portfolio of VR games developed with it, recent controversies about changes in their pricing plans, along with it being closed source have left us with doubts about its long term viability, whether Unity decides to stop developing the engine, or if in the future our client decides to move to move to a more monetised or commercial approach to the game.

  It also lacks official *WebXR* support, with the only option as of now being an unmaintained exporter, which would limit the platforms the game is able to be published to by the client.

  Finally, Unity uses C\# as the primary language for writing games, which none of the team has experience in, it also has an unfamiliar syntax, which means we would have to spend a long time learning the language, reducing the time we have to deliver this project.
- *Unreal Engine:* Unreal Engine was also a strong contender, which unlike Unity, has had a transparent and stable pricing plan, this still constitutes a cost if the client decided to persue a monetised or commercial approach to the game. As it is also closed source, it means that in the future we may be stuck if Unreal Engine becomes abandoned or the terms and conditions change. Likewise, it is also limited in languages that it supports (C++ and blueprint), which means a higher learning curve since none of the team has much experience in C++.


= Solution Requirements
// This section should give insight into the outcome of your requirements-gathering
// discussions with your client and stakeholders along with evidence of your team’s
// efforts to ground the requirements in these discussions. You should provide
// evidence of the logical organisation and prioritisation of your requirements.
// The approach you must use to specify the requirements of your client takes its
// inspiration from behaviour-driven development (BDD). Therefore, you must use
// User Stories and Gherkin (https://cucumber.io/docs/gherkin/reference/) to
// specify solution requirements as features and scenarios. The Gherkin pseudocode
// specifies how your system should behave in a concise, clear, and objective way;
// this makes it easy to assess whether the feature the pseudocode describes has
// been implemented. In turn, this allows stakeholders to understand when the
// underlying client requirements are fulfilled.

== Requirements elicitation// Recommended 1 page.
// Report on the steps undertaken by the group to elicit the client’s requirements,
// to develop them into User Stories, and to refine them into behavioural
// specifications using the Gherkin language. You should convince the reader that
// the specifications you have arrived at represent the requirements of the client
// to the best of your ability. Report on any difficulties or challenges you
// encountered and how you overcame them (if relevant). This section may mention
// any meetings or correspondence you had with the client, although screenshots of
// emails or DMs are not appropriate here. You can also report on the outcomes of
// any internal meetings that were especially influential on the refinement and
// validation of the requirements and specify any established methods or approaches
// you used to achieve this.

After initially receiving the project brief from IBM, our team did a quick run-through of it and made an initial draft of our expectations for the project. We then drafted and sent an email with Rafay as our main correspondent to our client introducing ourselves and asking some initial questions about the project such as the actual content of IBM SkillsBuild our client wants us to use, as well as requesting a meeting slot.

We received a reply in a timely manner, our client was not able to meet us during that specific week as he was busy at a conference, but we did receive additional details regarding our project such as the IBM standards of practice that our team is expected to follow, how our client wants us to make a more quest-based game, whether it be a single one or multiple, with some ideas on the game theme (space). He also requested us to complete some courses within IBM SkillsBuild to get us started. We encountered some difficulties here as these courses take quite a long time to complete, we were able to negotiate with the client to lower the amount of courses each team member actually had to complete which is a good compromise.

After the week passed, we tried to organise a one-on-one meeting between the team and the client but he declined, stating that he has too many teams to manage, so he decided to hold a virtual office hours instead so that every team would get a chance to ask him questions. Our team initially held some reservations about the idea. When we attended the office hours, we came with a list of questions prepared beforehand and our initial game design sketch. We were then able to slowly modify our game design sketch as the client answered the questions that we had and we are confident that are able to elicit the main requirements that the client wants.

The format of the office hours became a lot more agreeable to us after the first meeting. The client also stated that he will be holding more office hours mostly every week so we can have frequent contact with the client as we needed along the development cycle.

After the meeting with the client, we then spent two internal meetings further refining our game design sketch into actual user stories and behavioural requirements by initially drafting user stories using a "who", "what", "why" table. Initially, we set out on two main "who"s, IBM and the player of the game. However, after some deliberation between us and our advisor from the practical, we decided to be more specific when describing IBM, as they are quite the large company and have many stakeholders that may become involved with our project. These include the IBM SkillsBuild Development Team and their Product Development Team. We had to consider the data that our game could generate for IBM and how they might want to use it. As for the player of the game, we considered more on accessibility, and feedback. As the game is meant to be educational in nature, our team decided that the user stories should focus more on widening access to players of all types and them being able to learn something instead of the pure "fun factor" of the game.

Once the user stories were well-defined, the process of converting them into behavioural requirements using the Gherkin language became a simple task, allowing us to better translate user needs into concise, structured scenarios that are easy to understand for both technical and non-technical stakeholders.


== Behavioural requirements// Max 10 pages (approx 1 page per specification)
// Provide numbered, behavioural requirement specifications for between 8 and 10
// features of your proposed solution. Each scenario must be numbered such that
// (e.g.) BR1.2 refers to the second scenario in the first feature, BR3.1 refers to
// the first scenario in the third feature (etc.). For each specified feature, you
// must present:
// - One user story, based on your requirement elicitation work.
// - One valid Gherkin pseudocode listing (use a code or preformatted text block)
//    expressing this user story as a feature consisting of at least two scenarios.
//    You may choose to number the scenarios in Gherkin comments.
// - A very brief rationale explaining how implementing this feature will contribute
//    to the overall objectives of the client and its relative priority. You should
//    use an established system to express this prioritisation (e.g. MoSCoW)

//Initial stories, ideas, user stories should be small tasks that take 1-5 days, though there _may_ be a hierarchy to them

#text(
  size: 1.2em,
  heading(outlined: true, depth: 3, [Feature 1: Immersive VR view port]),
)

*User Story*: As a member of IBM SkillsBuild marketing team, we want to promote a unique immersive learning experience through a VR game, so players become interested in continuing our courses.

```gherkin
Feature 1: Immersive VR view port

  Scenario 1 (BR1.1): Looking around with headset on
    Given I am playing the game in an immersive VR environment
    When and I look around in real life
    Then the character should look around the same amount

  Scenario 2 (BR1.2): Left thumbstick input (look around)
    Given I am playing the game in an immersive VR environment
    When I move the left thumbstick
    Then my character should look in the direction the thumbstick is tilted.

  Scenario 3 (BR1.3): Right thumbstick input (move)
    Given I am playing the game in an immersive VR environment
    When I move the right thumbstick
    Then my character should move in the direction the thumbstick is tilted.

  Scenario 4 (BR1.4): Teleport through clicking the thumbstick in the direction
    Given I am playing the game in an immersive VR environment
    When I move my mouse
        And there is no menu/item in front of my character
    Then I should teleport to the location under the centre of the screen
```

=== Feature Rationale
As there are many free technical courses and educational games available, IBM SkillsBuild needs something to differentiate from the competition and appeal to more potential learners. By providing a unique immersive VR educational experience, SkillsBuild would stand out from other offerings, potentially engaging a user base which may not have been interested in the technical educational courses provided by IBM beforehand.

This feature is a *Must Have* (using the MoSCoW priorities), as it is a strict requirement from IBM, and having an immersive VR experience will uniquely promote SkillsBuild content, helping promote further learning.

#text(
  size: 1.2em,
  heading(outlined: true, depth: 3, [Feature 2: Traditional 3D view port]),
)

*User Story*: As a player, I may not have access to a VR headset, so I want to be able to play the game as a normal 3D game.

```gherkin
Feature 2: 3D Viewport

  Scenario 1 (BR2.1): Movement controls
    Given I am playing the game as a traditional 3D game
    When I press common keyboard movement controls (WASD)
    Then my character should move in the appropriate direction.

  Scenario 2 (BR2.2): Mouse Input
    Given I am playing the game as a traditional 3D game
    When I move my mouse
    Then my character should turn in accordance with how much the mouse moved.

  Scenario 3 (BR2.3): Right clicking in the distance
    Given I am playing the game as a traditional 3D game
    When I click my right click my mouse
    And there is no menu/item in front of my character
    Then I should teleport to the location under the centre of the screen
```

=== Feature Rationale
As many potential players may not have a VR headset, allowing the game to be played in traditional 3D format, will increase the reach that the game will have, potentially bringing more users to the IBM SkillsBuild platform, this will also will help those with accessibility issues who may struggle to play VR games, or who do not have access to a VR headset for various reasons (monetary, don't have one at hand, etc.).

This feature is a *Should Have* (using the MoSCoW priorities), as having a traditional 3D view port will allow the game to be used by more players who otherwise, due to a lack of access to a VR headset, would have not been able to play.


#text(
  size: 1.2em,
  heading(outlined: true, depth: 3, [Feature 3: Accessible through the web]),
)

*User story:* As a player, I want to be able to access the game through a website, so that I can do I can play it without having to install it to my device.

```gherkin
Feature 3: Accessible through the web

  Scenario 1 (BR3.1):  Has VR headset
    Given I am a player with a VR headset
    And I go to the website
    Then a prompt to play the game in immersive VR should appear
    And then request permission to display the game in immersive 360° VR

  Scenario 2 (BR3.2): No VR headset
    Given I am a player without a VR headset
    And I go to the website
    Then  a prompt stating game is better played with a VR headset should appear
    But an option should appear to play it as a traditional 3D game

  Scenario 3 (BR3.3): No WASM support
    Given I am a player going to the website
    And my browser does not support WASM
    Then I should receive a prompt that my browser is unsupported
    And provide a list of compatible browsers
```
=== Feature Rationale
Providing access to the game through the web ensures that players can engage with the game without needing to install the game on their device, simplifies distributing the game. and lowers the barrier to entry for potential players, making the game more accessible and easy to try out.

By showing users that both a traditional 3D and immersive VR experience are available, users are more likely to play (even if they don't have a VR headset) since they are now aware the option is available

This feature is a *Must Have* (using MoSCoW prioritisation), as easy accessibility to the game is key to broadening the audience and ensuring that as many players as possible can easily access and enjoy the game.


#text(
  size: 1.2em,
  heading(outlined: true, depth: 3, [Feature 4: Game resume-ability]),
)

*User story:* As a player, I want my progress to be automatically saved when I progress so that I can resume playing at some other point without having to repeat the same questions.


```gherkin
Feature 4: Game resuming (pausing and auto saving)

  Scenario 1 (BR4.1): Automatically save game progress while playing
    Given I am playing the game
    And I complete a question
    Then my new progress should be saved

  Scenario 2 (BR4.2): Pause the game when the browser is not focus
    Given I am playing the game
    When the browser goes out of focus
    Then the game should pause any ongoing actions
    And save my progress
    And show a menu where I can resume the game

  Scenario 3 (BR4.3): Pause the game when a menu button is pressed
    Given I am playing the game
    And I press a menu button on an input device
    Then the game should pause any ongoing actions
    And save my progress
    And show a menu where I can resume the game

  Scenario 4 (BR4.4): Resume from the pause menu
    Given I am in the pause menu
    And I press resume
    Then any ongoing actions should continue
    And the gameplay should resume from before it was paused

  Scenario 5 (BR4.5): Reopening the game
    Given I am opening the game
    And a game save is found
    Then show a menu where the game can be resumed from their save point
```
=== Feature Rationale
As many players may not be ready to commit the time needed for the full duration of the game initially at once, saving their progress automatically when they progress will ensure that the users resume their gameplay where they last left off. This will help encourage users to continue playing since they have already invested time into playing, and do not need to restart from the beginning (which can be repetitive or a waste of time).

By allowing the game to be paused, either manually or automatically (when a user), we can ensure that the player feels control over the pace of the game and provides adequate respect for any commitments the player may have.

This feature is a *Must Have* (using MoSCoW prioritisation), as having an auto-save and pause system is critical for player satisfaction and engagement. It ensures that players can enjoy the game at their own pace without the fear of losing progress, ultimately improving the user experience and increasing the likelihood that players will complete the game.


#text(
  size: 1.2em,
  heading(
    outlined: true,
    depth: 3,
    [Feature 5: Short, Engaging Game Sessions for Learning],
  ),
)

*User story:* As a member of the IBM SkillsBuild development team, I want game sessions to last around 10-20 minutes so that the game is suitable for short, engaging learning sessions.

```gherkin
Feature 5: Short Game Sessions

  Scenario 1 (BR5.1): Player completes a game session within 15 minutes
    Given a player has started a game session
    When they reach the end of a set of questions
    Then the session should complete within 10-20 minutes

  Scenario 2 (BR5.2): Notify player of session length
    Given a player is starting a game session
    When the session begins
    Then a message should display the expected session length (10-20 minutes)

  Scenario 3 (BR5.3): Player continues after session end
    Given the player finishes the current session
    And they wish to continue
    Then allow the player to begin a new session
```

=== Feature Rationale
This feature makes the game suitable for quick learning sessions that are aligned with the client’s objective to encourage continuous, incremental learning. By designing sessions to fit within short timeframes, the game becomes more accessible to players with limited time, increasing the likelihood of completion.

This feature is classified as a *Should Have* under MoSCoW priorities, as it is not critical but highly valuable for enhancing engagement.

#text(
  size: 1.2em,
  heading(
    outlined: true,
    depth: 3,
    [Feature 6: Course Selection for Customised Learning Paths],
  ),
)

*User story:* As a player, I want sections for multiple courses so that I can choose the courses and learn more about the ones that interest me.

```gherkin
Feature 6: Course Selection

  Scenario 1 (BR6.1): Player selects a specific course
    Given a list of available courses is presented
    When the player selects a course
    Then they should enter a game session based on that course’s content

  Scenario 2 (BR6.2): Return to course selection menu
    Given the player is in a game session
    When the session ends or the player pauses the game
    Then they should have the option to return to the course selection menu

  Scenario 3 (BR6.3): Provide course details before selection
    Given the player is viewing the list of available courses
    When they hover over or click on a course
    Then a brief description of the course content should display

  Scenario 4 (BR6.4): Recommend related courses after session completion
    Given the player has completed a game session for a specific course
    When the session ends
    Then the game should display a list of related or advanced courses they
         can select next
```

=== Feature Rationale
This feature supports the goal of promoting SkillsBuild courses by allowing players to choose learning paths based on their interests. This customisation can enhance motivation, as players are more likely to stay engaged when they control their learning topics. The game encourages further engagement with SkillsBuild content by offering course recommendations after each session.

This feature is a *Must Have* in MoSCoW prioritisation, as it directly aligns with IBM's objective to engage users in relevant learning content.

#text(
  size: 1.2em,
  heading(
    outlined: true,
    depth: 3,
    [Feature 7: Ability to review previous quiz answers],
  ),
)

*User Story:* As a player, I want to be able to review my previous quiz answers so I can see my performance during previous playthrough's.
```gherkin
Feature 7: Peformance History Logs

  Scenario 1 (BR7.1): Reviewing incorrect answers for learning
    Given I have completed a section of the quiz
    When I choose to review my answers
    Then the game displays my incorrect answers
    And provides correct answers and their explanations

  Scenario 2 (BR7.2): Comparing performance across attempts
    Given I have completed multiple playthroughs
    When I check my previous answers
    Then the game shows my past responses as trends in the form of graphs
    And highlights my overall strengths and weaknesses

  Scenario 3 (BR7.3): Reviewing time spent on each question
    Given I have completed a quiz playthrough
    When I choose to review my answers
    Then the game shows the time I spent on each question
    And highlights any questions where I spent an unusually long or short time
```

=== Feature Rationale
This feature is meant to enhance learning by allowing users to review past performance, focusing on incorrectly answered questions and trends over time. It also highlights questions where users took too long to respond, helping improve accuracy and time management for more effective learning.

This feature is a *Should Have* (using the MoSCoW priorities), as having access to data from previous playthrough's can really boost the learning and retention of information, ultimately fulfilling the main goal of the game, but it isn't fundamentally tied to any specific core mechanic of the game.

#text(
  size: 1.2em,
  heading(
    outlined: true,
    depth: 3,
    [Feature 8: Scoring System for Player Performance],
  ),
)

*User Story:* As a player, I want a scoring system that rewards me for correct answers and quick completion so that I feel motivated to improve my performance and fully engage with the game’s challenges.

```gherkin
Feature 8: Scoring System for Player Performance

  Scenario 1 (BR8.1): Player earns points for correct answers
    Given the player is presented with a question
    When the player answers correctly
    Then the system should add a predefined number of points to the player’s score.
    And the updated score should be displayed in the game’s interface.

  Scenario 2 (BR8.2): Player does not earn points for incorrect answers
    Given the player is presented with a question
    When the player answers incorrectly
    Then the system should not add any points to the player’s score.
    And the updated score should remain unchanged.

  Scenario 3 (BR8.3): Bonus points for faster completion
    Given the player completes all sections of the game
    When the completion time is calculated
    Then the system should award bonus points based on the player's total time.
    And the bonus points should be added to the player’s score
    And the final score, including the time bonus, should be displayed.
```

=== Feature Rationale
This feature is designed to boost motivation and engagement by rewarding players for correct answers and timely completion.

This feature is a *Should Have* (using the MoSCoW priorities), as it enhances the player experience and supports the educational objectives of the game by providing measurable feedback. While not essential to the game’s core functionality, it significantly improves engagement, making it a valuable addition.

#text(
  size: 1.2em,
  heading(
    outlined: true,
    depth: 3,
    [Feature 9: Localised High Score with Time Tracking],
  ),
)

*User Story:* As a member of the IBM SkillsBuild development team, I want a localised high score feature that includes the highest score and the time taken so that users can compare performances to others and strive to improve

```gherkin
Feature 9: Localised High Score with Time Tracking

  Scenario 1 (BR9.1): Display highest score with completion time
    Given I have completed the game
    When my final score and completion time are calculated
    Then my score and time should be compared to the stored high score and time
    And the high score list should be updated if my score or time is better
    And I should see the updated high score list displayed locally.

  Scenario 2 (BR9.2): View local high score list
    Given I am at the game menu
    When I choose to view the high score list
    Then I should see a list of the top scores and their corresponding completion
         times stored locally.

  Scenario 3 (BR9.3): Handle tied scores
    Given two players achieve the same score
    When their scores are added to the high score list
    Then the player with the faster completion time should appear higher on
         the list.
```

=== Feature Rationale

This feature is meant for users to enhance engagement and motivation by tracking their progress.

This feature is a *Could Have* (using the MoSCoW priorities), as a localised high score with time tracking enhances the player experience but is not essential for the game's primary learning objectives. While it allows players to compare their performance and strive for improvement, the core educational and gameplay goals can be achieved without this feature. It can be implemented later to further improve engagement.


#text(
  size: 1.2em,
  heading(
    outlined: true,
    depth: 3,
    [Feature 10: Adaptive Difficulty Within Game],
  ),
)

*User Story:* As a player, I want the difficulty of the game to be adaptive so that the game can provide an engaging experience that is not too easy nor too hard regardless of my skill level.

```gherkin
Feature 10: Variable Difficulty Within Game

  Scenario 1 (BR10.1): Game adapts difficulty for poor-performing player
    Given I have gotten three questions wrong in a row
    When I reach the next question
    Then the game adjusts the question bank used so that easier questions
         are asked
    And the game keeps track of how the difficulty had to be decreased for later
        feedback

  Scenario 2 (BR10.2): Game adapts difficulty for well-performing player
    Given I have gotten three questions correct in a row
    When I reach the next question
    Then the game adjusts the question bank used so that harder questions are
         asked
    And the game keeps track of how the difficulty had to be increased for later
        feedback

  Scenario 3 (BR10.3): Game maintains difficulty for average-performing player
    Given I am not getting three consecutive questions correct or three consecutive
          questions wrong.
    When I reach the next question
    Then the game maintains its difficulty by picking the questions from the same
         question bank as before
    And the game keeps track of how the difficulty did not change for later feedback
```

=== Feature Rationale
As the game is meant to be an educational one with an wide audience of vastly different skill levels. By dynamically adjusting the difficulty level based on the player, this feature ensures the game remains challenging enough to keep advanced players interested while not overwhelming less experienced players. This offers a personalised challenge to all players, which can improve player retention as we want them to actually complete the game. This also generates data about which topics or questions players of a certain demographic struggle or excel in, in which we can then modify existing question banks to better reflect difficulty of questions.

This feature is a *Must Have* (using the MoSCoW priorities), as having an adaptive difficulty can broaden access to as many players as possible, allowing the game to be more engaging and inclusive, improving the player experience overall.


#text(
  size: 1.2em,
  heading(
    outlined: true,
    depth: 3,
    [Feature 11: Accessibility options available in the game],
  ),
)

*User Story*: As a member of the IBM SkillsBuild development team, I want everyone, regardless of disability, to enjoy the game equally by providing accommodations such as visual, auditory, and motor accessibility options. This ensures inclusivity and allows all users to fully participate in and enjoy the game experience.

```gherkin
Feature 11: Accessibility Options

  Scenario 1 (BR11.1): Enable visual accessibility options
    Given I am navigating the game settings menu
    When I choose accessibility options
    Then I should see options for colorblind-friendly modes, text resizing, and
         contrast adjustment
    And I should be able to apply my selected settings to the game.

  Scenario 2 (BR11.2): Enable auditory accessibility options
    Given I am navigating the game settings menu
    When I choose accessibility options
    Then I should see options for subtitles, volume adjustments, and audio cues
    And I should be able to enable or disable these features as needed.

  Scenario 3 (BR11.3): Enable motor accessibility options
    Given I am navigating the game settings menu
    When I choose accessibility options
    Then I should see options for customizable controls and input sensitivity
         adjustments
    And I should be able to configure these to suit my needs.
```

=== Feature Rationale
The game will have a wide player base, including people from all different backgrounds, which is why it is important to accommodate people with all sorts of disabilities, this will vastly boost player participation.

This feature is a *Must Have* (using the MoSCoW priorities), as having accessibility options will make the game inclusive to everyone and aligns with our commitment to diversity.

#text(
  size: 1.2em,
  heading(
    outlined: true,
    depth: 3,
    [Feature 12: Local Multiplayer Within Game],
  ),
)

*User Story*: As a player, I want to be able to play this game with someone else so that we can enjoy a shared, cooperative gaming experience locally.

```gherkin
Feature 12: Local Multiplayer Within Game

  Scenario 1 (BR12.1): Player sets up local multiplayer mode
    Given the game is launched
    And I am on the main menu
    When I select "Local Multiplayer" from the menu
    Then I should see an option to "Add Player"
    And instructions on how to connect additional controllers or share input

  Scenario 2 (BR12.2): Players join a local multiplayer session
    Given the game is in "Local Multiplayer" mode
    When a second player connects a controller or shares input
    Then the game should recognize the second player
    And display "Player 2 has joined"
    And show unique identifiers for both players (e.g., Player 1 and Player 2)

  Scenario 3 (BR12.3): Start a cooperative game session
    Given two players are connected in "Local Multiplayer" mode
    And both players have confirmed their participation
    When I select a cooperative game mode
    Then the game should load the cooperative mission
    And display a shared screen for both players
    And each player should have control of their assigned character

  Scenario 4 (BR12.4): A player leaves the multiplayer session
    Given a cooperative game session is in progress
    And both players are playing
    When "Player 2" disconnects their controller
    Then the game should notify "Player 2 has left the game"
    And the session should continue with "Player 1" in single-player mode
```

=== Feature Rationale
Local multiplayer supports cooperative gameplay, encouraging teamwork and shared problem-solving. This dynamic can make the game more appealing to players seeking a more collaborative educational experience from the game.

This feature is a *Could Have* (using the MoSCoW priorities), as local multiplayer is more challenging to implement and can be pushed back to after the singleplayer mode is finished.

//#v(1em)
//#table(columns: (3), table.header([Who], [What], [Why]),

//[Player],
//[Accessible through the web without downloading the game],
//[So I don't have to download the game to play it.],

//[IBM Skillsbuild development team],
//[Ask questions to the player about SkillsBuild],
//[To get them engaged with the courses on SkillsBuild],


//[Player],
//[Sections for multiple courses],
//[So I can choose from the courses and learn further about the one that interests me.],

//[IBM Skillsbuild development team],
//[Engaging],
//[To keep the player interested and involved in the learning process.],

//[Player],
//[I want to move around in the VR space environment],
//[So that I can explore and interact with the game world.],

//[Player],
//[I want feedback on my quiz answers],
//[So that I can learn and improve as I play.],

//[IBM Skillsbuild development team],
//[Design game sessions to last around 10-15 minutes],
//[To make the game suitable for short, engaging learning sessions.],

//[IBM ],
//[Implement a scoring system based on correct answers and completion time],
//[To add a competitive element that motivates players to improve.],

//[IBM Product development team ],
//[A user engagement and completion report],
//[To realise how many player interactions were recorded, how many of those led to completion and how to improve engagement.],

//[IBM Product development/ SkillsBuild team],
//[Include a "Do you want to send this information to IBM checkbox"],
//[To further improve how to increase user engagement and how to improve the question back.],

//[Player],
//[Be able to revisit completed sections or review quiz answers],
//[So that I can reinforce what I've learned and ensure understanding],

//[IBM SkillsBuild development team],
//[Provide accessibility options, such as text-to-speech or adjustable text size],
//[To ensure all players, including those with disabilities, can fully engage with the game],

//[IBM SkillsBuild development team],
//[Include a localised high score and time taken],
//[So that players can see how they compare to the highest score, and strive to improve],

//[Player],
//[Be able to review past questions I've answered],
//[so that I can learn from my mistakes and not repeat them],

//)


= Project Management
== Risks and issues// Recommended 1 page.
// Briefly identify and discuss any potential risks (or issues with the potential
// to become risks) that could possibly impact the project. This is a wide-ranging
// exercise and could include aspects of the group, the client, the chosen software
// development methodology, hardware, software, current systems etc. You should
// evaluate any problems that these could cause and propose appropriate
// mitigations. You should explicitly calculate (or otherwise compute) and
// prioritise risks using one of the techniques for assessing risk covered in
// lecture. Your mitigations should not be generic.
=== Risk 1: Inexperience with technologies used
_Risk calculation using 5x5 risk matrix: Probability (4) #sym.times Impact (4) = 16 (Major Risk)_

As the team is relatively new to developing VR games, the scope of the project is quite wide. This may lead to troubles during development due to the entire team having to learn to code within a new environment. Some issues raised during team meetings include: Modelling 3D assets for the game, handling the physics engine of a 3D environment, and choosing a beginner-friendly engine for development.

In order to mitigate these risks, our team has proposed the following: utilise the game engine Godot for game development. As all the VR game development engines are going to be new to our team, we decided to use the open-source Godot engine as it has a beginner-friendly language (GDScript) which is similar to Python in syntax, aiding us in development making it so we don't have to entirely learn a new language. It also offers a built in physics engine with a lot of documentation so working with it would not be as challenging. As for the 3D models, our team plans to use free open-source models found on websites such as cgtrader or turbosquid so as to not infringe on copyright nor do we have to create new models ourselves.

=== Risk 2: Undersupply of Hardware for VR Development
_Risk calculation using 5x5 risk matrix: Probability (5) #sym.times Impact(2) = 10 (Major Risk)_

Due to the nature of the game, access to VR headsets for the entire team is vital during the development process due to the need of frequent testing of features. In an ideal world, every single one of our team members would have access to a headset in order for us to be able to independently test how the code impacts the final product. However, due to a less than ideal supply, the computer science department can only offer us one headset for now, and another later in due course. One of our team members has also independently purchased a headset for both development and recreational purposes and has indicated it would be available for the team to use. Our team now worries that the undersupply of VR headsets may impede the speed of development due to some team members being unable to test the project at their leisure.

In order to mitigate these risks, our team has proposed the following: Separate team members with one side focused on developing the VR environment, and the other focusing on the logic behind the game. That way, the members working on the VR environment can use the limited VR headsets to test out VR-specific features while the members working on the logic side can still work without a headset.

=== Risk 3: Course Completion Affecting Development Time
_Risk calculation using 5x5 risk matrix: Probability (3) #sym.times Impact(2) = 6 (Moderate Risk)_

During our communications with the client, he was quite insistent on each member of the team having completed at least 2 courses (that on average take around 10 hours to finish) from the IBM SkillsBuild website to obtain questions for the game. We are worried that the time taken for us to complete all of these courses individually may conflict with the team actually developing the product.

In order to mitigate these risks, our team, during our meeting with the client, proposed a less stringent solution of only having one or two of our team members complete the courses, the client countered with all members completing at least one course instead. With this compromise, we hope that our development time is not affected too much due to this.


== Development approach// Recommended 1 page.
=== Software Development Lifecycle (SDLC) Approach: Scrum
For our project, which is a web-based VR game aimed at educating new users about IBM SkillsBuild Courses, we have selected the Scrum methodology as our SDLC approach. This choice is grounded due to the specific needs and constraints of our project scope, team structure, client requirements and organisational capabilities.

=== Key characteristics of Scrum that aligns with our project
*1.	Iterative and Incremental Development:*

Scrum allows us to break our project down into smaller, more manageable iterations called sprints, which typically lasts around 1-2 weeks longs. Each sprint focuses on delivering a potential shippable product increment. Examples of these increments which are related to our project are:
- Developing the core game mechanics (e.g. quiz answering mechanism)
- Implementing the VR features
- Adding a user interface etc.

Therefore, by the building our project incrementally, we would be able to focus on smaller tasks to ensure that each component would smoothly integrate into our larger project. Furthermore, by using this iterative approach, we would also be able to mitigate risks by allowing for early testing of certain game features to ensure that it meets the desired outcome.

*2.	Client Collaboration and Feedback:*

The success of this project hinges on delivering an engaging and educational game that aligns with IBM SkillsBuild’s educational goals. Therefore, Scrum, which emphasises regular client involvement and communication through sprint reviews and demonstrations, was chosen. For example:
- After each sprint, completed features will be presented to the client for feedback.
- This will ensure that the game aligns with IBM’s expectations which would allow us to address any deviations early on in development.

*3.	Adaptability to Changing Requirements:*

VR game development is inherently exploratory and time consuming as it involves combining software development with immersive design principles. Moreover, new requirements or priorities may be introduced by the client.
- Scrum was chosen due to its flexibility that would allow us to re-prioritise the product backlog to address certain changes in the requirements without drastically disrupting the workflow, if at all.

*4.	Team Collaboration and Accountability:*

Scrum provides a structured framework that fosters teamwork, transparency and accountability by encouraging daily standups to provide members a platform to communicate their progress, highlight any challenges a member faces as well as to seek support from other members.

*Why not Waterfall?*

The waterfall model involves completing each phase such as the requirements gathering, design, development and testing in a linear sequence. While this approach may be suitable for projects with clearly defined and unchanging requirements, it does not suit our project for several reasons:

Evolving requirements
- Our project may be subject to adjustments based on user feedback as well as client input during the development phase. The waterfall approach does not allow for easy changes once the development phase begins.
Delayed feedback
- In the waterfall approach, feedback is usually obtained at the end of the project. This creates the risk of not being able to deliver a final project that meets the client’s expectations.
== Project schedule// Recommended 1 page.
// Provide a plausible project schedule, clearly identifying academic and
// non-academic deadlines for key aspects of the project. You may also want to
// indicate the date and nature of other key milestones. This can be provided in
// the format you deem most suitable (e.g. a Gantt chart), but whatever format you
// choose should provide sufficient detail to organise the work of your team
// throughout the year. This should be easily readable and take note of the
// deadlines for the summative aspects of the project. Your schedule should go
// beyond a restatement of the academic deadlines.

In addition to following deadlines, following our chosen SDLC (scrum), we will have standup meetings twice a week during the following times:
- Wednesday (9:00 - 10:00)
- Sunday (16:00 - 17:00)
With additional time allocated every fortnight (an extra two hours) on Sunday for sprint planning and sprint review, ensuring that issues placed on our kaban board (github projects) are updated accordingly, and giving us time to reflect on what went well, and what went wrong during the sprint, and identify areas of improvement.

#image("assets/gantt_chart.png")
