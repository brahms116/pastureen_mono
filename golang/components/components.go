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

type NavItemData struct {
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

func (n NavItemProps) ToData() NavItemData {
	return NavItemData{
		Text:     n.Text,
		Link:     n.Link,
		IsActive: n.IsActive,
	}
}

/* TOPBAR */
type TopbarData struct {
	NavItemsData []NavItemData
	LogoText     string
	LogoLink     string
	LogoSrc      string
}

type TopbarProps struct {
	NavItemsProps []NavItemProps
	LogoText      string
	LogoLink      string
	LogoSrc       string
}

func (t TopbarProps) ToData() TopbarData {
	navItemsData := make([]NavItemData, len(t.NavItemsProps))
	for i, v := range t.NavItemsProps {
		navItemsData[i] = v.ToData()
		if i == len(t.NavItemsProps)-1 {
			navItemsData[i].IsLastItem = true
		}
	}
	return TopbarData{
		NavItemsData: navItemsData,
		LogoText:     t.LogoText,
		LogoLink:     t.LogoLink,
		LogoSrc:      t.LogoSrc,
	}
}

/* LAYOUT */

type LayoutData struct {
	Title               string
	StylesheetUri       string
	CustomStylesheetUri string
	TopbarData          TopbarData
	BodyData            interface{}
}

type LayoutProps struct {
	Title               string
	StylesheetUri       string
	CustomStylesheetUri string
	TopbarProps         TopbarProps
	BodyData            interface{}
}

func (l LayoutProps) ToData() LayoutData {
	return LayoutData{
		Title:               l.Title,
		StylesheetUri:       l.StylesheetUri,
		CustomStylesheetUri: l.CustomStylesheetUri,
		TopbarData:          l.TopbarProps.ToData(),
		BodyData:            l.BodyData,
	}
}

/* ACTION_MENU */

type ActionItemData struct {
	ActionType      string
	ActionText      string
	ActionLink      string
	ActionIndicator string
	ActionTarget    string
}

type ActionMenuData struct {
	ItemsData []ActionItemData
}

type ActionItemProps interface {
	ToData() ActionItemData
}

type ActionMenuProps struct {
	ItemsProps []ActionItemProps
}

func (a ActionMenuProps) ToData() ActionMenuData {
	itemsData := make([]ActionItemData, len(a.ItemsProps))
	for i, v := range a.ItemsProps {
		itemsData[i] = v.ToData()
	}
	return ActionMenuData{
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

func (a HtmxActionItemProps) ToData() ActionItemData {
	return ActionItemData{
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

func (a UrlActionItemProps) ToData() ActionItemData {
	return ActionItemData{
		ActionType: "url",
		ActionText: a.ActionText,
		ActionLink: a.ActionLink,
	}
}

/* LIST_ITEM */

type ListItemData struct {
	ImageSrc    string
	ImageAlt    string
	Link        string
	Title       string
	Subtitle    string
	ActionsData ActionMenuData
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

func (l ListItemProps) ToData() ListItemData {
	return ListItemData{
		ImageSrc:    l.ImageSrc,
		ImageAlt:    l.ImageAlt,
		Link:        l.Link,
		ActionsData: l.ActionMenuProps.ToData(),
		Title:       l.Title,
		Subtitle:    l.Subtitle,
		Tags:        l.Tags,
	}
}
