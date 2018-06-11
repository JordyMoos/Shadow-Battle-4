#[macro_use] extern crate actix;
use std::time::Duration;
use actix::prelude::*;

#[derive(Message)]
struct Ping {
    pub id: usize,
}

struct Game {
    counter: usize,
    addr: Recipient<Unsync, Ping>,
}

impl Actor for Game {
    type Context = Context<Game>;
}

impl Handler<Ping> for Game {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        self.counter += 1;

        if self.counter > 10 {
            Arbiter::system().do_send(actix::msgs::SystemExit(0));
        } else {
            println!("Ping received {:?}", msg.id);

            ctx.run_later(Duration::new(0, 100_000_000), move |act, _| {
                act.addr.do_send(Ping { id: msg.id + 1 });
            });
        }
    }
}

fn main() {
    let system = System::new("test");

    let _: Addr<Unsync, _> = Game::create(|ctx| {
        let addr: Addr<Unsync, _> = ctx.address();
        let addr2: Addr<Unsync, _> = Game {
            counter: 0,
            addr: addr.recipient()
        }.start();

        addr2.do_send(Ping { id: 10 });

        Game {
            counter: 0,
            addr: addr2.recipient(),
        }
    });

    system.run();
}
