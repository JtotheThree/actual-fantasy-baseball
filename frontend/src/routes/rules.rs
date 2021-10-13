use yew::prelude::*;

pub struct Rules;

impl Component for Rules {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="px-4">
            <div
              class="
                flex
                justify-center
                items-center
                mx-auto
                max-w-2xl
                my-16
                p-16
                border-2
                border-black
                font-raleway
              "
            >
               <h1>{ "These are the damn RULES" }</h1>
            </div>
            </div>
        }
    }
}