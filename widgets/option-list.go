package widgets

import (
	"C"
	"friedow/tucan-search/plugins"

	"github.com/gotk3/gotk3/gdk"
	"github.com/gotk3/gotk3/gtk"
)
import (
	"encoding/json"
	"friedow/tucan-search/models"
	"log"
)

func OptionListNew() *gtk.ListBox {
	optionList, _ := gtk.ListBoxNew()
	optionList.SetMarginStart(8)
	optionList.SetMarginEnd(8)
	optionList.SetHeaderFunc(setHeader)

	pluginList := plugins.Plugins()
	for _, plugin := range pluginList {
		// bind the current value of plugin to the closure
		// https://go.dev/doc/faq#closures_and_goroutines
		plugin := plugin

		optionModels := plugin.GetOptionModels()
		for _, optionModel := range optionModels {
			optionModel := optionModel

			optionWidget := OptionWidgetNew(optionModel.Title, optionModel.ActionText)
			setOptionModel(optionWidget, optionModel)
			optionWidget.Connect("key_press_event", func() { plugin.OnActivate(optionModel) })

			optionList.Add(optionWidget)
		}
	}

	selectFirstRow(optionList)
	return optionList
}

func selectFirstRow(optionList *gtk.ListBox) {
	firstRow := optionList.GetRowAtIndex(0)
	if firstRow != nil {
		optionList.SelectRow(firstRow)
	}
}

func selectPreviousRow(optionList *gtk.ListBox) {
	selectedRowIndex := optionList.GetSelectedRow().GetIndex()
	if selectedRowIndex == 0 {
		return
	}

	previousRow := optionList.GetRowAtIndex(selectedRowIndex - 1)
	optionList.SelectRow(previousRow)
}

func selectNextRow(optionList *gtk.ListBox) {
	selectedRowIndex := optionList.GetSelectedRow().GetIndex()
	nextRowIndex := selectedRowIndex + 1
	if nextRowIndex == int(optionList.GetChildren().Length()) {
		return
	}

	nextRow := optionList.GetRowAtIndex(nextRowIndex)
	optionList.SelectRow(nextRow)
}

func setHeader(currentRow *gtk.ListBoxRow, previousRow *gtk.ListBoxRow) {
	if previousRow != nil && getPluginName(currentRow) == getPluginName(previousRow) {
		return
	}

	currentHeader, _ := currentRow.GetHeader()
	if currentHeader != nil {
		return
	}

	headerLabel, _ := gtk.LabelNew(getPluginName(currentRow))
	currentRow.SetHeader(headerLabel)
}

func setOptionModel(optionWidget *gtk.Box, optionModel models.OptionModel) {
	optionModelEncoded, _ := json.Marshal(optionModel)
	log.Print(string(optionModelEncoded))
	optionWidget.SetName(string(optionModelEncoded))
}

func getOptionModel(optionWidget *gtk.Widget) models.OptionModel {
	optionModelString, _ := optionWidget.GetName()

	optionModel := models.OptionModel{}
	json.Unmarshal([]byte(optionModelString), &optionModel)
	return optionModel
}

func getPluginName(row *gtk.ListBoxRow) string {
	currentOptionInterface, _ := row.GetChild()
	optionWidget := currentOptionInterface.ToWidget()
	optionModel := getOptionModel(optionWidget)
	return optionModel.PluginName
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
