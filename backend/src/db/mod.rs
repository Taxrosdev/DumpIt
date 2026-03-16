pub mod model;
pub mod schema;
use model::*;
use schema::*;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use nanoid::nanoid;
use std::env;

pub struct Database {
    pool: DbPool,
}

#[derive(diesel::MultiConnection)]
enum DbConnection {
    Pg(PgConnection),
    Sqlite(SqliteConnection),
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type DbPool = diesel::r2d2::Pool<ConnectionManager<DbConnection>>;

fn establish_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<DbConnection>::new(database_url);

    diesel::r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create the database connection pool")
}

impl Database {
    pub fn init() -> Self {
        let pool = establish_pool();
        let mut conn = pool.get().expect("Could not get connection to database");

        conn.run_pending_migrations(MIGRATIONS)
            .expect("Could not perform SQL migrations");

        Database {
            pool: establish_pool(),
        }
    }

    /// Returns a new id
    pub fn create_upload(&self, hash: String, size: i32, filename: String) -> String {
        let id = nanoid!();

        let upload = Upload {
            id: id.clone(),
            hash,
            size,
            filename,
            timestamp: chrono::Utc::now().timestamp(),
        };

        let mut conn = self.pool.get().expect("Connection pool timeout");

        diesel::insert_into(uploads::table)
            .values(&upload)
            .execute(&mut conn)
            .expect("Error saving new post");

        id
    }

    pub fn get_upload(&self, id: &str) -> Result<Option<Upload>, diesel::result::Error> {
        use schema::uploads::dsl::uploads;

        let mut conn = self.pool.get().unwrap();

        uploads
            .find(id)
            .select(Upload::as_select())
            .first(&mut conn)
            .optional()
    }
}
