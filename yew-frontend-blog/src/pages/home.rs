use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                    <div class="tile">
                    //    <div class="tile is-child container">
                    //         <figure class="image is-3by1">
                    //             <img alt="A random image for the input term 'yew'." src="https://source.unsplash.com/random/1200x400/?yew" />
                    //         </figure>
                    //     </div>

                    //     <div class="tile is-child container" >
                    //         { self.view_info_tiles() }
                    //     </div>
                    // </div>
                    <div class="card">
          <div class="card-image">
            <figure class="image is-4by3">
              <img src="https://bulma.io/images/placeholders/1280x960.png" alt="Placeholder image" />
            </figure>
          </div>
          <div class="card-content">
            <div class="media">
              <div class="media-left">
                <figure class="image is-48x48">
                  <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image" />
                </figure>
              </div>
              <div class="media-content">
                <p class="title is-4">{"John Smith"}</p>
                <p class="subtitle is-6">{"@johnsmith"}</p>
              </div>
            </div>

            <div class="content">
              {"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
              Phasellus nec iaculis mauris. <a>@bulmaio</a>."}
              <br/>
              <time datetime="2016-1-1">{"11:09 PM - 1 Jan 2016"}</time>
            </div>
          </div>
        </div>
        </div>
        </div>
                }
    }
}
impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <>
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "Who am I?" }</p>
                        <div class="content">
                            {r#"
                            With over a decade of experience in the field of software development, 
                            I specialize in full-stack development with a focus on building scalable and efficient systems. 
                            I am a problem solver at heart who enjoys tackling complex challenges and am always looking to improve upon the status quo. 
                            I am passionate about technology and am constantly learning and experimenting with new tools and techniques.
                            "#}
                            <br/>
                            <br/>
                            {r#"
                            I am a team player and enjoy collaborating with others to build great products. When I'm not coding, I like to read
                            and enjoy the outdoors. 
                            "#}
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
