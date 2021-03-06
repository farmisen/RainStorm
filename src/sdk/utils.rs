use core::prelude::*;
use GamePointers;
use super::{get_tracefilter, IClientEntityList, trace_t, Ray_t, QAngle,
	Entity, Animating, raw, Vector};
use sdk;
use libc;
pub fn rotate_movement(movement: (f32, f32, f32), orig_angles: QAngle, new_angles: QAngle) -> (f32, f32, f32) {
	let (fwd, right, up) = movement;
	let (orig_fwd, orig_right, orig_up) = orig_angles.to_vectors();
	let (orig_fwdnorm, orig_rightnorm, orig_upnorm) = (orig_fwd.norm(), orig_right.norm(), orig_up.norm());
	let (new_fwd, new_right, new_up) = new_angles.to_vectors();
	
	(
		fwd * new_fwd.dotproduct(&orig_fwd) + right * new_fwd.dotproduct(&orig_rightnorm) + up * new_fwd.dotproduct(&orig_upnorm),
		fwd * new_right.dotproduct(&orig_fwdnorm) + right * new_right.dotproduct(&orig_rightnorm) + up * new_right.dotproduct(&orig_upnorm),
		fwd * new_up.dotproduct(&orig_fwdnorm) + right * new_up.dotproduct(&orig_rightnorm) + up * new_up.dotproduct(&orig_upnorm)
	)
}
	
	
pub fn get_local_player_entity(ptrs: &GamePointers) -> raw::C_BaseEntityPtr {
	let localplayer_entidx = ptrs.ivengineclient.get_local_player();
	ptrs.icliententitylist.get_client_entity(localplayer_entidx).expect("Local player entity not found!")
}
pub fn predict(ptrs: &GamePointers, cmd: &super::CUserCmd) { 
	let mut me = get_local_player_entity(ptrs);
	let mut tempcmd = *cmd;
	tempcmd.buttons = cmd.buttons & !sdk::IN_ATTACK;
	match ptrs.globals {
		Some(globals) => {
			unsafe {
				let flags: u32 = *me.ptr_offset(0x0378);
				sdk::IPrediction::from_ptr(sdk::raw::getptr_iprediction()).run_command(Entity::from_ptr(me), &tempcmd);
				*me.mut_ptr_offset(0x0378) = flags;
			}
		},
		None => ()
	}
}

pub fn trace_to_entity_hitbox(ptrs: &GamePointers, viewangles: &QAngle) -> Option<(raw::C_BaseEntityPtr, i32)> {
	trace_to_entity(ptrs, viewangles, 0x4200400B)
}

pub fn trace_to_entity(ptrs: &GamePointers, viewangles: &QAngle, mask: u32) -> Option<(raw::C_BaseEntityPtr, i32)> {
	let me = get_local_player_entity(ptrs);
	let mut trace = unsafe { trace_t::new() };
	//let filter = sdk::create_tracefilter_from_predicate(should_hit_entity);


	let direction = viewangles.to_vector();
	
	let mut eyes = me.get_origin();
	
	unsafe {
		let eye_offsets: [f32, ..3] = *(me.ptr_offset(0xF8));
		eyes.x += (eye_offsets)[0];
		eyes.y += (eye_offsets)[1];
		eyes.z += (eye_offsets)[2];
	}
	let trace_direction = direction.scale( 8192.0f32 ) + eyes;
	
	let ray = Ray_t::new(&eyes, &trace_direction);
	
	ptrs.ienginetrace.trace_ray(&ray, mask, Some(get_tracefilter(me)), &mut trace);
	
	if trace.base.allsolid  {
		None
	} else if trace.ent.is_not_null() {
		Some((trace.ent, trace.hitbox ))
	} else {
		None
	}
}

pub fn is_commandnum_critical<WepType: sdk::CombatWeapon>(ptrs: &GamePointers, weapon: WepType, commandnum: i32) -> bool {
	let seed = unsafe { raw::calc_seed_from_command_number(commandnum) };
	
	unsafe {
		raw::is_shoot_critical(seed, weapon.get_ptr())
	}
}
/// Iterates through all entities.
pub struct EntityIterator {
	entlist: IClientEntityList,
	current_index: i32,
	stop_at: i32
}

impl EntityIterator {
	pub fn new(entlist: IClientEntityList) -> EntityIterator {
		let max_entindex = entlist.get_highest_entity_index();
		//log!("max entindex: {}\n", max_entindex);
		EntityIterator {
			entlist: entlist,
			current_index: 0,
			stop_at: max_entindex
		}
	}
}

impl Iterator<raw::C_BaseEntityPtr> for EntityIterator {
	fn next(&mut self) -> Option<raw::C_BaseEntityPtr> {
		while self.current_index <= self.stop_at { 
			let maybe_ent = self.entlist.get_client_entity(self.current_index);
			self.current_index += 1;

			match maybe_ent {
				Some(ent) => return Some(ent),
				None => continue
			}
		}
		// if we fell through here, we have reached the end
		// rest in peperonis
		None
	}
}

pub struct HitboxPositionIterator<EntType> {
	ent: EntType,
	modelinfo: sdk::IVModelInfo,
	current_hitbox: libc::c_int,
	num_hitboxes: libc::c_int
}
impl<EntType: Animating> HitboxPositionIterator<EntType> {
	pub fn new(ent: EntType, modelinfo: sdk::IVModelInfo) -> HitboxPositionIterator<EntType> {
		HitboxPositionIterator { ent: ent, modelinfo: modelinfo, current_hitbox: 0, num_hitboxes: ent.get_num_hitboxes(modelinfo) - 1 }
	}
}
impl<EntType: Animating> Iterator<sdk::Vector> for HitboxPositionIterator<EntType> {
	fn next(&mut self) -> Option<sdk::Vector> {
		if self.current_hitbox == self.num_hitboxes {
			None
		} else {
			let pos = self.ent.get_hitbox_position(self.modelinfo, self.current_hitbox);
			self.current_hitbox += 1;
			Some(pos)
		}
	}
}