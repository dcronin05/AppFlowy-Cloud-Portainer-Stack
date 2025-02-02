use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use super::access::AccessControl;
use crate::act::{Action, ActionVariant};
use crate::entity::ObjectType;
use crate::workspace::WorkspaceAccessControl;
use app_error::AppError;
use database_entity::dto::AFRole;

#[derive(Clone)]
pub struct WorkspaceAccessControlImpl {
  access_control: AccessControl,
}

impl WorkspaceAccessControlImpl {
  pub fn new(access_control: AccessControl) -> Self {
    Self { access_control }
  }
}

#[async_trait]
impl WorkspaceAccessControl for WorkspaceAccessControlImpl {
  async fn enforce_role(
    &self,
    uid: &i64,
    workspace_id: &str,
    role: AFRole,
  ) -> Result<(), AppError> {
    self
      .access_control
      .enforce(
        workspace_id,
        uid,
        ObjectType::Workspace(workspace_id),
        ActionVariant::FromRole(&role),
      )
      .await
  }

  async fn enforce_action(
    &self,
    uid: &i64,
    workspace_id: &str,
    action: Action,
  ) -> Result<(), AppError> {
    self
      .access_control
      .enforce(
        workspace_id,
        uid,
        ObjectType::Workspace(workspace_id),
        ActionVariant::FromAction(&action),
      )
      .await
  }

  #[instrument(level = "info", skip_all)]
  async fn insert_role(
    &self,
    uid: &i64,
    workspace_id: &Uuid,
    role: AFRole,
  ) -> Result<(), AppError> {
    self
      .access_control
      .update_policy(
        uid,
        ObjectType::Workspace(&workspace_id.to_string()),
        ActionVariant::FromRole(&role),
      )
      .await?;
    Ok(())
  }

  #[instrument(level = "info", skip_all)]
  async fn remove_user_from_workspace(
    &self,
    uid: &i64,
    workspace_id: &Uuid,
  ) -> Result<(), AppError> {
    self
      .access_control
      .remove_policy(uid, &ObjectType::Workspace(&workspace_id.to_string()))
      .await?;

    self
      .access_control
      .remove_policy(uid, &ObjectType::Collab(&workspace_id.to_string()))
      .await?;
    Ok(())
  }
}
