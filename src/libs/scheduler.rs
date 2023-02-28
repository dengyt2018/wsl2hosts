#[allow(unused, unused_imports)]

pub mod wsl_task_scheduler {
    use planif::enums::TaskCreationFlags;
    use planif::schedule_builder::{Action, ScheduleBuilder};
    use std::env;

    const WSL_HOSTS: &str = "wsl2hosts";

    pub fn create_task_scheduler() -> Result<(), Box<dyn std::error::Error>> {
        let binary_path = env::current_exe().unwrap();

        let sb = ScheduleBuilder::new().unwrap();
        sb.create_logon()
            .author(whoami::username().as_str())?
            .description("")?
            .trigger(WSL_HOSTS, true)?
            .action(Action::new(
                WSL_HOSTS,
                binary_path.into_os_string().to_str().unwrap(),
                "",
                "",
            ))?
            .start_boundary("2022-04-28T02:14:08.660633427+00:00")?
            .user_id("")?
            .build()?
            .register(WSL_HOSTS, TaskCreationFlags::CreateOrUpdate as i32)?;

        Ok(())
    }
}
