use std::{sync::Arc, thread};

use futures::{SinkExt, StreamExt};

use pharos::*;
use tokio::sync::Mutex;

// here we put a pharos object on our struct
//
struct Goddess {
    pharos: Pharos<GoddessEvent>,
}

impl Goddess {
    fn new() -> Self {
        Self {
            pharos: Pharos::default(),
        }
    }

    // Send Goddess sailing so she can tweet about it!
    //
    pub async fn sail(&mut self) {
        // It's infallible. Observers that error will be dropped, since the only kind of errors on
        // channels are when the channel is closed.
        //
        let _ = self.pharos.send(GoddessEvent { a: "1".to_string() }).await;
    }
}

// Event types need to implement clone, but you can wrap them in Arc if not. Also they will be
// cloned, so if you will have several observers and big event data, putting them in an Arc is
// definitely best. It has no benefit to put a simple dataless enum in an Arc though.
//
#[derive(Clone, Debug, PartialEq)]
//
pub struct GoddessEvent {
    a: String,
}

// This is the needed implementation of Observable. We might one day have a derive for this,
// but it's not so interesting, since you always have to point it to your pharos object,
// and when you want to be observable over several types of events, you might want to keep
// pharos in a hashmap over type_id, and a derive would quickly become a mess.
//
impl Observable<GoddessEvent> for Goddess {
    type Error = PharErr;

    fn observe(
        &mut self,
        options: ObserveConfig<GoddessEvent>,
    ) -> Observe<'_, GoddessEvent, Self::Error> {
        self.pharos.observe(options)
    }
}

#[tokio::main]
async fn main() {
    let isis = Arc::new(Mutex::new(Goddess::new()));
    let s2 = isis.clone();
    tokio::task::spawn(a(s2));
    // trigger an eve

    for _ in 0..100 {
        let s1 = isis.clone();
        tokio::task::spawn(async move {
            s1.lock().await.sail().await;
        });
        thread::sleep(std::time::Duration::from_nanos(10));
    }
}
async fn a(s2: Arc<Mutex<Goddess>>) {
    let mut events = s2
        .lock()
        .await
        .observe(Channel::Unbounded.into())
        .await
        .expect("observe");
    let mut index = 0;
    while let _event = events.next().await.unwrap() {
        println!("{}", index);
        index += 1;
    }
}
