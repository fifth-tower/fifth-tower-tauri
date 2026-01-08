use flow_model::ActionCommand;

pub(crate) struct KeyCombiCommand;
impl KeyCombiCommand {
    pub(crate) fn fill_command(
        _app_id: &str,
        _flow_id: &str,
        cursor: (i32, i32),
    ) -> Option<ActionCommand> {
        Some(ActionCommand::KeyCombi(cursor.0, cursor.1))
    }
}
