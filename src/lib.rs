use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "./index.js")]
extern "C" {
    #[wasm_bindgen(js_name = statsUpdated)]
    fn stats_updated(stats: JsValue);
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    pub type Display;

    #[wasm_bindgen(method, structural, js_namespace = ROT)]
    fn draw(this: &Display, x: i32, y: i32, ch: &str);

    #[wasm_bindgen(method, structural, js_name = draw, js_namespace = ROT)]
    fn draw_color(this: &Display, x: i32, y: i32, ch: &str, color: &str);
}

#[derive(Serialize)]
struct Stats {
    hitpoints: i32,
    max_hitpoints: i32,
    moves: i32,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
struct GridPoint {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
pub struct PlayerCore {
    loc: GridPoint,
    moves: i32,
    display: Display,
    hitpoints: i32,
    max_hitpoints: i32,
    icon: String,
    color: String,
}

#[wasm_bindgen]
impl PlayerCore {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32, icon: &str, color: &str, display: Display) -> Self {
        PlayerCore {
            loc: GridPoint { x, y },
            display,
            icon: icon.to_string(),
            color: color.to_string(),
            moves: 0,
            hitpoints: 100,
            max_hitpoints: 100,
        }
    }

    pub fn x(&self) -> i32 {
        self.loc.x
    }

    pub fn y(&self) -> i32 {
        self.loc.y
    }

    pub fn draw(&self) {
        self.display
            .draw_color(self.loc.x, self.loc.y, &self.icon, &self.color);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.loc = GridPoint { x, y };
        self.draw();
        self.moves += 1;
        self.emit_stats();
    }

    fn emit_stats(&self) {
        let stats = Stats {
            hitpoints: self.hitpoints,
            max_hitpoints: self.max_hitpoints,
            moves: self.moves,
        };
        stats_updated(JsValue::from_serde(&stats).unwrap());
    }

    pub fn take_damage(&mut self, hits: i32) -> i32 {
        self.hitpoints -= hits;
        self.emit_stats();
        self.hitpoints
    }
}

#[wasm_bindgen]
pub struct Engine {
    display: Display,
    points: HashMap<GridPoint, String>,
    prize_location: Option<GridPoint>,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(display: Display) -> Self {
        Engine {
            display,
            points: HashMap::new(),
            prize_location: None,
        }
    }

    pub fn on_dig(&mut self, x: i32, y: i32, val: i32) {
        if val == 0 {
            let pt = GridPoint { x, y };
            self.points.insert(pt, ".".to_string());
        }
    }

    pub fn draw_map(&self) {
        self.points
            .iter()
            .for_each(|(k, v)| self.display.draw(k.x, k.y, v));
    }

    pub fn move_player(&mut self, pc: &mut PlayerCore, x: i32, y: i32) {
        self.redraw_at(pc.loc);
        pc.move_to(x, y)
    }

    fn redraw_at(&self, g: GridPoint) {
        if let Some(v) = self.points.get(&g) {
            self.display.draw(g.x, g.y, v);
        }
    }

    pub fn place_box(&mut self, x: i32, y: i32) {
        let g = GridPoint { x, y };
        self.points.insert(g, "*".to_string());
    }

    pub fn open_box(&mut self, pc: &mut PlayerCore, x: i32, y: i32) {
        let spot = GridPoint { x, y };

        if self.points.get(&spot).unwrap() != "*" {
            alert("There is no prize box here.");
            return;
        }

        match &self.prize_location {
            None => {}
            Some(loc) if *loc == spot => {
                alert("Congratulations! You've found the WebAssembly Module!")
            }
            Some(_) => {
                alert("Woops! This was booby trap!");
                pc.take_damage(30);
            }
        }
        self.remove_box(spot);
    }

    fn remove_box(&mut self, loc: GridPoint) {
        self.points.insert(loc, ".".to_string());
    }

    pub fn mark_wasmprize(&mut self, x: i32, y: i32) {
        let g = GridPoint { x, y };
        match self.points.get(&g) {
            Some(v) if v == "*" => self.prize_location = Some(g),
            Some(_) | None => {}
        }
    }

    pub fn free_cell(&self, x: i32, y: i32) -> bool {
        let g = GridPoint { x, y };
        match self.points.get(&g) {
            None => false,
            Some(v) => v == "." || v == "*",
        }
    }
}
