use bevy::prelude::*;

/// Resource to track UI state
#[derive(Resource)]
pub struct UiState {
    /// Whether segment outlines are visible
    pub show_outlines: bool,
    /// Whether surfaces (3D meshes) are visible
    pub show_surfaces: bool,
    /// Whether camera is in isometric (orthographic) view
    pub isometric_view: bool,
    /// Orthographic viewport height (smaller = more zoomed in)
    pub ortho_zoom: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            show_outlines: false,
            show_surfaces: true,   // Surfaces visible by default
            isometric_view: false, // Perspective view by default
            ortho_zoom: 8.5,      // Default viewport height
        }
    }
}

/// Marker component for the outline toggle button
#[derive(Component)]
pub struct OutlineToggleButton;

/// Marker component for the surfaces toggle button
#[derive(Component)]
pub struct SurfacesToggleButton;

/// Marker component for the outline button text
#[derive(Component)]
pub struct OutlineButtonText;

/// Marker component for the surfaces button text
#[derive(Component)]
pub struct SurfacesButtonText;

/// Marker component for the isometric view toggle button
#[derive(Component)]
pub struct IsometricToggleButton;

/// Marker component for the isometric button text
#[derive(Component)]
pub struct IsometricButtonText;

/// Marker component for mesh entities that can be toggled
#[derive(Component)]
pub struct ToggleableMesh;

/// Setup the UI overlay
pub fn setup_ui(mut commands: Commands) {
    // Create a root node for the UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Toggle button for segment outlines
            parent
                .spawn((
                    Button,
                    OutlineToggleButton,
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                ))
                .with_children(|parent| {
                    parent.spawn((Text::new("Show Outlines: OFF"), OutlineButtonText));
                });

            // Toggle button for surfaces
            parent
                .spawn((
                    Button,
                    SurfacesToggleButton,
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.4, 0.2, 0.8)), // Green since default is ON
                ))
                .with_children(|parent| {
                    parent.spawn((Text::new("Show Surfaces: ON"), SurfacesButtonText));
                });

            // Toggle button for isometric view
            parent
                .spawn((
                    Button,
                    IsometricToggleButton,
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                ))
                .with_children(|parent| {
                    parent.spawn((Text::new("Isometric: OFF"), IsometricButtonText));
                });
        });
}

/// Handle button interactions
pub fn handle_ui_interactions(
    mut outline_interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<OutlineToggleButton>),
    >,
    mut surfaces_interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<SurfacesToggleButton>),
    >,
    mut isometric_interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<IsometricToggleButton>),
    >,
    mut ui_state: ResMut<UiState>,
) {
    // Handle outline toggle
    for interaction in &mut outline_interaction_query {
        if *interaction == Interaction::Pressed {
            ui_state.show_outlines = !ui_state.show_outlines;
        }
    }

    // Handle surfaces toggle
    for interaction in &mut surfaces_interaction_query {
        if *interaction == Interaction::Pressed {
            ui_state.show_surfaces = !ui_state.show_surfaces;
        }
    }

    // Handle isometric toggle
    for interaction in &mut isometric_interaction_query {
        if *interaction == Interaction::Pressed {
            ui_state.isometric_view = !ui_state.isometric_view;
        }
    }
}

/// Update button appearance and text based on state
pub fn update_button_appearance(
    mut queries: ParamSet<(
        Query<&mut BackgroundColor, With<OutlineToggleButton>>,
        Query<&mut BackgroundColor, With<SurfacesToggleButton>>,
        Query<&mut BackgroundColor, With<IsometricToggleButton>>,
        Query<&mut Text, With<OutlineButtonText>>,
        Query<&mut Text, With<SurfacesButtonText>>,
        Query<&mut Text, With<IsometricButtonText>>,
    )>,
    ui_state: Res<UiState>,
) {
    // Update outline button color
    for mut background_color in queries.p0().iter_mut() {
        if ui_state.show_outlines {
            *background_color = Color::srgba(0.2, 0.4, 0.2, 0.8).into();
        } else {
            *background_color = Color::srgba(0.1, 0.1, 0.1, 0.8).into();
        }
    }

    // Update surfaces button color
    for mut background_color in queries.p1().iter_mut() {
        if ui_state.show_surfaces {
            *background_color = Color::srgba(0.2, 0.4, 0.2, 0.8).into();
        } else {
            *background_color = Color::srgba(0.1, 0.1, 0.1, 0.8).into();
        }
    }

    // Update isometric button color
    for mut background_color in queries.p2().iter_mut() {
        if ui_state.isometric_view {
            *background_color = Color::srgba(0.2, 0.4, 0.2, 0.8).into();
        } else {
            *background_color = Color::srgba(0.1, 0.1, 0.1, 0.8).into();
        }
    }

    // Update outline button text
    for mut text in queries.p3().iter_mut() {
        let new_text = if ui_state.show_outlines {
            "Show Outlines: ON"
        } else {
            "Show Outlines: OFF"
        };
        // Reconstruct Text with new string
        *text = Text::new(new_text);
    }

    // Update surfaces button text
    for mut text in queries.p4().iter_mut() {
        let new_text = if ui_state.show_surfaces {
            "Show Surfaces: ON"
        } else {
            "Show Surfaces: OFF"
        };
        *text = Text::new(new_text);
    }

    // Update isometric button text
    for mut text in queries.p5().iter_mut() {
        let new_text = if ui_state.isometric_view {
            "Isometric: ON"
        } else {
            "Isometric: OFF"
        };
        *text = Text::new(new_text);
    }
}

/// Toggle mesh visibility based on UI state
pub fn toggle_mesh_visibility(
    mut mesh_query: Query<&mut Visibility, With<ToggleableMesh>>,
    ui_state: Res<UiState>,
) {
    for mut visibility in &mut mesh_query {
        if ui_state.show_surfaces {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
