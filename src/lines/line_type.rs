/// Line types
/// ## Basic
/// Simple lines one line wide.
/// example:
/// ```text
/// ┌──────────────────────────────────────────────────────────┐
/// │ Lorem ipsum dolor sit amet, consectetur adipiscing elit, │
/// │ sed do eiusmod tempor incididunt ut labore et dolore     │
/// │ magna aliqua. Ut enim ad minim veniam, quis nostrud      │
/// │ exercitation ullamco laboris nisi ut aliquip ex ea       │
/// │ commodo consequat.                                       │
/// └──────────────────────────────────────────────────────────┘
/// ```
/// ## Dotted
/// Dotted lines
/// ```text
/// ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
/// ╎ Lorem ipsum dolor sit amet, consectetur adipiscing elit, ╎
/// ╎ sed do eiusmod tempor incididunt ut labore et dolore     ╎
/// ╎ magna aliqua. Ut enim ad minim veniam, quis nostrud      ╎
/// ╎ exercitation ullamco laboris nisi ut aliquip ex ea       ╎
/// ╎ commodo consequat.                                       ╎
/// └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
/// ```
pub enum LineType {
    /// Simple lines one line wide.
    /// ```text
    /// ┌──────────────────────────────────────────────────────────┐
    /// │ Lorem ipsum dolor sit amet, consectetur adipiscing elit, │
    /// │ sed do eiusmod tempor incididunt ut labore et dolore     │
    /// │ magna aliqua. Ut enim ad minim veniam, quis nostrud      │
    /// │ exercitation ullamco laboris nisi ut aliquip ex ea       │
    /// │ commodo consequat.                                       │
    /// └──────────────────────────────────────────────────────────┘
    /// ```
    Basic,

    /// Dotted lines
    /// ```text
    /// ┌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┐
    /// ╎ Lorem ipsum dolor sit amet, consectetur adipiscing elit, ╎
    /// ╎ sed do eiusmod tempor incididunt ut labore et dolore     ╎
    /// ╎ magna aliqua. Ut enim ad minim veniam, quis nostrud      ╎
    /// ╎ exercitation ullamco laboris nisi ut aliquip ex ea       ╎
    /// ╎ commodo consequat.                                       ╎
    /// └╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┘
    /// ```
    Dotted,

    /// Bolded single lines
    /// ```text
    /// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
    /// ┃ Lorem ipsum dolor sit amet, consectetur adipiscing elit, ┃
    /// ┃ sed do eiusmod tempor incididunt ut labore et dolore     ┃
    /// ┃ magna aliqua. Ut enim ad minim veniam, quis nostrud      ┃
    /// ┃ exercitation ullamco laboris nisi ut aliquip ex ea       ┃
    /// ┃ commodo consequat.                                       ┃
    /// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    /// ```
    Bold,
    Double,
}