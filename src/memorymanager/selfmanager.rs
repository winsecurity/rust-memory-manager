
use crate::memorymanager::utils::{Memoryallocationtype, Pageprotectiontype};

use winapi::um::memoryapi::*;
use winapi::um::errhandlingapi::*;
use winapi::ctypes::*;

pub struct selfumm{
    address: usize,
    msize: usize,
    memoryallocationtype: Memoryallocationtype,
    currentpageprotection: Pageprotectiontype,
    oldprotection: Pageprotectiontype

}


impl selfumm{
    pub fn new(address: usize, msize: usize, mallocationtype:Memoryallocationtype, pageprotection:Pageprotectiontype) -> Result<Self,String>{

        let x:usize = if address==0 {0} else {address};



        let baseaddress =  unsafe{VirtualAlloc( x as *mut c_void,msize,
                                          mallocationtype.bits(),
                                          pageprotection.bits())};

        if baseaddress as usize == 0{
            Err(format!("VirtualAlloc failed, getlasterror: {}",unsafe{GetLastError()}))
        }
        else{
            Ok(selfumm{
                address: baseaddress as usize,
                msize,
                memoryallocationtype: mallocationtype,
                currentpageprotection: pageprotection,
                oldprotection: pageprotection.clone()
            })
        }



    }

    pub fn get_address(&self) -> usize{
        self.address
    }

    pub fn get_memorysize(&self) -> usize{
        self.msize
    }


}

impl Drop for selfumm{
    fn drop(&mut self){
        if self.address!=0 && self.msize!=0{
            println!("Freeing memory at: {:x?} of size: {}",self.address,self.msize);
            unsafe{VirtualFree(self.address as *mut c_void,0,0x00008000)};
        }
    }
}
