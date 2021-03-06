use core::prelude::*;
use Cheat;
use GamePointers;
use sdk;
use sdk::Entity;
use sdk::utils;
pub struct Spinbot {
	enabled: bool,
	
	currangle: f32
}


impl Cheat for Spinbot {
	fn new() -> Spinbot {
		Spinbot { enabled: false, currangle: 0.0 }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"Spinbot"
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		
		self.currangle = (self.currangle + 30.529)%360.0;
		
		let me = utils::get_local_player_entity(ptrs);
		
		unsafe { 
			if cmd.buttons & sdk::IN_ATTACK != 0 {
				if sdk::raw::c_baseplayer_isattacking(me) {
					return;
				} else {
					cmd.buttons &= !sdk::IN_ATTACK;
				}
			}
		}
		
		let oldviewangles = cmd.viewangles;
		
		cmd.viewangles.yaw = self.currangle;
		
		let (forwardmove, sidemove, upmove) = sdk::utils::rotate_movement((cmd.forwardmove, cmd.sidemove, cmd.upmove), oldviewangles, cmd.viewangles);
		cmd.forwardmove = forwardmove; cmd.sidemove = sidemove; cmd.upmove = upmove;
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}
