use ferrisui_core::{
    build_context::BuildContext,
    callback::Callback,
    color::Color,
    geometry::EdgeInsets,
    state::Signal,
    view::{AnyView, View},
    view_node::{NodeKind, ViewNode},
};
use std::sync::Arc;
pub type ValidatorFn<T> = Arc<dyn Fn(&T) -> Result<(), String> + Send + Sync>;

#[derive(Clone)]
pub struct Validator<T> {
    func: ValidatorFn<T>,
}

impl<T> Validator<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&T) -> Result<(), String> + Send + Sync + 'static,
    {
        Self { func: Arc::new(f) }
    }
    pub fn validate(&self, value: &T) -> Result<(), String> {
        (self.func)(value)
    }
}

pub mod validators {
    use super::Validator;

    pub fn required(message: &'static str) -> Validator<String> {
        Validator::new(move |val: &String| {
            if val.trim().is_empty() {
                Err(message.to_string())
            } else {
                Ok(())
            }
        })
    }

    pub fn min_length(min: usize, message: &'static str) -> Validator<String> {
        Validator::new(move |val: &String| {
            if val.trim().len() < min {
                Err(message.to_string())
            } else {
                Ok(())
            }
        })
    }

    /// Validates basic email syntax.
    pub fn email(message: &'static str) -> Validator<String> {
        Validator::new(move |val: &String| {
            let s = val.trim();
            if s.contains('@') && s.contains('.') && !s.starts_with('@') && !s.ends_with('.') {
                Ok(())
            } else {
                Err(message.to_string())
            }
        })
    }

    /// Validates a boolean switch/checkbox (e.g., terms & conditions acceptance).
    pub fn must_be_true(message: &'static str) -> Validator<bool> {
        Validator::new(move |val: &bool| {
            if *val {
                Ok(())
            } else {
                Err(message.to_string())
            }
        })
    }

    /// Validates a numeric range for sliders or number inputs.
    pub fn range_f32(min: f32, max: f32, message: &'static str) -> Validator<f32> {
        Validator::new(move |val: &f32| {
            if *val >= min && *val <= max {
                Ok(())
            } else {
                Err(message.to_string())
            }
        })
    }
}

/// Wraps any interactive widget (TextInput, Checkbox, Slider) with a label,
/// validation rules, reactive error messaging, and helper hint text.
pub struct FormField<T: 'static, V: View> {
    label: Option<String>,
    helper_text: Option<String>,
    value: Signal<T>,
    error: Signal<Option<String>>,
    validators: Vec<Validator<T>>,
    child: V,
    required: bool,
}

impl<T: Clone + 'static, V: View> FormField<T, V> {
    pub fn new(label: impl Into<String>, value: Signal<T>, child: V) -> Self {
        Self {
            label: Some(label.into()),
            helper_text: None,
            value,
            error: Signal::new(None),
            validators: Vec::new(),
            child,
            required: false,
        }
    }

    /// Mark the field as required (shows a subtle indicator).
    pub fn required(mut self, is_required: bool) -> Self {
        self.required = is_required;
        self
    }
    /// Add an informative helper / hint text below the input.
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    /// Attach a validation rule.
    pub fn validator(mut self, rule: Validator<T>) -> Self {
        self.validators.push(rule);
        self
    }

    /// Execute all registered validators against the current value.
    /// Updates the error signal and returns true if all passed.
    pub fn validate(&self) -> bool {
        let val = self.value.get();
        for v in &self.validators {
            if let Err(err) = v.validate(&val) {
                self.error.set(Some(err));
                return false;
            }
        }
        self.error.set(None);
        true
    }

    /// Clears any currently active validation error.
    pub fn clear_error(&self) {
        self.error.set(None);
    }
}

impl<T: Clone + 'static, V: View> View for FormField<T, V> {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let mut column = ViewNode::new(NodeKind::Flex {
            is_vertical: true,
            spacing: 6.0,
        });

        // 1. Label row (with optional required indicator)
        if let Some(ref label) = self.label {
            let label_text = if self.required {
                format!("{} *", label)
            } else {
                label.clone()
            };
            let label_node = ViewNode::new(NodeKind::Text {
                text: label_text,
                font_size: 13.0,
                color: Color::hex(0x94A3B8), // Slate 400
            });
            column = column.with_child(label_node);
        }

        // 2. The input widget itself
        column = column.with_child(self.child.build(ctx));
        // 3. Error message (Priority) or Helper text
        let current_err = self.error.get();
        if let Some(err) = current_err {
            let err_node = ViewNode::new(NodeKind::Text {
                text: err,
                font_size: 12.0,
                color: Color::hex(0xEF4444), // Crimson Red 500
            });
            column = column.with_child(err_node);
        } else if let Some(ref helper) = self.helper_text {
            let helper_node = ViewNode::new(NodeKind::Text {
                text: helper.clone(),
                font_size: 12.0,
                color: Color::hex(0x64748B), // Slate 500
            });
            column = column.with_child(helper_node);
        }
        column
    }
}

/// Groups multiple related form fields under a section title and description card.
pub struct FormSection {
    title: String,
    description: Option<String>,
    children: Vec<AnyView>,
}

impl FormSection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            children: Vec::new(),
        }
    }
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn field<V: View + 'static>(mut self, field: V) -> Self {
        self.children.push(AnyView::new(field));
        self
    }
}

impl View for FormSection {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        // Outer Card Container
        let mut card = ViewNode::new(NodeKind::Container {
            background: Color::hex(0x1E293B), // Dark Slate Card
            corner_radius: 12.0,
            padding: EdgeInsets::all(16.0),
        });
        let mut inner_vbox = ViewNode::new(NodeKind::Flex {
            is_vertical: true,
            spacing: 12.0,
        });
        // Section Title
        inner_vbox = inner_vbox.with_child(ViewNode::new(NodeKind::Text {
            text: self.title.clone(),
            font_size: 16.0,
            color: Color::WHITE,
        }));
        // Optional Description
        if let Some(ref desc) = self.description {
            inner_vbox = inner_vbox.with_child(ViewNode::new(NodeKind::Text {
                text: desc.clone(),
                font_size: 13.0,
                color: Color::hex(0x94A3B8),
            }));
        }
        // Section Fields
        for child in &self.children {
            inner_vbox = inner_vbox.with_child(child.build(ctx));
        }
        card = card.with_child(inner_vbox);
        card
    }
}

/// The root form container managing submit callbacks and vertical spacing.
pub struct Form {
    children: Vec<AnyView>,
    spacing: f32,
    on_submit: Option<Callback<()>>,
}

impl Form {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 20.0,
            on_submit: None,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
    pub fn section(mut self, section: FormSection) -> Self {
        self.children.push(AnyView::new(section));
        self
    }
    pub fn child<V: View + 'static>(mut self, child: V) -> Self {
        self.children.push(AnyView::new(child));
        self
    }

    pub fn on_submit<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_submit = Some(Callback::new(move |_| handler()));
        self
    }
}

impl View for Form {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let mut form_vbox = ViewNode::new(NodeKind::Flex {
            is_vertical: true,
            spacing: self.spacing,
        });
        for child in &self.children {
            form_vbox = form_vbox.with_child(child.build(ctx));
        }
        form_vbox
    }
}
