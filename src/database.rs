pub mod connection {
    use crate::api;
    use crate::api::database::{
        requests::{
            DeleteConnectionRequest, ListConnectionsRequest, PostgreSQLConnectionRequest,
            PostgreSQLConnectionRequestBuilder, ReadConnectionRequest, ResetConnectionRequest,
            RotateRootRequest,
        },
        responses::{ListConnectionsResponse, ReadConnectionResponse},
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Creates or updates a PostgreSQL connection
    ///
    /// See [PostgreSQLConnectionRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn postgres(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut PostgreSQLConnectionRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = PostgreSQLConnectionRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty_result(client, endpoint).await
    }

    /// Deletes a connection
    ///
    /// See [DeleteConnectionRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteConnectionRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all connections
    ///
    /// See [ListConnectionsRequest]
    #[instrument(skip(client), err)]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<ListConnectionsResponse, ClientError> {
        let endpoint = ListConnectionsRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads a connection
    ///
    /// See [ReadConnectionRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<ReadConnectionResponse, ClientError> {
        let endpoint = ReadConnectionRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reset a connection
    ///
    /// See [ResetConnectionRequest]
    #[instrument(skip(client), err)]
    pub async fn reset(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = ResetConnectionRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Rotates the root account configured in a connection
    ///
    /// See [RotateRootRequest]
    #[instrument(skip(client), err)]
    pub async fn rotate(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = RotateRootRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod role {
    use crate::api;
    use crate::api::database::{
        requests::{
            DeleteRoleRequest, GenerateCredentialsRequest, ListRolesRequest, ReadRoleRequest,
            SetRoleRequest, SetRoleRequestBuilder,
        },
        responses::{GenerateCredentialsResponse, ListRolesResponse, ReadRoleResponse},
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Generates credentials from a role
    ///
    /// See [GenerateCredentialsRequest]
    #[instrument(skip(client), err)]
    pub async fn creds(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<GenerateCredentialsResponse, ClientError> {
        let endpoint = GenerateCredentialsRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Deletes a role
    ///
    /// See [DeleteRoleRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all roles
    ///
    /// See [ListRolesRequest]
    #[instrument(skip(client), err)]
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads a role
    ///
    /// See [ReadRoleRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<ReadRoleResponse, ClientError> {
        let endpoint = ReadRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates or updates a role
    ///
    /// See [SetRoleRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut SetRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod static_role {
    use crate::api;
    use crate::api::database::{
        requests::{
            DeleteStaticRoleRequest, GetStaticCredentialsRequest, ListStaticRolesRequest,
            ReadStaticRoleRequest, RotateStaticRoleRequest, SetStaticRoleRequest,
            SetStaticRoleRequestBuilder,
        },
        responses::{
            GetStaticCredentialsResponse, ListStaticRolesResponse, ReadStaticRoleResponse,
        },
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Generates credentials from a static role
    ///
    /// See [GetStaticCredentialsRequest]
    #[instrument(skip(client), err)]
    pub async fn creds(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<GetStaticCredentialsResponse, ClientError> {
        let endpoint = GetStaticCredentialsRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Deletes a static role
    ///
    /// See [DeleteStaticRoleRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteStaticRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all static roles
    ///
    /// See [ListStaticRolesRequest]
    #[instrument(skip(client), err)]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<ListStaticRolesResponse, ClientError> {
        let endpoint = ListStaticRolesRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads a static role
    ///
    /// See [ReadStaticRoleRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<ReadStaticRoleResponse, ClientError> {
        let endpoint = ReadStaticRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Rotates the credentials associated with a static role
    ///
    /// See [RotateStaticRoleRequest]
    #[instrument(skip(client), err)]
    pub async fn rotate(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = RotateStaticRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Creates or updates a static role
    ///
    /// See [SetStaticRoleRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut SetStaticRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetStaticRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
