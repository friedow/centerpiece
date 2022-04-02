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

func selectPreviousRow(optionList *gtk.ListBox) {
	selectedRowIndex := optionList.GetSelectedRow().GetIndex()
	previousRow := optionList.GetRowAtIndex(selectedRowIndex - 1)
	optionList.SelectRow(previousRow)
}

func selectNextRow(optionList *gtk.ListBox) {
	selectedRowIndex := optionList.GetSelectedRow().GetIndex()
	nextRow := optionList.GetRowAtIndex(selectedRowIndex + 1)
	optionList.SelectRow(nextRow)
}

func setHeader(row *gtk.ListBoxRow, before *gtk.ListBoxRow) {

}

func OnOptionListKeyPress(optionList *gtk.ListBox, event *gdk.Event) {
	key := gdk.EventKeyNewFromEvent(event)

	if key.KeyVal() == gdk.KEY_Up {
		selectPreviousRow(optionList)
		return
	}

	if key.KeyVal() == gdk.KEY_Down {
		selectNextRow(optionList)
		return
	}

	// Propagate key_press_event to option on activate
	if key.KeyVal() == gdk.KEY_Return {
		selectedListBoxRow := optionList.GetSelectedRow()
		optionInterface, _ := selectedListBoxRow.GetChild()
		option := optionInterface.ToWidget()
		option.Event(event)
		return
	}
}
