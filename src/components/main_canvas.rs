use yew::prelude::*;
//use yew::html::ComponentLink;

// Pointer to roster, which is only one for the app.
use std::rc::Rc;
use std::cell::RefCell;

// A common definition for all messages:
use crate::shared_messages::SharedMessage;

// For browser debugging
use web_sys::console;

// Using the Roster as a model for the canvas
use crate::models::roster::{Roster, RosterElement};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub roster: Rc<RefCell<Roster>>,
    pub on_roster_updated: Callback<()>,
    pub is_dark_mode: bool,
}

pub struct MainCanvas {
    props: Props,
    tooltip_visible: bool,
    tooltip_content: Option<Html>,
    tooltip_x: i32,
    tooltip_y: i32,}

impl Component for MainCanvas {
    type Message = SharedMessage;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        MainCanvas {
            props: ctx.props().clone(),
            tooltip_visible: false,
            tooltip_content: None,
            tooltip_x: 0,
            tooltip_y: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SharedMessage::NotifyRosterUpdated => {
                console::log_1(&"Roster updated notification received in MAIN CANVAS".into());
                true
            }

            SharedMessage::DeleteElement(index) => {
                let mut roster = self.props.roster.borrow_mut();
                if index < roster.elements.len() {
                    roster.elements.remove(index);
                    self.props.on_roster_updated.emit(());
                }
                self.tooltip_visible = false; 
                true
            }

            SharedMessage::ShowTooltip(index) => {
                let roster = self.props.roster.borrow();
                if let Some(elem) = roster.elements.get(index) {
                    self.tooltip_content = Some(self.get_tooltip_content(ctx, elem, index));
                    self.tooltip_visible = true;
                }
                true
            }

            SharedMessage::MoveTooltip(x, y) => {
                self.tooltip_x = x;
                self.tooltip_y = y;
                true
            }

            SharedMessage::HideTooltip => {
                self.tooltip_visible = false;
                true
            }
            
            _ => panic!("Wrong message received!")
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let roster = self.props.roster.borrow();
        let total_points: u32 = roster.elements.iter()
            .map(|elem| self.get_element_points(elem)).sum();

        html! {
            <div class="central-area">
                <div class={if total_points > 60 { "total-points over-limit" } else { "total-points" }}>
                    { format!("Total Points: {}", total_points) }
                </div>
                {
                    for roster.elements.iter().enumerate().map(|(i, elem)| {

                        // Preparing a couple of variables for the conditional below.
                        let image_path = self.get_image(elem);
                        console::log_1(&format!("ImagePath is: {:?}", image_path).into());
                        let image_class = if {ctx.props().is_dark_mode} && {image_path == "character.png" || image_path == "support.png"} {
                            "inverted-roster-image"
                        } else {
                            "roster-image"
                        };

                        html!{
                            <div class="hoverable-area"
                                 onmouseover={ctx.link().callback(move |_| SharedMessage::ShowTooltip(i))}
                                 onmousemove={ctx.link().callback(move |e: MouseEvent| SharedMessage::MoveTooltip(e.client_x(), e.client_y()))}
                                 onmouseout={ctx.link().callback(|_| SharedMessage::HideTooltip)}
                                 ondblclick={ctx.link().callback(move |_| SharedMessage::DeleteElement(i))}>
                                <div class="content-container">
                                    { self.get_element_name(elem) }
                                    <img src={format!("./static/images/{}", image_path)} class={image_class} />
                                    <div class="points-label">{ if self.get_element_points(elem) > 1 {
                                            format!("{} Points", self.get_element_points(elem))
                                        }
                                        else {
                                            "1 Point".to_string()
                                        }}
                                    </div>
                                </div>
                            </div>
                        }
                    })
                }
                {
                    if self.tooltip_visible {
                        html! {
                            <div class="tooltip" style={format!("left: {}px; top: {}px;", self.tooltip_x, self.tooltip_y)}>
                                { self.tooltip_content.clone().unwrap_or_default() }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }

    fn changed(&mut self, _: &Context<Self>, new_props: &Self::Properties) -> bool {
        let old_elements = &self.props.roster.borrow().elements.clone();
        let new_elements = &new_props.roster.borrow().elements.clone();
        
        self.props = new_props.clone();
        true
    }
}

impl MainCanvas {
    // Simple rendering of the various elements of the roster.
    fn get_element_name(&self, elem: &RosterElement) -> String {
        match elem {
            RosterElement::ElemCharacter(character) => format!("Character: {:?}", character.name),
            RosterElement::ElemUnit(unit) => format!("Unit: {:?}", unit.name),
            RosterElement::ElemSupport(support) => format!("Support: {:?}", support.name),
            RosterElement::ElemOther((name, _ ,_)) => format!("{}", name),
        }
    }

    fn get_tooltip_content(&self, _ctx: &Context<Self>, elem: &RosterElement, _index: usize) -> Html {
        html! {
            <>
                { format!("Details about: {}", self.get_element_name(elem)) }
                <div>{ "Double click to delete" }</div>
            </>
        }
    }

    fn get_element_points(&self, elem: &RosterElement) -> u32 {
        match elem {
            RosterElement::ElemCharacter(character) => character.points,
            RosterElement::ElemUnit(unit) => unit.points,
            RosterElement::ElemSupport(support) => support.points,
            RosterElement::ElemOther((_, value, _)) => *value,
        }
    }
    
    fn get_image(&self, elem: &RosterElement) -> String {
        match elem {
            RosterElement::ElemCharacter(_) => "character.png".to_string(), // TODO TBR Unused
            RosterElement::ElemUnit(unit) => unit.image.clone(), // TODO TBR Unused
            RosterElement::ElemSupport(_) => "support.png".to_string(),// TODO TBR Unused
            RosterElement::ElemOther((_, _, image)) => image.clone(),
        }
    }

}
