


use sand_game::*;
use sand_game::test_texture::TestTexture;
use input::InputBuffer;

use std::sync::Arc;

mod input;
use crate::input::*;

mod debug_tools;
use debug_tools::*;

mod falling_sand;
use falling_sand::*;

use std::time::{SystemTime, UNIX_EPOCH};
use crate::falling_sand::{elements::ElementType, screen_texture::*};

use winit::{
    dpi::{PhysicalPosition, PhysicalSize}, event::*, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowBuilder}
};


struct ResizeEvent {
    physical_size: PhysicalSize<u32>,
    dpi_scale_factor: f64,
    send_event: bool,
}

const ELEMENT_COLORS: [[u8; 4]; 7] = [COLORS_BACKGROUND, COLORS_YELLOW, COLORS_BROWN, COLORS_BLUE, COLORS_LIGHT_GREEN, COLORS_DARK_ORANGE, COLORS_GREY];
const ELEMENT_LIST: [ElementType; 7] = [ElementType::Empty, ElementType::Sand, ElementType::Dirt, ElementType::Water, ElementType::Seed, ElementType::Egg, ElementType::Wall];

struct SandPen {
    size: usize,
    min_size: usize,
    max_size: usize,
    granule_rate: usize,
    pub element_index: usize,
}

impl SandPen {
    pub fn new(size: usize, min_size: usize, max_size: usize, start_type: ElementType) -> Self {

        let mut element_index = 0;
        for i in 0..ELEMENT_LIST.len() {
            if start_type == ELEMENT_LIST[i] {
                element_index = i;
            }
        }

        Self {
            size,
            min_size,
            max_size,
            granule_rate: 1,
            element_index,
        }
    }
    pub fn set_element(&mut self, element: ElementType) {
        for i in 0..ELEMENT_LIST.len() {
            if element == ELEMENT_LIST[i] {
                self.element_index = i;
            }
        }
    }
}


fn main() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let window = Arc::new(window);
    let window_id = window.id();

    let mut window_size = window.inner_size();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        let _ = window.request_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut engine: GraphicsEngine = pollster::block_on(GraphicsEngine::new(&window));

    let mut surface_configured = false;


    let mut average_fps = 0.0;

    let mut is_in_frame_by_frame_mode = false;
    let mut play_next_frame = false;

    let mut input: InputBuffer = InputBuffer::new();

    let mut sand_board: SandBoard = SandBoard::new(200, 200);
    sand_board.boring_ocean();

    //sand_board.first_ten();
    //sand_board.middle();

    let mut sand_pen = SandPen::new(4, 1, 20, ElementType::Sand);

    let mut timers = DebugTools::new();
    let fps_timer = timers.add_timer();
    timers.get_time_reset(fps_timer);


    let mut sand_texture: ScreenTexture = sand_board.output_texture();
    let dimensions = [sand_texture.dimensions.0 as u32, sand_texture.dimensions.1 as u32];

    //let test_texture: TestTexture = TestTexture::new("happy-tree.png", &engine.device, &engine.queue, &engine.bind_group_layouts["texture"]);
    let test_texture: TestTexture = TestTexture::make_texture_from_raw_bytes("sand_board", &sand_texture.pixel_data, dimensions,  &engine.device, &engine.queue, &engine.bind_group_layouts["texture"]);
    let mut resize_event: Option<ResizeEvent> = None;

    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id,} 
                if window_id == window.id() => {
                    if !input.get_input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                event:
                                    KeyEvent {
                                        state: ElementState::Pressed,
                                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => control_flow.exit(),
                            WindowEvent::Resized(physical_size) => {
                                let scale_factor = window.scale_factor();
                                let physical_size = window.inner_size();

                                resize_event = Some (ResizeEvent {
                                    physical_size,
                                    dpi_scale_factor: scale_factor,
                                    send_event: true,
                                })
                            }
                            WindowEvent::RedrawRequested => {
                                // This tells winit that we want another frame after this one


                                if resize_event.is_some() {
                                    engine.resize(&resize_event.as_ref().unwrap().physical_size);
                                    resize_event = None;
                                }

                                window.request_redraw();

                                timers.get_time_reset(fps_timer);
                                let fps = timers.read_last_recorded_as_fps(fps_timer);
                                let fraction = 0.98;
                                average_fps = (average_fps * fraction) + (fps * (1.0 - fraction)) ;
                                let fps_string = format!("{:.1}", average_fps);
                                
                                update(&mut sand_board, &mut engine, &mut input, &mut sand_pen, &fps_string, &mut is_in_frame_by_frame_mode, &mut play_next_frame);

                                match engine.render() {
                                    Ok(_) => {}
                                    // Reconfigure the surface if it's lost or outdated
                                    Err(wgpu::SurfaceError::Lost) => {
                                        let temp_size = engine.screen_size; // gets around a borrowing error... sure whatever
                                        engine.resize(&temp_size)
                                    },
                                    // The system is out of memory, we should probably quit
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        log::error!("OutOfMemory");
                                        control_flow.exit();
                                    }

                                    // This happens when the a frame takes too long to present
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout")
                                    },
                                    Err(e) => eprintln!("{:?}", e),
                                }
                            }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }).unwrap();


}


