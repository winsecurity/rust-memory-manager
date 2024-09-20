mod memorymanager;

use memorymanager::selfmanager::selfumm;
use memorymanager::utils::*;

fn main() {

    let mem = selfumm::new(0,100,Memoryallocationtype::MEM_RESERVE|Memoryallocationtype::MEM_COMMIT,Pageprotectiontype::PAGE_EXECUTE_READWRITE);
    if mem.is_ok(){
        let mem = mem.unwrap();
        println!("Memory allocated at: {:x?} of size: {}",mem.get_address(),mem.get_memorysize());
    }

}
