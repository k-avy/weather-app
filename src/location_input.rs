use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, rc::Rc};

use dominator::{ events, html, Dom};

use crate::app_state;

pub struct LocationInput {
    pub latitude: String,
    pub longitude: String,
}

impl LocationInput {
    pub fn new() -> Self {
        Self {
            latitude: String::new(),
            longitude: String::new(),
        }
    }

    pub fn render(&self) -> Dom {
        let app_state = self.clone();
        html!("div", {
            .children(vec![
                html!("label", {
                    .text("Latitude:")
                }),
                html!("input", {
                    .attribute("type", "text")
                    .attribute("value", &self.borrow().latitude)
                    .event({
                        let app_state = app_state.clone();
                        move |evt: events::Input| {
                            app_state.borrow_mut().latitude = (evt.value()).to_string();
                        }
                    })
                }),
                html!("label", {
                    .text("Longitude:")
                }),
                html!("input", {
                    .attribute("type", "text")
                    .attribute("value", &self.borrow().longitude)
                    .event({
                        let app_state = app_state.clone();
                        move |evt: events::Input| {
                            app_state.borrow_mut().longitude = evt.value();
                        }
                    })
                }),
                html!("button", {
                    .text("Submit")
                    .event({
                        move |_| {
                            app_state::render_weather();
                        }
                    })
                }),
            ])
        })
    }
}
