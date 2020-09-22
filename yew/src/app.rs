use crate::utils;
use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;
use yew::services::resize::{ResizeTask, WindowDimensions};
use yew::services::ResizeService;

const DIV_SIZE: f64 = 20.0;

#[allow(dead_code)]
pub struct App {
    link: ComponentLink<Self>,
    state: State,
    resize: ResizeTask,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Vec<bool>>,
}

pub enum Msg {
    Toggle(usize, usize, bool),
    Resize(WindowDimensions),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let root_element = utils::get_root_element().get_bounding_client_rect();
        let inner_height = (root_element.height() / DIV_SIZE).ceil() as usize;
        let inner_width = (root_element.width() / DIV_SIZE).floor() as usize;

        debug!("Creating a grid {:?}x{:?}", inner_height, inner_width);

        let entries = vec![vec![false; inner_width]; inner_height];
        let state = State { entries };

        let resize_callback = link.callback(Self::Message::Resize);

        App {
            link,
            state,
            resize: ResizeService::new().register(resize_callback),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        debug!("Change");

        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        debug!("Update");
        match msg {
            Msg::Toggle(i, j, state) => {
                self.state.toggle(i, j, state);
            }
            Msg::Resize(_) => {
                self.state.resize();
            }
        }
        true
    }

    fn view(&self) -> Html {
        debug!("Rendered");

        html! {
            <div class="grid-wrapper" >
                {
                    for self.state.entries.iter().enumerate().map(|(i, row)| {
                        html! {
                            <div class="row" key=format!("row-{}", i)>
                                {
                                    for row.iter().enumerate().map(|(j, cell)| self.view_cell(i, j, *cell))
                                }
                            </div>
                        }
                    })
                }
            </div>
        }
    }
}

impl App {
    fn view_cell(&self, i: usize, j: usize, cell: bool) -> Html {
        html! {
            <div key=format!("cell-{}-{}", i, j)
                 class=if cell {
                    "cell cell--active"
                 } else {
                    "cell"
                 }
                 onmouseenter=self.link.callback(move |_| Msg::Toggle(i, j, true))
                 onmouseleave=self.link.callback(move |_| Msg::Toggle(i, j, false))>
            </div>
        }
    }
}

impl State {
    fn toggle(&mut self, i: usize, j: usize, state: bool) {
        debug!("Toggle");
        self.entries[i][j] = state;
    }

    fn resize(&mut self) {
        debug!("Resize");

        let root_element = utils::get_root_element().get_bounding_client_rect();
        let inner_height = (root_element.height() / DIV_SIZE).ceil() as usize;
        let inner_width = (root_element.width() / DIV_SIZE).floor() as usize;

        debug!("Creating a grid {:?}x{:?}", inner_height, inner_width);

        let entries = vec![vec![false; inner_width]; inner_height];
        self.entries = entries;
    }
}
