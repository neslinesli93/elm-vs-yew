use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;
use yew::utils::window;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Vec<bool>>,
}

pub enum Msg {
    Toggle(usize, usize, bool),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let inner_height =
            (window().inner_height().unwrap().as_f64().unwrap() / 20.0).ceil() as usize;

        let inner_width =
            (window().inner_width().unwrap().as_f64().unwrap() / 20.0).floor() as usize;

        debug!("Creating a grid {:?}x{:?}", inner_height, inner_width);

        let entries = vec![vec![false; inner_width]; inner_height];
        let state = State { entries };

        App { link, state }
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
        }
        true
    }

    fn view(&self) -> Html {
        debug!("Rendered");

        html! {
            <div class="wrapper" >
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
        self.entries[i][j] = state;
    }
}
