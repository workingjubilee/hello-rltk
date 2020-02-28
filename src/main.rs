use rltk::{Rltk, GameState, Console};

struct State{}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Oh hello.");
    }
}
fn main() {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("Roguelike Tutorial").build();
    let gs = State{ };
    rltk::main_loop(context, gs);
}
