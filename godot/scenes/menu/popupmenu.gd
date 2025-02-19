extends Control

func _on_ai_course_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/game/world.tscn")

func _on_network_course_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/game/world.tscn")

func _on_course_3_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/game/world.tscn")

func _on_course_4_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/game/world.tscn")

func _on_exit_pressed() -> void:
	get_tree().quit()


func _input(_event):
	if Input.is_action_just_pressed("Show popup"):
		var popup = $MarginContainer/PopupPanel  # Adjust the path to your popup panel node if needed.
		if popup.visible:
			popup.hide()
		else:
			popup.popup_centered()
