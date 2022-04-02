package components

import (
	"C"
	"friedow/tucan-search/plugins"

	"github.com/gotk3/gotk3/gdk"
	"github.com/gotk3/gotk3/gtk"
)

func OptionListNew() *gtk.ListBox {
	optionList, _ := gtk.ListBoxNew()
	optionList.SetMarginStart(8)
	optionList.SetMarginEnd(8)
	optionList.SetHeaderFunc(setHeader)

	optionList.Connect("key_press_event", onOptionListKeyPress)

	pluginList := plugins.Plugins()
	for _, plugin := range pluginList {

		options := plugin.GetOptions()
		for _, option := range options {

			option.Connect("key_press_event", plugin.OnKeyPressed)
			optionList.Add(option)
		}
	}

	return optionList
}

func setHeader(row *gtk.ListBoxRow, before *gtk.ListBoxRow) {

}

func onOptionListKeyPress(optionList *gtk.ListBox, event *gdk.Event) {
	key := gdk.EventKeyNewFromEvent(event)

	if key.KeyVal() == gdk.KEY_Down || key.KeyVal() == gdk.KEY_Up {
		return
	}

	// emit event on option
	selectedOption := optionList.GetSelectedRow()
	iBox, _ := selectedOption.GetChild()
	box := iBox.ToWidget()
	box.Event(event)
}
