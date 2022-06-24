package models

type Option interface {
	OnActivate()
	IsVisible(queryPart string) bool
}
