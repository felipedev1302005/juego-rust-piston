// Importación de las bibliotecas necesarias
extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;

// Importaciones específicas de los módulos que vamos a usar
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{Button, Input, Key, RenderEvent};
use piston::window::WindowSettings;
use piston::{ButtonArgs, ButtonState, PressEvent};

// Definición de la estructura GameState para mantener el estado del juego
struct GameState {
    x: f64,
    y: f64,
}

impl GameState {
    // Implementación de un método constructor para GameState
    fn new() -> Self {
        GameState { x: 100.0, y: 100.0, }
    }

    // Implementación de un método para actualizar el estado del juego
    fn update(&mut self, e: &Input) {
        // Definición de la velocidad de movimiento del rectángulo
        const SPEED: f64 = 20.0;
        
        // Verificación de si el evento es un evento de botón
        if let Input::Button(ButtonArgs { state, button, .. }) = e {
            // Verificación de si el botón presionado es una tecla del teclado
            if let Button::Keyboard(key) = button {
                // Manejo del movimiento del rectángulo según la tecla presionada
                match key {
                    Key::Up => {
                        if *state == ButtonState::Press {
                            self.y -= SPEED;
                        }
                    }
                    Key::Down => {
                        if *state == ButtonState::Press {
                            self.y += SPEED;
                        }
                    }
                    Key::Left => {
                        if *state == ButtonState::Press {
                            self.x -= SPEED;
                        }
                    }
                    Key::Right => {
                        if *state == ButtonState::Press {
                            self.x += SPEED;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    // Configuración de OpenGL y la ventana del juego
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Movimiento con teclado", [640, 480])
        .graphics_api(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    // Inicialización del contexto de OpenGL y el objeto GlGraphics
    let mut gl = GlGraphics::new(opengl);

    // Configuración del bucle de eventos
    let mut events = Events::new(EventSettings::new().lazy(true));

    // Inicialización del estado del juego
    let mut game_state = GameState::new();

    // Bucle principal del juego
    while let Some(e) = events.next(&mut window) {
        // Manejo de eventos de renderizado
        if let Some(args) = e.render_args() {
            // Dibujar la escena del juego
            gl.draw(args.viewport(), |c, g| {
                use graphics::*;

                // Limpia la ventana con un color de fondo blanco
                clear([1.0, 1.0, 1.0, 1.0], g);

                // Dibuja un rectángulo rojo en la posición actualizada
                let red = [1.0, 0.0, 0.0, 1.0];
                rectangle(
                    red,
                    [game_state.x, game_state.y, 100.0, 100.0],
                    c.transform,
                    g
                );
            });
        }

        // Manejo de eventos de presión de teclas
        if let Some(Button::Keyboard(key)) = e.press_args() {
            // Verificación de si la tecla presionada es una tecla de flecha
            match key {
                Key::Up | Key::Down | Key::Left | Key::Right => {
                    // Actualiza el estado del juego según la tecla presionada
                    game_state.update(&Input::Button(ButtonArgs {
                        state: ButtonState::Press,
                        button: Button::Keyboard(key),
                        scancode: None,
                    }));
                }
                _ => {}
            }
        }
    }
}