//! This module contains the core application fabric for the wallet, including
//! the model, events, and effects that drive the application.

use crux_core::render::Render;
use crux_http::Http;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::capabilities::sse::ServerSentEvents;
use crate::capabilities::store::{Store, StoreEntry, StoreError};
use crate::model::credential::CredentialState;
use crate::model::{Count, Model};
use crate::view::ViewModel;

const API_URL: &str = "https://crux-counter.fly.dev";

/// Aspect of the application.
///
/// This allows the UI navigation to be reactive: controlled in response to the
/// user's actions.
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Aspect {
    /// Display and deletion of credentials stored in the wallet.
    #[default]
    CredentialList,

    /// Display of a single credential.
    CredentialDetail,

    /// Trigger a credential issuance using an offer QR code.
    IssuanceScan,

    /// Trigger a credential verification using a presentation request QR code.
    PresentationScan,
}

/// Events that can be sent to the wallet application.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    /// Event emitted by the shell when the app first loads.
    Ready,

    /// Event emitted by the core when the store capability has loaded
    /// credentials.
    #[serde(skip)]
    CredentialsLoaded(Result<Vec<StoreEntry>, StoreError>),

    /// Event emitted by the core when the store capability has stored a
    /// credential.
    #[serde(skip)]
    CredentialStored(Result<(), StoreError>),

    /// Event emitted by the shell to delete a credential from the wallet.
    Delete(String),

    /// Event emitted by the core when the store capability has deleted a
    /// credential.
    #[serde(skip)]
    CredentialDeleted(Result<(), StoreError>),

    /// Event emitted by the shell to navigate to a different aspect of the app.
    Navigate(Aspect),

    /// Event emitted by the shell to select a credential for detailed display.
    Select(String),

    /// Event emitted by the shell when the user scans an offer QR code.
    CreateOffer(String),

    /// Error event is emitted by the core when an error occurs.
    #[serde(skip)]
    Error(String),

    // TODO: Remove ---
    // events from the shell
    Get,
    Increment,
    Decrement,
    StartWatch,

    // events local to the core
    #[serde(skip)]
    Set(crux_http::Result<crux_http::Response<Count>>),
    #[serde(skip)]
    Update(Count),
    // ----------------
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    pub render: Render<Event>,
    pub http: Http<Event>,
    pub sse: ServerSentEvents<Event>,
    pub store: Store<Event>,
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Capabilities = Capabilities;
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;

    fn update(&self, msg: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match msg {
            Event::Ready => {
                // Initialization event. Set the aspect to the credential list
                // view.
                model.active_view = Aspect::CredentialList;
                model.credential = CredentialState::init();
                caps.store.list("credential".into(), Event::CredentialsLoaded);
                caps.render.render();
            }
            Event::CredentialsLoaded(Ok(entries)) => {
                model.active_view = Aspect::CredentialList;
                model.credential.set_credentials(entries.clone());

                // TODO: Remove this try-out code
                // if entries.len() < 1 {
                //     let json = include_bytes!("model/credentials.json");
                //     let credentials: Vec<Credential> =
                //         serde_json::from_slice(json).expect("should deserialize");
                //     let credential = credentials[entries.len()].clone();
                //     let data = serde_json::to_vec(&credential).expect("should serialize");
                //     caps.store.save("credential".into(), credential.id.clone(), data, Event::CredentialStored);
                // }
                // -------------------------------

                caps.render.render();
            }
            Event::CredentialsLoaded(Err(error)) => {
                model.error = Some(error.to_string());
                caps.render.render();
            }
            Event::CredentialStored(Ok(())) => {
                caps.store.list("credential".into(), Event::CredentialsLoaded);
            }
            Event::CredentialStored(Err(error)) => {
                model.error = Some(error.to_string());
                caps.render.render();
            }
            Event::Navigate(aspect) => {
                model.active_view = aspect;
                caps.render.render();
            }
            Event::Select(id) => {
                model.active_view = Aspect::CredentialDetail;
                model.credential.id = Some(id);
                caps.render.render();
            }
            Event::Delete(id) => {
                caps.store.delete("credential".into(), id, Event::CredentialDeleted);
            }
            Event::CredentialDeleted(Ok(())) => {
                model.credential.id = None;
                caps.store.list("credential".into(), Event::CredentialsLoaded);
                caps.render.render();
            }
            Event::CredentialDeleted(Err(error)) => {
                model.error = Some(error.to_string());
                caps.render.render();
            }
            Event::CreateOffer(_encoded_offer) => {
                caps.render.render();
            }
            Event::Error(e) => {
                model.error = Some(e);
                caps.render.render();
            }

            // TODO: Remove ---
            Event::Get => {
                caps.http.get(API_URL).expect_json().send(Event::Set);
            }
            Event::Set(Ok(mut response)) => {
                let count = response.take_body().unwrap();
                self.update(Event::Update(count), model, caps);
            }
            Event::Set(Err(e)) => {
                panic!("Oh no something went wrong: {e:?}");
            }
            Event::Update(count) => {
                model.count = count;
                caps.render.render();
            }
            Event::Increment => {
                // optimistic update
                model.count = Count {
                    value: model.count.value + 1,
                    updated_at: None,
                };
                caps.render.render();

                // real update
                let base = Url::parse(API_URL).unwrap();
                let url = base.join("/inc").unwrap();
                caps.http.post(url).expect_json().send(Event::Set);
            }
            Event::Decrement => {
                model.count = Count {
                    value: model.count.value - 1,
                    updated_at: None,
                };
                caps.render.render();

                // real update
                let base = Url::parse(API_URL).unwrap();
                let url = base.join("/dec").unwrap();
                caps.http.post(url).expect_json().send(Event::Set);
            }
            Event::StartWatch => {
                let base = Url::parse(API_URL).unwrap();
                let url = base.join("/sse").unwrap();
                caps.sse.get_json(url, Event::Update);
            } // ----------------
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        // --- TODO Remove ---
        let suffix = match model.count.updated_at {
            None => " (pending)".to_string(),
            Some(d) => format!(" ({d})"),
        };
        // --------------------

        Self::ViewModel {
            active_view: model.active_view.clone(),
            credential_view: model.credential.clone().into(),

            // --- TODO: Remove ---
            text: model.count.value.to_string() + &suffix,
            confirmed: model.count.updated_at.is_some(),
            // --------------------
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_let_bind::assert_let;
    use chrono::{TimeZone, Utc};
    use crux_core::assert_effect;
    use crux_core::testing::AppTester;
    use crux_http::protocol::{HttpRequest, HttpResponse, HttpResult};
    use crux_http::testing::ResponseBuilder;

    use super::{App, Event, Model};
    use crate::capabilities::sse::SseRequest;
    use crate::model::{Count, CredentialState};
    use crate::{Aspect, Effect};

    // ANCHOR: simple_tests
    /// Test that a `Get` event causes the app to fetch the current
    /// counter value from the web API
    #[test]
    fn get_counter() {
        // instantiate our app via the test harness, which gives us access to the model
        let app = AppTester::<App, _>::default();

        // set up our initial model
        let mut model = Model::default();

        // send a `Get` event to the app
        let mut update = app.update(Event::Get, &mut model);

        // check that the app emitted an HTTP request,
        // capturing the request in the process
        assert_let!(Effect::Http(request), &mut update.effects[0]);

        // check that the request is a GET to the correct URL
        let actual = &request.operation;
        let expected = &HttpRequest::get("https://crux-counter.fly.dev/").build();
        assert_eq!(actual, expected);

        // resolve the request with a simulated response from the web API
        let response =
            HttpResponse::ok().body(r#"{ "value": 1, "updated_at": 1672531200000 }"#).build();
        let update = app.resolve(request, HttpResult::Ok(response)).expect("an update");

        // check that the app emitted an (internal) event to update the model
        let actual = update.events;
        let expected = vec![Event::Set(Ok(ResponseBuilder::ok()
            .body(Count {
                value: 1,
                updated_at: Some(Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
            })
            .build()))];
        assert_eq!(actual, expected);
    }

    /// Test that a `Set` event causes the app to update the model
    #[test]
    fn set_counter() {
        // instantiate our app via the test harness, which gives us access to the model
        let app = AppTester::<App, _>::default();

        // set up our initial model
        let mut model = Model::default();

        // send a `Set` event (containing the HTTP response) to the app
        let update = app.update(
            Event::Set(Ok(ResponseBuilder::ok()
                .body(Count {
                    value: 1,
                    updated_at: Some(Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
                })
                .build())),
            &mut model,
        );

        // check that the app asked the shell to render
        assert_effect!(update, Effect::Render(_));

        // check that the view has been updated correctly
        insta::assert_yaml_snapshot!("set_counter", app.view(&model));
    }
    // ANCHOR_END: simple_tests

    // Test that an `Increment` event causes the app to increment the counter
    #[test]
    fn increment_counter() {
        // instantiate our app via the test harness, which gives us access to the model
        let app = AppTester::<App, _>::default();

        // set up our initial model as though we've previously fetched the counter
        let mut model = Model {
            active_view: Aspect::CredentialList,
            credential: CredentialState {
                id: None,
                credentials: vec![],
            },
            issuance: None,
            error: None,
            count: Count {
                value: 1,
                updated_at: Some(Utc.with_ymd_and_hms(2022, 12, 31, 23, 59, 0).unwrap()),
            },
        };

        // send an `Increment` event to the app
        let mut update = app.update(Event::Increment, &mut model);

        // check that the app asked the shell to render
        assert_effect!(update, Effect::Render(_));

        // we are expecting our model to be updated "optimistically" before the
        // HTTP request completes, so the value should have been updated
        // but not the timestamp
        insta::assert_yaml_snapshot!("increment_counter_optimistic", model);

        // check that the app also emitted an HTTP request,
        // capturing the request in the process
        assert_let!(Effect::Http(request), &mut update.effects[1]);

        // check that the request is a POST to the correct URL
        let actual = &request.operation;
        let expected = &HttpRequest::post("https://crux-counter.fly.dev/inc").build();
        assert_eq!(actual, expected);

        // resolve the request with a simulated response from the web API
        let response =
            HttpResponse::ok().body(r#"{ "value": 2, "updated_at": 1672531200000 }"#).build();
        let update = app.resolve(request, HttpResult::Ok(response)).expect("Update to succeed");

        // send the generated (internal) `Set` event back into the app
        let _ = app.update(update.events[0].clone(), &mut model);

        // check that the model has been updated correctly
        insta::assert_yaml_snapshot!("increment_counter_final", model);
    }

    /// Test that a `Decrement` event causes the app to decrement the counter
    #[test]
    fn decrement_counter() {
        // instantiate our app via the test harness, which gives us access to the model
        let app = AppTester::<App, _>::default();

        // set up our initial model as though we've previously fetched the counter
        let mut model = Model {
            active_view: Aspect::CredentialList,
            credential: CredentialState {
                id: None,
                credentials: vec![],
            },
            issuance: None,
            error: None,
            count: Count {
                value: 0,
                updated_at: Some(Utc.with_ymd_and_hms(2022, 12, 31, 23, 59, 0).unwrap()),
            },
        };

        // send a `Decrement` event to the app
        let mut update = app.update(Event::Decrement, &mut model);

        // check that the app asked the shell to render
        assert_effect!(update, Effect::Render(_));

        // we are expecting our model to be updated "optimistically" before the
        // HTTP request completes, so the value should have been updated
        // but not the timestamp
        insta::assert_yaml_snapshot!("decrement_counter_optimistic", model);

        // check that the app also emitted an HTTP request,
        // capturing the request in the process
        assert_let!(Effect::Http(request), &mut update.effects[1]);

        // check that the request is a POST to the correct URL
        let actual = &request.operation;
        let expected = &HttpRequest::post("https://crux-counter.fly.dev/dec").build();
        assert_eq!(actual, expected);

        // resolve the request with a simulated response from the web API
        let response =
            HttpResponse::ok().body(r#"{ "value": -1, "updated_at": 1672531200000 }"#).build();
        let update = app.resolve(request, HttpResult::Ok(response)).expect("a successful update");

        // run the event loop in order to send the (internal) `Set` event
        // back into the app
        for event in update.events {
            let _ = app.update(event, &mut model);
        }

        // check that the model has been updated correctly
        insta::assert_yaml_snapshot!("decrement_counter_final", model);
    }

    #[test]
    fn get_sse() {
        let app = AppTester::<App, _>::default();
        let mut model = Model::default();

        let update = app.update(Event::StartWatch, &mut model);

        assert_let!(Effect::ServerSentEvents(request), &update.effects[0]);

        let actual = &request.operation;
        let expected = &SseRequest {
            url: "https://crux-counter.fly.dev/sse".to_string(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_sse() {
        let app = AppTester::<App, _>::default();
        let mut model = Model::default();

        let count = Count {
            value: 1,
            updated_at: Some(Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap()),
        };
        let event = Event::Update(count);

        let update = app.update(event, &mut model);

        assert_effect!(update, Effect::Render(_));

        // check that the model has been updated correctly
        insta::assert_yaml_snapshot!("set_sse", model);
    }
}
