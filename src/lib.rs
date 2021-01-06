mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo::{events::EventListener};
use std::fmt;


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
#[derive(Clone,PartialEq)]
enum Cell{
    Alive,
    Dead,
    Stone
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Cell::Alive => "Alive",
            Cell::Dead => "Dead",
            Cell::Stone => "Stone"
        };
        write!(f, "{}", printable)
    }
}
#[derive(Clone,PartialEq,Copy)]
enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Copy,Clone)]
struct Bullet{
    direction:Direction,
    colum:u32,
    is_alive:bool
}

impl Bullet{
    pub fn new(x:u32,dir:Direction)->Self{
        Self{
            direction:dir,
            colum:x,
            is_alive:true
        }
    }

    pub fn move_on(&mut self,width:u32,height:u32,array: & Vec<Cell>){
        match &self.direction{
            UP =>{
                    self.colum-=width*2;
                    if self.colum<0{
                        self.is_alive=false;
                    }
                    else{
                         if Cell::Stone==array[self.colum as usize]|| Cell::Stone==array[(self.colum-width) as usize]{
                             self.is_alive=false;
                        }
                    }
            },
            DOWN=>{ self.colum+=width*2;
                if self.colum<0{
                    self.is_alive=false;
                }
                else{
                if Cell::Stone==array[self.colum as usize]|| Cell::Stone==array[(self.colum+width) as usize]{
                    self.is_alive=false;
                }
            }
            },
            LEFT=>{ self.colum-=2;
                if self.colum<0{
                    self.is_alive=false;
                }
                else{
                if Cell::Stone==array[self.colum as usize]|| Cell::Stone==array[(self.colum-1) as usize]{
                    self.is_alive=false;
                }
            }
            },
            RIGHT=>{ self.colum+=2; 
                if self.colum<0{
                    self.is_alive=false;
                }
                else{
                if Cell::Stone==array[self.colum as usize]|| Cell::Stone==array[(self.colum+1) as usize]{
                    self.is_alive=false;
                }
            }
            }

        }
    }
}

#[wasm_bindgen]
struct block{
    window:web_sys::Window,
    canvas:web_sys::HtmlCanvasElement,
    width:u32,
    height:u32,
    array:Vec<Cell>,
    tanks:Vec<Tanker>,
    hero:Tanker,
    old_hero:Tanker,
}
#[wasm_bindgen]
struct Tanker{
    x:usize,
    body: Vec<u32>,
    bullet:Vec<Bullet>,
    width:u32,
    height:u32,
    trigger:u32,
    direction:Direction,
}

impl Tanker{
    pub fn new(pos:u32,width:u32,height:u32,dir:Direction)->Self{
        Self{
            x:pos as usize,
            body:
            {   
                if Direction::LEFT==dir{
                    vec![pos,pos-1,pos-2,pos+width-1,pos-width-1,pos+width+1,pos-width+1]
                }
                else if Direction::RIGHT==dir{
                    vec![pos,pos+1,pos+2,pos+width+1,pos-width+1,pos+width-1,pos-width-1]
                }
                else if Direction::UP==dir{
                    vec![pos,pos-width,pos-(width*2),pos-width+1,pos-width-1,pos+width-1,pos+width+1]
                }
                else{
                    vec![pos,pos+width,pos+(width*2),pos+width+1,pos+width-1,pos-width-1,pos-width+1]
                }
            },
            bullet:vec![],
            width:width,
            height:height,
            trigger:0,
            direction:dir
        }
    }

    pub fn tank_move(&mut self,array: & Vec<Cell>){
       

        self.trigger+=1;
        if self.trigger==100{
            self.bullet.push(Bullet::new(self.body[0],self.direction));
            self.trigger=0;
        }


        for blk in self.body.iter_mut(){
         
            if Direction::RIGHT==self.direction{
                    // 
                *blk+=1; 
            }
            else if Direction::LEFT==self.direction{
                *blk-=1;
            }
            else if Direction::UP==self.direction{
                *blk-=self.width;
            }
            else if Direction::DOWN==self.direction{
                *blk+=self.width;
            }
           
        }
      
       
        if  Cell::Stone==*array.get((self.body[2]+1) as usize).unwrap(){
              //move right
            //   console_log!("h1");
            self.change_direction(3,array);
           
        }
        else if Cell::Stone==*array.get((self.body[2]+self.width) as usize).unwrap(){
            //move down
            // console_log!("hdown");
            self.change_direction(1,array);
        }
        else  if  Cell::Stone==*array.get((self.body[2]-1) as usize).unwrap(){
              //move left
            // console_log!("hleft");
            self.change_direction(4,array);
        }
        else  if Cell::Stone==*array.get((self.body[2]-self.width) as usize).unwrap(){
            //move up
            // console_log!("hup");
            self.change_direction(2,array);
        }

       for k in self.bullet.iter_mut(){
           k.move_on(self.width,self.height,array);
       }
    }

