use crate::repository::{PostRepository, UserRepository};

#[derive(Clone)]
pub struct AppState {
    pub user_repository: UserRepository,
    pub post_repository: PostRepository,
}
