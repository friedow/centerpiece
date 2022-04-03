package plugins

import (
	"friedow/tucan-search/models"
)

type Plugin interface {
	GetName() string
	GetOptionModels() []models.OptionModel
	OnActivate(optionModel models.OptionModel)
}

func Plugins() []Plugin {
	return []Plugin{
		GitRepositoriesPlugin{},
	}
}
