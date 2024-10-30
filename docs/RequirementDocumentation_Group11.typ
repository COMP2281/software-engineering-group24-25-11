
#set page(numbering: "1 of 1", margin: 1.75in)
#set par(leading: 0.55em, spacing: 0.55em, justify: true)
#set text(font: "New Computer Modern")
#show raw: set text(font: "New Computer Modern Mono")
#show heading: set block(above: 1.4em, below: 1em)

#set heading(outlined: false)

= Requirements Document for IBM VR Game
==== Group 11#h(1fr)10/11/2024
#table(
  columns: (10fr, 1fr), stroke: (x, y) => if y == 0 { (bottom: 0.7pt + black) }, table.header(text(weight: "bold", [
    Name
  ]), text(weight: "bold", [CIS])), [
    Alexandre Pinheiro Dias\
    Muhammad Rafay Abbas\
    Rohab Kashif\
    Siang Wei Law\
    Jing Lei Wong
  ], [
    cgfv65\
    djsh68\
    Dwfh45\
    rwbc54\
    zlnm44
  ],
)

#set heading(numbering: "1.", outlined: true)

#outline(indent: auto)

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
users about the courses presented on *IBM SkillsBuild*.\ \
The premise of the game is to repair a broken spaceship, which requires

answering short questions (in varying difficulty) to repair each sector of the
ship.\ \

Our client is _Mr John Mc Namara_ from IBM

== Project scope// Recommended 1/2 page.
// Specify the exact project scope, indicating the project’s boundaries. This
// should also include the purpose of the software project, your overall goals, and
// how these align with the interests of your and other stakeholders. This section
// should contain your vision for your product or service and should indicate the
// exact user base of the proposed product.

== System description// Recommended 1 page.
// You should provide an overview of the system to be built. Where appropriate, you
// should first briefly detail any existing/legacy systems. You should demonstrate
// what research into alternative solutions the team has undertaken by providing a
// brief description of comparable commercial or non-commercial solutions. You
// should evaluate the usefulness. In this section, you should focus on the
// technical features of the proposed solution itself, rather than on its
// behavioural requirements. Where these have not yet been determined, note that
// this is the case.

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

== Development approach// Recommended 1 page.
// Discuss the software development lifecycle (SDLC) approach the group would like
// to use for this project. You should justify your reasons for selecting your
// approach(es) as opposed to others. Your justification should be grounded in the
// specifics of your project scope, team, client, and organisational capabilities,
// as opposed to generic claims for the effectiveness of a given approach.

== Project schedule// Recommended 1 page.
// Provide a plausible project schedule, clearly identifying academic and
// non-academic deadlines for key aspects of the project. You may also want to
// indicate the date and nature of other key milestones. This can be provided in
// the format you deem most suitable (e.g. a Gantt chart), but whatever format you
// choose should provide sufficient detail to organise the work of your team
// throughout the year. This should be easily readable and take note of the
// deadlines for the summative aspects of the project. Your schedule should go
// beyond a restatement of the academic deadlines.
