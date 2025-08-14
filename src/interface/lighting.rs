use bevy::prelude::*;

/// Lighting configuration for the scene
///
/// LIGHTING THEORY EXPLANATION:
/// ===========================
/// Good 3D lighting requires multiple light sources to avoid harsh shadows and flat appearance:
///
/// 1. KEY LIGHT: Main directional light that provides primary illumination and shadows
/// 2. FILL LIGHT: Softer, secondary light that fills in shadows and reduces contrast
/// 3. AMBIENT LIGHT: Global illumination that ensures no surface is completely dark
///
/// SHADOW QUALITY FACTORS:
/// - Light intensity: Too bright creates harsh shadows, too dim creates flat appearance
/// - Light positioning: Angle affects shadow length and direction
/// - Fill light: Reduces shadow darkness and creates more natural lighting
/// - Ambient light: Prevents completely black shadows
#[derive(Resource, Clone)]
pub struct LightingConfig {
    pub directional_light_position: Vec3,
    pub directional_light_target: Vec3,
    pub shadows_enabled: bool,
    pub light_intensity: f32,
    pub ambient_light: Color,
    pub ambient_brightness: f32,
}

impl Default for LightingConfig {
    fn default() -> Self {
        Self {
            directional_light_position: Vec3::new(4.0, 8.0, 4.0),
            directional_light_target: Vec3::ZERO,
            shadows_enabled: true,
            light_intensity: 0.8, // Reduced from 1.0 for softer lighting
            ambient_light: Color::srgb(0.3, 0.3, 0.35), // Increased ambient light
            ambient_brightness: 0.4, // Increased from 0.1
        }
    }
}

/// Spawns the main directional light with the given configuration
///
/// LIGHTING SETUP EXPLANATION:
/// ==========================
/// We're creating a three-light setup for professional-quality lighting:
///
/// 1. MAIN DIRECTIONAL LIGHT (Key Light):
///    - Position: High and to the side for natural-looking shadows
///    - Intensity: Moderate (800 lux) to avoid harsh shadows
///    - Shadows: Enabled for depth and realism
///    - Purpose: Primary illumination and shadow casting
///
/// 2. FILL LIGHT:
///    - Position: Opposite side, lower height for subtle fill
///    - Intensity: Low (200 lux) to avoid competing with main light
///    - Shadows: Disabled to avoid shadow conflicts
///    - Color: Slightly blue-tinted for natural fill
///    - Purpose: Softens shadows and reduces contrast
///
/// 3. AMBIENT LIGHT:
///    - Global illumination that affects all surfaces equally
///    - Prevents completely black shadows
///    - Creates base illumination level
///
/// SHADOW QUALITY IMPROVEMENTS:
/// - Reduced main light intensity prevents harsh shadows
/// - Fill light softens shadow edges
/// - Increased ambient light prevents black shadows
/// - Proper light positioning creates natural shadow direction
pub fn spawn_directional_light(commands: &mut Commands, config: &LightingConfig) {
    println!("=== LIGHTING SETUP DEBUG ===");
    println!("Setting up professional three-light system:");
    println!();

    println!("MAIN DIRECTIONAL LIGHT (Key Light):");
    println!(
        "  Position: {:?} (high and to the side for natural shadows)",
        config.directional_light_position
    );
    println!(
        "  Target: {:?} (looking at center of scene)",
        config.directional_light_target
    );
    println!(
        "  Intensity: {} lux (moderate for soft shadows)",
        config.light_intensity * 800.0
    );
    println!(
        "  Shadows: {} (enabled for depth and realism)",
        config.shadows_enabled
    );
    println!();

    // Main directional light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: config.shadows_enabled,
            illuminance: config.light_intensity * 800.0, // Reduced intensity for softer shadows
            ..default()
        },
        Transform::from_xyz(
            config.directional_light_position.x,
            config.directional_light_position.y,
            config.directional_light_position.z,
        )
        .looking_at(config.directional_light_target, Vec3::Y),
        GlobalTransform::default(),
    ));

    println!("FILL LIGHT (Secondary Light):");
    println!("  Purpose: Softens shadows and reduces harsh contrast");

    // Add a second, softer fill light from the opposite direction
    let fill_light_pos = Vec3::new(
        -config.directional_light_position.x,
        config.directional_light_position.y * 0.5,
        -config.directional_light_position.z,
    );

    println!(
        "  Position: {:?} (opposite side, lower height)",
        fill_light_pos
    );
    println!(
        "  Intensity: {} lux (low to avoid competing with main light)",
        config.light_intensity * 200.0
    );
    println!("  Shadows: Disabled (prevents shadow conflicts)");
    println!("  Color: Slightly blue-tinted for natural fill");
    println!();

    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,                      // No shadows for fill light
            illuminance: config.light_intensity * 200.0, // Much softer fill light
            color: Color::srgb(0.9, 0.9, 1.0),           // Slightly blue-tinted fill
            ..default()
        },
        Transform::from_xyz(fill_light_pos.x, fill_light_pos.y, fill_light_pos.z)
            .looking_at(config.directional_light_target, Vec3::Y),
        GlobalTransform::default(),
    ));

    println!("AMBIENT LIGHT (Global Illumination):");
    println!("  Purpose: Prevents completely black shadows and creates base illumination");
    println!(
        "  Color: {:?} (neutral with slight blue tint)",
        config.ambient_light
    );
    println!(
        "  Brightness: {} (increased for better shadow quality)",
        config.ambient_brightness
    );
    println!();

    // Add ambient lighting
    commands.insert_resource(AmbientLight {
        color: config.ambient_light,
        brightness: config.ambient_brightness,
        affects_lightmapped_meshes: false,
    });

    println!("LIGHTING QUALITY IMPROVEMENTS:");
    println!("  1. Reduced main light intensity prevents harsh shadows");
    println!("  2. Fill light softens shadow edges and reduces contrast");
    println!("  3. Increased ambient light prevents black shadows");
    println!("  4. Proper light positioning creates natural shadow direction");
    println!("  5. Three-light setup mimics professional photography lighting");
    println!();
    println!("Lighting setup complete");
    println!("=== END LIGHTING SETUP DEBUG ===");
}
