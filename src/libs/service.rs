#[cfg(target_os = "windows")]
pub mod wsl_hosts_service {
    use service_manager::{
        ServiceInstallCtx, ServiceManager, ServiceStartCtx, ServiceUninstallCtx,
    };
    use std::env;
    use std::path::PathBuf;

    const WSL_HOSTS_SERVICE_NAME: &str = "WSL2Hosts.service.1";

    pub fn install_service() {
        let path = env::current_exe().unwrap().to_str().unwrap().to_string();
        let path = PathBuf::from(path);

        let label = WSL_HOSTS_SERVICE_NAME.parse().unwrap();

        manager()
            .install(ServiceInstallCtx {
                label,
                program: path,
                args: vec![],
            })
            .expect("Failed to install wsl service");

        start_service();
    }

    fn start_service() {
        let label = WSL_HOSTS_SERVICE_NAME.parse().unwrap();

        manager()
            .start(ServiceStartCtx { label })
            .expect("Failed to start wsl service");
    }

    pub fn delete_service() {
        let label = WSL_HOSTS_SERVICE_NAME.parse().unwrap();
        manager()
            .uninstall(ServiceUninstallCtx { label })
            .expect("wsl service failed to stop to uninstall");
    }

    fn manager() -> Box<dyn ServiceManager> {
        <dyn ServiceManager>::native().expect("Failed to detect management platform.")
    }
}
