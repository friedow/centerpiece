package apis

type cache interface {
}

func newCache() *cache {
	this := cache{}

	return &this
}