    pub fn change_direction(&mut self,mut dir:i8,array: & Vec<Cell>){
        
        if dir==1{
            if Cell::Stone==*array.get((self.body[0]-3) as usize).unwrap(){
                dir=2;
            }
            self.body[1]= self.body[0]-1;
            self.body[2]= self.body[0]-2;
            self.body[3]= self.body[0]+self.width-1;
            self.body[4]= self.body[0]-self.width-1;
            self.body[5]= self.body[0]+self.width+1;
            self.body[6]= self.body[0]-self.width+1;
            self.direction=Direction::LEFT;
        }
        if dir==2{
            if Cell::Stone==*array.get((self.body[0]+3) as usize).unwrap(){
                dir=3;
            }
            self.body[1]= self.body[0]+1;
            self.body[2]= self.body[0]+2;
            self.body[3]= self.body[0]+self.width+1;
            self.body[4]= self.body[0]-self.width+1;
            self.body[5]= self.body[0]+self.width-1;
            self.body[6]= self.body[0]-self.width-1;
            self.direction=Direction::RIGHT;

        }
        if dir==3{
            if Cell::Stone==*array.get((self.body[0]-self.width*2) as usize).unwrap(){
                dir=4;
            }
            self.body[1]= self.body[0]-(self.width);
            self.body[2]= self.body[0]-(self.width*2);
            self.body[3]= self.body[0]-(self.width)+1;
            self.body[4]= self.body[0]-(self.width)-1;
            self.body[5]= self.body[0]+(self.width)-1;
            self.body[6]= self.body[0]+(self.width)+1;
            
            self.direction=Direction::UP;
        }
        if dir==4{
            if Cell::Stone==*array.get((self.body[0]+self.width*2) as usize).unwrap(){
                dir=1;
            }
            self.body[1]= self.body[0]+(self.width);
            self.body[2]= self.body[0]+(self.width*2);
            self.body[3]= self.body[0]+(self.width)+1;
            self.body[4]= self.body[0]+(self.width)-1;
            self.body[5]= self.body[0]-(self.width)-1;
            self.body[6]= self.body[0]-(self.width)+1;
          
            self.direction=Direction::DOWN;
        }


    }

