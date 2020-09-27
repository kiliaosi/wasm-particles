use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::f64::consts::PI;

pub struct Particle {
  x: f64,
  y: f64,
  vx: f64,
  vy: f64,
  size: f64,
  color: String,
  lineColor: String,
}
#[wasm_bindgen()]
extern "C" {
  #[wasm_bindgen(js_namespace=Math)]
  fn random()->f64;

  #[wasm_bindgen(js_namespace=console)]
  fn log(msg:&str);

}

impl Particle{
  pub fn new(x: f64, y: f64, vx: f64, vy: f64,  size: f64 , color: String, line_color: String)-> Particle {
    Particle{
      x,
      y,
      vx,
      vy,
      color,
      size,
      lineColor: line_color,
    }
  }

  pub fn draw(&self, ctx:&web_sys::CanvasRenderingContext2d) {
    ctx.begin_path();
    ctx.set_fill_style(&JsValue::from(String::from(&self.color)));
    ctx.arc(self.x, self.y, self.size, 0f64, 2f64*PI).expect("arc error");
    ctx.fill();
  }

  pub fn update(&mut self, canvas_width: f64, canvas_height: f64) {
    if (self.x + self.size) >= canvas_width || (self.x - self.size) <= 0f64 {
      self.vx = -(self.vx);
    }
    if (self.y + self.size) >= canvas_height || (self.y - self.size) <= 0f64 {
        self.vy = -(self.vy)
    }
    self.x += self.vx;
    self.y += self.vy;
  }
}

pub fn random2(range1: f64, range2: f64)->f64{
  // let secret_number: f64 = rand::random();
  return (random() * (range2 - range1) + range1).floor();
}

#[wasm_bindgen(start)]
pub fn start()->Result<(), JsValue> {
  let window = web_sys::window().expect("get window error");
  let document = window.document().expect("get document error");

  let dom = document.get_element_by_id("myCanvas").expect("get dom error").dyn_into::<web_sys::HtmlCanvasElement>()?;

  let ctx = dom.get_context("2d")?.unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;
  let width = 1000f64;
  let height = 500f64;

  // NOTE: 初始化圆点队列
  let mut particle_list:Vec<Particle> = Vec::new();
  for i in 0..190 {
    let circle = Particle::new(
        random2(0f64, width),
        random2(0f64, height),
        random2(-6f64, 6f64) * (1f64 / 3.0),
        random2(-6f64, 6f64) * (1f64 / 3.0),
        3f64,
        "rgb(255,255,255)".to_string(),
        // format!("rgba(37,241,44")
        format!("rgba({},{},{}",random2(0f64, 255f64) as i64, random2(0f64, 255f64) as i64, random2(0f64,255f64) as i64)
    );
    particle_list.push(circle);
  }

  loops(Rc::new(ctx), particle_list);

  Ok(())
}

pub fn loops(ctx: Rc<web_sys::CanvasRenderingContext2d>, mut list: Vec<Particle>) {
  let f_fn = Rc::new(RefCell::new(None));
  let g_fn = Rc::clone(&f_fn);

  *g_fn.borrow_mut() = Some(Closure::wrap(Box::new(move||{
    
    ctx.set_fill_style(&JsValue::from(String::from("rgba(0,0,0, 0.6)")));
    ctx.fill_rect(0f64, 0f64, 1000f64, 500f64);

    for i in 0..list.len() {
        for j in 0..list.len() {
            if i == j {
              continue;
            }

            if (list[i].x - list[j].x).abs() > 180f64 || (list[i].x - list[j].x).abs() > 180f64 {
              continue;
            }

            let lx = list[j].x - list[i].x;
            let ly = list[j].y - list[i].y;
            let LL = (lx.powf(2.0) + ly.powf(2.0)).sqrt();
            //比对：当距离满足时，绘制线条，以rgba实现过渡  
            // log(&format!("{},{})", list[i].lineColor ,(180f64 - LL)/ 180f64));
            if (LL <= 180f64) {
                ctx.begin_path();
                ctx.set_stroke_style(&JsValue::from(format!("{},{})", list[i].lineColor ,((180f64 - LL)/ 180f64).abs())));
                // ctx.set_stroke_style(&JsValue::from(format!("rgba(255,255,255,{})", (180f64 - LL)/ 180f64)));
                ctx.move_to(list[i].x, list[i].y);
                ctx.set_line_width(0.5f64);
                ctx.line_to(list[j].x, list[j].y);
                ctx.stroke()
            }
        }
        list[i].draw(&ctx);
        list[i].update(1000f64, 500f64);
    }

    request_animetion_frame(f_fn.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));
  request_animetion_frame(g_fn.borrow().as_ref().unwrap());
}

pub fn window()->web_sys::Window{
  web_sys::window().expect("error in create window")
}

pub fn request_animetion_frame(fns: &Closure<dyn FnMut()>){
  window().request_animation_frame(fns.as_ref().unchecked_ref()).expect("animation error");
}