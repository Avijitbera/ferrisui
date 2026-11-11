use ferrisui::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TodoItem {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TodoFilter {
    All,
    Active,
    Completed,
}

pub struct TodoApp {
    todos: Signal<Vec<TodoItem>>,
    input_text: Signal<String>,
    input_focused: Signal<bool>,
    filter: Signal<TodoFilter>,
    next_id: Signal<u64>,
}

impl TodoApp {
    pub fn new() -> Self {
        let initial_todos = vec![
            TodoItem {
                id: 1,
                text: "Build FerrisUI GPU renderer & layout".to_string(),
                completed: true,
            },
            TodoItem {
                id: 2,
                text: "Wire up keyboard & focus navigation".to_string(),
                completed: true,
            },
            TodoItem {
                id: 3,
                text: "Try typing in this box and press Enter".to_string(),
                completed: false,
            },
            TodoItem {
                id: 4,
                text: "Check off tasks and filter views".to_string(),
                completed: false,
            },
        ];

        Self {
            todos: Signal::new(initial_todos),
            input_text: Signal::new(String::new()),
            input_focused: Signal::new(false),
            filter: Signal::new(TodoFilter::All),
            next_id: Signal::new(5),
        }
    }
}

impl View for TodoApp {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let todos_sig = self.todos.clone();
        let input_sig = self.input_text.clone();
        let filter_sig = self.filter.clone();
        let next_id_sig = self.next_id.clone();

        let all_todos = todos_sig.get();
        let current_filter = filter_sig.get();

        let active_count = all_todos.iter().filter(|t| !t.completed).count();
        let completed_count = all_todos.iter().filter(|t| t.completed).count();

        // 1. Filtered items
        let visible_todos: Vec<TodoItem> = all_todos
            .iter()
            .filter(|item| match current_filter {
                TodoFilter::All => true,
                TodoFilter::Active => !item.completed,
                TodoFilter::Completed => item.completed,
            })
            .cloned()
            .collect();

        // 2. Add todo helper
        let add_todo = {
            let todos = todos_sig.clone();
            let input = input_sig.clone();
            let next_id = next_id_sig.clone();
            let focus = self.input_focused.clone();
            move |raw_text: String| {
                let trimmed = raw_text.trim();
                if !trimmed.is_empty() {
                    let id = next_id.get();
                    next_id.set(id + 1);
                    todos.update(|list| {
                        list.push(TodoItem {
                            id,
                            text: trimmed.to_string(),
                            completed: false,
                        });
                    });
                    input.set(String::new());
                    focus.set(true);
                }
            }
        };

        // 3. Header
        let header = VStack::new()
            .spacing(4.0)
            .child(
                Text::new("FERRISUI TODOS")
                    .font_size(24.0)
                    .color(Color::hex(0x38BDF8)),
            )
            .child(
                Text::new("Interactive Phase 2 Milestone Demo")
                    .font_size(13.0)
                    .color(Color::hex(0x94A3B8)),
            );

        // 4. Input bar
        let input_for_submit = add_todo.clone();
        let input_for_click = add_todo.clone();
        let current_input_val = input_sig.clone();
        let focus_for_add_btn = self.input_focused.clone();

        let input_bar = HStack::new()
            .spacing(12.0)
            .child(
                TextInput::new(input_sig.clone())
                    .placeholder("What needs to be done? (Press Enter to add)")
                    .focused(self.input_focused.clone())
                    .on_submit(move |text| {
                        input_for_submit(text);
                    }),
            )
            .child(
                Button::new("+ ADD")
                    .style(ButtonStyle::filled())
                    .background(Color::hex(0x0284C7)) // Sky 600
                    .on_click(move || {
                        input_for_click(current_input_val.get());
                        focus_for_add_btn.set(true);
                    }),
            );

        // 5. Filter tabs
        let f_all = filter_sig.clone();
        let f_act = filter_sig.clone();
        let f_com = filter_sig.clone();

        let filter_bar = HStack::new()
            .spacing(8.0)
            .child(
                Button::new(format!("ALL ({})", all_todos.len()))
                    .style(if current_filter == TodoFilter::All {
                        ButtonStyle::filled()
                    } else {
                        ButtonStyle::outlined()
                    })
                    .on_click(move || f_all.set(TodoFilter::All)),
            )
            .child(
                Button::new(format!("ACTIVE ({})", active_count))
                    .style(if current_filter == TodoFilter::Active {
                        ButtonStyle::filled()
                    } else {
                        ButtonStyle::outlined()
                    })
                    .on_click(move || f_act.set(TodoFilter::Active)),
            )
            .child(
                Button::new(format!("COMPLETED ({})", completed_count))
                    .style(if current_filter == TodoFilter::Completed {
                        ButtonStyle::filled()
                    } else {
                        ButtonStyle::outlined()
                    })
                    .on_click(move || f_com.set(TodoFilter::Completed)),
            );

        // 6. Todo item rows
        let mut todo_rows = VStack::new().spacing(8.0);

        if visible_todos.is_empty() {
            todo_rows = todo_rows.child(
                Container::new()
                    .background(Color::rgba(1.0, 1.0, 1.0, 0.02))
                    .padding(EdgeInsets::symmetric(24.0, 16.0))
                    .corner_radius(8.0)
                    .child(
                        Text::new("No tasks to display in this filter.")
                            .font_size(14.0)
                            .color(Color::hex(0x64748B)),
                    ),
            );
        } else {
            for item in visible_todos {
                let item_id = item.id;
                let todos_toggle = todos_sig.clone();
                let todos_delete = todos_sig.clone();

                let row = Container::new()
                    .background(Color::hex(0x1E293B)) // Slate 800
                    .corner_radius(8.0)
                    .padding(EdgeInsets::symmetric(14.0, 10.0))
                    .child(
                        HStack::new()
                            .spacing(12.0)
                            .child(
                                Checkbox::new(item.completed)
                                    .accent_color(Color::hex(0x10B981)) // Emerald
                                    .on_change(move |is_checked| {
                                        todos_toggle.update(|list| {
                                            if let Some(todo) =
                                                list.iter_mut().find(|t| t.id == item_id)
                                            {
                                                todo.completed = is_checked;
                                            }
                                        });
                                    }),
                            )
                            .child(
                                Text::new(item.text.clone())
                                    .font_size(15.0)
                                    .align_left()
                                    .color(if item.completed {
                                        Color::hex(0x64748B) // Dimmed
                                    } else {
                                        Color::hex(0xF8FAFC) // Crisp white
                                    }),
                            )
                            .child(Spacer::new())
                            .child(
                                Button::new("✕")
                                    .style(ButtonStyle::text())
                                    .on_click(move || {
                                        todos_delete.update(|list| {
                                            list.retain(|t| t.id != item_id);
                                        });
                                    }),
                            ),
                    );

                todo_rows = todo_rows.child(row);
            }
        }

        // 7. Footer
        let todos_clear = todos_sig.clone();
        let footer = HStack::new()
            .spacing(16.0)
            .child(
                Text::new(format!("{} items remaining", active_count))
                    .font_size(13.0)
                    .color(Color::hex(0x94A3B8)),
            )
            .child(Spacer::new())
            .child(
                Button::new("CLEAR COMPLETED")
                    .style(ButtonStyle::text())
                    .on_click(move || {
                        todos_clear.update(|list| {
                            list.retain(|t| !t.completed);
                        });
                    }),
            );

        // 8. Main container layout
        Container::new()
            .background(Color::hex(0x0A0A0F)) // Deep dark background
            .padding(EdgeInsets::all(28.0))
            .child(
                VStack::new()
                    .spacing(18.0)
                    .child(header)
                    .child(Divider::new())
                    .child(input_bar)
                    .child(filter_bar)
                    .child(todo_rows)
                    .child(Divider::new())
                    .child(footer),
            )
            .build(_ctx)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting FerrisUI Todo App...");
    println!("Features:");
    println!("  - Keyboard typing and focus handling in single-line TextInput");
    println!("  - Press [Enter] or click [+ ADD] to create tasks");
    println!("  - Click checkbox to toggle completed state");
    println!("  - Filter tabs: All, Active, Completed");
    println!("  - Click [X] to delete an item, or [CLEAR COMPLETED]");

    FerrisUI::app(TodoApp::new())
        .title("FerrisUI — Interactive Todo App")
        .size(620.0, 720.0)
        .run()
}

