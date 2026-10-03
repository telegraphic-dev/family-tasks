use family_tasks::{proto, task_adapters};
use reboot::{ExternalContext, runtime::test_support::start_database};
use tokio_stream::wrappers::TcpListenerStream;
use uuid::Uuid;

async fn start_task_service(database_endpoint: &str) -> (String, tokio::task::JoinHandle<()>) {
    let (writes, reads) = task_adapters(database_endpoint).await.unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(proto::task_writes_server::TaskWritesServer::new(writes))
            .add_service(proto::task_reads_server::TaskReadsServer::new(reads))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap();
    });
    (format!("http://{address}"), server)
}

#[tokio::test]
async fn task_lifecycle_is_durable_and_create_replays() {
    let (database_endpoint, database, database_server) = start_database().await;
    let (address, server) = start_task_service(&database_endpoint).await;
    let context = ExternalContext::new("task-1");
    let create_key = Uuid::from_u128(1);
    let complete_key = Uuid::from_u128(2);
    let request = proto::CreateTaskRequest {
        household_id: "household-1".to_owned(),
        creator_id: "parent-1".to_owned(),
        title: "   ".to_owned(),
        notes: "Put bins out".to_owned(),
        due_date: "2026-10-03".to_owned(),
        assignee_id: "child-1".to_owned(),
    };
    let mut writes = proto::task_writes_client::TaskWritesClient::connect(address.clone())
        .await
        .unwrap();
    let mut reads = proto::task_reads_client::TaskReadsClient::connect(address)
        .await
        .unwrap();

    writes
        .create_task(
            context
                .writer_with_key(request.clone(), create_key)
                .unwrap(),
        )
        .await
        .unwrap();
    writes
        .create_task(context.writer_with_key(request, create_key).unwrap())
        .await
        .unwrap();
    assert_eq!(database.store_requests().len(), 1, "retry must replay");

    let details = reads
        .get_task_details(context.reader(proto::GetTaskDetailsRequest {}).unwrap())
        .await
        .unwrap()
        .into_inner();
    assert_eq!(details.household_id, "household-1");
    assert_eq!(details.title, "Untitled task");
    assert_eq!(details.status, "open");

    writes
        .complete_task(
            context
                .writer_with_key(proto::CompleteTaskRequest {}, complete_key)
                .unwrap(),
        )
        .await
        .unwrap();
    let details = reads
        .get_task_details(context.reader(proto::GetTaskDetailsRequest {}).unwrap())
        .await
        .unwrap()
        .into_inner();
    assert_eq!(details.status, "completed");

    let stores = database.store_requests();
    assert_eq!(stores.len(), 2);
    assert_eq!(
        stores[0].actor_upserts[0].state_type,
        "family_tasks.v1.TaskState"
    );

    server.abort();
    database_server.abort();
}