fn update(sand_board: &mut SandBoard, engine: &mut GraphicsEngine, input: &mut InputBuffer, sand_pen: &mut SandPen, fps_string: &String, frame_mode: &mut bool, advance_frame: &mut bool) {
    let mut added_granules = 0;

    if input.is_up_pressed {
        sand_pen.size += 1;
        if sand_pen.size > sand_pen.max_size {
            sand_pen.size = sand_pen.max_size
        }
    }
    if input.is_down_pressed {
        sand_pen.size -= 1;
        if sand_pen.size < sand_pen.min_size {
            sand_pen.size = sand_pen.min_size
        }
    }

    if input.is_1_pressed {
        sand_pen.set_element(ElementType::Sand);
        //println!("Like sand through an hour glass");
    }
    if input.is_2_pressed {
        sand_pen.set_element(ElementType::Water);
        //println!("I feel so empty inside");
    }
    if input.is_3_pressed {
        sand_pen.set_element(ElementType::Egg);
        //println!("The darkest depths call");
    }
    if input.is_4_pressed {
        sand_pen.set_element(ElementType::Empty);
        //println!("The darkest depths call");
    }
    if input.is_5_pressed {
        sand_pen.element_index = 4;
        //println!("The darkest depths call");
    }
    if input.is_q_pressed {
        sand_pen.element_index += 1;
        if sand_pen.element_index >= ELEMENT_LIST.len() {
            sand_pen.element_index = 0;
        }
    }
    if input.is_a_pressed {
        if sand_pen.element_index <= 0 {
            sand_pen.element_index = ELEMENT_LIST.len() - 1;
        }
        else {
            sand_pen.element_index -= 1;
        }  
    }

    if input.is_r_pressed {
        sand_board.reset();
    }

    let mut starting_granules = sand_board.get_granule_count();
    if input.is_left_clicked {
        let (x, y) = get_granule_index(sand_board, engine, input);
        let mut size = sand_pen.size;
        if ELEMENT_LIST[sand_pen.element_index] == ElementType::Seed {
            size = 1;
        }
        //println!("{} {}", x, y);
        if x > 0.0 && y > 0.0 {
            added_granules = sand_board.add_granules(x as usize, y as usize, size, ELEMENT_LIST[sand_pen.element_index as usize]);

        }
    }


    

    let mut before_tick_granules = sand_board.get_granule_count();

     
    if input.is_space_pressed {
        if sand_board.is_floor {
            sand_board.is_floor = false;
            //sand_board.is_floor = true;
        }
        else {
            sand_board.is_floor = true;
        }
           
    }

    if input.is_x_pressed {
        if *frame_mode {
            *frame_mode = false;
        }
        else {
            *frame_mode = true;
        }
        

    }

    if *frame_mode {
        if input.is_z_pressed {
            *advance_frame = true;
        }
    }
    
    if *frame_mode {
        if *advance_frame {
            sand_board.tick();
            *advance_frame = false;
        }
    }
    else {
        sand_board.tick();
    }
    

    let ending_granules = sand_board.get_granule_count();
    let granule_count_string = format!("{}", ending_granules);

    let mut sand_texture: ScreenTexture = sand_board.output_texture();
    sand_texture.print(fps_string.to_string(), (0,0));
    sand_texture.print(granule_count_string.to_string(), (0,1));

    let pen_size_string = format!("{}", sand_pen.size);
    sand_texture.print(pen_size_string.to_string(), (0,2));


     //println!("added: {} start: {} after_add: {} end: {} {} {}", added_granules, starting_granules, before_tick_granules, ending_granules, sand_board.left_sand, sand_board.right_sand);

    let (x, y) = get_granule_index(sand_board, engine, input);
        //println!("{} {}", x, y);
    if x > 0.0 && y > 0.0 {
        //sand_board.add_granule(x as usize, y as usize);
        let surrounding_data = sand_board.get_surrounding(x as usize, y as usize);
        sand_texture.encode_debug_info(surrounding_data);
    }

    let ui_position = (30, 3);
    draw_selection_bar(input.mouse_pos, input.is_left_clicked, sand_pen.element_index, ui_position, &mut sand_texture);

    input.reset_input();
    sand_board.left_sand = 0; 
    sand_board.right_sand = 0;
    
    let dimensions = [sand_texture.dimensions.0 as u32, sand_texture.dimensions.1 as u32];

    //let test_texture: TestTexture = TestTexture::new("happy-tree.png", &engine.device, &engine.queue, &engine.bind_group_layouts["texture"]);
    let test_texture: TestTexture = TestTexture::make_texture_from_raw_bytes("sand_board", &sand_texture.pixel_data, dimensions,  &engine.device, &engine.queue, &engine.bind_group_layouts["texture"]);
    engine.draw_objects.insert("sand".to_string(), test_texture);


    //let test_texture: TestTexture = TestTexture::new("Invader1.png", &engine.device, &engine.queue, &engine.bind_group_layouts["texture"]);
    // let test_texture: TestTexture = TestTexture::make_texture_from_raw_bytes("sand_board", &sand_texture.pixel_data, dimensions,  &engine.device, &engine.queue, &engine.bind_group_layouts["texture"]);
    //engine.draw_objects.insert("sand".to_string(), test_texture);


}


