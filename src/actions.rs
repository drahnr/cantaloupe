use actix::prelude::*;
use postgres::{Connection, TlsMode};
use rand::{thread_rng, Rng, ThreadRng};
use std::io;

use models::{Fortune, World};

/// Postgres interface
pub struct PgConnection {
    conn: Connection,
    rng: ThreadRng,
}

impl Actor for PgConnection {
    type Context = SyncContext<Self>;
}

impl PgConnection {
    pub fn new(db_url: &str) -> PgConnection {
        let conn = Connection::connect(db_url, TlsMode::None)
            .expect(&format!("Error connecting to {}", db_url));
        PgConnection {
            conn,
            rng: thread_rng(),
        }
    }
}

unsafe impl Send for PgConnection {}


pub struct UpdateWorld(pub u16);

impl Message for UpdateWorld {
    type Result = io::Result<Vec<World>>;
}

impl Handler<UpdateWorld> for PgConnection {
    type Result = io::Result<Vec<World>>;

    fn handle(&mut self, msg: UpdateWorld, _: &mut Self::Context) -> Self::Result {
        let get_world = self
            .conn
            .prepare_cached("SELECT id FROM world WHERE id=$1")
            .unwrap();
        let mut update = String::with_capacity(120 + 6 * msg.0 as usize);
        update
            .push_str("UPDATE world SET randomnumber = temp.randomnumber FROM (VALUES ");

        let mut worlds = Vec::with_capacity(msg.0 as usize);
        for _ in 0..msg.0 {
            let random_id = self.rng.gen_range::<i32>(1, 10_000);
            let rows = &get_world.query(&[&random_id]).unwrap();
            let w = World {
                id: rows.get(0).get(0),
                randomnumber: self.rng.gen_range(1, 10_000),
            };
            update.push_str(&format!("({}, {}),", w.id, w.randomnumber));
            worlds.push(w);
        }
        worlds.sort_by_key(|w| w.id);

        update.pop();
        update
            .push_str(" ORDER BY 1) AS temp(id, randomnumber) WHERE temp.id = world.id");
        self.conn.execute(&update, &[]).unwrap();

        Ok(worlds)
    }
}




#[cfg(test)]
mod test {
    
}