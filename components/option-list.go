package components

import (
	"friedow/tucan-search/plugins"
	"strings"

	"github.com/diamondburned/gotk4/pkg/gdk/v4"
	"github.com/diamondburned/gotk4/pkg/gtk/v4"
)

type OptionList struct {
	*gtk.ScrolledWindow

	optionList struct {
		*gtk.ListBox

		options []plugins.PluginOption
	}
}

func NewOptionList() *OptionList {
	this := OptionList{}

	this.optionList.ListBox = gtk.NewListBox()
	this.optionList.SetHeaderFunc(this.setHeader)
	this.optionList.AddCSSClass("option-list")

	this.optionList.options = plugins.PluginOptions()
	for _, option := range this.optionList.options {
		this.optionList.Append(option)
	}

	this.selectFirstRow()

	this.ScrolledWindow = gtk.NewScrolledWindow()
	this.SetChild(this.optionList)

	return &this
}

func (this *OptionList) OnKeyPress(keyVal uint) bool {
	if keyVal == gdk.KEY_Up {
		this.selectPreviousRow()
		return true
	}

	if keyVal == gdk.KEY_Down {
		this.selectNextRow()
		return true
	}

	this.optionList.InvalidateFilter()
	return false
}

func (this *OptionList) OnActivate() {

	row := this.optionList.SelectedRow()
	this.pluginOption(row).OnActivate()
}

func (this *OptionList) FilterOptions(query string) {
	preprocessedQuery := strings.ToLower(strings.Trim(query, " "))
	queryParts := strings.Split(preprocessedQuery, " ")

	for optionIndex, option := range this.optionList.options {
		row := this.optionList.RowAtIndex(optionIndex)

		this.setRowVisibility(row, option, queryParts)
	}

	this.selectFirstRow()
}

func (this *OptionList) setHeader(currentRow *gtk.ListBoxRow, previousRow *gtk.ListBoxRow) {
	currentHeader := currentRow.Header()

	if previousRow != nil && this.pluginName(currentRow) == this.pluginName(previousRow) {
		if currentHeader == nil {
			return
		}
		currentRow.SetHeader(nil)

	} else {
		if currentHeader != nil {
			return
		}

		header := gtk.NewLabel(this.pluginName(currentRow))

		headerBox := gtk.NewBox(gtk.OrientationHorizontal, 0)
		headerBox.Append(header)
		headerBox.AddCSSClass("header")
		if previousRow == nil {
			headerBox.AddCSSClass("first-header")
		}

		currentRow.SetHeader(headerBox)
	}
}

func (this *OptionList) visibleRows() []*gtk.ListBoxRow {
	visibleRows := []*gtk.ListBoxRow{}
	for optionIndex := range this.optionList.options {
		row := this.optionList.RowAtIndex(optionIndex)
		if row != nil && row.IsVisible() {
			visibleRows = append(visibleRows, row)
		}
	}
	return visibleRows
}

func (this *OptionList) visibleRowIndex(row *gtk.ListBoxRow) int {
	for visibleRowIndex, visibleRow := range this.visibleRows() {
		if this.pluginOption(visibleRow) == this.pluginOption(row) {
			return visibleRowIndex
		}
	}
	return -1
}

func (this *OptionList) selectFirstRow() {
	visibleRows := this.visibleRows()
	if len(visibleRows) <= 0 {
		return
	}

	firstRow := visibleRows[0]
	this.optionList.SelectRow(firstRow)
}

func (this *OptionList) selectPreviousRow() {
	currentRow := this.optionList.SelectedRow()
	if currentRow == nil {
		this.selectFirstRow()
		return
	}

	currentRowIndex := this.visibleRowIndex(currentRow)
	previousRowIndex := currentRowIndex - 1
	if previousRowIndex < 0 {
		return
	}
	previousRow := this.visibleRows()[previousRowIndex]
	this.optionList.SelectRow(previousRow)

	previousRow.SetCanFocus(true)
	previousRow.GrabFocus()
	previousRow.SetCanFocus(false)
}

func (this *OptionList) selectNextRow() {
	currentRow := this.optionList.SelectedRow()
	if currentRow == nil {
		this.selectFirstRow()
		return
	}

	currentRowIndex := this.visibleRowIndex(currentRow)
	nextRowIndex := currentRowIndex + 1
	if nextRowIndex >= len(this.visibleRows()) {
		return
	}
	nextRow := this.visibleRows()[nextRowIndex]
	this.optionList.SelectRow(nextRow)

	nextRow.SetCanFocus(true)
	nextRow.GrabFocus()
	nextRow.SetCanFocus(false)
}

func (this *OptionList) pluginOption(row *gtk.ListBoxRow) plugins.PluginOption {
	return this.optionList.options[row.Index()]
}

func (this *OptionList) pluginName(row *gtk.ListBoxRow) string {
	return this.pluginOption(row).PluginName()
}

func (this *OptionList) setRowVisibility(row *gtk.ListBoxRow, option plugins.PluginOption, queryParts []string) {
	for _, queryPart := range queryParts {
		if strings.Contains(strings.ToLower(option.PluginName()), queryPart) {
			row.Show()
			return
		}
		if option.IsVisible(queryPart) {
			row.Show()
			return
		}
	}
	row.Hide()
}
