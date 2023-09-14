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
  Text string
  Link string
  IsActive bool
  IsLastItem bool
}

type NavItemProps struct {
  Text string
  Link string
  IsActive bool
}

func (n NavItemProps) ToData() navItemData {
  return navItemData{
    Text: n.Text,
    Link: n.Link,
    IsActive: n.IsActive,
  }
}


/* TOPBAR */
type topBarData struct {
  NavItemsData []navItemData
  LogoLink string
  LogoSrc string
}


/* LAYOUT */

type layoutData struct {
  Title string
  TailwindCustomStyles string
  TopBarData topBarData
  BodyData interface{}
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
  Items []ActionItemProps
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
    ActionType:      "url",
    ActionText:      a.ActionText,
    ActionLink:      a.ActionLink,
  }
}
