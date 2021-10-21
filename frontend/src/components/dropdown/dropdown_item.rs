use yew::prelude::*;

pub struct DropdownItem {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(Callback::noop())]
    /// Click event for dropdown item
    pub onclick: Callback<String>,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// General property to add returned data
    #[prop_or_default]
    pub data: String,
    pub children: Children,
}

pub enum Msg {
    Clicked,
}

impl Component for DropdownItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onclick.emit(self.props.data.clone());
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }

        false
    }

    fn view(&self) -> Html {
        html! {
            <li class="py-1">
                <a class=self.props.class.clone() key=self.props.key.clone() onclick=self.link.callback(|_| Msg::Clicked)>
                    { self.props.children.clone() }
                </a>
            </li>
        }
    }
}

