use amethyst::{
    core::Time,
    ecs::{Join, WriteStorage},
    winit::{KeyboardInput, VirtualKeyCode, WindowEvent},
    SimpleTrans, Trans,
};

use crate::vehicle::Vehicle;

pub struct VehicleController;

impl VehicleController {
    pub fn handle_window_event(
        window_event: WindowEvent,
        vehicles: &mut WriteStorage<Vehicle>,
        time: &Time,
    ) -> SimpleTrans {
        match window_event {
            WindowEvent::KeyboardInput { input, .. } => {
                Self::handle_keyboard_input(input, vehicles, time)
            }
            // Handle other window events here if necessary
            _ => Trans::None,
        }
    }

    fn handle_keyboard_input(
        input: KeyboardInput,
        vehicles: &mut WriteStorage<Vehicle>,
        time: &Time,
    ) -> SimpleTrans {
        if let Some(key_code) = input.virtual_keycode {
            Self::process_key_press(key_code, vehicles, time)
        } else {
            Trans::None
        }
    }

    fn process_key_press(
        key_code: VirtualKeyCode,
        vehicles: &mut WriteStorage<Vehicle>,
        time: &Time,
    ) -> SimpleTrans {
        for vehicle in (&mut *vehicles).join() {
            let delta_time = time.delta_seconds();
            match key_code {
                VirtualKeyCode::Up => vehicle.accelerate(delta_time),
                VirtualKeyCode::Down => vehicle.decelerate(delta_time),
                VirtualKeyCode::Left => vehicle.turn_left(delta_time),
                VirtualKeyCode::Right => vehicle.turn_right(delta_time),
                VirtualKeyCode::Escape => return Trans::Quit,
                _ => (),
            }
            vehicle.update_position(delta_time);
        }
        Trans::None
    }
}
