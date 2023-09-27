extends LineEdit

func on_selected_seed(seed):
	UiGenData.seed = seed

func _ready():
	self.text_submitted.connect(on_selected_seed)
