use yew::prelude::*;

pub struct Button {
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
    pub text: String,
}

pub enum Msg {
    Clicked,
}

impl Component for Button {
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
            <button
                onclick=self.link.callback(|_| Msg::Clicked)
                class="align-self-end btn py-1 px-8 bg-gray-700 text-paper rounded-sm border-b-4 border-paper font-bold hover:bg-red-800 float-right">
                { self.props.text.clone() }
            </button>
        }
    }
}