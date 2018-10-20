use failure::Error;
use serde::de::Deserialize;
use yew::format::Json;
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::Task;
use yew::*;

pub struct Model<Data>
where
    Data: Clone + PartialEq + 'static,
    for<'de> Data: Deserialize<'de>,
    String: From<Data>,
{
    ws_service: WebSocketService,
    link: ComponentLink<Model<Data>>,
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
            url: "".to_string(),
            onconnect: None,
            ondisconnect: None,
            ondata: None,
        }
    }
}

impl<Data> Model<Data>
where
    Data: Clone + PartialEq + 'static,
    for<'de> Data: Deserialize<'de>,
    String: From<Data>,
{
    fn connect(&mut self, url: &str) {
        let link = &mut self.link;
        let callback = link.send_back(|Json(data)| Msg::Reception(data));
        let notification = link.send_back(|status| match status {
            WebSocketStatus::Opened => Msg::Connected,
            WebSocketStatus::Closed | WebSocketStatus::Error => Msg::Lost,
        });
        let task = self.ws_service.connect(url, callback, notification);
        self.url = Some(url.to_owned());
        self.ws = Some(task);
    }
}

impl<Data> Component for Model<Data>
where
    Data: Clone + PartialEq + std::convert::TryInto<String> + 'static,
    for<'de> Data: Deserialize<'de>,
    String: From<Data>,
{
    type Message = Msg<Data>;
    type Properties = Props<Data>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    where
        std::string::String: std::convert::From<Data>,
    {
        let mut model = Model {
            ws_service: WebSocketService::new(),
            link,
            url: None,
            ws: None,
            onconnect: props.onconnect,
            ondisconnect: props.ondisconnect,
            ondata: props.ondata,
        };

        model.connect(&props.url);

        model
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender
    where
        std::string::String: std::convert::From<Data>,
    {
        if let Some(ref url) = self.url {
            if *url != *props.url {
                self.connect(&props.url);
            }
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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

impl<Data> Renderable<Model<Data>> for Model<Data>
where
    Data: Clone + PartialEq + std::convert::TryInto<String> + 'static,
    for<'de> Data: Deserialize<'de>,
    String: From<Data>,
{
    fn view(&self) -> Html<Self> {
        html! {
            <div/>
        }
    }
}
