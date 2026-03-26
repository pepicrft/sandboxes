#[cfg(feature = "daytona")]
mod daytona_tests {
    use libsandbox::providers::daytona::DaytonaProvider;
    use libsandbox::*;

    fn provider() -> DaytonaProvider {
        let api_key =
            std::env::var("DAYTONA_API_KEY").expect("DAYTONA_API_KEY must be set for integration tests");
        let mut provider = DaytonaProvider::new(api_key);
        if let Ok(base_url) = std::env::var("DAYTONA_BASE_URL") {
            provider = provider.with_base_url(base_url);
        }
        if let Ok(org_id) = std::env::var("DAYTONA_ORGANIZATION_ID") {
            provider = provider.with_organization_id(org_id);
        }
        provider
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test --features daytona -- --ignored
    async fn test_sandbox_lifecycle() {
        let p = provider();

        // Create
        let config = SandboxConfig {
            image: Some("base".into()),
            ..Default::default()
        };
        let sandbox = p.create(config).await.expect("create failed");
        let id = sandbox.id.clone();
        println!("created sandbox: {}", id);

        // Wait for it to be ready
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        // Get
        let sandbox = p.get(&id).await.expect("get failed");
        println!("sandbox state: {:?}", sandbox.status);

        // List
        let sandboxes = p.list().await.expect("list failed");
        assert!(sandboxes.iter().any(|s| s.id == id));

        // Exec
        let result = p
            .exec(&id, Command::shell("echo hello"))
            .await
            .expect("exec failed");
        assert_eq!(result.exit_code, 0);
        assert!(result.stdout.contains("hello"));

        // File operations
        p.write_file(&id, "/tmp/test.txt", b"hello from libsandbox")
            .await
            .expect("write_file failed");

        let content = p
            .read_file(&id, "/tmp/test.txt")
            .await
            .expect("read_file failed");
        assert_eq!(content, b"hello from libsandbox");

        p.mkdir(&id, "/tmp/testdir")
            .await
            .expect("mkdir failed");

        let entries = p
            .list_dir(&id, "/tmp")
            .await
            .expect("list_dir failed");
        assert!(entries.iter().any(|e| e.name == "test.txt"));
        assert!(entries.iter().any(|e| e.name == "testdir"));

        p.delete(&id, "/tmp/test.txt")
            .await
            .expect("delete failed");

        // Stop
        p.stop(&id).await.expect("stop failed");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        // Start
        p.start(&id).await.expect("start failed");
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        // Destroy
        p.destroy(&id).await.expect("destroy failed");
        println!("sandbox destroyed");
    }

    #[tokio::test]
    #[ignore]
    async fn test_list_sandboxes() {
        let p = provider();
        let sandboxes = p.list().await.expect("list failed");
        println!("found {} sandboxes", sandboxes.len());
        for s in &sandboxes {
            println!("  {} - {:?}", s.id, s.status);
        }
    }
}
