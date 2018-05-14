use cmd_parsing::parse_cmd;
use model::Application;
use model::Event;

pub struct CommandHandler {}

impl CommandHandler {
    pub fn new() -> CommandHandler {
        CommandHandler {}
    }

    // Todo: Add feedback when no arguments are provided
    pub fn execute(&self, app: &Application, cmd: &str) {
        debug!("Running command: {}", cmd);
        if let Some(cmd) = parse_cmd(cmd) {
            let split_cmd: Vec<_> = cmd.command.split_whitespace().collect();
            match split_cmd.get(0).cloned().unwrap_or_default() {
                "quit" | "q" => app.context
                    .read()
                    .event_channel
                    .send(Event::ShutdownAll)
                    .unwrap(),
                "nick" => {
                    // Nick
                    if let Some(new_nick) = split_cmd.get(1) {
                        debug!("Setting nickname to: {}", new_nick);
                        app.context
                            .read()
                            .guild
                            .map(|guild| guild.edit_nickname(Some(new_nick)));
                    }
                }
                "clearnick" | "cnick" => {
                    app.context
                        .read()
                        .guild
                        .map(|guild| guild.edit_nickname(None));
                }
                "setchannel" | "schan" => if let Some(new_chan) = split_cmd.get(1) {
                    if let Ok(new_chan_id) = new_chan.parse() {
                        app.context
                            .read()
                            .event_channel
                            .send(Event::SetChannel(new_chan_id))
                            .unwrap()
                    } else {
                        // Invalid id
                    }
                },
                "togglesidebar" | "tbar" => {
                    let new_state = !app.view.message_view.showing_sidebar();
                    app.context.write().guild_sidebar_visible = new_state;
                    app.view.message_view.set_show_sidebar(new_state);
                }
                _ => {}
            }
        }
    }
}
