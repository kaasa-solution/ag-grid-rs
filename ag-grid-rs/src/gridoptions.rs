use serde::Serialize;
use serde_with::skip_serializing_none;
use web_sys::HtmlElement;

use crate::{column::ColumnDef, AgGrid, Grid, RowData, RowModelType};

/// An instance of an AG Grid [`GridOptions`].
///
/// With this struct, users can specify the initial options for their grid, before calling the
/// [`build()`] method to receive an instance of [`Grid`]. The various options are fully customisable
/// using the builder pattern, so you only need to specify what you need. The options mirror
/// those used in the AG Grid library.
///
/// [`GridOptions`]: https://www.ag-grid.com/javascript-data-grid/grid-options/
#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GridOptions {
    // Accessories
    // TODO

    // Clipboard
    // TODO

    // Column Definitions
    pub column_defs: Option<Vec<ColumnDef>>,
    pub default_col_def: Option<ColumnDef>,
    // default_col_group_def
    // column_types
    // maintain_column_order
    // suppress_dot_field_notation

    // Column Headers
    // TODO

    // Column Moving
    // TODO

    // Column Sizing
    // TODO

    // Components
    // TODO

    // Editing
    // TODO

    // Export
    // TODO

    // Filtering
    // TODO

    // Integrated Charts
    // TODO

    // Keyboard Navigation
    // TODO

    // Loading Cell Renderers
    // TODO

    // Localisation
    // TODO

    // Master Detail
    // TODO

    // Miscellaneous
    // TODO

    // Overlays
    // TODO

    // Pagination
    pub pagination: Option<bool>,
    // TODO

    // Pivot and Aggregation
    // TODO

    // Rendering
    // TODO

    // Row Drag and Drop
    // TODO

    // Row Full Width
    // TODO

    // Row Pinning
    // TODO

    // RowModel
    pub row_model_type: Option<RowModelType>,
    // get_row_id

    // RowModel: Client Side
    pub row_data: Option<Vec<RowData>>,
    // TODO

    // RowModel: Infinite
    pub cache_block_size: Option<u32>,
    // TODO

    // RowModel: Server Side
    // TODO

    // RowModel: Viewport
    // TODO

    // Scrolling
    // TODO

    // Selection
    // TODO

    // Sorting
    // TODO

    // Styling
    // TODO

    // Tooltips
    // TODO
}

impl GridOptions {
    pub fn new() -> Self {
        Default::default()
    }

    /// A finaliser method for the [`GridOptions`] struct. This method constructs the underlying
    /// JavaScript grid and returns a handle, [`Grid`], which provides access to the grid APIs.
    pub fn build(self, div: HtmlElement) -> Grid {
        let grid_options =
            serde_wasm_bindgen::to_value(&self).expect("failed converting GridOptions to JsValue");

        let js_grid = AgGrid::new(div, grid_options);

        Grid {
            grid_options: self,
            api: js_grid.gridOptions().api(),
            column_api: js_grid.gridOptions().columnApi(),
        }
    }

    /// Set the column definitions. Fields set here take precedence over those set
    /// in `default_col_def`.
    pub fn column_defs(mut self, column_defs: Vec<ColumnDef>) -> Self {
        self.column_defs = Some(column_defs);
        self
    }

    /// Set the default column definition. Fields set here have lower precedence
    /// than fields set on a per-column basis in `column_defs`.
    pub fn default_col_def(mut self, col_def: ColumnDef) -> Self {
        self.default_col_def = Some(col_def);
        self
    }

    /// Set the row data.
    pub fn row_data(mut self, row_data: Vec<RowData>) -> Self {
        self.row_data = Some(row_data);
        self
    }

    /// Sets the row model type.
    pub fn row_model_type(mut self, row_model_type: RowModelType) -> Self {
        self.row_model_type = Some(row_model_type);
        self
    }

    /// Set whether pagination is enabled.
    pub fn pagination(mut self, pagination: bool) -> Self {
        self.pagination = Some(pagination);
        self
    }

    /// How many rows for each block in the store, i.e. how many rows returned from the server at a time.
    pub fn cache_block_size(mut self, cache_block_size: u32) -> Self {
        self.cache_block_size = Some(cache_block_size);
        self
    }
}
