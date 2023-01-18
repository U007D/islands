pub const ERR_BAD_ARG_COUNT: &str = "Incorrect number of arguments received";
pub const ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8: &str =
    "Error: supplied command-line argument not convertible to UTF-8";
pub const ERR_ARG: &str = "Error in supplied command-line arguments";
pub const ERR_ARG_PARSE: &str = "Error parsing supplied command-line arguments";
pub const ERR_AT_LEAST_ONE_ELEMENT_REQUIRED: &str =
    "Error: At least one data element must be provided";
pub const ERR_FILE_CREATE: &str = "An error occurred attempting to create a file";
pub const ERR_FILE_OPEN: &str = "An error occurred attempting to open a file";
pub const ERR_FILE_READ: &str = "An error occurred performing file read I/O";
pub const ERR_FILE_WRITE: &str = "An error occurred performing file write I/O";
pub const ERR_IO: &str = "Error performing I/O";
pub const ERR_MEMORY: &str = "Memory error";
pub const ERR_RECT: &str =
    "An error occurred during conversion of a collection elements into a `Rect`";
pub const ERR_RECT_NON_RECTANGULAR_INPUT: &str =
    "Error processing `Rect` input: Input data is non-rectangular (row lengths must be identical)";
pub const ERR_RECT_NO_INPUT_DATA: &str =
    "Error processing `Rect` input: No data found. `Rect` requires at least one data element";
pub const ERR_ROW_COLUMN_OVERFLOW: &str =
    "Error: Number of specified rows x columns exceeds `usize::MAX`";

pub const PANIC_INDEX_OUT_OF_BOUNDS: &str = "Panic: Index out of bounds";

pub const BUT_EXPECTED: &str = "but expected";
pub const ROWS_BUT_INDEX_IS: &str = "rows, but index is";
pub const FROM: &str = "from";
pub const RECEIVED: &str = "Received";
pub const ROWS_COLUMNS: &str = "(rows, columns)";
pub const THERE_ARE: &str = "There are";
pub const TO: &str = "to";
pub const UNKNOWN_APP_NAME: &str = "<unknown application name>";
pub const USAGE: &str = "Usage";
