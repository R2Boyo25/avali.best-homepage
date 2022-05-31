#![allow(non_snake_case)] // To rustc: no. I have my own standards for naming.

use rocket_contrib::databases::{r2d2, DbError, DatabaseConfig, Poolable};

mod cassDBA {
    use cassandra_cpp;
    use std::fmt;
    use rocket_contrib::databases::r2d2;
    #[derive(Debug)] pub struct Error;
    impl ::std::error::Error for Error {  }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { Ok(()) }
    }
    pub struct Connection {
        connection: cassandra_cpp::Session
    }
    pub struct ConnectionManager {
        url: String
    }
    
    type Result<T> = ::std::result::Result<T, Error>;
    
    impl ConnectionManager {
        pub fn new(url: &str) -> Result<Self> { 
            return Ok(ConnectionManager{url: url.to_string()});
        }
    }
    
    impl self::r2d2::ManageConnection for ConnectionManager {
        type Connection = Connection;
        type Error = Error;
        fn connect(&self) -> Result<Connection> { 
            let mut cluster = cassandra_cpp::Cluster::default();
            cluster.set_contact_points(&self.url);
            return Ok(Connection{connection: cluster.connect().unwrap()});
        }
        fn is_valid(&self, _: &mut Connection) -> Result<()> { Ok(()) }
        fn has_broken(&self, _: &mut Connection) -> bool { false }
    }
}

impl Poolable for cassDBA::Connection {
    type Manager = cassDBA::ConnectionManager;
    type Error = DbError<cassDBA::Error>;

    fn pool(config: DatabaseConfig) -> Result<r2d2::Pool<Self::Manager>, Self::Error> {
        let manager = cassDBA::ConnectionManager::new(config.url)
            .map_err(DbError::Custom)?;

        r2d2::Pool::builder()
            .max_size(config.pool_size)
            .build(manager)
            .map_err(DbError::PoolError)
    }
}