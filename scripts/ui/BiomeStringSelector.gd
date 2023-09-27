extends LineEdit

func on_submitted(string):
	UiGenData.biome = string

func _ready():
	self.text_submitted.connect(on_submitted)
