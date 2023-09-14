package components

import (
	"embed"
	"html/template"
)

/* TEMPALTES */

//go:embed templates/*.html
var f embed.FS

func GetTemplate() template.Template {
	t := template.Must(template.ParseFS(f, "templates/*.html"))
	return *t
}

/* NAV_ITEMS */
type navItemData struct {
	Text       string
	Link       string
	IsActive   bool
	IsLastItem bool
}

type NavItemProps struct {
	Text     string
	Link     string
	IsActive bool
}

func (n NavItemProps) ToData() navItemData {
	return navItemData{
		Text:     n.Text,
		Link:     n.Link,
		IsActive: n.IsActive,
	}
}

/* TOPBAR */
type topbarData struct {
	NavItemsData []navItemData
	LogoText     string
	LogoLink     string
	LogoSrc      string
}

type TopbarProps struct {
	NavItemsProps []NavItemProps
	LogoText      string
	LogoLink     string
	LogoSrc       string
}

func (t TopbarProps) ToData() topbarData {
	navItemsData := make([]navItemData, len(t.NavItemsProps))
	for i, v := range t.NavItemsProps {
		navItemsData[i] = v.ToData()
		if i == len(t.NavItemsProps)-1 {
			navItemsData[i].IsLastItem = true
		}
	}
	return topbarData{
		NavItemsData: navItemsData,
		LogoText:     t.LogoText,
		LogoLink:     t.LogoLink,
		LogoSrc:      t.LogoSrc,
	}
}

/* LAYOUT */

type layoutData struct {
	Title                string
	TailwindCustomStyles string
	TopbarData           topbarData
	BodyData             interface{}
}

type LayoutProps struct {
	Title                string
	TailwindCustomStyles string
	TopbarProps          TopbarProps
	BodyData             interface{}
}

func (l LayoutProps) ToData() layoutData {
	return layoutData{
		Title:                l.Title,
		TailwindCustomStyles: l.TailwindCustomStyles,
		TopbarData:           l.TopbarProps.ToData(),
		BodyData:             l.BodyData,
	}
}

/* ACTION_MENU */

type actionMenuItemData struct {
	ActionType      string
	ActionText      string
	ActionLink      string
	ActionIndicator string
	ActionTarget    string
}

type actionMenuData struct {
	ItemsData []actionMenuItemData
}

type ActionItemProps interface {
	ToData() actionMenuItemData
}

type ActionMenuProps struct {
	ItemsProps []ActionItemProps
}

func (a ActionMenuProps) ToData() actionMenuData {
	itemsData := make([]actionMenuItemData, len(a.ItemsProps))
	for i, v := range a.ItemsProps {
		itemsData[i] = v.ToData()
	}
	return actionMenuData{
		ItemsData: itemsData,
	}
}

/* HTMX_ACTION_ITEM */

type HtmxActionItemProps struct {
	ActionText      string
	ActionIndicator string
	ActionTarget    string
	ActionLink      string
}

func (a HtmxActionItemProps) ToData() actionMenuItemData {
	return actionMenuItemData{
		ActionType:      "htmx",
		ActionText:      a.ActionText,
		ActionIndicator: a.ActionIndicator,
		ActionTarget:    a.ActionTarget,
		ActionLink:      a.ActionLink,
	}
}

/* URL_ACTION_ITEM */

type UrlActionItemProps struct {
	ActionLink string
	ActionText string
}

func (a UrlActionItemProps) ToData() actionMenuItemData {
	return actionMenuItemData{
		ActionType: "url",
		ActionText: a.ActionText,
		ActionLink: a.ActionLink,
	}
}

/* LIST_ITEM */

type listItemData struct {
	ImageSrc    string
	ImageAlt    string
	Link        string
	Title       string
	Subtitle    string
	ActionsData []actionMenuItemData
	Tags        []string
}

type ListItemProps struct {
	ImageSrc        string
	ImageAlt        string
	Link            string
	ActionMenuProps ActionMenuProps
	Title           string
	Subtitle        string
	Tags            []string
}

func (l ListItemProps) ToData() listItemData {
	actionsData := make([]actionMenuItemData, len(l.ActionMenuProps.ItemsProps))
	for i, v := range l.ActionMenuProps.ItemsProps {
		actionsData[i] = v.ToData()
	}
	return listItemData{
		ImageSrc:    l.ImageSrc,
		ImageAlt:    l.ImageAlt,
		Link:        l.Link,
		ActionsData: actionsData,
		Title:       l.Title,
		Subtitle:    l.Subtitle,
	}
}
