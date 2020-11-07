#![recursion_limit="1024"]
#![feature(test)]

use std::collections::HashMap;

use yew::prelude::*;

const HTML: &'static str = include_str!("child_component.html");

pub enum AppMsg {}

pub struct App {
    pub data: HashMap<String, String>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut data = HashMap::new();
        data.insert("foo".into(), "bar".into());

        Self { data }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_html() }
                <hr />
                { self.view_html_include() }
                <hr />
                { self.view_html_include_internal() }
                <hr />
                { self.view_html_include_internal2() }
            </>
        }
    }
}

impl App {
    pub fn view_html(&self) -> Html {
        html! {
            <>
                <SubComponent>
                    {
                        self.data.iter()
                            .map(|(k, v)| {
                                html!{<li>{format!("{} = {}", k, v)}</li>}
                            })
                            .collect::<Html>()
                    }
                </SubComponent>
                <p>{"Hello from Yew"}</p>
            </>
        }
    }

    pub fn view_html_include_internal(&self) -> Html {
        html! { HTML }
    }

    pub fn view_html_include_internal2(&self) -> Html {
        html!{ include_str!("child_component.html") }
    }

    pub fn view_html_include(&self) -> Html {
        let div = yew::utils::document().create_element("div").unwrap();
        div.set_inner_html(HTML);
        Html::VRef(div.into())
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct SubComponentProps {
    #[prop_or_default]
    pub children: Children,
}
pub enum SubComponentMsg {}

pub struct SubComponent {
    props: SubComponentProps,
}

impl Component for SubComponent {
    type Message = SubComponentMsg;
    type Properties = SubComponentProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="box">
                <ul>
                    { self.props.children.clone() }
                </ul>
            </div>
        }
    }
}