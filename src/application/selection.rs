use uuid::Uuid;

/// The types of selection the user can make
pub enum SelectionType {
    /// A single vertex
    Vertex,
    /// A single segment
    Segment,
    /// A single polygon
    Polygon,
    /// A single solid
    Solid,
}

/// A selection of geometry
pub struct Selection {
    /// The ID of the selected geometry
    pub id: Uuid,
    /// The type of selection
    pub selection_type: SelectionType,
}
