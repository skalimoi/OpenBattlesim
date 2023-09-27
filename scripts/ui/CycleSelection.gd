extends LineEdit

func on_selected_cycles(cycles):
	UiGenData.erosion_cycles = cycles

func _ready():
	self.text_submitted.connect(on_selected_cycles)
