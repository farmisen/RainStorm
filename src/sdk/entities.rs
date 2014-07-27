use super::raw;
use super::{Vector, IVModelInfo};
use core;

pub trait BaseEntity: core::kinds::Copy {
	unsafe fn from_ptr(ptr: raw::C_BaseEntityPtr) -> Self;
	fn get_ptr(&self) -> raw::C_BaseEntityPtr;
	
	fn get_origin(&self) -> Vector {
		unsafe { raw::c_baseentity_getorigin(self.get_ptr()) }
	}
	fn worldspacecenter(&self) -> Vector {
		unsafe { raw::c_baseentity_worldspacecenter(self.get_ptr()) }
	}
	fn get_index(&self) -> i32 {
		unsafe { raw::c_baseentity_getindex(self.get_ptr()) }
	}
	
	fn get_classname<'a>(&'a self) -> &'a str {
		unsafe {
			let cstr_classname = raw::c_baseentity_getclassname(self.get_ptr());
			// TODO: null check?
			core::str::raw::c_str_to_static_slice(cstr_classname)
		}
	}
	fn mut_ptr_offset<DataType>(&mut self, offset: uint) -> *mut DataType {
		(((self.get_ptr().to_uint()) + offset) as *mut DataType)
	}
	fn ptr_offset<DataType>(&self, offset: uint) -> *const DataType {
		(((self.get_ptr().to_uint()) + offset) as *const DataType)
	}
}

/*impl<EntityType: BaseEntity> core::cmp::PartialEq for EntityType {
	fn eq(&self, other: &EntityType) -> bool {
		self.get_index() == other.get_index()
	}
}*/

pub trait BaseAnimating: BaseEntity {
	fn get_hitbox_position(&self, modelinfo: IVModelInfo, hitbox: i32) -> Vector {
		unsafe { 
			let mut origin = core::mem::uninitialized();
			raw::c_baseanimating_gethitboxposition(self.get_ptr(), modelinfo.get_ptr(), hitbox, &mut origin);
			origin
		}
	}
	fn get_bone_position(&self, modelinfo: IVModelInfo, bone: i32) -> Vector {
		unsafe { 
			let mut origin = core::mem::uninitialized();
			raw::c_baseanimating_getboneposition(self.get_ptr(), modelinfo.get_ptr(), bone, &mut origin);
			origin
		}
	}
	fn get_num_bones(&self, modelinfo: IVModelInfo) -> i32 {
		unsafe {
			raw::c_baseanimating_getnumbones(self.get_ptr(), modelinfo.get_ptr())
		}
	}
	fn get_num_hitboxes(&self, modelinfo: IVModelInfo) -> i32 {
		unsafe {
			raw::c_baseanimating_getnumhitboxes(self.get_ptr(), modelinfo.get_ptr())
		}
	}
}

pub trait BaseCombatWeapon: BaseAnimating {
	fn is_melee(&self) -> bool;
}

// FIXME: this is weird
pub struct CombatWeapon {
	ptr: raw::C_BaseEntityPtr
}
impl BaseEntity for CombatWeapon {
	fn get_ptr(&self) -> raw::C_BaseEntityPtr {
		self.ptr
	}
	unsafe fn from_ptr(ptr: raw::C_BaseEntityPtr) -> CombatWeapon {
		CombatWeapon {ptr: ptr}
	}
}
impl BaseAnimating for CombatWeapon {}
impl BaseCombatWeapon for CombatWeapon {
	fn is_melee(&self) -> bool {
		true // FIXME
	}
}

pub struct TFPlayer {
	ptr: raw::C_BaseEntityPtr
}

impl TFPlayer {
	pub fn get_life_state(&self) -> i8 {
		unsafe { *(self.ptr_offset::<i8>(0x00A1)) }
	}
	pub fn get_team(&self) -> u32 {
		unsafe {*(self.ptr_offset(0x00AC))}
	}
	pub fn get_class(&self) -> u32 {
		unsafe {*(self.ptr_offset(0x1524))}
	}
}
impl BaseEntity for TFPlayer {
	fn get_ptr(&self) -> raw::C_BaseEntityPtr {
		self.ptr
	}
	unsafe fn from_ptr(ptr: raw::C_BaseEntityPtr) -> TFPlayer {
		TFPlayer {ptr: ptr}
	}
}
impl BaseAnimating for TFPlayer {}
