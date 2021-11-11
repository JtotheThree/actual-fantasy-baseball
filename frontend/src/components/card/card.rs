use yew::prelude::*;
use yew::{utils, App};

pub struct Card {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(None)]
    pub header: Option<Html>,
    #[prop_or(None)]
    pub subheader: Option<Html>,
    #[prop_or(None)]
    pub body: Option<Html>,
    #[prop_or(None)]
    pub footer: Option<Html>,
    #[prop_or_default]
    pub class_name: String,
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div
                class="flex flex-col mx-auto shadow border-gray-700 w-full"
            >
                {get_content(
                    self.props.header.clone(),
                    self.props.subheader.clone(),
                    self.props.body.clone(),
                    self.props.footer.clone(),
                )}
            </div>
        }
    }
}

fn get_content(
    header: Option<Html>,
    subheader: Option<Html>,
    body: Option<Html>,
    footer: Option<Html>,
) -> Html {
    html! {
        <>
        // We could do an image here I guess??
        {
            // HEADER
            if let Some(header) = header {
                html! {
                    <div class="py-1 text-xl font-bold text-center bg-gray-700 text-paper">{ header }</div>
                }
            } else {
                html! {}
            }
        }
        {
            // SUBHEADER
            if let Some(subheader) = subheader {
                html!{
                    <div class="mb-3 text-sm text-gray-500">{ subheader }</div>
                } 
            } else { html! {} }
        }
        <div class="px-4 py-2">
        {
            // BODY
            if let Some(body) = body {
                html!{
                    <div class="text-base text-gray-700">{ body }</div>
                } 
            } else { html! {} }
        }
        </div>
        {
            // FOOTER
            if let Some(footer) = footer {
                html!{
                    <div class="mx-4 mt-2 mb-4">{ footer }</div>
                } 
            } else { html! {} }
        }
        </>
    }
}