    pub fn get_cell(&self)->u32{
        self.body[0]
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

      


       
        
        let canvass: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

        let width=window.inner_width().unwrap();
        let height=window.inner_height().unwrap();
        let uwidth=width.as_f64().unwrap();
        let uheight=height.as_f64().unwrap();
        let uwidth:u32=(uwidth as u32)/10u32;
        let uheight:u32=(uheight as u32)/10u32;
        let mut ar:Vec<Cell>=Vec::new();

        


        for y in 0..uheight*uwidth{
        //    if(y%2==0){
           
            if y/uwidth==0|| y%uwidth==0 ||y/uwidth==uheight-1||(y+1)%uwidth==0||((y+50)%uwidth==0&&y/uwidth<uheight-50){
                ar.push(Cell::Stone);
            }
            else{
                ar.push(Cell::Dead);
            }
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
            tanks:vec![Tanker::new(700,uwidth as u32,uheight as u32,Direction::LEFT),Tanker::new(2050,uwidth as u32,uheight as u32,Direction::RIGHT),Tanker::new(3050,uwidth as u32,uheight as u32,Direction::UP)],
            // tanks:vec![],
            hero:Tanker::new(600,uwidth as u32,uheight as u32,Direction::RIGHT),
            old_hero:Tanker::new(600,uwidth as u32,uheight as u32,Direction::RIGHT),

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
     for k in self.tanks.iter(){
        for j in k.bullet.iter(){
         //    console_log!("values tanks {}",*j);
          self.array[j.colum as usize]=Cell::Dead;
        }
     // self.array[self.tanks[k].x]=Cell::Alive;
     }
     for j in self.hero.body.iter(){
        //    console_log!("values tanks {}",*j);
         self.array[*j as usize]=Cell::Dead;
       }

   

    for ik in self.tanks.iter_mut(){
        ik.tank_move(&self.array);
    }

   for k in self.tanks.iter(){
       for j in k.body.iter(){
        //    console_log!("values tanks {}",*j);
         self.array[*j as usize]=Cell::Alive;
       }
    // self.array[self.tanks[k].x]=Cell::Alive;
    }

    for k in self.tanks.iter(){
        for j in k.bullet.iter(){
         //    console_log!("values tanks {}",*j);
          self.array[j.colum as usize]=Cell::Alive;
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
    // self.bullets=self.bullets.iter_mut().filter(|v|{ 
    //     v.is_alive
    // }).map(|v|{*v}).collect::<Vec<_>>();

    
    
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
            if Cell::Alive==self.array[ii]||Cell::Stone==self.array[ii]{
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
        // console_log!("direction {}",self.hero.body[1]);
        // let pos:&u32= self.body.first().unwrap();
        let mut dir=0i32;
        let width=self.width;
      for i in 0..self.hero.body.len(){
        self.old_hero.body[i]=self.hero.body[i];
      }
        if p==65{
        // //    pos,pos+1,pos+2,pos+width+1,pos-width+1,pos+width-1,pos-width-1
           
           
        //    console_log!("val l {:?} {}",self.hero.body,self.width);
        self.hero.body[1]= self.hero.body[0]-1;
        self.hero.body[2]= self.hero.body[0]-2;
        self.hero.body[3]= self.hero.body[0]+self.width-1;
        self.hero.body[4]= self.hero.body[0]-self.width-1;
        self.hero.body[5]= self.hero.body[0]+self.width+1;
        self.hero.body[6]= self.hero.body[0]-self.width+1;
           if (self.hero.body[0])%self.width==0||Direction::LEFT!=self.hero.direction{
            // console_log!("hit left");
                dir=0;
            }
            else{
                dir=-1i32;
             }
             self.hero.direction=Direction::LEFT;

        }
        else if p==68{
            self.hero.body[1]= self.hero.body[0]+1;
            self.hero.body[2]= self.hero.body[0]+2;
            self.hero.body[3]= self.hero.body[0]+self.width+1;
            self.hero.body[4]= self.hero.body[0]-self.width+1;
            self.hero.body[5]= self.hero.body[0]+self.width-1;
            self.hero.body[6]= self.hero.body[0]-self.width-1;
            // console_log!("val r {:?} {}",self.hero.body,self.width);
          
           
            if (self.hero.body[2]+1)%self.width==0||Direction::RIGHT!=self.hero.direction{
                // console_log!("hit right");
                dir=0;
            }
            else{
                dir=1i32;
             }
             self.hero.direction=Direction::RIGHT;

           
        }
        else if p==87{
            self.hero.body[1]= self.hero.body[0]-(self.width);
            self.hero.body[2]= self.hero.body[0]-(self.width*2);
            self.hero.body[3]= self.hero.body[0]-(self.width)+1;
            self.hero.body[4]= self.hero.body[0]-(self.width)-1;
            self.hero.body[5]= self.hero.body[0]+(self.width)-1;
            self.hero.body[6]= self.hero.body[0]+(self.width)+1;
            // console_log!("val {:?} {}",self.hero.body,self.width);
        
            if (self.hero.body[4]/self.width)==0||Direction::UP!=self.hero.direction{
                dir=0;
            }
            else{
                dir=-1i32*(self.width as i32);
            }
            self.hero.direction=Direction::UP;

        }
        else if p==83{
            self.hero.body[1]= self.hero.body[0]+(self.width);
            self.hero.body[2]= self.hero.body[0]+(self.width*2);
            self.hero.body[3]= self.hero.body[0]+(self.width)+1;
            self.hero.body[4]= self.hero.body[0]+(self.width)-1;
            self.hero.body[5]= self.hero.body[0]-(self.width)-1;
            self.hero.body[6]= self.hero.body[0]-(self.width)+1;
            // console_log!("val {:?} {} {}",self.hero.body,(self.hero.body[3]/self.width),self.height-1);
      
            if (self.hero.body[3]/self.width)==self.height-1||Direction::DOWN!=self.hero.direction{
                dir=0;
            }
            else{
                dir=self.width as i32;
            }
            self.hero.direction=Direction::DOWN;
           
        }
      
        for j in self.old_hero.body.iter(){
            self.array[*j as usize]=Cell::Dead;
           }

        self.hero.body=self.hero.body.iter_mut().map(|v|{*v+dir as u32}).collect();
       

        
    }


}



