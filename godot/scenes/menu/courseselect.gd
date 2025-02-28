extends Control

# CourseSelect.gd - Minimal futuristic course selection popup for IBM Skill Builds VR game
# Inspired by Paradroid C64 with spaceship theming

signal course_selected(course_name)

# Course data structure
var courses = [
	{
		"id": "AI001",
		"name": "Artificial Intelligence",
		"description": "Explore the fundamentals of AI, machine learning, and neural networks. This course covers key concepts in artificial intelligence, including supervised and unsupervised learning, neural network architectures, and practical applications in various industries.",
		"duration": "4 HOURS",
		"difficulty": 3,
		"prerequisites": "NONE"
	},
	{
		"id": "DATA002",
		"name": "Data Fundamentals",
		"description": "Learn essential data structures, analytics, and visualization techniques. Master database concepts, data cleaning, statistical analysis, and build skills in transforming raw data into actionable insights.",
		"duration": "3 HOURS",
		"difficulty": 2,
		"prerequisites": "NONE"
	},
	{
		"id": "NET003",
		"name": "Networking Basics",
		"description": "Master core networking concepts, protocols, and security practices. This course covers TCP/IP fundamentals, routing, switching, network topologies, and basic network troubleshooting skills.",
		"duration": "3.5 HOURS",
		"difficulty": 2,
		"prerequisites": "NONE"
	},
	{
		"id": "ARCH004",
		"name": "Computer Architecture",
		"description": "Understand hardware components, memory systems, and processor design. This advanced course explores CPU architecture, instruction set design, memory hierarchies, and performance optimization techniques.",
		"duration": "5 HOURS",
		"difficulty": 4,
		"prerequisites": "DATA002"
	}
]

# UI references
var course_buttons = []
var current_selected = 0
var tween

func _ready():
	# Setup visual elements and animations
	tween = get_tree().create_tween()
	
	# Get UI References
	var course_button_container = $MarginContainer/VBoxContainer/ContentContainer/CourseListPanel/MarginContainer/VBoxContainer/ScrollContainer/CourseButtonContainer
	course_buttons = [
		course_button_container.get_node("AIButton"),
		course_button_container.get_node("DataButton"),
		course_button_container.get_node("NetworkButton"),
		course_button_container.get_node("ArchButton")
	]
	
	# Connect button signals
	for i in range(course_buttons.size()):
		course_buttons[i].connect("pressed", _on_course_button_pressed.bind(i))
		course_buttons[i].connect("mouse_entered", _on_button_hover.bind(i))
		course_buttons[i].connect("mouse_exited", _on_button_exit.bind(i))
	
	# Connect menu control buttons
	$MarginContainer/VBoxContainer/ContentContainer/CourseDetailsPanel/MarginContainer/VBoxContainer/ButtonContainer/CancelButton.connect("pressed", _on_cancel_pressed)
	$MarginContainer/VBoxContainer/ContentContainer/CourseDetailsPanel/MarginContainer/VBoxContainer/ButtonContainer/StartButton.connect("pressed", _on_start_pressed)
	
	# Initial selection
	select_course(0)
	
	# Animate opening
	var panel = $BackgroundPanel
	panel.modulate.a = 0
	panel.scale = Vector2(0.95, 0.95)
	
	tween = get_tree().create_tween()
	tween.set_parallel(true)
	tween.tween_property(panel, "modulate:a", 1.0, 0.3)
	tween.tween_property(panel, "scale", Vector2(1, 1), 0.3).set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_BACK)

func select_course(index):
	if index < 0 or index >= courses.size():
		return
		
	current_selected = index
	
	# Update button states
	for i in range(course_buttons.size()):
		var is_selected = i == index
		course_buttons[i].add_theme_color_override("font_color", Color(0.15, 0.52, 0.74, 1.0) if is_selected else Color(0.89, 0.91, 0.94, 0.8))
	
	# Update details panel
	var details_panel = $MarginContainer/VBoxContainer/ContentContainer/CourseDetailsPanel/MarginContainer/VBoxContainer
	var course = courses[index]
	
	# Create fade effect
	details_panel.modulate.a = 0.7
	tween = get_tree().create_tween()
	tween.tween_property(details_panel, "modulate:a", 1.0, 0.2)
	
	# Update course info
	details_panel.get_node("CourseTitleLabel").text = course.name
	details_panel.get_node("CourseIdLabel").text = "COURSE ID: " + course.id
	details_panel.get_node("DescriptionLabel").text = course.description
	details_panel.get_node("DurationContainer/DurationValueLabel").text = course.duration
	
	# Format difficulty as bars
	var difficulty_text = ""
	for i in range(5):
		difficulty_text += "■" if i < course.difficulty else "□"
	details_panel.get_node("DifficultyContainer/DifficultyValueLabel").text = difficulty_text
	
	# Update prerequisites
	details_panel.get_node("RequirementsContainer/RequirementsValueLabel").text = course.prerequisites

func _on_course_button_pressed(index):
	select_course(index)
	
func _on_button_hover(index):
	# Visual feedback on hover
	if index != current_selected:
		course_buttons[index].add_theme_color_override("font_color", Color(0.89, 0.91, 0.94, 1.0))

func _on_button_exit(index):
	# Reset visual feedback when mouse exits
	if index != current_selected:
		course_buttons[index].add_theme_color_override("font_color", Color(0.89, 0.91, 0.94, 0.8))

func _on_cancel_pressed():
	# Animate closing
	tween = get_tree().create_tween()
	tween.set_parallel(true)
	tween.tween_property($BackgroundPanel, "modulate:a", 0.0, 0.2)
	tween.tween_property($BackgroundPanel, "scale", Vector2(0.95, 0.95), 0.2).set_ease(Tween.EASE_IN)
	tween.chain().tween_callback(queue_free)

func _on_start_pressed():
	if current_selected >= 0:
		# Emit signal with selected course
		emit_signal("course_selected", courses[current_selected].name)
		_on_cancel_pressed()

func _input(event):
	# Handle escape key to close
	if event is InputEventKey and event.pressed and event.keycode == KEY_ESCAPE:
		_on_cancel_pressed()
