use core::prelude::*;
use Cheat;
use sdk;
use libc;
use CheatManager;
use GamePointers;
use rand::Rng;
use core::raw::Repr;

pub struct NameChanger {
	enabled: bool,
	rng: ::rand::isaac::IsaacRng
}

impl Cheat for NameChanger {
	fn new() -> NameChanger {
		NameChanger { enabled: false, rng: ::rand::isaac::IsaacRng::new_unseeded() }
	}
	fn get_name<'a>(&'a self) -> &'a str {
		"NameChanger"
	}
	fn postinit(&mut self, ptrs: &GamePointers) {
		let icvar = unsafe { (ptrs.icvar.to_option().unwrap()) };
		let namevar = icvar.find_var("name");
		match namevar {
			Some(name) => unsafe { (*name).changeandfreeze(::CString::new(::core::mem::transmute("le reddit army xD\0")).unwrap()); log!("name frozen OK :U\n") },
			None => {log!("No name CVar? u wot m8\n"); unsafe { libc::exit(1); }}
		}
	}
	fn process_usercmd(&mut self, ptrs: &GamePointers, cmd: &mut sdk::CUserCmd) {
		if !self.enabled {
			return;
		}
		let me: &mut sdk::C_BaseEntity = unsafe {
			let localplayer_entidx = ptrs.ivengineclient.to_option().unwrap().get_local_player();
			let local_baseentity = ptrs.icliententitylist.to_option().unwrap().get_client_entity(localplayer_entidx);
			if local_baseentity.is_not_null() {
				unsafe { ::core::mem::transmute(local_baseentity) }
			} else {
				log!("IClientEntity of local player (id: {}) not found!\n", localplayer_entidx); unsafe { libc::exit(1) }; 
			}
		};
		
		let icvar = unsafe { (ptrs.icvar.to_option().unwrap()) };
		let mut names: ::Vec<[u8, ..300]> = ::Vec::new();
		
		// TODO: some smart timer BS
		
		// FIXME: ugly string crappery
		::utils::map_all_players(ptrs.icliententitylist, |ptr| {
			let mut buf = [0u8, ..300];
			let len = unsafe { ptrs.ivengineclient.to_option().unwrap() }.get_player_name(unsafe {&*ptr}, buf.as_mut_slice());
			if len == 0 { return; }
			for (dst, src) in (buf.as_mut_slice().mut_slice_from(len as uint).mut_iter()).zip(b"\xe2\x80\x8b".iter()) {
				*dst = *src;
			}
			
			let str_name = ::core::str::from_utf8(buf.as_slice());
			//log!("player named {}\n", str_name);
			if unsafe { *((*ptr).ptr_offset::<u32>(0x00AC)) != *(me.ptr_offset::<u32>(0x00AC)) } {
				// different teams
				names.push(buf);
			}
		});
		
		
		let maybe_new_name = self.rng.choose(names.as_slice());
		match maybe_new_name {
			Some(new_name) => {
				let namevar = icvar.find_var("name");
				match namevar {
					Some(name) => unsafe { (*name).setvalue_raw(::sdk::Str(::CString::new_raw(new_name.as_slice().repr().data as *const u8))); log!("name changed OK :U\n") },
					None => {log!("No name CVar? u wot m8\n"); unsafe { libc::exit(1); }}
				}
			},
			None => {
				quit!("no name cvar?\n");
			}
		}
	}
	fn enable(&mut self) { self.enabled = true; }
	fn disable(&mut self) { self.enabled = false; }
}