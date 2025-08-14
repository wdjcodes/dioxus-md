use dioxus::prelude::*;

/// Input types supported by Material Design TextFields
#[derive(Clone, PartialEq)]
pub enum TextFieldTypes {
    Text,
    TextArea,
    Email,
    Number,
    Password,
    Search,
    Tel,
    Url,
    Unknown,
}

/// Combined attributes for Material Design TextFields and html `<input>` tags
#[derive(Clone, PartialEq, Props)]
pub struct TextFieldAttrs {

    /// Gets or sets whether or not the text field is in a disabled state.
    #[props(default = false)]
    pub disabled: bool,

    /// Gets or sets whether or not the text field is in a visually invalid state.
    #[props(default = false)]
    pub error: bool,

    /// The error message that replaces supporting text when [`error`] is true.
    #[props(into, default = "")]
    pub error_text: String,

    /// The floating Material label of the textfield component. It informs the user
    /// about what information is requested for a text field. It is aligned with
    /// the input text, is always visible, and it floats when focused or when text
    /// is entered into the textfield. This label also sets accessibilty labels,
    /// but the accessible label is overriden by `aria-label`.
    #[props(into, default = "Example")]
    pub label: String,

    /// Disables the asterisk on the floating label, when the text field is required.
    #[props(default = false)]
    pub no_asterisk: bool,

    /// Indicates that the user must specify a value for the input before the
    /// owning form can be submitted and will render an error state when
    /// `reportValidity()` is invoked when value is empty. Additionally the
    /// floating label will render an asterisk `"*"` when true.
    #[props(default = false)]
    pub required: bool,

    /// The current value of the text field. It is always a string.
    #[props(default = use_signal(||"".to_string()))]
    pub value: Signal<String>,

    /// An optional prefix to display before the input value.
    #[props(into, default = "")]
    pub prefix_text: String,

    /// An optional suffix to display after the input value.
    #[props(into, default = "")]
    pub suffix_text: String,

    /// Conveys additional information below the text field, such as how it should
    /// be used.
    #[props(into, default = "")]
    pub supporting_text: String,

    // ********* Standard HTML <input> attributes should get these for free. *******************

    /// Defines the greatest value in the range of permitted values.
    #[props(into, default = "")]
    pub max: String,

    /// The maximum number of characters a user can enter into the text field. Set
    /// to -1 for none. 
    #[props(default = -1)]
    pub maxlength: i32,

    /// Defines the most negative value in the range of permitted values.
    #[props(into, default = "")]
    pub min: String,

    /// The minimum number of characters a user can enter into the text field. Set
    /// to -1 for none.
    #[props(default = -1)]
    pub minlength: i32,

    /// A regular expression that the text field's value must match to pass 
    /// constraint validation.
    #[props(into, default = "")]
    pub pattern: String,

    /// Defines the text displayed in the textfield when it has no value. Provides
    /// a brief hint to the user as to the expected type of data that should be
    /// entered into the control. Unlike `label`, the placeholder is not visible
    /// and does not float when the textfield has a value.
    #[props(into, default = "")]
    pub placeholder: String,

    /// Indicates whether or not a user should be able to edit the text field's value.
    #[props(default = false)]
    pub read_only: bool,

    /// Indicates that input accepts multiple email addresses.
    #[props(default = false)]
    pub multiple: bool,

    /// Returns or sets the element's step attribute, which works with min and max
    /// to limit the increments at which a numeric or date-time value can be set.
    #[props(into, default = "")]
    pub step: String,

    /// The `<input>` type to use, defaults to "text". The type greatly changes how
    /// the text field behaves.
    /// 
    /// Text fields support a limited number of `<input>` types:
    /// 
    /// Currently Supported
    /// - text
    /// 
    /// Currently Unsupported
    /// - textarea
    /// - email
    /// - number
    /// - password
    /// - search
    /// - tel
    /// - url
    #[props(default = TextFieldTypes::Text)]
    pub r#type: TextFieldTypes,


    /// Describes what, if any, type of autocomplete functionality the input
    /// should provide.
    #[props(into, default = "")]
    pub auto_complete: String,
}
