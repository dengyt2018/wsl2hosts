pub mod wsl_task_scheduler {
    use crate::libs::{time_rfc3339, Mode};
    use planif::enums::TaskCreationFlags;
    use planif::schedule_builder::{Action, ScheduleBuilder};
    use std::env;
    use windows::core::BSTR;
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED, VARIANT,
    };
    use windows::Win32::System::TaskScheduler::{ITaskService, TaskScheduler};

    const WSL_HOSTS: &str = "wsl2hosts";

    pub fn create_task_scheduler() -> Result<(), Box<dyn std::error::Error>> {
        let binary_path = env::current_exe().unwrap();
        let binary_path = binary_path.to_str().unwrap();

        let sb = ScheduleBuilder::new().unwrap();

        sb.create_logon()
            .author(whoami::username().as_str())?
            .description("WSL ip to Hosts")?
            .trigger(WSL_HOSTS, true)?
            .action(Action::new("wsl2hosts_action", binary_path, "", ""))?
            .start_boundary(time_rfc3339().as_str())?
            .user_id("")?
            .build()?
            .register(WSL_HOSTS, TaskCreationFlags::CreateOrUpdate as i32)?;
        Ok(())
    }

    pub fn task_scheduler(status: Mode) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let task_service: ITaskService = create_task_service()?;

            let root_folder = task_service.GetFolder(&Default::default())?;

            let name = BSTR::from(WSL_HOSTS);

            let registered_task = root_folder.GetTask(&name)?;

            match status {
                Mode::Run => {
                    registered_task.Run(VARIANT::default())?;
                }
                Mode::Stop => {
                    registered_task.Stop(0)?;
                }
                Mode::Remove => {
                    root_folder.DeleteTask(&name, 0)?;
                }
                _ => {}
            }
            Ok(())
        }
    }

    fn create_task_service() -> Result<ITaskService, Box<dyn std::error::Error>> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED)?;

            let task_service: ITaskService = CoCreateInstance(&TaskScheduler, None, CLSCTX_ALL)?;
            task_service.Connect(
                VARIANT::default(),
                VARIANT::default(),
                VARIANT::default(),
                VARIANT::default(),
            )?;
            Ok(task_service)
        }
    }
}
