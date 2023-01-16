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
        <div class="hero is-fullheight">
          <div class="hero-body">
            <div class="columns is-vcentered is-centered">
              <div class="column is-one-third">
                <div class="card">
                  <div class="card-image">
                    <figure class="image is-3by3">
                      //<img alt="A random image for the input term 'yew'." src="https://source.unsplash.com/random/1200x400/?yew" />
                      <img alt="A random image for the input term 'yew'." src="https://imgs.search.brave.com/9cqoc4CqCRk_13shWTC-IGt23GdlSXmTnH8DWlwOgzo/rs:fit:1200:1200:1/g:ce/aHR0cDovL3MzLmFt/YXpvbmF3cy5jb20v/bXR2LW1haW4tYXNz/ZXRzL2ZpbGVzL3Jl/c291cmNlcy9nZW9y/Z2Utb24taG9yc2Vi/YWNrLXdlYi5qcGc" />
                    </figure>
                  </div>
                  <div class="card-content">
                    <div class="media">
                      <div class="media-left">
                        <figure class="image is-48x48">
                          <img src="https://imgs.search.brave.com/QKRkrJUJpF55xBW0Q1gieuTv5gG2xCi8EfQG1BfMR9I/rs:fit:1200:1200:1/g:ce/aHR0cHM6Ly92ZWN0/b3JpZmllZC5jb20v/aW1hZ2Uvc2hpYmEt/aW51LXZlY3Rvci0x/My5wbmc" alt="Placeholder image" />
                        </figure>
                      </div>
                      <div class="media-content">
                        <p class="title is-4">{"Doge Washington"}</p>
                        <p class="subtitle is-6">{"@dogewashinton"}</p>
                      </div>
                    </div>

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
