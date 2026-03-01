pub mod ddl;
pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

pub use handler::routes;
#[allow(unused_imports)]
pub use model::Segundominio;
pub use repository::{SegundominioRepository, SegundominioRepositoryImpl};
pub use service::{SegundominioService, SegundominioServiceImpl};
