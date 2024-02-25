use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::{sse::Event, Sse},
};
use tokio_stream::{Stream, StreamExt};

use crate::state::{AppState, CounterUpdate};

pub(crate) async fn sse_updates(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.update_rx.resubscribe();
    let stream = tokio_stream::wrappers::BroadcastStream::new(rx);

    Sse::new(
        stream
            .map(|msg| {
                println!("Received event {:?}", msg.clone().ok());

                Event::default()
                    .event(match msg {
                        Ok(inner) => format!(
                            "CounterUpdate-{}",
                            match inner {
                                CounterUpdate::NewValue(id, _)
                                | CounterUpdate::Deleted(id)
                                | CounterUpdate::Created(id) => id,
                            }
                        ),
                        Err(_) => "CounterError".to_owned(),
                    })
                    .data(match msg {
                        Ok(inner) => match inner {
                            CounterUpdate::NewValue(_, new_val) => format!("{new_val}"),
                            CounterUpdate::Deleted(_) | CounterUpdate::Created(_) => {
                                "0".to_string()
                            }
                        },
                        Err(e) => {
                            println!("Error in event stream {e:?}");
                            "0".to_string()
                        }
                    })
            })
            .map(Ok),
    )
    .keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(2))
            .text("keep-alive-text"),
    )
}
