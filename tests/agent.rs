use consul::agent::{Agent, RegisterAgentCheck, RegisterAgentService};
use consul::{Client, Config};

#[tokio::test]
async fn test_service() {
    let config = Config::new().unwrap();
    let client = Client::new(config);
    client
        .register_service(
            &RegisterAgentService {
                Name: "test".to_string(),
                ID: "test".to_string(),
                Address: "127.0.0.1".to_string(),
                Port: 11451,
                ..Default::default()
            },
            false,
        )
        .await
        .unwrap();
    let list = client
        .agent_services(Some(r#"ID == "test""#))
        .await
        .unwrap();
    assert!(list.contains_key("test"));
    client.deregister_service("test").await.unwrap();
}

#[tokio::test]
async fn test_check() {
    let config = Config::new().unwrap();
    let client = Client::new(config);
    client
        .register_check(&RegisterAgentCheck {
            Name: "test name".to_string(),
            ID: "test".to_string(),
            Interval: "20s".to_string(),
            DeregisterCriticalServiceAfter: "50s".to_string(),
            Timeout: "10s".to_string(),
            HTTP: Some("https://baidu.com".to_string()),
            Method: Some("GET".to_string()),
            ..Default::default()
        })
        .await
        .unwrap();
    let list = client.checks().await.unwrap();
    assert!(list.contains_key("test"));
    client.deregister_check("test").await.unwrap();
}
