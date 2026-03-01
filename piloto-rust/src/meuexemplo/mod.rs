pub mod ddl;
pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

pub use handler::routes;
#[allow(unused_imports)]
pub use model::Meuexemplo;
pub use repository::{MeuexemploRepository, MeuexemploRepositoryImpl};
pub use service::{MeuexemploService, MeuexemploServiceImpl};
