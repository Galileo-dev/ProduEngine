// #[path = "./renderer/vulkan.rs"]
// mod vulkan;
// use vulkan::create_vulkan_instance;
#[path = "./renderer/vulkan.rs"]
mod vulkan;
use vulkan::create_vulkan_instance;

struct Player {}
struct Monster {}
struct Npc {}

enum Entity {
    Player(Player),
    Monster(Monster),
    Npc(Npc),
}

pub type EntityIndex = u32;

pub struct GameState {
    entities: Vec<Option<Entity>>,
    players: Vec<EntityIndex>,
    counter: f64,
}
enum Input {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let mut game_state = GameState {
        entities: vec![],
        players: vec![],
        counter: 0.0,
    };

    render_system(&mut game_state);
}

fn npc_behaviour_system(game_state: &mut GameState, input_state: &Input) {}

fn monster_behaviour_system(game_state: &mut GameState) {}

fn physics_system(game_state: &mut GameState) {}

fn render_system(game_state: &mut GameState) {
    // create_vulkan_instance()
    create_vulkan_instance()
}

fn audio_system(game_state: &mut GameState) {}

fn wait_vsync() {}

fn capture_input_state() -> Input {
    return Input::DOWN;
}
