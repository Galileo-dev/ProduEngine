mod renderer {
    pub mod main_renderer;
}

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
enum Input{
    UP,
    DOWN,
    LEFT,
    RIGHT
}


fn main(){
    let mut game_state = GameState{
        entities: vec![],
        players: vec![],
        counter: 0.0
    };
    renderer::main_renderer::create_vulkan_instance();

    // loop {
    //     let input_state = capture_input_state();
    //     npc_behaviour_system(&mut game_state , &input_state);
    //     monster_behaviour_system(&mut game_state);

    //     physics_system(&mut game_state);
    //     // ...

    //     render_system(&mut game_state);
    //     audio_system(&mut game_state);

    //     wait_vsync();
    //     game_state.counter += 1.0;
    // }
}


fn npc_behaviour_system(game_state:&mut GameState, input_state: &Input){
    
}

fn monster_behaviour_system(game_state:&mut GameState){
    
}

fn physics_system(game_state:&mut GameState){
    
}

fn render_system(game_state:&mut GameState){
    renderer::main_renderer::render(game_state);
    println!("Game Loop Counter: {}", game_state.counter);
}

fn audio_system(game_state:&mut GameState){
    
}

fn wait_vsync(){
    
}



fn capture_input_state() -> Input {
    return Input::DOWN;
}