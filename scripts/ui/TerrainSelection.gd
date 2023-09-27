extends OptionButton

func on_file_selected(path):
	UiGenData.heightmap_path = path
	$"../../RichTextLabel2".text = "Heightmap: %s" % [UiGenData.heightmap_path]
	$"../../RichTextLabel2".visible = true
func win_popup():
	$"../../../../../FileDialog".popup_centered(Vector2i(700,300))
	$"../../../../../FileDialog".file_selected.connect(on_file_selected)
	
func on_pressed_heightmap():
	$"../../HeightmapSelect".pressed.connect(win_popup)
	$"../../HeightmapSelect".visible = true
func _ready():
	add_items()
	self.item_selected.connect(on_selected_element)
	
func add_items():
	self.add_item("Random", 0)
	self.add_item("Heightmap", 1)

func on_selected_element(index):
	match index:
		0:
			UiGenData.heightmap_path = "Random"
		1:
			on_pressed_heightmap()
			
			
