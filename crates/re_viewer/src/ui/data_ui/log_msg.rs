use re_log_types::{
    ArrowMsg, BeginRecordingMsg, DataTable, EntityPathOpMsg, LogMsg, RecordingInfo,
};

use crate::{misc::ViewerContext, ui::UiVerbosity};

use super::DataUi;

impl DataUi for LogMsg {
    fn data_ui(
        &self,
        ctx: &mut ViewerContext<'_>,
        ui: &mut egui::Ui,
        verbosity: UiVerbosity,
        query: &re_arrow_store::LatestAtQuery,
    ) {
        match self {
            LogMsg::BeginRecordingMsg(msg) => msg.data_ui(ctx, ui, verbosity, query),
            LogMsg::EntityPathOpMsg(_, msg) => msg.data_ui(ctx, ui, verbosity, query),
            LogMsg::ArrowMsg(_, msg) => msg.data_ui(ctx, ui, verbosity, query),
            LogMsg::Goodbye(_) => {
                ui.label("Goodbye");
            }
        }
    }
}

impl DataUi for BeginRecordingMsg {
    fn data_ui(
        &self,
        _ctx: &mut ViewerContext<'_>,
        ui: &mut egui::Ui,
        _verbosity: UiVerbosity,
        _query: &re_arrow_store::LatestAtQuery,
    ) {
        ui.code("BeginRecordingMsg");
        let BeginRecordingMsg { msg_id: _, info } = self;
        let RecordingInfo {
            application_id,
            recording_id,
            started,
            recording_source,
            is_official_example,
        } = info;

        egui::Grid::new("fields").num_columns(2).show(ui, |ui| {
            ui.monospace("application_id:");
            ui.label(application_id.to_string());
            ui.end_row();

            ui.monospace("recording_id:");
            ui.label(format!("{recording_id:?}"));
            ui.end_row();

            ui.monospace("started:");
            ui.label(started.format());
            ui.end_row();

            ui.monospace("recording_source:");
            ui.label(format!("{recording_source}"));
            ui.end_row();

            ui.monospace("is_official_example:");
            ui.label(format!("{is_official_example}"));
            ui.end_row();
        });
    }
}

impl DataUi for EntityPathOpMsg {
    fn data_ui(
        &self,
        ctx: &mut ViewerContext<'_>,
        ui: &mut egui::Ui,
        verbosity: UiVerbosity,
        query: &re_arrow_store::LatestAtQuery,
    ) {
        let EntityPathOpMsg {
            msg_id: _,
            time_point,
            path_op,
        } = self;

        egui::Grid::new("fields").num_columns(2).show(ui, |ui| {
            ui.monospace("time_point:");
            time_point.data_ui(ctx, ui, verbosity, query);
            ui.end_row();

            ui.monospace("path_op:");
            path_op.data_ui(ctx, ui, verbosity, query);
            ui.end_row();
        });
    }
}

impl DataUi for ArrowMsg {
    fn data_ui(
        &self,
        ctx: &mut ViewerContext<'_>,
        ui: &mut egui::Ui,
        verbosity: UiVerbosity,
        query: &re_arrow_store::LatestAtQuery,
    ) {
        let table: DataTable = match self.try_into() {
            Ok(table) => table,
            Err(err) => {
                ui.label(
                    ctx.re_ui
                        .error_text(format!("Error parsing ArrowMsg: {err}")),
                );
                return;
            }
        };

        // TODO(cmc): Come up with something a bit nicer once data tables become a common sight.
        for row in table.as_rows() {
            egui::Grid::new("fields").num_columns(2).show(ui, |ui| {
                ui.monospace("entity_path:");
                ctx.entity_path_button(ui, None, row.entity_path());
                ui.end_row();

                ui.monospace("time_point:");
                row.timepoint().data_ui(ctx, ui, verbosity, query);
                ui.end_row();

                ui.monospace("components:");
                row.cells().data_ui(ctx, ui, verbosity, query);
                ui.end_row();
            });
        }
    }
}
