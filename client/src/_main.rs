

// type Context = ();

// struct Model {
//     counter: u32,
// }

// enum Msg {
//     DoIt,
// }

// impl Component<Context> for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
//         Model { counter: 0 }
//     }

//     fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
//         match msg {
//             Msg::DoIt => {
//                 self.counter += 1;
//                 js!{alert("Hello World!")};
//                 true
//             }
//         }
//     }
// }

// impl Renderable<Context, Model> for Model {
//     fn view(&self) -> Html<Context, Self> {
//         html! {
//             <div class="button", onclick=|_| Msg::DoIt,>{
//                 format!("You clicked {} {}", self.counter,
//                     match self.counter {
//                         0 | 1 => "time",
//                         _ => "times"}
//                 )}
//             </div>
//         }
//     }
// }