use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::{Duration, SystemTime};

use druid::widget::{Flex, Label};
use druid::{
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, ExtEventSink, Lens, Selector,
    Target, Widget, WidgetExt, WindowDesc,
};

const TICK: Selector<String> = Selector::new("tick");

#[cfg(debug_assertions)]
pub fn resource_path(end: PathBuf) -> PathBuf {
    // Relative path for debug builds
    end
}

#[cfg(not(debug_assertions))]
pub fn resource_path(end: PathBuf) -> PathBuf {
    // Path relative to current executable for release builds
    match std::env::current_exe() {
        Ok(mut p) => {
            p.pop();
            p.join(end)
        }
        Err(_) => panic!("Can't locate current directory"),
    }
}

#[derive(Data, Lens, Clone)]
pub struct State {
    time: String,
}

struct Delegate;

impl AppDelegate<State> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut State,
        _env: &Env,
    ) -> bool {
        if let Some(time) = cmd.get(TICK) {
            data.time = time.clone();
            false
        } else {
            false
        }
    }
}

fn main() {
    let main_window = WindowDesc::new(ui_builder);
    let data = State {
        time: "".to_string(),
    };
    let launcher = AppLauncher::with_window(main_window);

    let sink = launcher.get_external_handle();
    let delegate = Delegate {};

    thread::spawn(move || {
        loop {
            // Get current unix time by calling "clock" executable
            println!("path={:?}", resource_path(PathBuf::from("resources/mac/clock")));
            let path = resource_path(PathBuf::from("resources/mac/clock"))
                .canonicalize()
                .expect("couldn't build canonical path");
            let output = process::Command::new(path)
                .output()
                .expect("couldn't spawn clock command");
            let unix_time = String::from_utf8(output.stdout).expect("couldn't parse datetime");

            // Alert GUI thread
            sink.submit_command(TICK, unix_time, None)
                .expect("Failed to submit command");

            // Sleep one second
            thread::sleep(Duration::from_secs(1));
        }
    });

    launcher
        .delegate(delegate)
        .use_simple_logger()
        .launch(data)
        .expect("failed to launch");
}

fn ui_builder() -> impl Widget<State> {
    let label = Label::dynamic(|data: &State, _env| data.time.clone())
        .padding(5.0)
        .center();
    Flex::column().with_child(label)
}
