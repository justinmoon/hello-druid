use std::thread;
use std::time::{Duration, SystemTime};

use druid::widget::{Flex, Label};
use druid::{
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, ExtEventSink, Lens, Selector,
    Target, Widget, WidgetExt, WindowDesc,
};

const TICK: Selector<String> = Selector::new("tick");

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
            // Print current time
            let unix_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            println!("{}", unix_time.as_secs());

            // Alert GUI thread
            sink.submit_command(TICK, unix_time.as_secs().to_string(), None)
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
