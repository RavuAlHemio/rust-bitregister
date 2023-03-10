#![allow(non_camel_case_types)]
pub mod ep_descr {
    pub mod bank0 {
        #[repr(transparent)]
        pub struct ADDR {
            value: u32,
        }
        impl ADDR {
            #[inline]
            pub fn read<'a>(&'a self) -> ADDR_R<'a> {
                ADDR_R { register: self }
            }
            #[inline]
            pub fn write<'a>(&'a mut self) -> ADDR_W<'a> {
                self.value = Self::default().value;
                ADDR_W { register: self }
            }
        }
        impl Default for ADDR {
            fn default() -> Self {
                Self { value: 0 }
            }
        }
        pub struct ADDR_R<'a> {
            register: &'a ADDR,
        }
        impl<'a> ADDR_R<'a> {
            #[inline]
            pub fn addr(&self) -> u32 {
                ((self.register.value >> 0) & 4294967295) as u32
            }
        }
        pub struct ADDR_W<'a> {
            register: &'a mut ADDR,
        }
        impl<'a> ADDR_W<'a> {
            #[inline]
            pub fn addr(&mut self, value: u32) -> &mut Self {
                self.register.value = (self.register.value & (u32::MAX ^ 4294967295))
                    | ((value as u32) << 0) & 4294967295;
                self
            }
        }
        #[repr(transparent)]
        pub struct PCKSIZE {
            value: u32,
        }
        impl PCKSIZE {
            #[inline]
            pub fn read<'a>(&'a self) -> PCKSIZE_R<'a> {
                PCKSIZE_R { register: self }
            }
            #[inline]
            pub fn write<'a>(&'a mut self) -> PCKSIZE_W<'a> {
                self.value = Self::default().value;
                PCKSIZE_W { register: self }
            }
        }
        impl Default for PCKSIZE {
            fn default() -> Self {
                Self { value: 0 }
            }
        }
        pub struct PCKSIZE_R<'a> {
            register: &'a PCKSIZE,
        }
        impl<'a> PCKSIZE_R<'a> {
            #[inline]
            pub fn byte_count(&self) -> u16 {
                ((self.register.value >> 0) & 16383) as u16
            }
            #[inline]
            pub fn multi_packet_size(&self) -> u16 {
                ((self.register.value >> 14) & 16383) as u16
            }
            #[inline]
            pub fn size(&self) -> u8 {
                ((self.register.value >> 28) & 7) as u8
            }
            #[inline]
            pub fn auto_zlp(&self) -> bool {
                ((self.register.value >> 31) & 1) != 0
            }
        }
        pub struct PCKSIZE_W<'a> {
            register: &'a mut PCKSIZE,
        }
        impl<'a> PCKSIZE_W<'a> {
            #[inline]
            pub fn byte_count(&mut self, value: u16) -> &mut Self {
                self.register.value =
                    (self.register.value & (u32::MAX ^ 16383)) | ((value as u32) << 0) & 16383;
                self
            }
            #[inline]
            pub fn multi_packet_size(&mut self, value: u16) -> &mut Self {
                self.register.value = (self.register.value & (u32::MAX ^ 268419072))
                    | ((value as u32) << 14) & 268419072;
                self
            }
            #[inline]
            pub fn size(&mut self, value: u8) -> &mut Self {
                self.register.value = (self.register.value & (u32::MAX ^ 1879048192))
                    | ((value as u32) << 28) & 1879048192;
                self
            }
            #[inline]
            pub fn auto_zlp(&mut self, value: bool) -> &mut Self {
                self.register.value = (self.register.value & (u32::MAX ^ 2147483648))
                    | ((if value { 1 as u32 } else { 0 as u32 }) << 31) & 2147483648;
                self
            }
        }
        #[repr(transparent)]
        pub struct EXTREG {
            value: u32,
        }
        impl EXTREG {
            #[inline]
            pub fn read<'a>(&'a self) -> EXTREG_R<'a> {
                EXTREG_R { register: self }
            }
            #[inline]
            pub fn write<'a>(&'a mut self) -> EXTREG_W<'a> {
                self.value = Self::default().value;
                EXTREG_W { register: self }
            }
        }
        impl Default for EXTREG {
            fn default() -> Self {
                Self { value: 0 }
            }
        }
        pub struct EXTREG_R<'a> {
            register: &'a EXTREG,
        }
        impl<'a> EXTREG_R<'a> {
            #[inline]
            pub fn subpid(&self) -> u8 {
                ((self.register.value >> 0) & 15) as u8
            }
            #[inline]
            pub fn variable(&self) -> u16 {
                ((self.register.value >> 4) & 2047) as u16
            }
        }
        pub struct EXTREG_W<'a> {
            register: &'a mut EXTREG,
        }
        impl<'a> EXTREG_W<'a> {
            #[inline]
            pub fn subpid(&mut self, value: u8) -> &mut Self {
                self.register.value =
                    (self.register.value & (u32::MAX ^ 15)) | ((value as u32) << 0) & 15;
                self
            }
            #[inline]
            pub fn variable(&mut self, value: u16) -> &mut Self {
                self.register.value =
                    (self.register.value & (u32::MAX ^ 32752)) | ((value as u32) << 4) & 32752;
                self
            }
        }
        #[repr(transparent)]
        pub struct STATUS_BK {
            value: u8,
        }
        impl STATUS_BK {
            #[inline]
            pub fn read<'a>(&'a self) -> STATUS_BK_R<'a> {
                STATUS_BK_R { register: self }
            }
            #[inline]
            pub fn write<'a>(&'a mut self) -> STATUS_BK_W<'a> {
                self.value = Self::default().value;
                STATUS_BK_W { register: self }
            }
        }
        impl Default for STATUS_BK {
            fn default() -> Self {
                Self { value: 0 }
            }
        }
        pub struct STATUS_BK_R<'a> {
            register: &'a STATUS_BK,
        }
        impl<'a> STATUS_BK_R<'a> {
            #[inline]
            pub fn crcerr(&self) -> bool {
                ((self.register.value >> 0) & 1) != 0
            }
            #[inline]
            pub fn errorflow(&self) -> bool {
                ((self.register.value >> 0) & 1) != 0
            }
        }
        pub struct STATUS_BK_W<'a> {
            register: &'a mut STATUS_BK,
        }
        impl<'a> STATUS_BK_W<'a> {
            #[inline]
            pub fn crcerr(&mut self, value: bool) -> &mut Self {
                self.register.value = (self.register.value & (u8::MAX ^ 1))
                    | ((if value { 1 as u8 } else { 0 as u8 }) << 0) & 1;
                self
            }
            #[inline]
            pub fn errorflow(&mut self, value: bool) -> &mut Self {
                self.register.value = (self.register.value & (u8::MAX ^ 1))
                    | ((if value { 1 as u8 } else { 0 as u8 }) << 0) & 1;
                self
            }
        }
    }
    #[repr(packed)]
    pub struct BANK0 {
        pub addr: bank0::ADDR,
        pub pcksize: bank0::PCKSIZE,
        pub extreg: bank0::EXTREG,
        pub status_bk: bank0::STATUS_BK,
        _reserved0: [u8; 3usize],
    }
    pub mod bank1 {
        #[repr(transparent)]
        pub struct ADDR {
            value: u32,
        }
        impl ADDR {
            #[inline]
            pub fn read<'a>(&'a self) -> ADDR_R<'a> {
                ADDR_R { register: self }
            }
            #[inline]
            pub fn write<'a>(&'a mut self) -> ADDR_W<'a> {
                self.value = Self::default().value;
                ADDR_W { register: self }
            }
        }
        impl Default for ADDR {
            fn default() -> Self {
                Self { value: 0 }
            }
        }
        pub struct ADDR_R<'a> {
            register: &'a ADDR,
        }
        impl<'a> ADDR_R<'a> {
            #[inline]
            pub fn addr(&self) -> u32 {
                ((self.register.value >> 0) & 4294967295) as u32
            }
        }
        pub struct ADDR_W<'a> {
            register: &'a mut ADDR,
        }
        impl<'a> ADDR_W<'a> {
            #[inline]
            pub fn addr(&mut self, value: u32) -> &mut Self {
                self.register.value = (self.register.value & (u32::MAX ^ 4294967295))
                    | ((value as u32) << 0) & 4294967295;
                self
            }
        }
        #[repr(transparent)]
        pub struct PCKSIZE {
            value: u32,
        }
        impl PCKSIZE {
            #[inline]
            pub fn read<'a>(&'a self) -> PCKSIZE_R<'a> {
                PCKSIZE_R { register: self }
            }
            #[inline]
            pub fn write<'a>(&'a mut self) -> PCKSIZE_W<'a> {
                self.value = Self::default().value;
                PCKSIZE_W { register: self }
            }
        }
        impl Default for PCKSIZE {
            fn default() -> Self {
                Self { value: 0 }
            }
        }
        pub struct PCKSIZE_R<'a> {
            register: &'a PCKSIZE,
        }
        impl<'a> PCKSIZE_R<'a> {
            #[inline]
            pub fn byte_count(&self) -> u16 {
                ((self.register.value >> 0) & 16383) as u16
            }
            #[inline]
            pub fn multi_packet_size(&self) -> u16 {
                ((self.register.value >> 14) & 16383) as u16
            }
            #[inline]
            pub fn size(&self) -> u8 {
                ((self.register.value >> 28) & 7) as u8
            }
            #[inline]
            pub fn auto_zlp(&self) -> bool {
                ((self.register.value >> 31) & 1) != 0
            }
        }
        pub struct PCKSIZE_W<'a> {
            register: &'a mut PCKSIZE,
        }
        impl<'a> PCKSIZE_W<'a> {
            #[inline]
            pub fn byte_count(&mut self, value: u16) -> &mut Self {
                self.register.value =
                    (self.register.value & (u32::MAX ^ 16383)) | ((value as u32) << 0) & 16383;
                self
            }
            #[inline]
            pub fn multi_packet_size(&mut self, value: u16) -> &mut Self {
                self.register.value = (self.register.value & (u32::MAX ^ 268419072))
                    | ((value as u32) << 14) & 268419072;
                self
            }
            #[inline]
            pub fn size(&mut self, value: u8) -> &mut Self {
                self.register.value = (self.register.value & (u32::MAX ^ 1879048192))
                    | ((value as u32) << 28) & 1879048192;
                self
            }
            #[inline]
            pub fn auto_zlp(&mut self, value: bool) -> &mut Self {
                self.register.value = (self.register.value & (u32::MAX ^ 2147483648))
                    | ((if value { 1 as u32 } else { 0 as u32 }) << 31) & 2147483648;
                self
            }
        }
        #[repr(transparent)]
        pub struct STATUS_BK {
            value: u8,
        }
        impl STATUS_BK {
            #[inline]
            pub fn read<'a>(&'a self) -> STATUS_BK_R<'a> {
                STATUS_BK_R { register: self }
            }
            #[inline]
            pub fn write<'a>(&'a mut self) -> STATUS_BK_W<'a> {
                self.value = Self::default().value;
                STATUS_BK_W { register: self }
            }
        }
        impl Default for STATUS_BK {
            fn default() -> Self {
                Self { value: 0 }
            }
        }
        pub struct STATUS_BK_R<'a> {
            register: &'a STATUS_BK,
        }
        impl<'a> STATUS_BK_R<'a> {
            #[inline]
            pub fn crcerr(&self) -> bool {
                ((self.register.value >> 0) & 1) != 0
            }
            #[inline]
            pub fn errorflow(&self) -> bool {
                ((self.register.value >> 1) & 1) != 0
            }
        }
        pub struct STATUS_BK_W<'a> {
            register: &'a mut STATUS_BK,
        }
        impl<'a> STATUS_BK_W<'a> {
            #[inline]
            pub fn crcerr(&mut self, value: bool) -> &mut Self {
                self.register.value = (self.register.value & (u8::MAX ^ 1))
                    | ((if value { 1 as u8 } else { 0 as u8 }) << 0) & 1;
                self
            }
            #[inline]
            pub fn errorflow(&mut self, value: bool) -> &mut Self {
                self.register.value = (self.register.value & (u8::MAX ^ 2))
                    | ((if value { 1 as u8 } else { 0 as u8 }) << 1) & 2;
                self
            }
        }
    }
    #[repr(packed)]
    pub struct BANK1 {
        pub addr: bank1::ADDR,
        pub pcksize: bank1::PCKSIZE,
        _reserved0: [u8; 4usize],
        pub status_bk: bank1::STATUS_BK,
        _reserved1: [u8; 3usize],
    }
}
#[repr(packed)]
pub struct EP_DESCR {
    pub bank0: ep_descr::BANK0,
    pub bank1: ep_descr::BANK1,
}
