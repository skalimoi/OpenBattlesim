@tool
class_name OnscreenOutputPlugin extends EditorPlugin

const MAIN_PANEL = preload("res://addons/onscreen_output/main_panel.tscn")

var main_panel_instance : ScrnOutputMainPanel

func _init() -> void:
	add_autoload_singleton("scrnOutput", "res://addons/onscreen_output/output.tscn")

func _enter_tree():
	main_panel_instance = MAIN_PANEL.instantiate()
	# Add the main panel to the editor's main viewport.
	get_editor_interface().get_editor_main_screen().add_child(main_panel_instance)
	# Hide the main panel. Very much required.
	_set_visible(false)
	
	# logic for showing the main scene
	main_screen_changed.connect(func(screen_name):
		if screen_name == "Onscreen Output":
			_set_visible(true)
		else:
			_set_visible(false))
	
func _exit_tree():
	if main_panel_instance:
		main_panel_instance.queue_free()

func _set_visible(visible):
	if main_panel_instance:
		main_panel_instance.visible = visible

func _get_plugin_name():
	return "Onscreen Output"
	
func _has_main_screen():
	return true
	
func _get_plugin_icon():
	return get_editor_interface().get_base_control().get_theme_icon("Node", "EditorIcons")

