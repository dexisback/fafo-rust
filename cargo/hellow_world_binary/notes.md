Cargo is the Rust package manager. It is a tool that allows Rust packages to declare their various dependencies and ensure that you’ll always get a repeatable build.


To accomplish this goal, Cargo does four things:

- Introduces two metadata files with various bits of package information.
- Fetches and builds your package’s dependencies.
- Invokes rustc or another build tool with the correct parameters to build your package.
- Introduces conventions to make working with Rust packages easier.


---
todo-app:

#[derive(Debug, Clone, Serialise)] etc etc kyu karte rehte hum?
-> these are derive macros. they automatically generate implementations of traits for your structs
->Debug: allows a value to be printed using {:?} or {:#?}
->Clone: allows explicitly creating a copy using .clone()
->Serialise : allows converting a rust value into JSON
->Deserialise: allows constructing a rust value from JSON

Why mod.rs and pub mod todo?
because rust as a module system. modules organise code into namespaces, control visibility, and define how files connect.


pub mod todo -> means there is a child module called todo. its public, so other modules can access it 

(toh hum later on use crate::models::todo:Todo) use kar sakte 

crate -> this application -> models -> todo -> Todo struct 



 typical modern rust proejct structure convention:
 src/
├── main.rs
├── lib.rs
├── config.rs
├── state.rs
└── modules/
    └── todos/
        ├── mod.rs
        ├── model.rs
        ├── handler.rs
        ├── service.rs
        └── repository.rs




Arc<RwLock<HashMap<Uuid, Todo>>> :
-> arc means automatically reference counter. it allows multiple owners to share the same underlying value

-> why? because axum needs to make application state available to multiple handlers. Arc lets us clone the ownership to multiple handlers. Arc lets us clone the ownership handle without cloning the underlying HashMap

