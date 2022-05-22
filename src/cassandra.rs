#![allow(non_snake_case)] // To rustc: no. I have my own standards for naming.

use rocket_contrib::databases::{r2d2, DbError, DatabaseConfig, Poolable};

mod cassDBA {
    use std::fmt;
    use rocket_contrib::databases::r2d2;
    #[derive(Debug)] pub struct Error;
    impl ::std::error::Error for Error {  }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { Ok(()) }
    }
    
    pub struct Connection;
    pub struct ConnectionManager;
    
    type Result<T> = ::std::result::Result<T, Error>;
    
    impl ConnectionManager {
        pub fn new(url: &str) -> Result<Self> { Err(Error) }
    }
    
    impl self::r2d2::ManageConnection for ConnectionManager {
         type Connection = Connection;
         type Error = Error;
         fn connect(&self) -> Result<Connection> { panic!() }
         fn is_valid(&self, _: &mut Connection) -> Result<()> { panic!() }
         fn has_broken(&self, _: &mut Connection) -> bool { panic!() }
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