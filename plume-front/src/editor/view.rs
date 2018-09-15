use yew::prelude::*;

use editor::block::{Block, BlockKind};
use editor::title::Title;

pub struct View;

impl Component for View {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        View
    }

    fn update(&mut self, _: ()) -> ShouldRender {
        true
    }
}

impl Renderable<View> for View {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <header class="flex",>
                    <a href="/dashboard",>
                        <i class="icon icon-home",></i>
                    </a>
                    <div class="grow",></div>
                    <p class="status",><i class="icon icon-check",></i>{ "Saved" }</p>
                    <a>{ "Publish" }</a>
                </header>
                <main>
                    <Block: show_placeholder=true, placeholder=Some("Title"), kind=BlockKind::Header(1, String::new()), />
                    <Block: show_placeholder=true, has_menu=true,/>
                    // <Block: kind=BlockKind::Header, />
                    <Block: kind=BlockKind::Image(String::from("https://baptiste.gelez.xyz/static/media/FFB741B3-CDA1-90B7-0D65-57EC32AECC76.jpg")), />
                    // <Block: kind=BlockKind::Video, />
                    // <Block: kind=BlockKind::Audio, />
                    // <Block: kind=BlockKind::Separator, />
                    // <Block: kind=BlockKind::Embed, />
                    // <Block: kind=BlockKind::Code, />
                    // <Block: kind=BlockKind::Quote, />
                </main>
            </div>
        }
    }
}
