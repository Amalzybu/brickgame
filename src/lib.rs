mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo::{events::EventListener};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    // let canvas=documen
    // Manufacture the element we're gonna append
    let val = document.create_element("canvas")?;
    let canvass = document.get_element_by_id("canvas").unwrap();
    // val.set_inner_html("Hello from Rust!");

    // body.append_child(&val)?;
    let canvas: web_sys::HtmlCanvasElement = canvass
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();

 

    let windows: web_sys::Window =web_sys::window().unwrap();
    let width=windows.inner_width()?;
    let height=windows.inner_height()?;
    let uwidth=width.as_f64().unwrap();
    let uheight=height.as_f64().unwrap();
    // console_log!("createdddd module has {}  {} pages of memory",uwidth,uheight );
    let context = canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap();
    // context.width=windows.innerWidth;
    canvas.set_width(uwidth as u32);
    canvas.set_height(uheight as u32);
//context.set_fill_style(&"#0000FF".into());     
    context.set_fill_style(&"#8fba84".into());        

    context.fill_rect(0.0, 0.0, uwidth, uheight);

    context.set_fill_style(&"#076ab0".into());
     let mut x1:f64=100.0;
     let mut y1:f64=100.0;
    context.fill_rect(x1, y1 , 100.0,100.0);
   
    
            x1+=20.0;
            y1+=20.0;
            context.set_fill_style(&"#076ab0".into());
            context.fill_rect(y1, x1 , 100f64,100f64);
            // console_log!("xxxxx yyyyy xxxxxx");
    
    // for i in 1..5{
    //     let temp=(i*100)as f64;
    //     context.fill_rect(temp, temp , 100.00, 100.00);
    // }
  
    // web_sys::console::log_2(&"Color : %s ".into(),&context.fill_style().into());



    Ok(())
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, brick-tankgame!");
}
#[derive(Clone)]
enum Cell{
    Alive,
    Dead
}

#[wasm_bindgen]
struct block{
    window:web_sys::Window,
    canvas:web_sys::HtmlCanvasElement,
    width:u32,
    height:u32,
    array:Vec<Cell>,
    tanks:Vec<Tanker>,
    hero:Tanker
}
#[wasm_bindgen]
struct Tanker{
    x:usize,
    body: Vec<u32>,
    bullet:Vec<u32>,
}
#[wasm_bindgen]
impl Tanker{
    pub fn new(pos:u32,width:u32,height:u32)->Self{
        Self{
            x:pos as usize,
            body:vec![pos,pos+1,pos+2,pos+width+1,pos-width+1,pos+width-1,pos-width-1],
            bullet:vec![]
        }
    }

    pub fn tank_move(&mut self){
        for blk in self.body.iter_mut(){
         
            *blk+=1; 
        }
    }

    pub fn change_direction(&mut self,p:u32){
        if p==68{
           let pos:&u32= self.body.first().unwrap();
           self.body.clear();
           self.body.push(*pos)

        }

    }

}

#[wasm_bindgen]
impl block{
    pub fn new()->block{
        utils::set_panic_hook();
        let windows: web_sys::Window =web_sys::window().unwrap();
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap();

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
           console_log!("hello world");
           
        }) as Box<dyn FnMut(_)>);


       
        
        let canvass: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

        canvass.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        closure.forget(); 
        let width=window.inner_width().unwrap();
        let height=window.inner_height().unwrap();
        let uwidth=width.as_f64().unwrap();
        let uheight=height.as_f64().unwrap();
        let uwidth:u32=(uwidth as u32)/10u32;
        let uheight:u32=(uheight as u32)/10u32;
        let mut ar:Vec<Cell>=Vec::new();

 


        for y in 0..uheight*uwidth{
        //    if(y%2==0){
            ar.push(Cell::Dead);
        //    }
        //    else{
        //        ar.push(Cell::Alive);
        //    }
        }

        Self{
            window:windows,
            canvas:canvass,
            width:uwidth as u32,
            height:uheight as u32,
            array:ar,
            tanks:vec![Tanker::new(500,uwidth as u32,uheight as u32),Tanker::new(200,uwidth as u32,uheight as u32)],
            hero:Tanker::new(600,uwidth as u32,uheight as u32)

        }
    }


    pub fn draw(&mut self){
    
    // self.array.clear();
    for k in self.tanks.iter(){
        for j in k.body.iter(){
         //    console_log!("values tanks {}",*j);
          self.array[*j as usize]=Cell::Dead;
        }
     // self.array[self.tanks[k].x]=Cell::Alive;
     }

     for j in self.hero.body.iter(){
        //    console_log!("values tanks {}",*j);
         self.array[*j as usize]=Cell::Dead;
       }

    for ik in self.tanks.iter_mut(){
        ik.tank_move();
    }

   for k in self.tanks.iter(){
       for j in k.body.iter(){
        //    console_log!("values tanks {}",*j);
         self.array[*j as usize]=Cell::Alive;
       }
    // self.array[self.tanks[k].x]=Cell::Alive;
    }

    for j in self.hero.body.iter(){
        //    console_log!("values tanks {}",*j);
         self.array[*j as usize]=Cell::Alive;
       }
    // for y in 0..self.height*self.width{
    //        if(y%2==0){
    //         self.array.push(Cell::Dead);
    //        }
    //        else{
    //         self.array.push(Cell::Alive);
    //        }
    //     }
    
    }

        
        
    

    pub fn clear_background(&self){
   

    
        let width=self.window.inner_width().unwrap();
        let height=self.window.inner_height().unwrap();
        let uwidth=width.as_f64().unwrap();
        let uheight=height.as_f64().unwrap();
        // console_log!("createdddd module has {}  {} pages of memory",uwidth,uheight );
        let context = self.canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
        // context.width=windows.innerWidth;
    //context.set_fill_style(&"#0000FF".into());     
        context.set_fill_style(&"#8fba84".into());        
    
        context.fill_rect(0.0, 0.0, uwidth, uheight);
        context.set_fill_style(&"#000000".into());        
        

        for ii in 0..self.array.len(){
            if let Cell::Alive=self.array[ii]{
                // console_log!("values tanks {} {} {}",ii,(ii%self.width  as usize),(ii%self.height as usize));
                context.set_fill_style(&"#000000".into());        
                context.fill_rect((10.0*(ii%self.width  as usize) as f64)+1f64, (10.0 *(ii/self.width as usize) as f64)+1f64, 9f64, 9f64);
            }
            else{
                context.set_fill_style(&"#4e825f".into());
                context.fill_rect((10.0*(ii%self.width as usize) as f64)+1f64, (10.0 *(ii/self.width as usize) as f64)+1f64, 9f64, 9f64);
            }
        }

        // context.fill_rect(20.0, 20.0, 10f64, 10f64);
        // context.fill_rect(20.0, 31.0, 10f64, 10f64);
    }
    pub fn mov_dir(&mut self,p:u32){
        console_log!("direction {}",p);
        
    }


}



