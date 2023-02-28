pub mod wsl_hosts_service {
    use log::{debug, error, info};
    use std::env;
    use std::ffi::OsString;

    use crate::libs::service::wsl_hosts_service::start_service::start_service;
    use crate::libs::utils::decode_output;
    use std::process::Command;
    use windows_service::service::{
        ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType,
    };
    use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

    const WSL_HOSTS_SERVICE_NAME: &str = "wsl2hosts";
    const WSL_HOSTS_SERVICE_DISPLAY_NAME: &str = "WSL ip to Hosts File";

    pub fn install_service() {
        debug!("Start install service");
        let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access);

        match service_manager {
            Ok(service) => {
                let service_binary_path = env::current_exe().unwrap();

                let service_info = ServiceInfo {
                    name: OsString::from(WSL_HOSTS_SERVICE_NAME),
                    display_name: OsString::from(WSL_HOSTS_SERVICE_DISPLAY_NAME),
                    service_type: ServiceType::OWN_PROCESS,
                    start_type: ServiceStartType::AutoStart,
                    error_control: ServiceErrorControl::Normal,
                    executable_path: service_binary_path,
                    launch_arguments: vec![],
                    dependencies: vec![],
                    account_name: None, // None means run as System
                    account_password: None,
                };

                match service.create_service(&service_info, ServiceAccess::CHANGE_CONFIG) {
                    Ok(srv) => {
                        if srv
                        .set_description(
                            "A program that set wsl ip to windows hosts file as windows service.",
                        )
                        .is_ok()
                    {
                        info!("Install {} service success", WSL_HOSTS_SERVICE_NAME);
                        start_service();
                    } else {
                            error!("Set description failed");
                        }
                    }
                    Err(e) => {
                        error!("Create {} service failed {}", WSL_HOSTS_SERVICE_NAME, e);
                    }
                }
            }
            Err(e) => {
                error!("Create service manager failed {}", e);
            }
        }
        debug!("Finish install service");
    }

    pub mod start_service {
        use crate::libs::hosts::parse_hosts::parse_hosts;
        use crate::libs::service::wsl_hosts_service::WSL_HOSTS_SERVICE_NAME;
        use crate::libs::utils::decode_output;
        use log::{debug, error};
        use std::ffi::OsString;
        use std::process::Command;
        use std::sync::mpsc;
        use std::time::Duration;
        use windows_service::service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        };
        use windows_service::service_control_handler::ServiceControlHandlerResult;
        use windows_service::{
            define_windows_service, service_control_handler, service_dispatcher, Result,
        };

        pub fn run() -> Result<()> {
            service_dispatcher::start(WSL_HOSTS_SERVICE_NAME, ffi_service_main)
        }

        define_windows_service!(ffi_service_main, wls2hosts_service_main);

        pub fn wls2hosts_service_main(_arguments: Vec<OsString>) {
            if let Err(e) = run_service() {
                error!(
                    "Running {} Service Handle error {}",
                    WSL_HOSTS_SERVICE_NAME, e
                );
            }
        }

        pub fn run_service() -> Result<()> {
            debug!("Create service mpsc::channel");
            let (shutdown_tx, shutdown_rx) = mpsc::channel();

            let event_handler = move |control_event| -> ServiceControlHandlerResult {
                match control_event {
                    ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

                    // Handle stop
                    ServiceControl::Stop => {
                        shutdown_tx.send(()).unwrap();
                        debug!("Send Service Control Stop");
                        ServiceControlHandlerResult::NoError
                    }
                    _ => ServiceControlHandlerResult::NotImplemented,
                }
            };

            debug!("Register service Handle");
            let status_handle =
                service_control_handler::register(WSL_HOSTS_SERVICE_NAME, event_handler)?;

            status_handle.set_service_status(ServiceStatus {
                service_type: ServiceType::OWN_PROCESS,
                current_state: ServiceState::Running,
                controls_accepted: ServiceControlAccept::STOP,
                exit_code: ServiceExitCode::Win32(0),
                checkpoint: 0,
                wait_hint: Duration::default(),
                process_id: None,
            })?;

            loop {
                debug!("Service Start parse hosts in loop");
                parse_hosts();

                // Poll shutdown event.
                match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
                    // Break the loop either upon stop or channel disconnect
                    Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

                    // Continue work if no events were received within the timeout
                    Err(mpsc::RecvTimeoutError::Timeout) => (),
                };
            }

            // Tell the system that service has stopped.
            status_handle.set_service_status(ServiceStatus {
                service_type: ServiceType::OWN_PROCESS,
                current_state: ServiceState::Stopped,
                controls_accepted: ServiceControlAccept::empty(),
                exit_code: ServiceExitCode::Win32(0),
                checkpoint: 0,
                wait_hint: Duration::default(),
                process_id: None,
            })?;

            Ok(())
        }

        pub fn start_service() {
            if let Ok(stdout) = Command::new("net")
                .args(["start", WSL_HOSTS_SERVICE_NAME])
                .output()
            {
                let output = decode_output(&stdout.stdout);
                if !output.contains("successfully") {
                    eprintln!("start service failed {}", &output);
                } else {
                    println!("{}", &output);
                }
            }
        }
    }

    /*    pub fn delete_service() -> windows_service::Result<()> {
        use std::thread::sleep;
        use std::time::{Duration, Instant};
        use windows_service::service::ServiceState

        let manager_access = ServiceManagerAccess::CONNECT;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

        let service_access =
            ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
        let service = service_manager.open_service(WSL_HOSTS_SERVICE_NAME, service_access)?;

        service.delete()?;

        if service.query_status()?.current_state != ServiceState::Stopped {
            service.stop()?;
        }

        drop(service);

        let start = Instant::now();
        let timeout = Duration::from_secs(5);
        while start.elapsed() < timeout {
            if let Err(windows_service::Error::Winapi(e)) =
                service_manager.open_service(WSL_HOSTS_SERVICE_NAME, ServiceAccess::QUERY_STATUS)
            {
                // windows_sys::Win32::Foundation::ERROR_SERVICE_DOES_NOT_EXIST;
                if e.raw_os_error() == Some(1060_i32) {
                    println!("wsl2hosts service is deleted.");
                    return Ok(());
                }
            }
            sleep(Duration::from_secs(1));
        }
        println!("wsl2hosts service is marked for deletion.");

        Ok(())
    }*/

    pub fn delete_service() {
        let stdout = Command::new("sc")
            .args(["delete", WSL_HOSTS_SERVICE_NAME])
            .output()
            .expect("execute sc delete failed.");

        let output = decode_output(&stdout.stdout);

        if !output.contains("SUCCESS") {
            eprintln!("delete service failed {}", &output);
        } else {
            eprintln!("delete service success");
        }
    }
}