fn get_granule_index(sand_board: &SandBoard, engine: &GraphicsEngine, input: &InputBuffer) -> (f64, f64) {

    // add offset x
    // add offset y


    let x_offset = engine.screen_size.width / 8;
    let y_offset = engine.screen_size.height / 8;

    let x_scaled = (sand_board.width as f64 / engine.screen_size.width as f64) / 1.5;
    let y_scaled = (sand_board.height as f64 / engine.screen_size.height as f64) / 1.5;

    let x_adj = (input.mouse_pos.x - x_offset as f64) * x_scaled * 2.0;
    let y_adj = (input.mouse_pos.y - y_offset as f64) * y_scaled * 2.0;

    return (x_adj, y_adj)
}

fn draw_selection_bar(mouse_pos: PhysicalPosition<f64>, clicked: bool, selected: usize, position: (usize, usize), sand_texture: &mut ScreenTexture) -> usize {
    let rect_size = (10, 10);
    let margin = 5;
    for i in 0..ELEMENT_LIST.len() {
        let x = position.0 + (i *(rect_size.0) + margin); 
        let is_selected = selected == i;
        input_box(mouse_pos, clicked, (x as i64, position.1 as i64), is_selected, rect_size, sand_texture, ELEMENT_COLORS[i]);
    }
    return 0;
}

fn input_box(mouse_pos: PhysicalPosition<f64>, clicked: bool, top_left: (i64, i64), selected: bool, size: (usize, usize), sand_texture: &mut ScreenTexture, color: [u8; 4]) -> bool {
    let margin = 2;
    let filled_rect_position = (top_left.0 as usize + margin, top_left.1 as usize + margin);
    let filled_rect_size = (size.0 - (margin * 2), size.1 - (margin * 2));
    sand_texture.print_filled_rect(filled_rect_position, filled_rect_size, color);

    if selected {
        sand_texture.print_empty_rect((top_left.0 as usize, top_left.1 as usize), size, COLORS_WHITE);
    }

    if is_hovered(mouse_pos, top_left, size) && clicked {
        sand_texture.print_empty_rect((top_left.0 as usize, top_left.1 as usize), size, COLORS_WHITE);
        return true;
    }
    return false;
}


fn is_hovered(mouse_pos: PhysicalPosition<f64>, top_left: (i64, i64), size: (usize, usize)) -> bool {
    if (mouse_pos.x > top_left.0 as f64 && (mouse_pos.x + size.0 as f64) < top_left.0 as f64) && 
        (mouse_pos.y > top_left.1 as f64 && (mouse_pos.y + size.1 as f64) < top_left.1 as f64) {
            return true;
    }
    return false;
}