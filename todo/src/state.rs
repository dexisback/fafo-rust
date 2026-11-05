//creating the shared application state
//new for db:
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    // pub todos: Arc<RwLock<HashMap<Uuid, Todo>>>,
    pub db: PgPool,
}

//defining a method for the appstate struct
// impl AppState {
//     pub fn new() -> Self {
//         Self {
//             todos: Arc::new(RwLock::new(HashMap::new())),
//         }
//     }
// }
//PgPool already handles shared access to database connections internally. It's designed to be cheaply cloned and shared across handlers.


//arc allows sharing ownership of the same state across multiple handlers
//hashmap stores todos indexed by UUID
//arc: allows multiple parts of the application to share ownership of the same state
//rwLock: allows concurrent readers, but writes nahi (similar to go)
//appstate: our application wide state, which Axum will make available to handlers
