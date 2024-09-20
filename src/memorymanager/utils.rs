
use bitflags::bitflags;

bitflags!{
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Memoryallocationtype: u32{
        const MEM_COMMIT = 0x1000;
        const MEM_RESERVE = 0x00002000;
        const MEM_RESET = 0x00080000;
        const MEM_RESET_UNDO = 0x1000000;
        const MEM_LARGE_PAGES = 0x20000000;
        const MEM_PHYSICAL = 0x00400000;
        const MEM_TOP_DOWN = 0x00100000;
        const MEM_WRITE_WATCH = 0x00200000;
    }


}




bitflags!{
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Pageprotectiontype: u32{
        const PAGE_EXECUTE = 0x10;
        const PAGE_EXECUTE_READ = 0x20;
        const PAGE_EXECUTE_READWRITE = 0x40;
        const PAGE_EXECUTE_WRITECOPY = 0x80;
        const PAGE_NOACCESS = 0x1;
        const PAGE_READONLY = 0x2;
        const PAGE_READWRITE = 0x04;
        const PAGE_WRITECOPY = 0x8;
        const PAGE_TARGETS_INVALID = 0x40000000;
        const PAGE_GUARD = 0x100;
        const PAGE_NOCACHE = 0x200;
        const PAGE_WRITECOMBINE = 0x400;

    }

}


/*impl Pageprotectiontype{
    pub fn get_value(&self) ->u32{
        match *self{
            Pageprotectiontype::PAGE_EXECUTE => 0x10,
            Pageprotectiontype::PAGE_EXECUTE_READ => 0x20,
            Pageprotectiontype::PAGE_EXECUTE_READWRITE => 0x40,
            Pageprotectiontype::PAGE_EXECUTE_WRITECOPY => 0x80,
            Pageprotectiontype::PAGE_NOACCESS => 0x1,
            Pageprotectiontype::PAGE_READONLY => 0x2,
            Pageprotectiontype::PAGE_READWRITE => 0x04,
            Pageprotectiontype::PAGE_WRITECOPY => 0x8,
            Pageprotectiontype::PAGE_TARGETS_INVALID => 0x40000000,
            Pageprotectiontype::PAGE_GUARD => 0x100,
            Pageprotectiontype::PAGE_NOCACHE => 0x200,
            Pageprotectiontype::PAGE_WRITECOMBINE => 0x400,
        }
    }
}*/
