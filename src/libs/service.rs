#![allow(unused, dead_code, unused_variables, unused_mut, unused_imports)]

pub mod wsl_hosts_service {
    use std::path::Path;
    use std::thread::sleep;
    use std::{env, time};
    use windows::core::PCSTR;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Security::SC_HANDLE;
    use windows::Win32::System::Services::{
        CloseServiceHandle, CreateServiceA, DeleteService, OpenSCManagerA, OpenServiceA,
        QueryServiceStatusEx, StartServiceA, SC_MANAGER_ALL_ACCESS, SC_STATUS_PROCESS_INFO,
        SERVICE_ALL_ACCESS, SERVICE_AUTO_START, SERVICE_DEMAND_START, SERVICE_ERROR_NORMAL,
        SERVICE_NOTIFY_DELETED, SERVICE_RUNNING, SERVICE_WIN32_OWN_PROCESS,
    };

    const WSL_HOSTS_SERVICE_NAME: &str = "WSL2Hosts";

    fn install_service() {
        unsafe {
            let path = env::current_exe().unwrap().to_str().unwrap().to_string();
            let path = path.as_ptr().to_owned();
            let service_name = PCSTR::from_raw(WSL_HOSTS_SERVICE_NAME.as_ptr());

            let mut sch_scmanager = OpenSCManagerA(None, None, SC_MANAGER_ALL_ACCESS).unwrap();
            let mut sch_service = CreateServiceA(
                sch_scmanager,
                service_name,
                service_name,
                SERVICE_ALL_ACCESS,
                SERVICE_WIN32_OWN_PROCESS,
                SERVICE_AUTO_START,
                SERVICE_ERROR_NORMAL,
                PCSTR::from_raw(path),
                None,
                None,
                None,
                None,
                None,
            );

            if let Ok(s) = sch_service {
                CloseServiceHandle(s);
            }

            CloseServiceHandle(sch_scmanager);
        }
    }

    fn start_service() {
        unsafe {
            let service_name = PCSTR::from_raw(WSL_HOSTS_SERVICE_NAME.as_ptr());

            let mut sch_scmanager = OpenSCManagerA(None, None, SC_MANAGER_ALL_ACCESS).unwrap();
            let mut sch_service = OpenServiceA(sch_scmanager, service_name, SERVICE_ALL_ACCESS);

            if let Ok(s) = sch_service {
                StartServiceA(s, None);
            }

            CloseServiceHandle(sch_scmanager);
        }
    }

    fn delete_service() {
        unsafe {
            let service_name = PCSTR::from_raw(WSL_HOSTS_SERVICE_NAME.as_ptr());

            let mut sch_scmanager = OpenSCManagerA(None, None, SC_MANAGER_ALL_ACCESS).unwrap();
            let mut sch_service = OpenServiceA(sch_scmanager, service_name, SERVICE_ALL_ACCESS);

            if let Ok(s) = sch_service {
                DeleteService(s);
                CloseServiceHandle(s);
            }

            CloseServiceHandle(sch_scmanager);
        }
    }

    #[test]
    fn test_service() {
        install_service();
        start_service();
    }
}
