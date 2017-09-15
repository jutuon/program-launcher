


use conrod::{UiBuilder, UiCell, Ui};

use DEFAULT_WINDOW_HEIGHT;
use DEFAULT_WINDOW_WIDTH;

use backend_library::ProgramLibraryManager;

pub struct UiManager {
    widget_ids: WidgetIds,
    ui: Ui,
    list_selection_index: usize,
}

impl UiManager {
    pub fn new() -> UiManager {
        let mut ui = UiBuilder::new([DEFAULT_WINDOW_WIDTH as f64, DEFAULT_WINDOW_HEIGHT as f64]).build();
        let widget_ids = WidgetIds::new(ui.widget_id_generator());

        ui.fonts.insert_from_file("font/OpenSans-Regular.ttf").unwrap();

        UiManager {
            widget_ids,
            ui,
            list_selection_index: 0,
        }
    }

    pub fn ui_mut(&mut self) -> &mut Ui {
        &mut self.ui
    }

    pub fn ui(&self) -> &Ui {
        &self.ui
    }

    pub fn set_widgets(&mut self, program_library: &ProgramLibraryManager) {
        set_widgets(self.ui.set_widgets(), &mut self.widget_ids, &mut self.list_selection_index, program_library);
    }
}


widget_ids! {
    struct WidgetIds {
        canvas,

        canvas_left,
        canvas_right,

        canvas_program_info,
        canvas_console,

        console_text,

        program_title,
        button_launch,
        button_update_and_build,

        program_list,

    }
}



fn set_widgets(mut ui_cell: UiCell, ids: &mut WidgetIds, selection_i: &mut usize, program_library: &ProgramLibraryManager) {
    use conrod::widget::{Canvas, Widget, Button, Text, ListSelect};
    use conrod::{color, Colorable, Labelable, Positionable, Sizeable};

    use conrod::widget::list_select::Event;

    Canvas::new()
        .flow_right(&[
            (ids.canvas_left, Canvas::new().color(color::LIGHT_BLUE).length(250.0)),
            (ids.canvas_right, Canvas::new().flow_up(&[
                (ids.canvas_console, Canvas::new().color(color::LIGHT_GREY).length(250.0)),
                (ids.canvas_program_info, Canvas::new().pad(10.0).color(color::LIGHT_GRAY))
            ])),
        ])
        .set(ids.canvas, &mut ui_cell);


    let (mut events, scrollbar) = ListSelect::single(program_library.programs().len())
        .flow_down()
        .scrollbar_next_to()
        .item_size(30.0)
        .wh_of(ids.canvas_left)
        .top_left_of(ids.canvas_left)
        .set(ids.program_list, &mut ui_cell);

    while let Some(event) = events.next(&ui_cell, |i| i < program_library.programs().len()) {
        match event {
            Event::Item(item) => {
                let color = if item.i == *selection_i {
                    color::LIGHT_GREEN
                } else {
                    color::LIGHT_GRAY
                };
                let button = Button::new()
                    .color(color)
                    .label_color(color::BLACK)
                    .label(&program_library.programs()[item.i].name);
                item.set(button, &mut ui_cell);
            },
            Event::Selection(selection) => {
                *selection_i = selection;
            },
            _ => (),
        }
    }

    if let Some(s) = scrollbar {
        s.set(&mut ui_cell);
    }

    Text::new(&program_library.programs()[*selection_i].name)
        .top_left_of(ids.canvas_program_info)
        .set(ids.program_title, &mut ui_cell);

    let button_event = Button::new()
        .label("Launch")
        .down_from(ids.program_title, 10.0)
        .w_h(100.0,50.0)
        .set(ids.button_launch, &mut ui_cell);

    for _click in button_event {
        println!("Launch button");
    }

    let button_event = Button::new()
        .label("Update and build")
        .right_from(ids.button_launch, 10.0)
        .w_h(150.0,50.0)
        .set(ids.button_update_and_build, &mut ui_cell);

    for _click in button_event {
        println!("Update and build button");
    }

    Text::new("Console text")
        .middle_of(ids.canvas_console)
        .set(ids.console_text, &mut ui_cell);
}

