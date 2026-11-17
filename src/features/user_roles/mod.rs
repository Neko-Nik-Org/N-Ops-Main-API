mod repository;
mod service;
mod model;
mod dto;


pub use service::{create_new_role, delete_role};
pub use dto::{CreateNewRoleDto, ListRolesDto};
pub use repository::{list_roles, fetch_role};
