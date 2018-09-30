use failure::Error;
use ::serde::de::Deserialize;
use stdweb::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::Task;
use yew::*;

pub struct Model<Data> {
    url: Option<String>,
    ws: Option<WebSocketTask>,
    onconnect: Option<Callback<()>>,
    ondisconnect: Option<Callback<()>>,
    ondata: Option<Callback<Data>>,
}

#[allow(dead_code)]
#[allow(unused_attributes)]
pub enum Msg<Data> {
    Connected,
    Reception(Result<Data, Error>),
    Emission(Data),
    Disconnect,
    Lost,
}

#[derive(PartialEq, Clone)]
pub struct Props<Data> {
    pub url: String,
    pub onconnect: Option<Callback<()>>,
    pub ondisconnect: Option<Callback<()>>,
    pub ondata: Option<Callback<Data>>,
}

impl<Data> Default for Props<Data> {
    fn default() -> Self {
        Props {
            url: "ws://127.0.0.1:9001/".to_string(),
            onconnect: None,
            ondisconnect: None,
            ondata: None,
        }
    }
}

impl<Data> Model<Data> {
    fn connect<CTX>(&mut self, url: &str, env: &mut Env<CTX, Self>)
    where
        CTX: AsMut<WebSocketService> + 'static,
        Data: Clone + PartialEq + std::convert::TryInto<String> + 'static,
        for<'de> Data: Deserialize<'de>,
    {
        let callback = env.send_back(|Json(data)| Msg::Reception(data));
        let notification = env.send_back(|status| match status {
            WebSocketStatus::Opened => Msg::Connected,
            WebSocketStatus::Closed | WebSocketStatus::Error => Msg::Lost,
        });
        let ws_service: &mut WebSocketService = env.as_mut();
        let task = ws_service.connect(url, callback, notification);
        self.url = Some(url.to_owned());
        self.ws = Some(task);
    }
}

impl<CTX, Data> Component<CTX> for Model<Data>
where
    CTX: AsMut<WebSocketService> + 'static,
    Data: Clone + PartialEq + std::convert::TryInto<String> + 'static,
    for<'de> Data: Deserialize<'de>,
{
    type Message = Msg<Data>;
    type Properties = Props<Data>;

    fn create(props: Self::Properties, env: &mut Env<CTX, Self>) -> Self {
        let mut model = Model {
            url: None,
            ws: None,
            onconnect: props.onconnect,
            ondisconnect: props.ondisconnect,
            ondata: props.ondata,
        };

        model.connect(&props.url, env);

        model
    }

    fn change(&mut self, props: Self::Properties, env: &mut Env<CTX, Self>) -> ShouldRender {
        if let Some(ref url) = self.url {
            if *url != *props.url {
                self.connect(&props.url, env);
            }
        }
        false
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Emission(data) => {
                let data: Option<String> = data.try_into().ok();
                if let Some(data) = data {
                    self.ws.as_mut().unwrap().send(Ok(data));
                }
            }
            Msg::Connected => {
                self.onconnect.as_ref().map(|callback| callback.emit(()));
            }
            Msg::Disconnect => {
                self.ws.take().unwrap().cancel();
                self.ondisconnect.as_ref().map(|callback| callback.emit(()));
            }
            Msg::Lost => {
                self.ws = None;
                self.ondisconnect.as_ref().map(|callback| callback.emit(()));
            }
            Msg::Reception(data) => {
                if let Ok(data) = data {
                    self.ondata.as_ref().map(|callback| callback.emit(data));
                }
            }
        }
        false
    }
}

impl<CTX, Data> Renderable<CTX, Model<Data>> for Model<Data>
where
    CTX: AsMut<WebSocketService> + 'static,
    Data: Clone + PartialEq + std::convert::TryInto<String> + 'static,
    for<'de> Data: Deserialize<'de>,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div/>
        }
    }
}
