


use conrod::{UiBuilder, UiCell, Ui};

use DEFAULT_WINDOW_HEIGHT;
use DEFAULT_WINDOW_WIDTH;

use backend_library::task_manager::TaskManager;
use backend_library::data::ProgramLibrary;

use std::collections::VecDeque;

use window::Window;

pub struct UiManager {
    widget_ids: WidgetIds,
    ui: Ui,
    list_selection_index: usize,
    console_text: String,
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
            console_text: String::new(),
        }
    }

    pub fn ui_mut(&mut self) -> &mut Ui {
        &mut self.ui
    }

    pub fn ui(&self) -> &Ui {
        &self.ui
    }

    pub fn update_console_text(&mut self, console_text: &VecDeque<String>) {
        // TODO: long console lines may cause problems like running out of memory
        // TODO: copying all text at every update is inefficient
        self.console_text.clear();

        for text in console_text {
            self.console_text.push_str(text);
            self.console_text.push('\n');
        }
    }

    pub fn set_widgets<T: Window>(&mut self, task_manager: &mut TaskManager, programs: &ProgramLibrary, window: &mut T) {
        set_widgets(self.ui.set_widgets(), &mut self.widget_ids, &mut self.list_selection_index, task_manager, programs, &self.console_text, window);
    }
}

widget_ids! {
    struct WidgetIds {

        canvas,

        tabs,

        // Library tab
        canvas_library,

        canvas_library_layout,

        canvas_left,
        canvas_right,

        canvas_program_info,
        canvas_console,

        console_text,

        program_title,
        program_commands_list,

        program_list,

        // Settings tab
        canvas_settings,

        toggle_full_screen,
        text_full_screen,

    }
}



fn set_widgets<T: Window>(mut ui_cell: UiCell, ids: &mut WidgetIds, selection_i: &mut usize, task_manager: &mut TaskManager, program_library: &ProgramLibrary, console_text: &str, window: &mut T) {
    use conrod::widget::{Canvas, Widget, Button, Text, ListSelect, List, Tabs, Toggle};
    use conrod::{color, Colorable, Labelable, Positionable, Sizeable};

    use conrod::widget::list_select::Event;

    // UI layout

    Canvas::new()
        .color(color::GREEN)
        .set(ids.canvas, &mut ui_cell);

    Tabs::new(&[(ids.canvas_library, "Library"), (ids.canvas_settings, "Settings")])
        .starting_canvas(ids.canvas_library)
        .middle_of(ids.canvas)
        .wh_of(ids.canvas)
        .color(color::LIGHT_GRAY)
        .layout_horizontally()
        .set(ids.tabs, &mut ui_cell);

    Canvas::new()
        .top_left_of(ids.canvas_library)
        .wh_of(ids.canvas_library)
        .flow_right(&[
            (ids.canvas_left, Canvas::new().color(color::LIGHT_BLUE).length(250.0)),
            (ids.canvas_right, Canvas::new().flow_up(&[
                (ids.canvas_console, Canvas::new().color(color::LIGHT_GREY).pad(10.0).scroll_kids_vertically().length(250.0)),
                (ids.canvas_program_info, Canvas::new().pad(10.0).color(color::LIGHT_GRAY))
            ])),
        ])
        .set(ids.canvas_library_layout, &mut ui_cell);


    // Program list

    let (mut events, scrollbar) = ListSelect::single(program_library.programs.len())
        .flow_down()
        .scrollbar_next_to()
        .item_size(30.0)
        .wh_of(ids.canvas_left)
        .top_left_of(ids.canvas_left)
        .set(ids.program_list, &mut ui_cell);

    while let Some(event) = events.next(&ui_cell, |i| i < program_library.programs.len()) {
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
                    .label(&program_library.programs[item.i].name);
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

    // Current program

    let current_program = &program_library.programs[*selection_i];

    Text::new(&current_program.name)
        .top_left_of(ids.canvas_program_info)
        .set(ids.program_title, &mut ui_cell);


    let (mut items, scrollbar) = List::flow_right(current_program.command_queues.len())
        .item_size(150.0)
        .w_of(ids.canvas_program_info)
        .h(40.0)
        .down_from(ids.program_title, 10.0)
        .set(ids.program_commands_list, &mut ui_cell);

    while let Some(item) = items.next(&ui_cell) {
        let current_command_queue = &current_program.command_queues[item.i];

        let button = Button::new()
            .label(&current_command_queue.name)
            .color(color::GRAY)
            .label_color(color::BLACK);

        let button_event = item.set(button, &mut ui_cell);

        for _click in button_event {
            task_manager.new_queue_if_no_running_process(&current_command_queue.commands, &current_program.working_directory, &current_program.download_command);
        }
    }

    if let Some(s) = scrollbar {
        s.set(&mut ui_cell);
    }

    // Console


    Text::new(console_text)
        .bottom_left_of(ids.canvas_console)
        .w_of(ids.canvas_console)
        .set(ids.console_text, &mut ui_cell);


    // Settings

    let event = Toggle::new(window.full_screen())
        .label("Full screen mode")
        .label_color(color::WHITE)
        .w_h(150.0, 50.0)
        .color(color::LIGHT_BLUE)
        .top_left_with_margin_on(ids.canvas_settings, 20.0)
        .set(ids.toggle_full_screen, &mut ui_cell);

    let text = if window.full_screen() {
        "Enabled"
    } else {
        "Disabled"
    };

    Text::new(text)
        .right_from(ids.toggle_full_screen, 20.0)
        .set(ids.text_full_screen, &mut ui_cell);

    for new_state in event {
        window.set_full_screen(new_state);
    }
}

