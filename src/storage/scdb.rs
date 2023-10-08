use scylla::{Session, SessionBuilder};




pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn create_session(uri: &str) -> Result<Session>{
    print!("{}",uri);
    SessionBuilder::new()
        .known_node(uri)
        .build()
        .await
        .map_err(From::from)
}

static CREATE_DB: &str = r#"
  CREATE KEYSPACE IF NOT EXISTS nereid_connect
    WITH REPLICATION = {
      'class': 'SimpleStrategy',
      'replication_factor': 1
    };
"#;



static CREATE_USERS_TABLE_QUERY: &str = r#"
  CREATE TABLE IF NOT EXISTS nereid_connect.users (
    user_id UUID,
    first_name text,
    last_name text,
    email text,
    password_hash blob,
    role text,
    time timestamp,
    PRIMARY KEY(user_id)
  );
"#;


pub async fn create_scylladb(session: &Session) -> Result<()> {
    session
        .query(CREATE_DB, ())
        .await
        .map(|_| ())
        .map_err(From::from)
}


pub async fn initialize_scylladb(session: &Session) -> Result<()> {
    print!("{}",CREATE_USERS_TABLE_QUERY);
    session
        .query(CREATE_USERS_TABLE_QUERY, ())
        .await
        .map(|_| ())
        .map_err(From::from)
}


async fn create_user_table(session: &Session) -> Result<()> {
    session
        .query(CREATE_USERS_TABLE_QUERY, ())
        .await
        .map(|_| ())
        .map_err(From::from)
}



