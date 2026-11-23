mod repository;
mod service;
mod model;
mod dto;


pub use service::{create_new_role, delete_role, update_role_info, update_role_permissions};
pub use dto::{CreateNewRoleDto, ListRolesDto, UpdateRoleInfoDto, UpdateRolePermissionsDto};
pub use repository::{list_roles, fetch_role, create_role, check_role_exists};
