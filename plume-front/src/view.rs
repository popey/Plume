use yew::prelude::*;

pub struct View;

impl Component for View {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        View
    }

    fn update(&mut self, msg: ()) -> ShouldRender {
        true
    }
}

impl Renderable<View> for View {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <header>
                    <h1 contenteditable=true,>
                        {"Title:"}
                    </h1>
                </header>
                <div style="background: #DADADA; min-height: 80vh;", contenteditable="",>
                </div>
            </div>
        }
    }
}
